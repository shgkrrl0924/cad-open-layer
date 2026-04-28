//! Adapter between Maket's floorplan model and our Layer 3 `Floorplan`.
//!
//! Used in the Stage 1 4-week PoC for "no-redraw" residential export validation.
//!
//! Three possible adapter paths (final shape pending Maket scope call):
//! - 10A: Maket exposes JSON / Protobuf model directly → 1:1 mapping
//! - 10B: Maket exports DXF v1 → we parse + extract semantic
//! - 10C: Maket shares partial spec → we write custom adapter
//!
//! See `docs/deep-dive.md` §10 for the speculation.
//!
//! # Status
//! Stub. Adapter shape locked at scope call (TBD).

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

use cad_core::error::Result;
use cad_semantic::Floorplan;
use serde::{Deserialize, Serialize};

/// Maket's external floorplan representation.
/// Field shapes are speculation pending scope call confirmation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaketPlan {
    pub units: String,
    #[serde(default)]
    pub walls: Vec<MaketWall>,
    #[serde(default)]
    pub openings: Vec<MaketOpening>,
    #[serde(default)]
    pub rooms: Vec<MaketRoom>,
    #[serde(default)]
    pub dimensions: Vec<MaketDimension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaketWall {
    pub id: String,
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub thickness: f64,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaketOpening {
    pub id: String,
    pub kind: String,
    pub host_wall: String,
    pub position: f64,
    pub width: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaketRoom {
    pub id: String,
    pub label: Option<String>,
    pub vertices: Vec<[f64; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaketDimension {
    pub id: String,
    pub kind: String,
    pub a: [f64; 2],
    pub b: [f64; 2],
    pub measured: f64,
    pub text_override: Option<String>,
}

/// Convert a Maket plan to our internal Floorplan.
pub fn maket_to_floorplan(_plan: &MaketPlan) -> Result<Floorplan> {
    // TODO: implement after scope call
    Ok(Floorplan::default())
}

/// Convert our internal Floorplan back to a Maket plan.
pub fn floorplan_to_maket(_plan: &Floorplan) -> Result<MaketPlan> {
    // TODO
    Ok(MaketPlan {
        units: "metric".to_string(),
        walls: vec![],
        openings: vec![],
        rooms: vec![],
        dimensions: vec![],
    })
}
