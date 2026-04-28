//! Layer 2 → Layer 3 extraction algorithms.
//!
//! Implements the 5 core algorithms from `docs/algorithms.md`:
//! 1. Wall extraction (parallel-line-pair detection)
//! 2. Opening detection (block-to-wall projection)
//! 3. Dimension chain reconstruction
//! 4. Room detection (DCEL face extraction)
//! 5. Grid extraction
//!
//! Plus the `WallExtractor` trait abstraction for multi-style support
//! (parallel-line-pair, polyline-outline, hatch-boundary).
//!
//! # Status
//! Stub. Implementation in progress (Stage 1, Week 1-2).

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

pub mod dimension;
pub mod grid;
pub mod opening;
pub mod probe;
pub mod room;
pub mod wall;

use cad_core::error::Result;
use cad_core::Entity;
use cad_semantic::{Floorplan, Wall};

/// Trait for extracting walls from a set of DXF entities.
/// Multiple implementations support different wall-drawing conventions.
pub trait WallExtractor {
    fn extract(&self, entities: &[Entity]) -> Result<Vec<Wall>>;
}

/// Top-level convenience: run all 5 extraction algorithms on a flat entity
/// list and return a fully-populated [`Floorplan`].
///
/// Order matters: walls first (other algorithms depend on them), then the
/// rest can run in parallel order. Default configs are used; for fine
/// control, call each algorithm directly.
pub fn extract_floorplan(entities: &[Entity]) -> Result<Floorplan> {
    let mut walls = wall::ParallelLinePairExtractor::default().extract(entities)?;
    wall::classify_walls(&mut walls, &wall::HierarchyConfig::default());
    let openings = opening::detect_openings(entities, &walls, &opening::OpeningConfig::default())?;
    let rooms = room::detect_rooms(&walls, entities, &room::RoomConfig::default())?;
    let dimensions = dimension::reconstruct_dimensions(
        entities,
        &walls,
        &dimension::DimensionConfig::default(),
    )?;
    let grid_opt = grid::extract_grid(entities, &grid::GridConfig::default())?;
    let grids = grid_opt.map(|g| vec![g]).unwrap_or_default();

    Ok(Floorplan { walls, openings, rooms, dimensions, grids })
}
