//! MCP server core. Exposes the CAD Open Layer pipeline as tools an AI
//! agent can call. Transport-agnostic — `main.rs` wires this to stdio.
//!
//! ## Protocol shape (JSON-RPC 2.0)
//!
//! - `initialize` → returns server capabilities + protocol version.
//! - `tools/list` → returns the tool catalogue.
//! - `tools/call` → dispatch to a named tool with `arguments`.
//!
//! ## Tools
//!
//! - `parse_dxf` — parse a DXF byte string (base64-encoded), return entity
//!   counts + parse_warnings.
//! - `extract_floorplan` — full Layer 1 → Layer 3 pipeline, return wire
//!   floorplan JSON.
//!
//! Stage-1 design intentionally takes the DXF bytes inline rather than via
//! a session/file-handle so the server is stateless. Larger flows can move
//! to a session model later.

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const PROTOCOL_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "cad-open-layer";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: &'static str,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

impl JsonRpcResponse {
    pub fn ok(id: Value, result: Value) -> Self {
        Self { jsonrpc: "2.0", id, result: Some(result), error: None }
    }
    pub fn err(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(JsonRpcError { code, message: message.into() }),
        }
    }
}

/// Dispatch a single JSON-RPC request to the appropriate handler. Pure
/// function — easy to unit-test without spinning up the stdio loop.
pub fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    let id = req.id.unwrap_or(Value::Null);
    match req.method.as_str() {
        "initialize" => JsonRpcResponse::ok(id, initialize_response()),
        "tools/list" => JsonRpcResponse::ok(id, tools_list_response()),
        "tools/call" => match call_tool(req.params) {
            Ok(v) => JsonRpcResponse::ok(id, v),
            Err(msg) => JsonRpcResponse::err(id, -32000, msg),
        },
        // MCP spec: notifications/initialized is fire-and-forget; respond
        // with an empty result so clients that wait don't hang.
        "notifications/initialized" => JsonRpcResponse::ok(id, Value::Null),
        other => JsonRpcResponse::err(id, -32601, format!("method not found: {other}")),
    }
}

fn initialize_response() -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "serverInfo": { "name": SERVER_NAME, "version": SERVER_VERSION },
        "capabilities": { "tools": {} }
    })
}

fn tools_list_response() -> Value {
    json!({
        "tools": [
            {
                "name": "parse_dxf",
                "description": "Parse a DXF file and return entity / block / warning counts. Useful as a sanity probe before full extraction.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "dxf_text": {
                            "type": "string",
                            "description": "Raw DXF file contents as ASCII text."
                        }
                    },
                    "required": ["dxf_text"]
                }
            },
            {
                "name": "extract_floorplan",
                "description": "Full pipeline: parse DXF + extract semantic floorplan (walls, openings, rooms). Returns a wire snapshot suitable for visualization or downstream agent reasoning.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "dxf_text": {
                            "type": "string",
                            "description": "Raw DXF file contents as ASCII text."
                        }
                    },
                    "required": ["dxf_text"]
                }
            }
        ]
    })
}

#[derive(Debug, Deserialize)]
struct ToolCallParams {
    name: String,
    #[serde(default)]
    arguments: Value,
}

#[derive(Debug, Deserialize)]
struct DxfArgs {
    dxf_text: String,
}

fn call_tool(params: Value) -> Result<Value, String> {
    let call: ToolCallParams =
        serde_json::from_value(params).map_err(|e| format!("invalid params: {e}"))?;
    let args: DxfArgs = serde_json::from_value(call.arguments)
        .map_err(|e| format!("invalid arguments: {e}"))?;

    match call.name.as_str() {
        "parse_dxf" => {
            let doc = cad_dxf_parser::parse_all(std::io::Cursor::new(args.dxf_text.as_bytes()))
                .map_err(|e| e.to_string())?;
            Ok(wrap_text_content(json!({
                "entities": doc.entities.len(),
                "blocks": doc.blocks.len(),
                "header_vars": doc.header.len(),
                "parse_warnings": doc.parse_warnings.len(),
            })))
        }
        "extract_floorplan" => {
            let wire = cad_wasm::extract_wire(args.dxf_text.as_bytes())?;
            Ok(wrap_text_content(serde_json::to_value(&wire).map_err(|e| e.to_string())?))
        }
        other => Err(format!("unknown tool: {other}")),
    }
}

/// MCP `tools/call` results are wrapped in a `content` array of typed
/// blocks. We always return one `text` block whose payload is the
/// JSON-stringified result.
fn wrap_text_content(payload: Value) -> Value {
    json!({
        "content": [
            { "type": "text", "text": payload.to_string() }
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_request(method: &str, params: Value) -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(1)),
            method: method.into(),
            params,
        }
    }

    #[test]
    fn initialize_returns_server_info() {
        let resp = handle_request(make_request("initialize", json!({})));
        let result = resp.result.unwrap();
        assert_eq!(result["serverInfo"]["name"], SERVER_NAME);
        assert_eq!(result["protocolVersion"], PROTOCOL_VERSION);
    }

    #[test]
    fn tools_list_includes_both_tools() {
        let resp = handle_request(make_request("tools/list", json!({})));
        let tools = resp.result.unwrap()["tools"].clone();
        let names: Vec<&str> = tools
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert!(names.contains(&"parse_dxf"));
        assert!(names.contains(&"extract_floorplan"));
    }

    #[test]
    fn unknown_method_returns_error() {
        let resp = handle_request(make_request("nope", json!({})));
        assert!(resp.result.is_none());
        assert_eq!(resp.error.unwrap().code, -32601);
    }

    #[test]
    fn parse_dxf_tool_invokes_layer_1() {
        const SMALL_DXF: &str = include_str!(
            "../../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf"
        );
        let resp = handle_request(make_request(
            "tools/call",
            json!({
                "name": "parse_dxf",
                "arguments": { "dxf_text": SMALL_DXF }
            }),
        ));
        let result = resp.result.unwrap();
        let text = result["content"][0]["text"].as_str().unwrap();
        let payload: Value = serde_json::from_str(text).unwrap();
        assert!(payload["entities"].as_u64().unwrap() > 0);
    }

    #[test]
    fn extract_floorplan_tool_invokes_full_pipeline() {
        const SMALL_DXF: &str = include_str!(
            "../../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf"
        );
        let resp = handle_request(make_request(
            "tools/call",
            json!({
                "name": "extract_floorplan",
                "arguments": { "dxf_text": SMALL_DXF }
            }),
        ));
        let result = resp.result.unwrap();
        let text = result["content"][0]["text"].as_str().unwrap();
        let payload: Value = serde_json::from_str(text).unwrap();
        assert!(payload["walls"].as_array().unwrap().len() >= 4);
    }
}
