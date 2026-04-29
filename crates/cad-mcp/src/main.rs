//! MCP stdio server entry point.
//!
//! Reads newline-delimited JSON-RPC requests from stdin and writes
//! responses to stdout. Errors / log lines go to stderr so the JSON
//! channel stays clean.
//!
//! Run via:
//! ```text
//! cargo run -p cad-mcp
//! ```
//!
//! Then connect with any MCP-compatible client (Claude Desktop, etc.) by
//! pointing at the resulting binary as a stdio transport.

use std::io::{self, BufRead, Write};

use cad_mcp::{handle_request, JsonRpcRequest, JsonRpcResponse};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut line = String::new();
    let mut reader = stdin.lock();

    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            // EOF — client closed transport.
            return Ok(());
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let resp = match serde_json::from_str::<JsonRpcRequest>(trimmed) {
            Ok(req) => handle_request(req),
            Err(e) => {
                JsonRpcResponse::err(serde_json::Value::Null, -32700, format!("parse error: {e}"))
            }
        };
        let bytes = serde_json::to_vec(&resp).unwrap_or_else(|e| {
            format!(r#"{{"jsonrpc":"2.0","error":{{"code":-32603,"message":"serialize: {e}"}}}}"#)
                .into_bytes()
        });
        out.write_all(&bytes)?;
        out.write_all(b"\n")?;
        out.flush()?;
    }
}
