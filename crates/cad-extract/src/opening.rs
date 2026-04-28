//! Opening (door / window) detection — Algorithm #2 from `docs/algorithms.md`.
//!
//! Two detection strategies depending on DXF convention:
//!
//! 1. **Door = LINE + ARC pair** (synthetic corpus + many AutoCAD plans):
//!    one LINE is the open leaf, one ARC is the swing trail. Hinge = ARC
//!    center = LINE endpoint. Door width = ARC radius = LINE length.
//!
//! 2. **Door = INSERT block reference** (modern Revit / library-based plans):
//!    the block geometry contains arc + leaf, scaled and rotated by the
//!    INSERT. Block-name parsing yields width hint.
//!
//! 3. **Window = parallel LINE pair** offset by ~80mm (synthetic corpus):
//!    similar geometry to a wall pair but on WINDOWS layer with a thinner
//!    thickness range.
//!
//! 4. **Window = INSERT block reference** (alternate convention).
//!
//! For Stage 1 PoC the synthetic corpus uses conventions 1 and 3.

use cad_core::error::Result;
use cad_core::{Entity, LayerName, Point};
use cad_semantic::{DoorSwing, Opening, OpeningId, OpeningKind, Side, Wall, WallId};

#[derive(Debug, Clone)]
pub struct OpeningConfig {
    pub door_layers: Vec<LayerName>,
    pub window_layers: Vec<LayerName>,
    pub default_door_width: f64,
    pub default_window_width: f64,
    /// Maximum distance between an opening's hinge/center and the host wall's
    /// centerline (perpendicular).
    pub max_offset_from_wall: f64,
    /// Endpoint coincidence tolerance (mm).
    pub endpoint_eps: f64,
    /// Window pair: minimum and maximum thickness (depth into wall).
    pub window_depth_min: f64,
    pub window_depth_max: f64,
}

impl Default for OpeningConfig {
    fn default() -> Self {
        Self {
            door_layers: vec![
                "DOORS".into(),
                "DOOR".into(),
                "A-DOOR".into(),
                "문".into(),
            ],
            window_layers: vec![
                "WINDOWS".into(),
                "WINDOW".into(),
                "A-GLAZ".into(),
                "창".into(),
                "창문".into(),
            ],
            default_door_width: 800.0,
            default_window_width: 1200.0,
            max_offset_from_wall: 600.0,
            endpoint_eps: 5.0,
            window_depth_min: 30.0,
            window_depth_max: 200.0,
        }
    }
}

/// Detect all openings (doors + windows) in `entities`, linking each to a
/// host wall from `walls` when possible.
pub fn detect_openings(
    entities: &[Entity],
    walls: &[Wall],
    config: &OpeningConfig,
) -> Result<Vec<Opening>> {
    let mut openings: Vec<Opening> = vec![];
    let mut next_id: OpeningId = 1;

    let doors = detect_doors_line_arc(entities, walls, config, &mut next_id);
    openings.extend(doors);

    let windows = detect_windows_parallel_pair(entities, walls, config, &mut next_id);
    openings.extend(windows);

    let inserts = detect_openings_from_inserts(entities, walls, config, &mut next_id);
    openings.extend(inserts);

    Ok(openings)
}

