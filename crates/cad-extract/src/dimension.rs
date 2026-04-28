//! Dimension reconstruction — Algorithm #3.
//!
//! Two source patterns are recognized:
//!
//! 1. **DIMENSION entity** (`Entity::Dimension`): native DXF dimension. Group
//!    code 70 = type, 13/14 = extension line origins, 1 = text override,
//!    42 = computed measurement. We parse this directly.
//!
//! 2. **LINE + TEXT pattern** (the convention used by the synthetic small
//!    floorplan corpus): dimension lines and extension lines are drawn as
//!    `LINE` entities on the DIMENSIONS layer, with a `TEXT` entity carrying
//!    the measured value. We detect each TEXT on a dimension layer, parse
//!    its value as a number, and link it to the wall whose length matches.

use cad_core::error::Result;
use cad_core::{DimensionKindRaw, Entity, LayerName, Point};
use cad_semantic::{
    Axis, Dimension, DimensionId, DimensionKind, DimensionTarget, Wall, WallId,
};

#[derive(Debug, Clone)]
pub struct DimensionConfig {
    pub layers: Vec<LayerName>,
    /// Acceptable mismatch between the text value and a wall length, in mm
    /// (or whatever the file's source units are — caller normalizes).
    pub length_match_eps: f64,
    /// When matching a TEXT to a wall by spatial proximity, the maximum
    /// distance to consider.
    pub spatial_match_max: f64,
}

impl Default for DimensionConfig {
    fn default() -> Self {
        Self {
            layers: vec![
                "DIMENSIONS".into(),
                "DIMENSION".into(),
                "A-DIMS".into(),
                "치수".into(),
            ],
            length_match_eps: 1.0,    // 1mm tolerance
            spatial_match_max: 1500.0, // 1.5m
        }
    }
}

pub fn reconstruct_dimensions(
    entities: &[Entity],
    walls: &[Wall],
    config: &DimensionConfig,
) -> Result<Vec<Dimension>> {
    let mut result: Vec<Dimension> = vec![];
    let mut next_id: DimensionId = 1;

    // Pattern 1: native DIMENSION entities.
    for e in entities {
        if let Entity::Dimension {
            kind,
            text_override,
            measured_value,
            defining_points,
            layer,
            ..
        } = e
        {
            if !config.layers.iter().any(|l| l == layer) {
                continue;
            }
            let measured = measured_value.unwrap_or_else(|| {
                // Compute from defining points (13/14 = extension origins)
                defining_points[3].distance(&defining_points[4])
            });
            let target = link_dimension_target(
                &defining_points[3],
                &defining_points[4],
                walls,
                config,
            );
            result.push(Dimension {
                id: next_id,
                kind: dim_kind_from_raw(*kind),
                origin: defining_points[3],
                target: defining_points[4],
                measured_value: measured,
                text_override: text_override.clone(),
                linked_to: target,
            });
            next_id += 1;
        }
    }

    // Pattern 2: TEXT-on-dimension-layer with numeric value.
    for e in entities {
        if let Entity::Text { position, value, layer, .. } = e {
            if !config.layers.iter().any(|l| l == layer) {
                continue;
            }
            let measured = match parse_dimension_text(value) {
                Some(v) => v,
                None => continue,
            };
            let host_wall = find_wall_by_length_and_proximity(
                walls,
                measured,
                position,
                config,
            );
            let (origin, target, target_link) = match host_wall {
                Some(w) => {
                    let v = &w.centerline.vertices;
                    if v.len() >= 2 {
                        let first = v[0];
                        let last = v[v.len() - 1];
                        (first, last, Some(DimensionTarget::WallLength(w.id)))
                    } else {
                        (*position, *position, None)
                    }
                }
                None => (*position, *position, None),
            };
            result.push(Dimension {
                id: next_id,
                kind: DimensionKind::Linear,
                origin,
                target,
                measured_value: measured,
                text_override: Some(value.clone()),
                linked_to: target_link,
            });
            next_id += 1;
        }
    }

    Ok(result)
}

fn dim_kind_from_raw(raw: DimensionKindRaw) -> DimensionKind {
    match raw.primary_kind() {
        0 => DimensionKind::Linear,
        1 => DimensionKind::Aligned,
        2 => DimensionKind::Angular,
        3 => DimensionKind::Diameter,
        4 => DimensionKind::Radius,
        _ => DimensionKind::Linear,
    }
}

fn parse_dimension_text(s: &str) -> Option<f64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Strip simple unit suffixes the user might embed.
    let cleaned: String = trimmed
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();
    cleaned.parse::<f64>().ok().filter(|v| *v > 0.0)
}

