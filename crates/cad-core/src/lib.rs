//! Common types, errors, units, and transforms for CAD Open Layer.
//!
//! This crate is the foundation. All other `cad-*` crates depend on it.
//!
//! See `docs/architecture.md` for the 3-layer design.

#![allow(missing_docs)] // Stage 1 stub — full docs added incrementally
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::redundant_pub_crate)]

pub mod entity;
pub mod error;
pub mod geom;
pub mod transform;
pub mod units;

pub use entity::{BlockDef, DimensionKindRaw, Entity, EntityProps, LayerName, RawEntity};
pub use error::{CadError, ParseWarning, Result};
pub use geom::{BoundingBox, Point, Segment, Vec2, Vec3};
pub use transform::Transform2D;
pub use units::DxfUnits;

/// Default tolerance for geometric matching, in meters (1mm).
pub const DEFAULT_EPSILON: f64 = 1e-3;
