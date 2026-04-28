//! Standard R2000-compatible BLOCK definitions for door, window, and
//! annotation primitives.
//!
//! These blocks are emitted into the DXF BLOCKS section so that INSERT
//! references in the ENTITIES section resolve correctly when the file is
//! opened in AutoCAD or compatible viewers.
//!
//! Conventions:
//! - All blocks defined at origin (0, 0, 0).
//! - Doors: hinge at origin, leaf along +X, swing arc from 0° to 90° (CCW).
//! - Windows: rectangle from origin to (width, depth). Default depth 80mm.
//! - Coordinates in millimeters (project default).
//!
//! Block names follow `<KIND>_<width_mm>` pattern (e.g., `DOOR_800`,
//! `WINDOW_1200`). Width hint encoded in the name lets parsers double-check
//! against block geometry.

use std::f64::consts::PI;

use cad_core::{BlockDef, Entity, EntityProps, Point};

/// Default door swing arc radius equals door width (standard architectural
/// representation: the leaf swings 90° around its hinge).
fn door_arc(width_mm: f64, layer: &str) -> Entity {
    Entity::Arc {
        center: Point::new(0.0, 0.0, 0.0),
        radius: width_mm,
        start_angle: 0.0,
        end_angle: PI / 2.0,
        layer: layer.into(),
        props: EntityProps::default(),
    }
}

/// Door leaf: line from hinge along +X for `width_mm` distance (representing
/// the door in its open position, perpendicular to wall).
fn door_leaf(width_mm: f64, layer: &str) -> Entity {
    Entity::Line {
        p1: Point::new(0.0, 0.0, 0.0),
        p2: Point::new(width_mm, 0.0, 0.0),
        layer: layer.into(),
        props: EntityProps::default(),
    }
}

/// Build a standard architectural door block at the given width (mm).
///
/// Block geometry:
/// - Door leaf: LINE (0,0) → (width, 0)
/// - Swing arc: ARC center=(0,0) radius=width, 0° → 90°
///
/// Block name: `DOOR_<width>` e.g., `DOOR_800`.
#[must_use]
pub fn standard_door_block(width_mm: u32) -> BlockDef {
    let w = f64::from(width_mm);
    BlockDef {
        name: format!("DOOR_{width_mm}"),
        layer: "DOORS".into(),
        base_point: Point::new(0.0, 0.0, 0.0),
        flags: 0,
        entities: vec![door_leaf(w, "DOORS"), door_arc(w, "DOORS")],
        props: EntityProps::default(),
    }
}

/// Build a standard architectural window block at the given width (mm) and
/// depth (mm). Depth represents the wall thickness slot the window sits in.
///
/// Block geometry:
/// - Bottom edge: LINE (0,0) → (width, 0)
/// - Top edge:    LINE (0, depth) → (width, depth)
/// - Left edge:   LINE (0, 0) → (0, depth)
/// - Right edge:  LINE (width, 0) → (width, depth)
///
/// Block name: `WINDOW_<width>` e.g., `WINDOW_1200`.
#[must_use]
pub fn standard_window_block(width_mm: u32, depth_mm: u32) -> BlockDef {
    let w = f64::from(width_mm);
    let d = f64::from(depth_mm);
    let layer = "WINDOWS";
    let entities = vec![
        Entity::Line {
            p1: Point::new(0.0, 0.0, 0.0),
            p2: Point::new(w, 0.0, 0.0),
            layer: layer.into(),
            props: EntityProps::default(),
        },
        Entity::Line {
            p1: Point::new(0.0, d, 0.0),
            p2: Point::new(w, d, 0.0),
            layer: layer.into(),
            props: EntityProps::default(),
        },
        Entity::Line {
            p1: Point::new(0.0, 0.0, 0.0),
            p2: Point::new(0.0, d, 0.0),
            layer: layer.into(),
            props: EntityProps::default(),
        },
        Entity::Line {
            p1: Point::new(w, 0.0, 0.0),
            p2: Point::new(w, d, 0.0),
            layer: layer.into(),
            props: EntityProps::default(),
        },
    ];
    BlockDef {
        name: format!("WINDOW_{width_mm}"),
        layer: "WINDOWS".into(),
        base_point: Point::new(0.0, 0.0, 0.0),
        flags: 0,
        entities,
        props: EntityProps::default(),
    }
}

/// Architectural dimension arrow tick (45° slash, common in arch dim style).
///
/// Block geometry:
/// - LINE (-tick/2, -tick/2) → (tick/2, tick/2)
///
/// Block name: `DIM_TICK`.
#[must_use]
pub fn dimension_tick_block(tick_size_mm: f64) -> BlockDef {
    let half = tick_size_mm / 2.0;
    let entities = vec![Entity::Line {
        p1: Point::new(-half, -half, 0.0),
        p2: Point::new(half, half, 0.0),
        layer: "DIMENSIONS".into(),
        props: EntityProps::default(),
    }];
    BlockDef {
        name: "DIM_TICK".into(),
        layer: "DIMENSIONS".into(),
        base_point: Point::new(0.0, 0.0, 0.0),
        flags: 0,
        entities,
        props: EntityProps::default(),
    }
}

