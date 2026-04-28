//! Layer 3 → Layer 1 synthesis.
//!
//! Takes a `Floorplan` (semantic model) and emits a DXF file that opens
//! cleanly in AutoCAD with no architect redraw needed.
//!
//! See `docs/deep-dive.md` §5 for BLOCK TABLE, DIMSTYLE, and layer setup.
//!
//! # Status
//! Stub. Implementation in progress (Stage 1, Week 3).

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

pub mod blocks;
pub mod synthesis;

pub use blocks::{
    dimension_tick_block, nearest_door_block, nearest_window_block, standard_door_block,
    standard_library, standard_window_block, DEFAULT_WINDOW_DEPTH_MM,
    STANDARD_DOOR_WIDTHS_MM, STANDARD_WINDOW_WIDTHS_MM,
};
pub use synthesis::{floorplan_to_dxf, SynthesizeConfig};