/// Recognize doors/windows that are emitted as INSERT block references
/// (the convention used by the synthesis pipeline). Block names follow the
/// `DOOR_<width>` or `WINDOW_<width>` pattern.
fn detect_openings_from_inserts(
    entities: &[Entity],
    walls: &[Wall],
    config: &OpeningConfig,
    next_id: &mut OpeningId,
) -> Vec<Opening> {
    let mut result = vec![];

    for e in entities {
        if let Entity::Insert {
            block_name,
            position,
            scale_x,
            rotation,
            layer,
            ..
        } = e
        {
            let kind = match classify_block(block_name, layer, config) {
                Some(k) => k,
                None => continue,
            };

            let standard_width = parse_block_width(block_name);
            if standard_width <= 0.0 {
                continue;
            }
            let actual_width = standard_width * scale_x.abs();

            let host = find_host_wall(position, walls, config.max_offset_from_wall);
            let position_along = host.map_or(0.0, |w| position_along_wall(position, w));

            let swing = match kind {
                OpeningKind::Door => Some(DoorSwing {
                    hinge_side: if rotation.sin() >= 0.0 { Side::Left } else { Side::Right },
                    opens_inward: true,
                }),
                _ => None,
            };

            result.push(Opening {
                id: *next_id,
                kind,
                host_wall: host.map(|w| w.id),
                position_along_wall: position_along,
                width: actual_width,
                height: None,
                swing,
            });
            *next_id += 1;
        }
    }

    result
}

fn classify_block(
    block_name: &str,
    layer: &LayerName,
    config: &OpeningConfig,
) -> Option<OpeningKind> {
    let upper = block_name.to_uppercase();
    if upper.starts_with("DOOR") || upper.contains("DOOR_") {
        return Some(OpeningKind::Door);
    }
    if upper.starts_with("WINDOW") || upper.starts_with("WIN_") || upper.contains("WINDOW_") {
        return Some(OpeningKind::Window);
    }
    // Fall back to layer name when block naming is non-conventional.
    if config.door_layers.iter().any(|l| l == layer) {
        return Some(OpeningKind::Door);
    }
    if config.window_layers.iter().any(|l| l == layer) {
        return Some(OpeningKind::Window);
    }
    None
}

fn parse_block_width(block_name: &str) -> f64 {
    let suffix: String = block_name
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let suffix: String = suffix.chars().rev().collect();
    suffix.parse::<f64>().unwrap_or(0.0)
}

fn detect_doors_line_arc(
    entities: &[Entity],
    walls: &[Wall],
    config: &OpeningConfig,
    next_id: &mut OpeningId,
) -> Vec<Opening> {
    let mut result: Vec<Opening> = vec![];

    // Collect arcs and lines on door layers.
    struct ArcView {
        center: Point,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    }
    struct LineView {
        a: Point,
        b: Point,
    }

    let mut arcs: Vec<ArcView> = vec![];
    let mut lines: Vec<LineView> = vec![];

    for e in entities {
        match e {
            Entity::Arc { center, radius, start_angle, end_angle, layer, .. }
                if config.door_layers.iter().any(|l| l == layer) =>
            {
                arcs.push(ArcView {
                    center: *center,
                    radius: *radius,
                    start_angle: *start_angle,
                    end_angle: *end_angle,
                });
            }
            Entity::Line { p1, p2, layer, .. }
                if config.door_layers.iter().any(|l| l == layer) =>
            {
                lines.push(LineView { a: *p1, b: *p2 });
            }
            _ => {}
        }
    }

    // For each arc, find the matching leaf line whose endpoint == arc center
    // (within ε). The arc center is the hinge.
    for arc in &arcs {
        let leaf = lines.iter().find(|l| {
            l.a.approx_eq(&arc.center, config.endpoint_eps)
                || l.b.approx_eq(&arc.center, config.endpoint_eps)
        });

        let width = leaf.map_or(arc.radius, |l| l.a.distance(&l.b));

        let host = find_host_wall(&arc.center, walls, config.max_offset_from_wall);
        let position = host.map_or(0.0, |w| position_along_wall(&arc.center, w));

        // Swing: based on arc start angle direction.
        let swing = Some(DoorSwing {
            hinge_side: if arc.start_angle.sin() >= 0.0 { Side::Left } else { Side::Right },
            opens_inward: arc.end_angle > arc.start_angle,
        });

        result.push(Opening {
            id: *next_id,
            kind: OpeningKind::Door,
            host_wall: host.map(|w| w.id),
            position_along_wall: position,
            width,
            height: None,
            swing,
        });
        *next_id += 1;
    }

    result
}

