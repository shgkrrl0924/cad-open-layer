//! Convention probe — detects which wall-drawing style a DXF uses.
//!
//! See `docs/considerations.md` §C.1 and `docs/deep-dive.md` §8.

#![allow(missing_docs)]

use cad_core::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallStyle {
    ParallelLinePair,
    PolylineOutline,
    HatchBoundary,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerNamingConvention {
    English,
    Korean,
    Mixed,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ConventionProbe {
    pub wall_style: WallStyle,
    pub layer_naming: LayerNamingConvention,
}

pub fn probe(_entities: &[Entity]) -> ConventionProbe {
    // TODO: count Line vs LwPolyline vs Hatch on wall layers, detect style.
    // Heuristic thresholds in deep-dive.md §8.
    ConventionProbe {
        wall_style: WallStyle::Unknown,
        layer_naming: LayerNamingConvention::Unknown,
    }
}
