//! Room detection — Algorithm #4 from `docs/algorithms.md` and §3 of
//! `docs/deep-dive.md`.
//!
//! Pipeline:
//! 1. Extend wall endpoints to nearest perpendicular wall (close broken
//!    connectivity within `max_extension` meters).
//! 2. Find all pairwise wall-segment intersections.
//! 3. Split segments at intersections to get atomic edges.
//! 4. Build a planar graph: vertex set + half-edge adjacency.
//! 5. Sort outgoing edges at each vertex by angle, set `next` pointers.
//! 6. Trace cycles (faces). Bounded faces = rooms.
//! 7. Filter by `min_area`, match TEXT labels to face polygons, build `Room`.

use std::collections::HashMap;

use cad_core::error::Result;
use cad_core::{Entity, Point};
use cad_geometry::polygon::{point_in_polygon, signed_area, Polygon};
use cad_geometry::polyline::Polyline;
use cad_semantic::{Room, RoomId, Wall, WallId};

#[derive(Debug, Clone)]
pub struct RoomConfig {
    /// Maximum distance to extend a wall endpoint to reach a perpendicular wall.
    pub max_extension: f64,
    /// Minimum face area to be promoted to a Room (drops slivers / artifacts).
    pub min_area_sq_m: f64,
    /// Snap precision for vertex coincidence (mm).
    pub snap_grid: f64,
}

impl Default for RoomConfig {
    fn default() -> Self {
        Self {
            max_extension: 250.0, // mm
            min_area_sq_m: 1.0,
            snap_grid: 1.0, // 1mm
        }
    }
}

pub fn detect_rooms(walls: &[Wall], texts: &[Entity], config: &RoomConfig) -> Result<Vec<Room>> {
    // Step 1: extend wall endpoints
    let extended = extend_endpoints(walls, config.max_extension);

    // Step 2: collect segments + assign source wall ID
    let mut segments: Vec<(Point, Point, WallId)> = vec![];
    for ew in &extended {
        let v = &ew.centerline;
        for i in 0..v.len() - 1 {
            segments.push((v[i], v[i + 1], ew.id));
        }
    }

    // Step 3: find pairwise intersections + split
    let atomic = split_at_intersections(&segments, config.snap_grid);

    // Step 4-6: build graph + extract bounded faces
    let face_polygons = extract_bounded_faces(&atomic, config.snap_grid);

    // Step 7: rooms with labels + areas
    let mut rooms: Vec<Room> = vec![];
    let mut next_id: RoomId = 1;
    for face in face_polygons {
        let poly = Polygon::new(Polyline::new(face.vertices.clone(), true));
        let area_mm2 = poly.area();
        let area_sq_m = area_mm2 / 1_000_000.0;
        if area_sq_m < config.min_area_sq_m {
            continue;
        }
        let label = find_label_inside(texts, &poly.outer.vertices);
        rooms.push(Room {
            id: next_id,
            boundary: poly,
            label,
            area_sq_m,
            bounding_walls: face.bounding_walls,
        });
        next_id += 1;
    }

    Ok(rooms)
}

#[derive(Debug, Clone)]
struct ExtendedWall {
    id: WallId,
    centerline: Vec<Point>,
}

#[derive(Debug, Clone)]
struct AtomicSegment {
    a: Point,
    b: Point,
    wall_id: WallId,
}

#[derive(Debug, Clone)]
struct FaceBoundary {
    vertices: Vec<Point>,
    bounding_walls: Vec<WallId>,
}

fn extend_endpoints(walls: &[Wall], max_extension: f64) -> Vec<ExtendedWall> {
    let mut result = Vec::with_capacity(walls.len());
    for w in walls {
        let v = &w.centerline.vertices;
        if v.len() < 2 {
            continue;
        }
        let start = v[0];
        let end = v[v.len() - 1];

        let new_start = extend_one_endpoint(start, end, walls, w.id, max_extension);
        let new_end = extend_one_endpoint(end, start, walls, w.id, max_extension);

        result.push(ExtendedWall {
            id: w.id,
            centerline: vec![new_start, new_end],
        });
    }
    result
}

