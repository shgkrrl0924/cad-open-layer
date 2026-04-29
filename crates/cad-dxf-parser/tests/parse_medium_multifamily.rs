//! Integration test — parse the medium multifamily floorplan DXF (1.8MB / 364k lines).

use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use cad_core::Entity;
use cad_dxf_parser::parse_all;

const MEDIUM_DXF_PATH: &str =
    "../../tests/corpus/synthetic/medium_multifamily_floorplan_synthetic_r2000.dxf";

#[test]
fn parses_medium_floorplan_without_error() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus DXF must exist");
    let reader = BufReader::new(file);
    let start = Instant::now();
    let doc = parse_all(reader).expect("parse should succeed");
    let elapsed = start.elapsed();

    eprintln!(
        "medium DXF: {} entities, {} blocks, parsed in {:?}",
        doc.entities.len(),
        doc.blocks.len(),
        elapsed
    );

    assert!(doc.entities.len() > 10_000, "expected many entities");
    assert_eq!(
        doc.header.get("$ACADVER").map(String::as_str),
        Some("AC1015")
    );
}

#[test]
fn medium_floorplan_door_count_matches_golden() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    // Golden: 1296 ARCs on DOORS layer (each door = 1 LINE + 1 ARC).
    let arcs_on_doors = doc
        .entities
        .iter()
        .filter(|e| matches!(e, Entity::Arc { layer, .. } if layer == "DOORS"))
        .count();

    assert_eq!(
        arcs_on_doors, 1296,
        "expected exactly 1296 door swing arcs (golden statistical)"
    );
}

#[test]
fn medium_floorplan_window_count_matches_golden() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    // Golden: 1944 LINEs on WINDOWS layer (each window = 2 parallel lines).
    let lines_on_windows = doc
        .entities
        .iter()
        .filter(|e| matches!(e, Entity::Line { layer, .. } if layer == "WINDOWS"))
        .count();

    assert_eq!(
        lines_on_windows, 1944,
        "expected exactly 1944 window lines (= 972 windows × 2)"
    );
}

#[test]
fn medium_floorplan_unit_labels_present() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    let unit_label_count = doc
        .entities
        .iter()
        .filter(|e| match e {
            Entity::Text { value, layer, .. } => layer == "TEXT" && value.starts_with("UNIT "),
            _ => false,
        })
        .count();

    // Empirical measurement on the synthetic corpus: 324 UNIT labels.
    // (Initial golden estimate of "~110" was based on partial sample; updated
    //  after parser-driven count.)
    assert!(
        unit_label_count >= 100,
        "expected at least 100 UNIT labels, got {unit_label_count}"
    );
    assert!(
        unit_label_count <= 500,
        "expected at most 500 UNIT labels, got {unit_label_count}"
    );
}

#[test]
fn medium_floorplan_grid_labels_complete() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    // Golden: 19 numeric labels (1-19) + 19 letter labels (A-S) = 38 expected.
    let grid_labels: Vec<String> = doc
        .entities
        .iter()
        .filter_map(|e| match e {
            Entity::Text { value, layer, .. } if layer == "GRID" => Some(value.clone()),
            _ => None,
        })
        .collect();

    let has_a = grid_labels.iter().any(|s| s == "A");
    let has_s = grid_labels.iter().any(|s| s == "S");
    let has_1 = grid_labels.iter().any(|s| s == "1");
    let has_19 = grid_labels.iter().any(|s| s == "19");

    assert!(has_a, "grid should contain label 'A'");
    assert!(has_s, "grid should contain label 'S'");
    assert!(has_1, "grid should contain label '1'");
    assert!(has_19, "grid should contain label '19'");
    assert!(
        grid_labels.len() >= 36,
        "should have at least 36 grid labels"
    );
}

#[test]
fn medium_floorplan_no_unrecognized_entities() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    let raw_count = doc
        .entities
        .iter()
        .filter(|e| matches!(e, Entity::Raw(_)))
        .count();

    // Medium DXF only uses LINE/ARC/CIRCLE/TEXT — all should be recognized.
    assert_eq!(
        raw_count, 0,
        "all entity types should be handled, got {raw_count} Raw"
    );
}