fn detect_windows_parallel_pair(
    entities: &[Entity],
    walls: &[Wall],
    config: &OpeningConfig,
    next_id: &mut OpeningId,
) -> Vec<Opening> {
    let mut result: Vec<Opening> = vec![];

    // Collect window-layer lines.
    let mut window_lines: Vec<(Point, Point)> = entities
        .iter()
        .filter_map(|e| match e {
            Entity::Line { p1, p2, layer, .. }
                if config.window_layers.iter().any(|l| l == layer) =>
            {
                Some((*p1, *p2))
            }
            _ => None,
        })
        .collect();

    // Pair detection (same idea as wall pair, but window-specific depth range).
    let mut used = vec![false; window_lines.len()];
    let n = window_lines.len();

    for i in 0..n {
        if used[i] {
            continue;
        }
        for j in (i + 1)..n {
            if used[j] {
                continue;
            }
            let (a1, a2) = window_lines[i];
            let (b1, b2) = window_lines[j];
            if let Some((center, width)) = check_window_pair(a1, a2, b1, b2, config) {
                let host =
                    find_host_wall(&center, walls, config.max_offset_from_wall);
                let position = host.map_or(0.0, |w| position_along_wall(&center, w));
                result.push(Opening {
                    id: *next_id,
                    kind: OpeningKind::Window,
                    host_wall: host.map(|w| w.id),
                    position_along_wall: position,
                    width,
                    height: None,
                    swing: None,
                });
                *next_id += 1;
                used[i] = true;
                used[j] = true;
                break;
            }
        }
    }

    // Make borrow checker happy.
    let _ = &mut window_lines;

    result
}

fn check_window_pair(
    a1: Point,
    a2: Point,
    b1: Point,
    b2: Point,
    config: &OpeningConfig,
) -> Option<(Point, f64)> {
    // Lines must be roughly parallel (axis-aligned in synthetic corpus, so we
    // check by direction vector approximate equality via length ratio).
    let dir_a_x = a2.x - a1.x;
    let dir_a_y = a2.y - a1.y;
    let dir_b_x = b2.x - b1.x;
    let dir_b_y = b2.y - b1.y;
    let len_a = dir_a_x.hypot(dir_a_y);
    let len_b = dir_b_x.hypot(dir_b_y);
    if len_a < 100.0 || len_b < 100.0 {
        return None;
    }
    // Direction-agnostic parallel check via cross product magnitude.
    let cross = dir_a_x * dir_b_y - dir_a_y * dir_b_x;
    let cross_normalized = cross.abs() / (len_a * len_b);
    if cross_normalized > 0.05 {
        // ~3° off
        return None;
    }

    // Length similarity (window pair lines are equal length).
    let length_ratio = (len_a / len_b).max(len_b / len_a);
    if length_ratio > 1.05 {
        return None;
    }

    // Perpendicular distance between the two lines.
    let dist = perp_dist_to_line(a1, b1, b2);
    if dist < config.window_depth_min || dist > config.window_depth_max {
        return None;
    }

    // Center of the pair (midpoint of midpoints).
    let mid_a = Point::new((a1.x + a2.x) / 2.0, (a1.y + a2.y) / 2.0, 0.0);
    let mid_b = Point::new((b1.x + b2.x) / 2.0, (b1.y + b2.y) / 2.0, 0.0);
    let center = Point::new((mid_a.x + mid_b.x) / 2.0, (mid_a.y + mid_b.y) / 2.0, 0.0);

    Some((center, len_a))
}

fn perp_dist_to_line(p: Point, line_a: Point, line_b: Point) -> f64 {
    let ab_x = line_b.x - line_a.x;
    let ab_y = line_b.y - line_a.y;
    let len = ab_x.hypot(ab_y);
    if len < 1e-9 {
        return p.distance(&line_a);
    }
    let cross = (p.x - line_a.x) * ab_y - (p.y - line_a.y) * ab_x;
    cross.abs() / len
}

