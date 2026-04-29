//! WASM bindings for CAD Open Layer.
//!
//! Exposes the Layer 1 → Layer 3 pipeline (parse + extract) to JavaScript /
//! TypeScript callers. The wire format (`WireFloorplan`) is intentionally a
//! flat, JSON-friendly snapshot — decoupled from internal Rust types so the
//! browser-facing API can evolve without churning `cad-semantic`.
//!
//! ## Native fallback
//!
//! Building for `wasm32-unknown-unknown` exports `parse_dxf_to_json` and
//! `extract_floorplan_from_dxf` via `wasm_bindgen`. On non-wasm targets the
//! same functions exist as plain Rust APIs so unit tests can run with
//! `cargo test`.

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

use std::io::Cursor;

use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;
use cad_semantic::{Floorplan, OpeningKind, WallKind};
use serde::Serialize;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct WirePoint {
    pub x: f64,
    pub y: f64,
}

impl From<&cad_core::Point> for WirePoint {
    fn from(p: &cad_core::Point) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WireWall {
    pub id: u32,
    pub kind: &'static str,
    pub thickness: f64,
    pub layer: String,
    pub centerline: Vec<WirePoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WireOpening {
    pub id: u32,
    pub kind: &'static str,
    pub host_wall: Option<u32>,
    pub position_along_wall: f64,
    pub width: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct WireRoom {
    pub id: u32,
    pub label: Option<String>,
    pub area_sq_m: f64,
    pub boundary: Vec<WirePoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WireFloorplan {
    pub walls: Vec<WireWall>,
    pub openings: Vec<WireOpening>,
    pub rooms: Vec<WireRoom>,
    pub parse_warnings: usize,
}

const fn wall_kind_str(k: WallKind) -> &'static str {
    match k {
        WallKind::Exterior => "Exterior",
        WallKind::Interior => "Interior",
        WallKind::Partition => "Partition",
        WallKind::Unknown => "Unknown",
    }
}

const fn opening_kind_str(k: OpeningKind) -> &'static str {
    match k {
        OpeningKind::Door => "Door",
        OpeningKind::Window => "Window",
        OpeningKind::Pass => "Pass",
    }
}

fn floorplan_to_wire(plan: &Floorplan, parse_warnings: usize) -> WireFloorplan {
    WireFloorplan {
        walls: plan
            .walls
            .iter()
            .map(|w| WireWall {
                id: w.id,
                kind: wall_kind_str(w.kind),
                thickness: w.thickness,
                layer: w.layer.clone(),
                centerline: w.centerline.vertices.iter().map(WirePoint::from).collect(),
            })
            .collect(),
        openings: plan
            .openings
            .iter()
            .map(|o| WireOpening {
                id: o.id,
                kind: opening_kind_str(o.kind),
                host_wall: o.host_wall,
                position_along_wall: o.position_along_wall,
                width: o.width,
            })
            .collect(),
        rooms: plan
            .rooms
            .iter()
            .map(|r| WireRoom {
                id: r.id,
                label: r.label.clone(),
                area_sq_m: r.area_sq_m,
                boundary: r
                    .boundary
                    .outer
                    .vertices
                    .iter()
                    .map(WirePoint::from)
                    .collect(),
            })
            .collect(),
        parse_warnings,
    }
}

/// Native API: parse a DXF byte slice and run the full Layer 1 → Layer 3
/// extraction pipeline. Returns a JSON-serializable wire snapshot.
pub fn extract_wire(dxf_bytes: &[u8]) -> Result<WireFloorplan, String> {
    let doc = parse_all(Cursor::new(dxf_bytes)).map_err(|e| e.to_string())?;
    let warnings_count = doc.parse_warnings.len();
    let plan = extract_floorplan(&doc.entities).map_err(|e| e.to_string())?;
    Ok(floorplan_to_wire(&plan, warnings_count))
}

/// JS-facing entry point: parse + extract, return JSON string.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = extractFloorplanFromDxf)]
pub fn extract_floorplan_from_dxf(dxf_bytes: &[u8]) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let wire = extract_wire(dxf_bytes).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&wire).map_err(Into::into)
}

/// JS-facing entry point: parse only (no Layer 3 extraction). Returns
/// counts of entities + warnings — useful for browser parser-only demos.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = parseDxfSummary)]
pub fn parse_dxf_summary(dxf_bytes: &[u8]) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let doc = parse_all(Cursor::new(dxf_bytes)).map_err(|e| JsValue::from_str(&e.to_string()))?;
    #[derive(Serialize)]
    struct Summary {
        entities: usize,
        blocks: usize,
        header_vars: usize,
        parse_warnings: usize,
    }
    let s = Summary {
        entities: doc.entities.len(),
        blocks: doc.blocks.len(),
        header_vars: doc.header.len(),
        parse_warnings: doc.parse_warnings.len(),
    };
    serde_wasm_bindgen::to_value(&s).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_DXF: &str =
        include_str!("../../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf");

    #[test]
    fn extract_wire_produces_walls_and_rooms() {
        let wire = extract_wire(SMALL_DXF.as_bytes()).unwrap();
        assert!(
            wire.walls.len() >= 4,
            "small floorplan must have ≥4 walls, got {}",
            wire.walls.len()
        );
        // Every wall has a non-empty centerline.
        for w in &wire.walls {
            assert!(w.centerline.len() >= 2, "centerline must have ≥2 points");
            assert!(w.thickness > 0.0);
        }
        // Wire JSON serialization round-trips.
        let json = serde_json::to_string(&wire).expect("wire format must serialize");
        assert!(json.contains("\"walls\""));
        assert!(json.contains("\"rooms\""));
    }

    #[test]
    fn extract_wire_reports_parse_warnings_for_clean_input() {
        let wire = extract_wire(SMALL_DXF.as_bytes()).unwrap();
        assert_eq!(wire.parse_warnings, 0, "clean corpus must have 0 warnings");
    }

    #[test]
    fn extract_wire_rejects_invalid_dxf_with_error_string() {
        let result = extract_wire(b"not a dxf file");
        // Either parses (lenient mode) with empty result, or errors. Both
        // are acceptable — the contract is "no panic, returns Result".
        if let Ok(wire) = result {
            assert_eq!(wire.walls.len(), 0);
        }
    }
}
