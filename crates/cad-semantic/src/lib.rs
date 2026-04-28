//! Architectural Semantic Layer for CAD drawings.
//!
//! Layer 3 of CAD Open Layer. The user-facing data model representing
//! architectural intent (walls, openings, rooms, dimensions) rather than
//! raw DXF entities.
//!
//! See `docs/algorithms.md` and `docs/deep-dive.md` §1 (round-trip equivalence).

#![allow(missing_docs)]

pub mod equivalence;

use cad_core::{LayerName, Point};
use cad_geometry::polygon::Polygon;
use cad_geometry::polyline::Polyline;

pub type WallId = u32;
pub type OpeningId = u32;
pub type RoomId = u32;
pub type DimensionId = u32;
pub type GridId = u32;

/// Top-level container for an architectural floorplan.
#[derive(Debug, Clone, Default)]
pub struct Floorplan {
    pub walls: Vec<Wall>,
    pub openings: Vec<Opening>,
    pub rooms: Vec<Room>,
    pub dimensions: Vec<Dimension>,
    pub grids: Vec<Grid>,
}

#[derive(Debug, Clone)]
pub struct Wall {
    pub id: WallId,
    pub centerline: Polyline,
    pub thickness: f64,
    pub height: Option<f64>,
    pub layer: LayerName,
    pub kind: WallKind,
    pub openings: Vec<OpeningId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallKind {
    Exterior,
    Interior,
    Partition,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Opening {
    pub id: OpeningId,
    pub kind: OpeningKind,
    pub host_wall: Option<WallId>,
    pub position_along_wall: f64,
    pub width: f64,
    pub height: Option<f64>,
    pub swing: Option<DoorSwing>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpeningKind {
    Door,
    Window,
    Pass,
}

#[derive(Debug, Clone, Copy)]
pub struct DoorSwing {
    pub hinge_side: Side,
    pub opens_inward: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: RoomId,
    pub boundary: Polygon,
    pub label: Option<String>,
    pub area_sq_m: f64,
    pub bounding_walls: Vec<WallId>,
}

#[derive(Debug, Clone)]
pub struct Dimension {
    pub id: DimensionId,
    pub kind: DimensionKind,
    pub origin: Point,
    pub target: Point,
    pub measured_value: f64,
    pub text_override: Option<String>,
    pub linked_to: Option<DimensionTarget>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionKind {
    Linear,
    Aligned,
    Angular,
    Diameter,
    Radius,
}

#[derive(Debug, Clone)]
pub enum DimensionTarget {
    WallLength(WallId),
    OpeningWidth(OpeningId),
    RoomDimension { room: RoomId, axis: Axis },
    Custom { points: Vec<Point> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Diagonal,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub id: GridId,
    pub x_axes: Vec<GridLine>,
    pub y_axes: Vec<GridLine>,
}

#[derive(Debug, Clone)]
pub struct GridLine {
    /// Position in the perpendicular direction (X for vertical lines / horizontal coordinate
    /// when looking at axis layout, Y for horizontal lines).
    pub position: f64,
    /// Label text from a nearby TEXT entity ("A", "B", "1", "12", etc.).
    pub label: Option<String>,
    /// Endpoint A of the original DXF LINE (preserved for round-trip).
    pub start: Point,
    /// Endpoint B.
    pub end: Point,
}