/// Find the wall whose centerline passes closest to `point`, within
/// `max_offset`. Returns None if none qualify.
fn find_host_wall<'a>(
    point: &Point,
    walls: &'a [Wall],
    max_offset: f64,
) -> Option<&'a Wall> {
    let mut best: Option<(&Wall, f64)> = None;
    for w in walls {
        let dist = wall_centerline_distance(point, w);
        if dist <= max_offset && best.map_or(true, |(_, d)| dist < d) {
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

/// Distance along the wall centerline from start to the projection of `point`.
fn position_along_wall(point: &Point, wall: &Wall) -> f64 {
    let v = &wall.centerline.vertices;
    if v.len() < 2 {
        return 0.0;
    }
    // Walk along centerline; find segment with minimum perpendicular distance,
    // then accumulate distance up to projected point.
    let mut accumulated = 0.0;
    let mut best_dist = f64::INFINITY;
    let mut best_position = 0.0;

    for i in 0..v.len() - 1 {
        let a = v[i];
        let b = v[i + 1];
        let ab_x = b.x - a.x;
        let ab_y = b.y - a.y;
        let len_sq = ab_x.mul_add(ab_x, ab_y * ab_y);
        if len_sq < 1e-12 {
            continue;
        }
        let t = ((point.x - a.x) * ab_x + (point.y - a.y) * ab_y) / len_sq;
        let t_clamped = t.clamp(0.0, 1.0);
        let proj = Point::new(a.x + ab_x * t_clamped, a.y + ab_y * t_clamped, 0.0);
        let dist = point.distance(&proj);
        if dist < best_dist {
            best_dist = dist;
            let segment_len = len_sq.sqrt();
            best_position = accumulated + t_clamped * segment_len;
        }
        accumulated += len_sq.sqrt();
    }

    let _ = best_dist;
    let _ = WallId::default();
    best_position
}

#[cfg(test)]
mod tests {
    use super::*;
    use cad_core::EntityProps;

    fn arc(center: Point, radius: f64, start_deg: f64, end_deg: f64, layer: &str) -> Entity {
        Entity::Arc {
            center,
            radius,
            start_angle: start_deg.to_radians(),
            end_angle: end_deg.to_radians(),
            layer: layer.into(),
            props: EntityProps::default(),
        }
    }
    fn line(a: Point, b: Point, layer: &str) -> Entity {
        Entity::Line { p1: a, p2: b, layer: layer.into(), props: EntityProps::default() }
    }

    #[test]
    fn detects_door_from_line_plus_arc_pair() {
        let entities = vec![
            line(Point::new(4000.0, 1200.0, 0.0), Point::new(4800.0, 1200.0, 0.0), "DOORS"),
            arc(Point::new(4000.0, 1200.0, 0.0), 800.0, 0.0, 90.0, "DOORS"),
        ];
        let walls = vec![]; // no host walls — should still detect the door (with host=None)
        let openings = detect_openings(&entities, &walls, &OpeningConfig::default()).unwrap();
        assert_eq!(openings.len(), 1);
        assert_eq!(openings[0].kind, OpeningKind::Door);
        assert!((openings[0].width - 800.0).abs() < 1e-6);
        assert!(openings[0].host_wall.is_none());
    }

    #[test]
    fn detects_window_from_parallel_pair() {
        let entities = vec![
            line(Point::new(1000.0, 6500.0, 0.0), Point::new(3000.0, 6500.0, 0.0), "WINDOWS"),
            line(Point::new(1000.0, 6580.0, 0.0), Point::new(3000.0, 6580.0, 0.0), "WINDOWS"),
        ];
        let walls = vec![];
        let openings = detect_openings(&entities, &walls, &OpeningConfig::default()).unwrap();
        assert_eq!(openings.len(), 1);
        assert_eq!(openings[0].kind, OpeningKind::Window);
        assert!((openings[0].width - 2000.0).abs() < 1.0);
    }
}