/// Standard mm-based door widths (residential + commercial).
pub const STANDARD_DOOR_WIDTHS_MM: &[u32] = &[600, 700, 800, 900, 1000, 1200];

/// Standard mm-based window widths.
pub const STANDARD_WINDOW_WIDTHS_MM: &[u32] = &[600, 900, 1200, 1500, 1800, 2400];

/// Default window depth (matches synthetic small floorplan corpus convention).
pub const DEFAULT_WINDOW_DEPTH_MM: u32 = 80;

/// Build the full standard library: door + window blocks for all standard
/// widths plus the dimension tick. Returns the collection ready to be merged
/// into a DXF document's block table.
#[must_use]
pub fn standard_library() -> Vec<BlockDef> {
    let mut out: Vec<BlockDef> = vec![];
    for &w in STANDARD_DOOR_WIDTHS_MM {
        out.push(standard_door_block(w));
    }
    for &w in STANDARD_WINDOW_WIDTHS_MM {
        out.push(standard_window_block(w, DEFAULT_WINDOW_DEPTH_MM));
    }
    out.push(dimension_tick_block(150.0));
    out
}

/// Map a desired width (mm) to the closest standard width in the library.
/// Returns the standard width and the block name.
#[must_use]
pub fn nearest_door_block(width_mm: f64) -> (u32, String) {
    let nearest = closest(width_mm, STANDARD_DOOR_WIDTHS_MM);
    (nearest, format!("DOOR_{nearest}"))
}

#[must_use]
pub fn nearest_window_block(width_mm: f64) -> (u32, String) {
    let nearest = closest(width_mm, STANDARD_WINDOW_WIDTHS_MM);
    (nearest, format!("WINDOW_{nearest}"))
}

fn closest(target: f64, options: &[u32]) -> u32 {
    *options
        .iter()
        .min_by(|a, b| {
            let da = (f64::from(**a) - target).abs();
            let db = (f64::from(**b) - target).abs();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(&options[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn door_block_has_leaf_and_arc() {
        let b = standard_door_block(800);
        assert_eq!(b.name, "DOOR_800");
        assert_eq!(b.entities.len(), 2);
        let has_line = b.entities.iter().any(|e| matches!(e, Entity::Line { .. }));
        let has_arc = b.entities.iter().any(|e| matches!(e, Entity::Arc { .. }));
        assert!(has_line && has_arc);

        // Verify dimensions.
        for e in &b.entities {
            if let Entity::Line { p1, p2, .. } = e {
                assert!((p1.x - 0.0).abs() < 1e-6);
                assert!((p2.x - 800.0).abs() < 1e-6);
            }
            if let Entity::Arc { radius, start_angle, end_angle, .. } = e {
                assert!((radius - 800.0).abs() < 1e-6);
                assert!((start_angle - 0.0).abs() < 1e-6);
                assert!((end_angle - PI / 2.0).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn window_block_has_four_edges() {
        let b = standard_window_block(1200, 80);
        assert_eq!(b.name, "WINDOW_1200");
        assert_eq!(b.entities.len(), 4);
    }

    #[test]
    fn standard_library_contains_all_widths() {
        let lib = standard_library();
        let names: Vec<&str> = lib.iter().map(|b| b.name.as_str()).collect();
        assert!(names.contains(&"DOOR_800"));
        assert!(names.contains(&"WINDOW_1200"));
        assert!(names.contains(&"DIM_TICK"));
        assert_eq!(
            lib.len(),
            STANDARD_DOOR_WIDTHS_MM.len() + STANDARD_WINDOW_WIDTHS_MM.len() + 1
        );
    }

    #[test]
    fn nearest_door_block_picks_closest_size() {
        assert_eq!(nearest_door_block(810.0), (800, "DOOR_800".to_string()));
        assert_eq!(nearest_door_block(950.0), (900, "DOOR_900".to_string()));
        assert_eq!(nearest_door_block(1100.0), (1000, "DOOR_1000".to_string()));
        assert_eq!(nearest_door_block(0.0), (600, "DOOR_600".to_string()));
        assert_eq!(nearest_door_block(5000.0), (1200, "DOOR_1200".to_string()));
    }

    #[test]
    fn nearest_window_block_picks_closest_size() {
        assert_eq!(nearest_window_block(2000.0), (1800, "WINDOW_1800".to_string()));
        assert_eq!(nearest_window_block(1700.0), (1800, "WINDOW_1800".to_string()));
        assert_eq!(nearest_window_block(1100.0), (1200, "WINDOW_1200".to_string()));
    }

    #[test]
    fn dimension_tick_geometry() {
        let b = dimension_tick_block(150.0);
        assert_eq!(b.name, "DIM_TICK");
        assert_eq!(b.entities.len(), 1);
        if let Entity::Line { p1, p2, .. } = &b.entities[0] {
            // 45° slash from (-75,-75) to (75,75).
            assert!((p1.x - (-75.0)).abs() < 1e-6);
            assert!((p2.x - 75.0).abs() < 1e-6);
        }
    }
}
