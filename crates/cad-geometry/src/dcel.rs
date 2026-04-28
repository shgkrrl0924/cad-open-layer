//! Doubly-Connected Edge List (DCEL) for planar graph face extraction.
//!
//! Used by Algorithm #4 (Room Detection) in `docs/algorithms.md` §4.
//!
//! # Status
//! Stub. Full implementation in `docs/deep-dive.md` §3.

#![allow(missing_docs)]

use cad_core::{Point, Segment};
use slotmap::{new_key_type, SlotMap};

new_key_type! {
    pub struct VertexKey;
    pub struct HalfEdgeKey;
    pub struct FaceKey;
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Point,
    pub incident_edge: Option<HalfEdgeKey>,
}

#[derive(Debug, Clone)]
pub struct HalfEdge {
    pub origin: VertexKey,
    pub twin: HalfEdgeKey,
    pub next: Option<HalfEdgeKey>,
    pub prev: Option<HalfEdgeKey>,
    pub face: Option<FaceKey>,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub outer_edge: HalfEdgeKey,
    pub is_unbounded: bool,
}

pub struct Dcel {
    pub vertices: SlotMap<VertexKey, Vertex>,
    pub edges: SlotMap<HalfEdgeKey, HalfEdge>,
    pub faces: SlotMap<FaceKey, Face>,
}

impl Dcel {
    #[must_use]
    pub fn new() -> Self {
        Self {
            vertices: SlotMap::with_key(),
            edges: SlotMap::with_key(),
            faces: SlotMap::with_key(),
        }
    }

    /// Build a DCEL from a set of segments.
    /// Segments are split at their pairwise intersections (within `eps`).
    #[must_use]
    pub fn build_from_segments(_segments: &[Segment], _eps: f64) -> Self {
        // TODO: implement (deep-dive.md §3.2)
        Self::new()
    }
}

impl Default for Dcel {
    fn default() -> Self {
        Self::new()
    }
}
