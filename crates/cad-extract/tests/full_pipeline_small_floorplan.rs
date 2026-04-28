//! End-to-end pipeline test: parse → extract → verify all 5 layers.

use std::fs::File;
use std::io::BufReader;

use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;
use cad_semantic::{DimensionTarget, OpeningKind};

const SMALL_DXF_PATH: &str = "../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf";

#[test]
fn full_pipeline_produces_complete_floorplan() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let plan = extract_floorplan(&doc.entities).unwrap();

    eprintln!(
        "FULL PIPELINE: {} walls, {} openings, {} rooms, {} dimensions, {} grids",
        plan.walls.len(),
        plan.openings.len(),
        plan.rooms.len(),
        plan.dimensions.len(),
        plan.grids.len()
    );

    assert_eq!(plan.walls.len(), 8, "8 walls (golden)");
    assert_eq!(plan.openings.len(), 12, "12 openings (golden)");
    assert_eq!(plan.rooms.len(), 5, "5 rooms (golden)");
    assert!(!plan.dimensions.is_empty(), "should detect dimensions");
    assert_eq!(plan.grids.len(), 1, "single grid struct");
}

#[test]
fn dimensions_link_to_correct_walls() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    let plan = extract_floorplan(&doc.entities).unwrap();

    eprintln!("Dimensions detected:");
    for d in &plan.dimensions {
        eprintln!(
            "  D{}: measured={:.0}, text={:?}, linked_to={:?}",
            d.id, d.measured_value, d.text_override, d.linked_to
        );
    }

    // Per golden: 2 visible dimensions — "10000" (south wall length) and "6500" (west wall length).
    let measured: Vec<f64> = plan.dimensions.iter().map(|d| d.measured_value).collect();
    assert!(
        measured.iter().any(|v| (*v - 10000.0).abs() < 1.0),
        "should detect 10000mm dimension, got {:?}",
        measured
    );
    assert!(
        measured.iter().any(|v| (*v - 6500.0).abs() < 1.0),
        "should detect 6500mm dimension, got {:?}",
        measured
    );

    // Both should link to a wall via WallLength.
    let wall_links: Vec<_> = plan
        .dimensions
        .iter()
        .filter_map(|d| match &d.linked_to {
            Some(DimensionTarget::WallLength(id)) => Some(*id),
            _ => None,
        })
        .collect();
    assert!(
        wall_links.len() >= 2,
        "expected at least 2 wall-length dimensions, got {wall_links:?}"
    );
}

#[test]
fn grid_has_4_x_axes_and_4_y_axes() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    let plan = extract_floorplan(&doc.entities).unwrap();

    let grid = &plan.grids[0];
    eprintln!("Grid X axes:");
    for x in &grid.x_axes {
        eprintln!("  pos={:.0}, label={:?}", x.position, x.label);
    }
    eprintln!("Grid Y axes:");
    for y in &grid.y_axes {
        eprintln!("  pos={:.0}, label={:?}", y.position, y.label);
    }

    // Per golden: X-axes at A/B/C/D = (0, 4000, 7000, 10000), Y-axes at 1/2/3/4 = (0, 2500, 3300, 6500).
    assert_eq!(grid.x_axes.len(), 4, "expected 4 X-axes");
    assert_eq!(grid.y_axes.len(), 4, "expected 4 Y-axes");

    let x_labels: Vec<_> = grid.x_axes.iter().filter_map(|a| a.label.clone()).collect();
    let y_labels: Vec<_> = grid.y_axes.iter().filter_map(|a| a.label.clone()).collect();
    for &expected in &["A", "B", "C", "D"] {
        assert!(
            x_labels.iter().any(|l| l == expected),
            "X-axis labels should include {expected}, got {x_labels:?}"
        );
    }
    for &expected in &["1", "2", "3", "4"] {
        assert!(
            y_labels.iter().any(|l| l == expected),
            "Y-axis labels should include {expected}, got {y_labels:?}"
        );
    }
}

#[test]
fn full_pipeline_room_labels() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    let plan = extract_floorplan(&doc.entities).unwrap();

    let labels: Vec<String> = plan.rooms.iter().filter_map(|r| r.label.clone()).collect();
    for &expected in &["LIVING", "KITCHEN", "BED 1", "BED 2"] {
        assert!(
            labels.iter().any(|l| l == expected),
            "expected room label {expected}, got {labels:?}"
        );
    }
}

#[test]
fn full_pipeline_openings_link_to_walls() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    let plan = extract_floorplan(&doc.entities).unwrap();

    let doors = plan.openings.iter().filter(|o| o.kind == OpeningKind::Door).count();
    let windows = plan.openings.iter().filter(|o| o.kind == OpeningKind::Window).count();
    assert_eq!(doors, 6);
    assert_eq!(windows, 6);

    let linked = plan.openings.iter().filter(|o| o.host_wall.is_some()).count();
    assert_eq!(linked, 12, "all openings should link to walls");
}
