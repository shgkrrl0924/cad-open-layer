//! Entity builders: convert accumulated group-code pairs to `Entity`.
//!
//! The parser is intentionally lenient: malformed numeric values do not abort
//! parsing (real DXFs are messy). Instead, every parse failure pushes a
//! `ParseWarning` so the caller can detect silent corruption, addressing the
//! codex finding that bad coordinates would otherwise become valid-looking
//! origin geometry.

use cad_core::{DimensionKindRaw, Entity, EntityProps, LayerName, ParseWarning, Point, RawEntity};

use crate::reader::Pair;

/// Top-level dispatch from entity type string to typed builder.
pub(crate) fn build_entity(
    kind: &str,
    groups: &[Pair],
    warnings: &mut Vec<ParseWarning>,
) -> Entity {
    match kind {
        "LINE" => build_line(groups, warnings),
        "CIRCLE" => build_circle(groups, warnings),
        "ARC" => build_arc(groups, warnings),
        "LWPOLYLINE" => build_lwpolyline(groups, warnings),
        "TEXT" => build_text(groups, warnings),
        "MTEXT" => build_mtext(groups, warnings),
        "INSERT" => build_insert(groups, warnings),
        "DIMENSION" => build_dimension(groups, warnings),
        // Unknown entity types — preserve verbatim for round-trip.
        other => {
            let layer = extract_layer(groups);
            Entity::Raw(RawEntity {
                kind: other.to_string(),
                layer,
                groups: groups
                    .iter()
                    .map(|p| (p.code, p.value.clone()))
                    .collect(),
            })
        }
    }
}

fn build_line(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut p1 = Point::new(0.0, 0.0, 0.0);
    let mut p2 = Point::new(0.0, 0.0, 0.0);
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "LINE";

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            10 => p1.x = parse_f(&g.value, g.code, entity, warnings),
            20 => p1.y = parse_f(&g.value, g.code, entity, warnings),
            30 => p1.z = parse_f(&g.value, g.code, entity, warnings),
            11 => p2.x = parse_f(&g.value, g.code, entity, warnings),
            21 => p2.y = parse_f(&g.value, g.code, entity, warnings),
            31 => p2.z = parse_f(&g.value, g.code, entity, warnings),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {} // subclass marker
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    Entity::Line { p1, p2, layer, props }
}

fn build_circle(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut center = Point::new(0.0, 0.0, 0.0);
    let mut radius = 0.0;
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "CIRCLE";

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            10 => center.x = parse_f(&g.value, g.code, entity, warnings),
            20 => center.y = parse_f(&g.value, g.code, entity, warnings),
            30 => center.z = parse_f(&g.value, g.code, entity, warnings),
            40 => radius = parse_f(&g.value, g.code, entity, warnings),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    Entity::Circle { center, radius, layer, props }
}

fn build_arc(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut center = Point::new(0.0, 0.0, 0.0);
    let mut radius = 0.0;
    let mut start_angle = 0.0;
    let mut end_angle = 0.0;
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "ARC";

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            10 => center.x = parse_f(&g.value, g.code, entity, warnings),
            20 => center.y = parse_f(&g.value, g.code, entity, warnings),
            30 => center.z = parse_f(&g.value, g.code, entity, warnings),
            40 => radius = parse_f(&g.value, g.code, entity, warnings),
            50 => start_angle = parse_f(&g.value, g.code, entity, warnings).to_radians(),
            51 => end_angle = parse_f(&g.value, g.code, entity, warnings).to_radians(),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    Entity::Arc { center, radius, start_angle, end_angle, layer, props }
}

fn build_lwpolyline(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut vertices: Vec<Point> = vec![];
    let mut bulges: Vec<f64> = vec![];
    let mut closed = false;
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "LWPOLYLINE";

    let mut current_x: Option<f64> = None;
    let mut current_y: Option<f64> = None;
    let mut current_bulge: f64 = 0.0;

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            70 => {
                let flag = parse_i(&g.value, g.code, entity, warnings);
                closed = (flag & 1) != 0;
            }
            10 => {
                // New vertex starts. Push previous if any.
                if let (Some(x), Some(y)) = (current_x, current_y) {
                    vertices.push(Point::new(x, y, 0.0));
                    bulges.push(current_bulge);
                    current_bulge = 0.0;
                }
                current_x = Some(parse_f(&g.value, g.code, entity, warnings));
                current_y = None;
            }
            20 => current_y = Some(parse_f(&g.value, g.code, entity, warnings)),
            42 => current_bulge = parse_f(&g.value, g.code, entity, warnings),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    if let (Some(x), Some(y)) = (current_x, current_y) {
        vertices.push(Point::new(x, y, 0.0));
        bulges.push(current_bulge);
    }

    Entity::LwPolyline { vertices, bulges, closed, layer, props }
}

fn build_text(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut position = Point::new(0.0, 0.0, 0.0);
    let mut value = String::new();
    let mut height = 0.0;
    let mut rotation = 0.0;
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "TEXT";

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            10 => position.x = parse_f(&g.value, g.code, entity, warnings),
            20 => position.y = parse_f(&g.value, g.code, entity, warnings),
            30 => position.z = parse_f(&g.value, g.code, entity, warnings),
            40 => height = parse_f(&g.value, g.code, entity, warnings),
            1 => value = g.value.clone(),
            50 => rotation = parse_f(&g.value, g.code, entity, warnings).to_radians(),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    Entity::Text { position, value, height, rotation, layer, props }
}