fn find_wall_by_length_and_proximity<'a>(
    walls: &'a [Wall],
    measured: f64,
    text_pos: &Point,
    config: &DimensionConfig,
) -> Option<&'a Wall> {
    // Candidates: walls whose centerline length is close to `measured`.
    let candidates: Vec<&Wall> = walls
        .iter()
        .filter(|w| {
            let len = w.centerline.length();
            (len - measured).abs() < config.length_match_eps
        })
        .collect();

    if candidates.is_empty() {
        return None;
    }

    // Pick by nearest perpendicular distance to text_pos.
    let mut best: Option<(&Wall, f64)> = None;
    for w in candidates {
        let dist = wall_centerline_distance(text_pos, w);
        if dist <= config.spatial_match_max
            && best.map_or(true, |(_, d)| dist < d)
        {
            best = Some((w, dist));
        }
    }
    best.map(|(w, _)| w)
}

fn wall_centerline_distance(point: &Point, wall: &Wall) -> f64 {
    let v = &wall.centerline.vertices;
    if v.len() < 2 {
        return f64::INFINITY;
    }
    let mut min_dist = f64::INFINITY;
    for i in 0..v.len() - 1 {
        let d = perp_dist_to_segment(*point, v[i], v[i + 1]);
        if d < min_dist {
            min_dist = d;
        }
    }
    min_dist
}

fn perp_dist_to_segment(p: Point, a: Point, b: Point) -> f64 {
    let ab_x = b.x - a.x;
    let ab_y = b.y - a.y;
    let len_sq = ab_x.mul_add(ab_x, ab_y * ab_y);
    if len_sq < 1e-12 {
        return p.distance(&a);
    }
    let t = ((p.x - a.x) * ab_x + (p.y - a.y) * ab_y) / len_sq;
    let t_clamped = t.clamp(0.0, 1.0);
    let proj = Point::new(a.x + ab_x * t_clamped, a.y + ab_y * t_clamped, 0.0);
    p.distance(&proj)
}

/// For a Pattern-1 DIMENSION entity, decide what's being measured.
fn link_dimension_target(
    origin: &Point,
    target: &Point,
    walls: &[Wall],
    config: &DimensionConfig,
) -> Option<DimensionTarget> {
    // Check 1: do origin/target match endpoints of the same wall?
    for w in walls {
        let v = &w.centerline.vertices;
        if v.len() < 2 {
            continue;
        }
        let a = v[0];
        let b = v[v.len() - 1];
        if (origin.approx_eq(&a, config.length_match_eps)
            && target.approx_eq(&b, config.length_match_eps))
            || (origin.approx_eq(&b, config.length_match_eps)
                && target.approx_eq(&a, config.length_match_eps))
        {
            return Some(DimensionTarget::WallLength(w.id));
        }
    }

    // Check 2: same-axis room dimension (origin and target same X or Y).
    if (origin.x - target.x).abs() < 1.0 {
        return Some(DimensionTarget::Custom { points: vec![*origin, *target] });
    }

    Some(DimensionTarget::Custom { points: vec![*origin, *target] })
}

#[allow(dead_code)]
fn axis_of(origin: &Point, target: &Point) -> Axis {
    let dx = (origin.x - target.x).abs();
    let dy = (origin.y - target.y).abs();
    if dx < dy * 0.1 {
        Axis::Y
    } else if dy < dx * 0.1 {
        Axis::X
    } else {
        Axis::Diagonal
    }
}

#[allow(dead_code)]
fn _ensure_compile(_: WallId) {}

#[cfg(test)]
mod tests {
    use super::*;
    use cad_core::EntityProps;
    use cad_geometry::polyline::Polyline;
    use cad_semantic::WallKind;

    fn wall(id: u32, a: Point, b: Point) -> Wall {
        Wall {
            id,
            centerline: Polyline::new(vec![a, b], false),
            thickness: 200.0,
            height: None,
            layer: "WALLS".into(),
            kind: WallKind::Unknown,
            openings: vec![],
        }
    }

    #[test]
    fn parses_numeric_text_value() {
        assert_eq!(parse_dimension_text("10000"), Some(10000.0));
        assert_eq!(parse_dimension_text("6500"), Some(6500.0));
        assert_eq!(parse_dimension_text("123.45"), Some(123.45));
        assert_eq!(parse_dimension_text("hello"), None);
        assert_eq!(parse_dimension_text(""), None);
        assert_eq!(parse_dimension_text("0"), None);
    }

    #[test]
    fn links_text_to_wall_by_length_and_proximity() {
        let walls = vec![
            wall(1, Point::new(0.0, 100.0, 0.0), Point::new(10000.0, 100.0, 0.0)),
            wall(2, Point::new(9900.0, 0.0, 0.0), Point::new(9900.0, 6500.0, 0.0)),
        ];
        let entities = vec![Entity::Text {
            position: Point::new(5000.0, 420.0, 0.0),
            value: "10000".into(),
            height: 200.0,
            rotation: 0.0,
            layer: "DIMENSIONS".into(),
            props: EntityProps::default(),
        }];
        let dims = reconstruct_dimensions(&entities, &walls, &DimensionConfig::default()).unwrap();
        assert_eq!(dims.len(), 1);
        assert_eq!(dims[0].measured_value, 10000.0);
        match &dims[0].linked_to {
            Some(DimensionTarget::WallLength(id)) => assert_eq!(*id, 1),
            other => panic!("expected WallLength(1), got {other:?}"),
        }
    }
}