/// Extend `endpoint` (with `other_end` defining the wall direction) by up to
/// `max_extension` to reach the nearest other wall. Returns the (possibly
/// extended) endpoint coordinate.
fn extend_one_endpoint(
    endpoint: Point,
    _other_end: Point,
    walls: &[Wall],
    self_id: WallId,
    max_extension: f64,
) -> Point {
    let mut best: Option<(f64, Point)> = None;
    for other in walls {
        if other.id == self_id {
            continue;
        }
        let v = &other.centerline.vertices;
        if v.len() < 2 {
            continue;
        }
        let oa = v[0];
        let ob = v[v.len() - 1];

        let intersection = infinite_line_intersection(endpoint, _other_end, oa, ob);
        if let Some(p) = intersection {
            let dist = endpoint.distance(&p);
            if dist <= max_extension && best.is_none_or(|(d, _)| dist < d) {
                best = Some((dist, p));
            }
        }
    }
    best.map_or(endpoint, |(_, p)| p)
}

/// Compute the intersection of two infinite lines, each defined by two
/// distinct points. Returns None if parallel.
fn infinite_line_intersection(a1: Point, a2: Point, b1: Point, b2: Point) -> Option<Point> {
    let dx1 = a2.x - a1.x;
    let dy1 = a2.y - a1.y;
    let dx2 = b2.x - b1.x;
    let dy2 = b2.y - b1.y;
    let denom = dx1.mul_add(dy2, -(dy1 * dx2));
    if denom.abs() < 1e-9 {
        return None;
    }
    let t = ((b1.x - a1.x).mul_add(dy2, -((b1.y - a1.y) * dx2))) / denom;
    Some(Point::new(dx1.mul_add(t, a1.x), dy1.mul_add(t, a1.y), 0.0))
}