fn build_insert(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut block_name = String::new();
    let mut position = Point::new(0.0, 0.0, 0.0);
    let mut scale_x = 1.0;
    let mut scale_y = 1.0;
    let mut scale_z = 1.0;
    let mut rotation = 0.0;
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "INSERT";

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            2 => block_name = g.value.clone(),
            10 => position.x = parse_f(&g.value, g.code, entity, warnings),
            20 => position.y = parse_f(&g.value, g.code, entity, warnings),
            30 => position.z = parse_f(&g.value, g.code, entity, warnings),
            41 => scale_x = parse_f(&g.value, g.code, entity, warnings),
            42 => scale_y = parse_f(&g.value, g.code, entity, warnings),
            43 => scale_z = parse_f(&g.value, g.code, entity, warnings),
            50 => rotation = parse_f(&g.value, g.code, entity, warnings).to_radians(),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    Entity::Insert {
        block_name,
        position,
        scale_x,
        scale_y,
        scale_z,
        rotation,
        layer,
        props,
    }
}

fn build_mtext(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut position = Point::new(0.0, 0.0, 0.0);
    let mut value_chunks: Vec<String> = vec![];
    let mut height = 0.0;
    let mut rotation = 0.0;
    let mut attachment_point: i16 = 1;
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "MTEXT";

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            10 => position.x = parse_f(&g.value, g.code, entity, warnings),
            20 => position.y = parse_f(&g.value, g.code, entity, warnings),
            30 => position.z = parse_f(&g.value, g.code, entity, warnings),
            40 => height = parse_f(&g.value, g.code, entity, warnings),
            1 | 3 => value_chunks.push(g.value.clone()),
            50 => rotation = parse_f(&g.value, g.code, entity, warnings).to_radians(),
            71 => attachment_point = parse_i(&g.value, g.code, entity, warnings) as i16,
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    Entity::MText {
        position,
        value: value_chunks.join(""),
        height,
        rotation,
        attachment_point,
        layer,
        props,
    }
}

fn build_dimension(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> Entity {
    let mut block_name: Option<String> = None;
    let mut text_override: Option<String> = None;
    let mut measured_value: Option<f64> = None;
    let mut points = [
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
    ];
    let mut kind_raw: u16 = 0;
    let mut rotation = 0.0;
    let mut layer: LayerName = "0".into();
    let mut props = EntityProps::default();
    let entity = "DIMENSION";

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            2 => block_name = Some(g.value.clone()),
            1 => text_override = Some(g.value.clone()),
            42 => measured_value = Some(parse_f(&g.value, g.code, entity, warnings)),
            70 => kind_raw = parse_i(&g.value, g.code, entity, warnings) as u16,
            50 => rotation = parse_f(&g.value, g.code, entity, warnings).to_radians(),
            10 => points[0].x = parse_f(&g.value, g.code, entity, warnings),
            20 => points[0].y = parse_f(&g.value, g.code, entity, warnings),
            30 => points[0].z = parse_f(&g.value, g.code, entity, warnings),
            11 => points[1].x = parse_f(&g.value, g.code, entity, warnings),
            21 => points[1].y = parse_f(&g.value, g.code, entity, warnings),
            31 => points[1].z = parse_f(&g.value, g.code, entity, warnings),
            12 => points[2].x = parse_f(&g.value, g.code, entity, warnings),
            22 => points[2].y = parse_f(&g.value, g.code, entity, warnings),
            32 => points[2].z = parse_f(&g.value, g.code, entity, warnings),
            13 => points[3].x = parse_f(&g.value, g.code, entity, warnings),
            23 => points[3].y = parse_f(&g.value, g.code, entity, warnings),
            33 => points[3].z = parse_f(&g.value, g.code, entity, warnings),
            14 => points[4].x = parse_f(&g.value, g.code, entity, warnings),
            24 => points[4].y = parse_f(&g.value, g.code, entity, warnings),
            34 => points[4].z = parse_f(&g.value, g.code, entity, warnings),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(parse_i(&g.value, g.code, entity, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    Entity::Dimension {
        kind: DimensionKindRaw(kind_raw),
        block_name,
        text_override,
        measured_value,
        defining_points: points,
        rotation,
        layer,
        props,
    }
}

fn extract_layer(groups: &[Pair]) -> LayerName {
    groups
        .iter()
        .find(|p| p.code == 8)
        .map_or_else(|| "0".into(), |p| p.value.clone())
}

/// Lenient float parse. On failure, returns 0.0 AND records a warning so
/// callers can detect silent corruption (codex medium finding).
fn parse_f(s: &str, code: i32, entity: &'static str, warnings: &mut Vec<ParseWarning>) -> f64 {
    match s.trim().parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            warnings.push(ParseWarning {
                code,
                value: s.to_string(),
                kind: "f64",
                entity,
            });
            0.0
        }
    }
}

/// Lenient int parse. On failure, returns 0 AND records a warning.
fn parse_i(s: &str, code: i32, entity: &'static str, warnings: &mut Vec<ParseWarning>) -> i64 {
    match s.trim().parse::<i64>() {
        Ok(v) => v,
        Err(_) => {
            warnings.push(ParseWarning {
                code,
                value: s.to_string(),
                kind: "i64",
                entity,
            });
            0
        }
    }
}
