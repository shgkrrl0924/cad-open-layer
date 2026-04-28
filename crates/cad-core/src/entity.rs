//! Layer 1 DXF entity types.

use crate::geom::Point;

pub type LayerName = String;

/// Top-level entity type. Output of Layer 1 DXF parser.
#[derive(Debug, Clone)]
pub enum Entity {
    Line {
        p1: Point,
        p2: Point,
        layer: LayerName,
        props: EntityProps,
    },
    Circle {
        center: Point,
        radius: f64,
        layer: LayerName,
        props: EntityProps,
    },
    Arc {
        center: Point,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        layer: LayerName,
        props: EntityProps,
    },
    LwPolyline {
        vertices: Vec<Point>,
        bulges: Vec<f64>,
        closed: bool,
        layer: LayerName,
        props: EntityProps,
    },
    Text {
        position: Point,
        value: String,
        height: f64,
        rotation: f64,
        layer: LayerName,
        props: EntityProps,
    },
    Insert {
        block_name: String,
        position: Point,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        rotation: f64,
        layer: LayerName,
        props: EntityProps,
    },
    MText {
        position: Point,
        value: String,
        height: f64,
        rotation: f64,
        attachment_point: i16,
        layer: LayerName,
        props: EntityProps,
    },
    Dimension {
        kind: DimensionKindRaw,
        block_name: Option<String>,
        text_override: Option<String>,
        measured_value: Option<f64>,
        defining_points: [Point; 5],
        rotation: f64,
        layer: LayerName,
        props: EntityProps,
    },
    /// Unknown / unsupported entity type. Preserved verbatim for round-trip.
    Raw(RawEntity),
}

/// DIMENSION group code 70 raw bit value. The lower 4 bits encode the
/// dimension type per DXF spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimensionKindRaw(pub u16);

impl DimensionKindRaw {
    pub fn primary_kind(self) -> u8 {
        (self.0 & 0x0F) as u8
    }
}

impl Entity {
    /// Return the layer this entity is on.
    #[must_use]
    pub fn layer(&self) -> &LayerName {
        match self {
            Self::Line { layer, .. }
            | Self::Circle { layer, .. }
            | Self::Arc { layer, .. }
            | Self::LwPolyline { layer, .. }
            | Self::Text { layer, .. }
            | Self::Insert { layer, .. }
            | Self::MText { layer, .. }
            | Self::Dimension { layer, .. } => layer,
            Self::Raw(r) => &r.layer,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityProps {
    pub handle: Option<String>,
    pub color: Option<i16>,
    pub linetype: Option<String>,
    /// XDATA (group codes 1001-1071) — application-specific data, must be preserved.
    pub xdata: Vec<(i32, String)>,
    /// Unknown group codes encountered during parsing — preserved for round-trip.
    pub raw_extras: Vec<(i32, String)>,
}

/// Unknown entity type. Preserves all raw group codes for round-trip emission.
#[derive(Debug, Clone)]
pub struct RawEntity {
    pub kind: String,
    pub layer: LayerName,
    pub groups: Vec<(i32, String)>,
}

/// BLOCK definition from the BLOCKS section of a DXF file.
#[derive(Debug, Clone)]
pub struct BlockDef {
    pub name: String,
    pub layer: LayerName,
    pub base_point: Point,
    pub flags: i64,
    pub entities: Vec<Entity>,
    pub props: EntityProps,
}
