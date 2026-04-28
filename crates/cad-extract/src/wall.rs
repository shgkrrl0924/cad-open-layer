//! Wall extraction — Algorithm #1 from `docs/algorithms.md`.
//!
//! The `ParallelLinePairExtractor` finds walls drawn as two parallel lines.
//! It also picks up "lone" wall lines (interior partitions drawn as single
//! line, no pair) and emits them with a configurable default thickness.

use cad_core::error::Result;
use cad_core::{Entity, LayerName, Point};
use cad_geometry::polyline::Polyline;
use cad_semantic::{Wall, WallId, WallKind};

use crate::WallExtractor;

/// Configuration for wall hierarchy classification (Exterior / Interior /
/// Partition). Thresholds are in millimeters by default.
#[derive(Debug, Clone)]
pub struct HierarchyConfig {
    pub exterior_threshold: f64,
    pub interior_threshold: f64,
}

impl Default for HierarchyConfig {
    fn default() -> Self {
        Self {
            exterior_threshold: 180.0,
            interior_threshold: 100.0,
        }
    }
}

/// Classify each wall as Exterior / Interior / Partition by layer-name hint
/// and thickness.
///
/// Heuristic (in order):
/// 1. Layer-name contains EXTR / 외벽 → Exterior
/// 2. Layer-name contains INTR / 내벽 → Interior
/// 3. Layer-name contains PART / 파티션 → Partition
/// 4. thickness ≥ exterior_threshold → Exterior
/// 5. thickness ≥ interior_threshold → Interior
/// 6. otherwise → Partition
pub fn classify_walls(walls: &mut [cad_semantic::Wall], config: &HierarchyConfig) {
    for w in walls.iter_mut() {
        w.kind = classify_one_wall(w, config);
    }
}

fn classify_one_wall(w: &cad_semantic::Wall, config: &HierarchyConfig) -> WallKind {
    let layer_lower = w.layer.to_lowercase();
    if layer_lower.contains("extr") || w.layer.contains("외벽") {
        return WallKind::Exterior;
    }
    if layer_lower.contains("intr") || w.layer.contains("내벽") {
        return WallKind::Interior;
    }
    if layer_lower.contains("part") || w.layer.contains("파티션") {
        return WallKind::Partition;
    }
    if w.thickness >= config.exterior_threshold {
        WallKind::Exterior
    } else if w.thickness >= config.interior_threshold {
        WallKind::Interior
    } else {
        WallKind::Partition
    }
}

#[derive(Debug, Clone)]
pub struct WallConfig {
    /// Layer names to consider as walls.
    pub layers: Vec<LayerName>,
    /// Minimum perpendicular distance for a parallel-line pair to be a wall.
    pub thickness_min: f64,
    /// Maximum perpendicular distance for a parallel-line pair to be a wall.
    pub thickness_max: f64,
    /// Maximum angular difference between two lines to be considered parallel.
    pub parallel_eps_rad: f64,
    /// Required projected overlap as a ratio of the shorter line.
    pub overlap_min_ratio: f64,
    /// Thickness assigned to lone interior partition walls.
    pub default_partition_thickness: f64,
    /// Lines below this length are filtered as noise (tick marks, etc.).
    pub min_line_length: f64,
}

impl Default for WallConfig {
    fn default() -> Self {
        Self {
            layers: vec![
                "WALLS".into(),
                "WALL".into(),
                "A-WALL".into(),
                "벽".into(),
                "벽체".into(),
                // Korean hierarchy hints — these layer names also trigger
                // WallKind classification in classify_walls().
                "외벽".into(),
                "내벽".into(),
                "파티션".into(),
            ],
            // Note: input units are whatever the DXF says. The small floorplan
            // is in millimeters (so 50mm = 0.05). We keep f64 with no unit
            // conversion here — caller should normalize beforehand or pick a
            // config matching the file's units. Defaults below assume
            // millimeters per the synthetic corpus.
            thickness_min: 50.0,
            thickness_max: 500.0,
            parallel_eps_rad: 0.5_f64.to_radians(),
            overlap_min_ratio: 0.7,
            default_partition_thickness: 50.0,
            min_line_length: 200.0,
        }
    }
}