fn split_at_intersections(segments: &[(Point, Point, WallId)], snap: f64) -> Vec<AtomicSegment> {
    // For each segment, collect intersection points with all other segments.
    let n = segments.len();
    let mut breakpoints: Vec<Vec<Point>> = (0..n).map(|_| vec![]).collect();

    for i in 0..n {
        for j in (i + 1)..n {
            let (a1, a2, _) = segments[i];
            let (b1, b2, _) = segments[j];
            if let Some(p) = segment_intersection(a1, a2, b1, b2) {
                breakpoints[i].push(p);
                breakpoints[j].push(p);
            }
        }
    }

    // For each segment, sort breakpoints by parameter along segment, then
    // split into sub-segments.
    let mut atomic: Vec<AtomicSegment> = vec![];
    for i in 0..n {
        let (a, b, wid) = segments[i];
        let mut points: Vec<Point> = breakpoints[i].clone();
        points.push(a);
        points.push(b);

        // Sort by t-parameter along segment (a → b).
        points.sort_by(|p, q| {
            let tp = param_along(*p, a, b);
            let tq = param_along(*q, a, b);
            tp.partial_cmp(&tq).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Snap + dedupe.
        let snapped: Vec<Point> = points.into_iter().map(|p| snap_point(p, snap)).collect();
        let mut dedup: Vec<Point> = vec![];
        for p in snapped {
            if dedup
                .last()
                .is_none_or(|q: &Point| !p.approx_eq(q, snap * 0.5))
            {
                dedup.push(p);
            }
        }

        for k in 0..dedup.len().saturating_sub(1) {
            atomic.push(AtomicSegment {
                a: dedup[k],
                b: dedup[k + 1],
                wall_id: wid,
            });
        }
    }
    atomic
}

fn segment_intersection(a1: Point, a2: Point, b1: Point, b2: Point) -> Option<Point> {
    let dx1 = a2.x - a1.x;
    let dy1 = a2.y - a1.y;
    let dx2 = b2.x - b1.x;
    let dy2 = b2.y - b1.y;
    let denom = dx1.mul_add(dy2, -(dy1 * dx2));
    if denom.abs() < 1e-9 {
        return None;
    }
    let t = (b1.x - a1.x).mul_add(dy2, -((b1.y - a1.y) * dx2)) / denom;
    let s = (b1.x - a1.x).mul_add(dy1, -((b1.y - a1.y) * dx1)) / denom;
    if t > -1e-6 && t < 1.0 + 1e-6 && s > -1e-6 && s < 1.0 + 1e-6 {
        return Some(Point::new(dx1.mul_add(t, a1.x), dy1.mul_add(t, a1.y), 0.0));
    }
    None
}

fn param_along(p: Point, a: Point, b: Point) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len_sq = dx.mul_add(dx, dy * dy);
    if len_sq < 1e-12 {
        return 0.0;
    }
    (p.x - a.x).mul_add(dx, (p.y - a.y) * dy) / len_sq
}

fn snap_point(p: Point, grid: f64) -> Point {
    Point::new(
        (p.x / grid).round() * grid,
        (p.y / grid).round() * grid,
        (p.z / grid).round() * grid,
    )
}

#[derive(Debug, Clone, Copy)]
struct HalfEdge {
    origin: usize,
    target: usize,
    twin: usize,
    next: Option<usize>,
    wall_id: WallId,
}

fn extract_bounded_faces(atomic: &[AtomicSegment], snap: f64) -> Vec<FaceBoundary> {
    if atomic.is_empty() {
        return vec![];
    }

    // Build vertex set with snap-based deduplication.
    let mut vertex_pos: Vec<Point> = vec![];
    let mut vertex_idx: HashMap<(i64, i64), usize> = HashMap::new();

    let key = |p: Point| {
        let s = (1.0 / snap).max(1.0);
        ((p.x * s).round() as i64, (p.y * s).round() as i64)
    };

    let intern =
        |p: Point, vertex_pos: &mut Vec<Point>, vertex_idx: &mut HashMap<(i64, i64), usize>| {
            let k = key(p);
            if let Some(&idx) = vertex_idx.get(&k) {
                idx
            } else {
                let idx = vertex_pos.len();
                vertex_pos.push(p);
                vertex_idx.insert(k, idx);
                idx
            }
        };

    let mut edges: Vec<HalfEdge> = vec![];
    for seg in atomic {
        let i = intern(seg.a, &mut vertex_pos, &mut vertex_idx);
        let j = intern(seg.b, &mut vertex_pos, &mut vertex_idx);
        if i == j {
            continue;
        }
        let e_ij = edges.len();
        let e_ji = e_ij + 1;
        edges.push(HalfEdge {
            origin: i,
            target: j,
            twin: e_ji,
            next: None,
            wall_id: seg.wall_id,
        });
        edges.push(HalfEdge {
            origin: j,
            target: i,
            twin: e_ij,
            next: None,
            wall_id: seg.wall_id,
        });
    }

    // Group outgoing edges per origin vertex.
    let n_verts = vertex_pos.len();
    let mut outgoing: Vec<Vec<usize>> = (0..n_verts).map(|_| vec![]).collect();
    for (ei, e) in edges.iter().enumerate() {
        outgoing[e.origin].push(ei);
    }

    // Sort outgoing edges by angle.
    for v in 0..n_verts {
        let center = vertex_pos[v];
        outgoing[v].sort_by(|&a, &b| {
            let ta = vertex_pos[edges[a].target];
            let tb = vertex_pos[edges[b].target];
            let aa = (ta.y - center.y).atan2(ta.x - center.x);
            let ab = (tb.y - center.y).atan2(tb.x - center.x);
            aa.partial_cmp(&ab).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    // Set `next` pointers using the DCEL formula:
    //   for adjacent outgoing pair (e_i, e_{i+1}):
    //     edges[e_i.twin].next = e_{i+1}
    for outs in &outgoing {
        let k = outs.len();
        if k == 0 {
            continue;
        }
        for idx in 0..k {
            let cur = outs[idx];
            let nxt = outs[(idx + k - 1) % k]; // CW neighbor → next of twin
            let twin = edges[cur].twin;
            edges[twin].next = Some(nxt);
        }
    }

    // Trace cycles.
    let mut visited = vec![false; edges.len()];
    let mut faces: Vec<FaceBoundary> = vec![];

    for start in 0..edges.len() {
        if visited[start] {
            continue;
        }
        let mut cycle: Vec<usize> = vec![];
        let mut cur = start;
        let mut steps = 0;
        loop {
            if visited[cur] {
                break;
            }
            visited[cur] = true;
            cycle.push(cur);
            cur = match edges[cur].next {
                Some(n) => n,
                None => break,
            };
            steps += 1;
            if steps > 100_000 {
                break;
            }
            if cur == start {
                break;
            }
        }

        if cycle.len() < 3 {
            continue;
        }

        // Compute signed area: positive = CCW (bounded), negative = CW (unbounded).
        let pts: Vec<Point> = cycle.iter().map(|&e| vertex_pos[edges[e].origin]).collect();
        let sa = signed_area(&pts);
        if sa > 0.0 {
            // Bounded face: collect bounding walls (unique).
            let mut bounding: Vec<WallId> = cycle.iter().map(|&e| edges[e].wall_id).collect();
            bounding.sort_unstable();
            bounding.dedup();
            faces.push(FaceBoundary {
                vertices: pts,
                bounding_walls: bounding,
            });
        }
    }

    faces
}

fn find_label_inside(texts: &[Entity], polygon: &[Point]) -> Option<String> {
    for t in texts {
        let (pos, value, layer) = match t {
            Entity::Text {
                position,
                value,
                layer,
                ..
            } if layer == "TEXT" || layer == "TEXTS" || layer == "ROOMS" => {
                (*position, value.clone(), layer.clone())
            }
            Entity::MText {
                position,
                value,
                layer,
                ..
            } if layer == "TEXT" || layer == "TEXTS" || layer == "ROOMS" => {
                (*position, value.clone(), layer.clone())
            }
            _ => continue,
        };
        if point_in_polygon(&pos, polygon) {
            let _ = layer;
            return Some(value);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use cad_core::EntityProps;
    use cad_semantic::WallKind;

    fn rect_walls() -> Vec<Wall> {
        vec![
            wall(
                1,
                vec![Point::new(0.0, 0.0, 0.0), Point::new(10000.0, 0.0, 0.0)],
                200.0,
            ),
            wall(
                2,
                vec![
                    Point::new(10000.0, 0.0, 0.0),
                    Point::new(10000.0, 6000.0, 0.0),
                ],
                200.0,
            ),
            wall(
                3,
                vec![
                    Point::new(10000.0, 6000.0, 0.0),
                    Point::new(0.0, 6000.0, 0.0),
                ],
                200.0,
            ),
            wall(
                4,
                vec![Point::new(0.0, 6000.0, 0.0), Point::new(0.0, 0.0, 0.0)],
                200.0,
            ),
        ]
    }

    fn wall(id: u32, points: Vec<Point>, thickness: f64) -> Wall {
        Wall {
            id,
            centerline: Polyline::new(points, false),
            thickness,
            height: None,
            layer: "WALLS".into(),
            kind: WallKind::Unknown,
            openings: vec![],
        }
    }

    #[test]
    fn detects_single_rectangular_room() {
        let walls = rect_walls();
        let rooms = detect_rooms(&walls, &[], &RoomConfig::default()).unwrap();
        assert_eq!(rooms.len(), 1, "single rectangle should yield 1 room");
        let r = &rooms[0];
        assert!(
            (r.area_sq_m - 60.0).abs() < 0.5,
            "10m × 6m = 60m², got {}",
            r.area_sq_m
        );
    }

    #[test]
    fn extends_endpoint_to_nearby_perpendicular_wall() {
        // Two walls that ALMOST meet (100mm gap).
        let walls = vec![
            wall(
                1,
                vec![Point::new(0.0, 100.0, 0.0), Point::new(10000.0, 100.0, 0.0)],
                200.0,
            ),
            wall(
                2,
                vec![Point::new(100.0, 0.0, 0.0), Point::new(100.0, 6000.0, 0.0)],
                200.0,
            ),
        ];
        let extended = extend_endpoints(&walls, 250.0);
        // W1's start (0, 100) should be moved to W2's centerline x=100 → (100, 100).
        let w1 = &extended[0];
        assert!(
            w1.centerline[0].approx_eq(&Point::new(100.0, 100.0, 0.0), 1.0),
            "W1 start should extend to (100, 100), got {:?}",
            w1.centerline[0]
        );
    }

    #[test]
    fn assigns_label_to_room_containing_text() {
        let walls = rect_walls();
        let texts = vec![Entity::Text {
            position: Point::new(5000.0, 3000.0, 0.0),
            value: "LIVING".into(),
            height: 250.0,
            rotation: 0.0,
            layer: "TEXT".into(),
            props: EntityProps::default(),
        }];
        let rooms = detect_rooms(&walls, &texts, &RoomConfig::default()).unwrap();
        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].label.as_deref(), Some("LIVING"));
    }
}
