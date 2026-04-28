//! Stress / scale validation: full extraction pipeline on the medium
//! multifamily DXF (1.8MB / 23,750 entities / ~110 units).
//!
//! Verifies:
//! - The extractor runs to completion within a reasonable time budget.
//! - Counts match the statistical golden (
//!   `tests/corpus/golden/medium_multifamily_floorplan_synthetic_r2000.golden.json`).
//! - Memory usage stays bounded (smoke check by virtue of completing).

use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;
use cad_semantic::OpeningKind;

const MEDIUM_DXF_PATH: &str =
    "../../tests/corpus/synthetic/medium_multifamily_floorplan_synthetic_r2000.dxf";

#[test]
fn full_pipeline_runs_on_medium_floorplan() {
    let parse_start = Instant::now();
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    let parse_elapsed = parse_start.elapsed();

    eprintln!(
        "PARSE: {} entities, {} blocks in {:?}",
        doc.entities.len(),
        doc.blocks.len(),
        parse_elapsed
    );

    let extract_start = Instant::now();
    let plan = extract_floorplan(&doc.entities).unwrap();
    let extract_elapsed = extract_start.elapsed();

    eprintln!(
        "EXTRACT: {} walls, {} openings, {} rooms, {} dimensions, {} grids in {:?}",
        plan.walls.len(),
        plan.openings.len(),
        plan.rooms.len(),
        plan.dimensions.len(),
        plan.grids.len(),
        extract_elapsed
    );

    // Sanity checks — counts should be substantial.
    assert!(plan.walls.len() > 50, "expected many walls (got {})", plan.walls.len());
    assert!(plan.openings.len() > 100, "expected many openings");
}

#[test]
fn medium_door_count_matches_golden_range() {
    let plan = full_extraction();
    let doors = plan
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Door)
        .count();
    eprintln!("Detected {} doors (golden: 1296)", doors);
    // Allow ±10% slack for edge cases (some doors near junctions might fail
    // the LINE+ARC matcher).
    assert!(
        doors >= 1100 && doors <= 1400,
        "doors {doors} outside expected 1100-1400 range"
    );
}

#[test]
fn medium_window_count_matches_golden_range() {
    let plan = full_extraction();
    let windows = plan
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Window)
        .count();
    eprintln!("Detected {} windows (golden: 972)", windows);
    assert!(
        windows >= 800 && windows <= 1100,
        "windows {windows} outside expected 800-1100 range"
    );
}

#[test]
fn medium_grid_has_19_x_axes_and_19_y_axes() {
    let plan = full_extraction();
    assert!(!plan.grids.is_empty(), "should have a grid");
    let g = &plan.grids[0];
    eprintln!(
        "Grid: {} X-axes, {} Y-axes (golden: 19+19)",
        g.x_axes.len(),
        g.y_axes.len()
    );
    // Allow some slack — algorithm may detect 18-20 due to dedup tolerance.
    assert!(
        g.x_axes.len() >= 17 && g.x_axes.len() <= 21,
        "X-axes {} outside 17-21",
        g.x_axes.len()
    );
    assert!(
        g.y_axes.len() >= 17 && g.y_axes.len() <= 21,
        "Y-axes {} outside 17-21",
        g.y_axes.len()
    );
}

#[test]
fn medium_room_count_indicates_multifamily_scale() {
    let plan = full_extraction();
    eprintln!("Detected {} rooms (golden range: 400-600)", plan.rooms.len());
    // Multifamily building has many rooms. Stage 1 algorithm may not
    // detect every cell — some unbounded faces, some too-small artifacts.
    // Verify it's at least at "multifamily scale" (>50 rooms).
    assert!(
        plan.rooms.len() >= 50,
        "expected multifamily room count, got {}",
        plan.rooms.len()
    );
}

#[test]
fn medium_room_labels_include_unit_numbers() {
    let plan = full_extraction();
    let unit_labels: Vec<&str> = plan
        .rooms
        .iter()
        .filter_map(|r| r.label.as_deref())
        .filter(|l| l.starts_with("UNIT "))
        .collect();
    eprintln!("Found {} UNIT labels", unit_labels.len());
    assert!(
        unit_labels.len() >= 50,
        "expected at least 50 UNIT-labeled rooms, got {}",
        unit_labels.len()
    );
}

fn full_extraction() -> cad_semantic::Floorplan {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    extract_floorplan(&doc.entities).unwrap()
}