/// Wall extractor for the parallel-line-pair convention.
#[derive(Debug, Clone, Default)]
pub struct ParallelLinePairExtractor {
    pub config: WallConfig,
}

impl ParallelLinePairExtractor {
    pub fn new(config: WallConfig) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone, Copy)]
struct LineSeg {
    a: Point,
    b: Point,
    layer_idx: usize,
}

impl LineSeg {
    fn length(&self) -> f64 {
        self.a.distance(&self.b)
    }
    fn angle_unsigned(&self) -> f64 {
        let dy = self.b.y - self.a.y;
        let dx = self.b.x - self.a.x;
        let mut a = dy.atan2(dx);
        // Normalize to [0, π) — direction-agnostic.
        if a < 0.0 {
            a += std::f64::consts::PI;
        }
        if a >= std::f64::consts::PI {
            a -= std::f64::consts::PI;
        }
        a
    }
}

impl WallExtractor for ParallelLinePairExtractor {
    fn extract(&self, entities: &[Entity]) -> Result<Vec<Wall>> {
        // Step 1: filter LINEs on configured wall layers, drop noise tick marks.
        let mut candidates: Vec<(usize, LineSeg, LayerName)> = entities
            .iter()
            .enumerate()
            .filter_map(|(idx, e)| match e {
                Entity::Line { p1, p2, layer, .. } => {
                    if self.config.layers.iter().any(|l| l == layer) {
                        let seg = LineSeg { a: *p1, b: *p2, layer_idx: idx };
                        if seg.length() >= self.config.min_line_length {
                            Some((idx, seg, layer.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        // Step 2: O(n²) pair detection (PoC scale ≤ 200 walls is fine).
        let n = candidates.len();
        let mut pair_used = vec![false; n];
        let mut walls: Vec<Wall> = vec![];
        let mut next_id: WallId = 1;

        for i in 0..n {
            if pair_used[i] {
                continue;
            }
            let (_, seg_i, layer_i) = &candidates[i].clone();
            for j in (i + 1)..n {
                if pair_used[j] {
                    continue;
                }
                let (_, seg_j, _) = &candidates[j].clone();
                if let Some((centerline, thickness)) = check_wall_pair(seg_i, seg_j, &self.config) {
                    walls.push(Wall {
                        id: next_id,
                        centerline,
                        thickness,
                        height: None,
                        layer: layer_i.clone(),
                        kind: WallKind::Unknown,
                        openings: vec![],
                    });
                    next_id += 1;
                    pair_used[i] = true;
                    pair_used[j] = true;
                    break;
                }
            }
        }

        // Step 3: lone interior partition walls (singles not paired).
        for i in 0..n {
            if pair_used[i] {
                continue;
            }
            let (_, seg, layer) = &candidates[i];
            walls.push(Wall {
                id: next_id,
                centerline: Polyline::new(vec![seg.a, seg.b], false),
                thickness: self.config.default_partition_thickness,
                height: None,
                layer: layer.clone(),
                kind: WallKind::Partition,
                openings: vec![],
            });
            next_id += 1;
        }

        Ok(walls)
    }
}

/// Test whether two line segments form a wall pair. Returns (centerline,
/// thickness) when they do.
fn check_wall_pair(
    a: &LineSeg,
    b: &LineSeg,
    config: &WallConfig,
) -> Option<(Polyline, f64)> {
    // 1. Parallel check (direction-agnostic).
    let angle_a = a.angle_unsigned();
    let angle_b = b.angle_unsigned();
    let mut diff = (angle_a - angle_b).abs();
    if diff > std::f64::consts::PI / 2.0 {
        diff = std::f64::consts::PI - diff;
    }
    if diff > config.parallel_eps_rad {
        return None;
    }

    // 2. Perpendicular distance — distance from a's midpoint to the infinite
    //    line containing b.
    let dist = perpendicular_distance(&a.midpoint(), b);
    if dist < config.thickness_min || dist > config.thickness_max {
        return None;
    }

    // 3. Projected overlap on b's direction.
    let overlap = overlap_ratio(a, b);
    if overlap < config.overlap_min_ratio {
        return None;
    }

    // 4. Build centerline: average of (a's endpoints) and (their projections on b).
    let (proj_a_start, _) = project_onto_line(&a.a, b);
    let (proj_a_end, _) = project_onto_line(&a.b, b);
    let mid_start = midpoint(&a.a, &proj_a_start);
    let mid_end = midpoint(&a.b, &proj_a_end);

    Some((Polyline::new(vec![mid_start, mid_end], false), dist))
}

trait Midpoint {
    fn midpoint(&self) -> Point;
}

impl Midpoint for LineSeg {
    fn midpoint(&self) -> Point {
        midpoint(&self.a, &self.b)
    }
}

fn midpoint(a: &Point, b: &Point) -> Point {
    Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0)
}

fn perpendicular_distance(p: &Point, line: &LineSeg) -> f64 {
    let (proj, _) = project_onto_line(p, line);
    p.distance(&proj)
}

/// Project a point onto the infinite line containing `line`. Returns
/// (projected point, t-parameter where t=0 at line.a, t=1 at line.b).
fn project_onto_line(p: &Point, line: &LineSeg) -> (Point, f64) {
    let ab_x = line.b.x - line.a.x;
    let ab_y = line.b.y - line.a.y;
    let ap_x = p.x - line.a.x;
    let ap_y = p.y - line.a.y;
    let len_sq = ab_x.mul_add(ab_x, ab_y * ab_y);
    if len_sq < 1e-12 {
        return (line.a, 0.0);
    }
    let t = ap_x.mul_add(ab_x, ap_y * ab_y) / len_sq;
    (
        Point::new(line.a.x + ab_x * t, line.a.y + ab_y * t, line.a.z),
        t,
    )
}

/// Compute how much of `a` projects onto `b`'s span [0,1], expressed as a
/// ratio of the shorter line's length.
fn overlap_ratio(a: &LineSeg, b: &LineSeg) -> f64 {
    let (_, t1) = project_onto_line(&a.a, b);
    let (_, t2) = project_onto_line(&a.b, b);
    let t_min = t1.min(t2);
    let t_max = t1.max(t2);
    let clipped_min = t_min.max(0.0);
    let clipped_max = t_max.min(1.0);
    let overlap = (clipped_max - clipped_min).max(0.0);
    let b_len = b.length();
    let overlap_len = overlap * b_len;
    let shorter = a.length().min(b_len);
    if shorter < 1e-9 {
        return 0.0;
    }
    overlap_len / shorter
}

#[derive(Debug, Clone, Default)]
pub struct PolylineOutlineExtractor {
    pub config: WallConfig,
}

impl WallExtractor for PolylineOutlineExtractor {
    fn extract(&self, _entities: &[Entity]) -> Result<Vec<Wall>> {
        // Stage 1.5 — not implemented yet. Returns empty for now.
        Ok(vec![])
    }
}

#[derive(Debug, Clone, Default)]
pub struct HatchBoundaryExtractor {
    pub config: WallConfig,
}

impl WallExtractor for HatchBoundaryExtractor {
    fn extract(&self, _entities: &[Entity]) -> Result<Vec<Wall>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cad_core::EntityProps;

    fn line(p1: Point, p2: Point, layer: &str) -> Entity {
        Entity::Line { p1, p2, layer: layer.into(), props: EntityProps::default() }
    }

    #[test]
    fn extracts_single_wall_from_parallel_pair() {
        let entities = vec![
            line(Point::new(0.0, 0.0, 0.0), Point::new(10000.0, 0.0, 0.0), "WALLS"),
            line(Point::new(0.0, 200.0, 0.0), Point::new(10000.0, 200.0, 0.0), "WALLS"),
        ];
        let walls = ParallelLinePairExtractor::default().extract(&entities).unwrap();
        assert_eq!(walls.len(), 1);
        assert!((walls[0].thickness - 200.0).abs() < 1e-6);
    }

    #[test]
    fn extracts_partition_wall_from_lone_line() {
        let entities = vec![
            line(Point::new(4000.0, 200.0, 0.0), Point::new(4000.0, 6300.0, 0.0), "WALLS"),
        ];
        let walls = ParallelLinePairExtractor::default().extract(&entities).unwrap();
        assert_eq!(walls.len(), 1);
        assert_eq!(walls[0].kind, WallKind::Partition);
    }

    #[test]
    fn ignores_short_tick_marks() {
        // Tick marks: 100mm long, below min_line_length=200.
        let entities = vec![
            line(Point::new(500.0, 6450.0, 0.0), Point::new(600.0, 6450.0, 0.0), "WALLS"),
            line(Point::new(1000.0, 6450.0, 0.0), Point::new(1100.0, 6450.0, 0.0), "WALLS"),
        ];
        let walls = ParallelLinePairExtractor::default().extract(&entities).unwrap();
        assert_eq!(walls.len(), 0, "tick marks should be filtered");
    }

    #[test]
    fn ignores_non_wall_layers() {
        let entities = vec![
            line(Point::new(0.0, 0.0, 0.0), Point::new(10000.0, 0.0, 0.0), "FURNITURE"),
            line(Point::new(0.0, 200.0, 0.0), Point::new(10000.0, 200.0, 0.0), "FURNITURE"),
        ];
        let walls = ParallelLinePairExtractor::default().extract(&entities).unwrap();
        assert_eq!(walls.len(), 0);
    }

    fn wall_for_classify(layer: &str, thickness: f64) -> Wall {
        Wall {
            id: 0,
            centerline: Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1000.0, 0.0, 0.0)], false),
            thickness,
            height: None,
            layer: layer.into(),
            kind: WallKind::Unknown,
            openings: vec![],
        }
    }

    #[test]
    fn classify_layer_hints_override_thickness() {
        let cfg = HierarchyConfig::default();

        // Korean hints — even with sub-partition thickness, layer hint wins.
        assert_eq!(classify_one_wall(&wall_for_classify("외벽", 50.0), &cfg), WallKind::Exterior);
        assert_eq!(classify_one_wall(&wall_for_classify("내벽", 50.0), &cfg), WallKind::Interior);
        assert_eq!(classify_one_wall(&wall_for_classify("파티션", 500.0), &cfg), WallKind::Partition);

        // English hints (case-insensitive substring match).
        assert_eq!(classify_one_wall(&wall_for_classify("EXTR-WALL", 50.0), &cfg), WallKind::Exterior);
        assert_eq!(classify_one_wall(&wall_for_classify("intr_wall", 50.0), &cfg), WallKind::Interior);
        assert_eq!(classify_one_wall(&wall_for_classify("PART_01", 500.0), &cfg), WallKind::Partition);
    }

    #[test]
    fn classify_thickness_fallback_when_no_hint() {
        let cfg = HierarchyConfig::default();
        assert_eq!(classify_one_wall(&wall_for_classify("WALLS", 200.0), &cfg), WallKind::Exterior);
        assert_eq!(classify_one_wall(&wall_for_classify("WALLS", 150.0), &cfg), WallKind::Interior);
        assert_eq!(classify_one_wall(&wall_for_classify("WALLS", 50.0), &cfg), WallKind::Partition);
    }
}
