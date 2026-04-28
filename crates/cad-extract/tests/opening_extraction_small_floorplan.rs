//! Integration test — opening detection against the small floorplan corpus.

use std::fs::File;
use std::io::BufReader;

use cad_dxf_parser::parse_all;
use cad_extract::opening::{detect_openings, OpeningConfig};
use cad_extract::wall::ParallelLinePairExtractor;
use cad_extract::WallExtractor;
use cad_semantic::OpeningKind;

const SMALL_DXF_PATH: &str = "../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf";

#[test]
fn extracts_openings_from_small_floorplan() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();
    let openings = detect_openings(&doc.entities, &walls, &OpeningConfig::default()).unwrap();

    eprintln!("Extracted {} openings:", openings.len());
    for o in &openings {
        eprintln!(
            "  O{}: {:?}, host_wall={:?}, position={:.0}mm, width={:.0}mm",
            o.id, o.kind, o.host_wall, o.position_along_wall, o.width
        );
    }

    let doors: Vec<_> = openings.iter().filter(|o| o.kind == OpeningKind::Door).collect();
    let windows: Vec<_> = openings.iter().filter(|o| o.kind == OpeningKind::Window).collect();

    // Per golden: 6 doors + 6 windows = 12 openings.
    assert_eq!(doors.len(), 6, "expected 6 doors, got {}", doors.len());
    assert_eq!(windows.len(), 6, "expected 6 windows, got {}", windows.len());
    assert_eq!(openings.len(), 12, "expected 12 total openings");
}

#[test]
fn small_floorplan_doors_have_uniform_800mm_width() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();
    let openings = detect_openings(&doc.entities, &walls, &OpeningConfig::default()).unwrap();

    for o in openings.iter().filter(|o| o.kind == OpeningKind::Door) {
        assert!(
            (o.width - 800.0).abs() < 1.0,
            "door width should be 800mm, got {}",
            o.width
        );
    }
}

#[test]
fn small_floorplan_doors_link_to_walls() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();
    let openings = detect_openings(&doc.entities, &walls, &OpeningConfig::default()).unwrap();

    let doors: Vec<_> = openings.iter().filter(|o| o.kind == OpeningKind::Door).collect();
    let linked = doors.iter().filter(|d| d.host_wall.is_some()).count();

    // At least 5 of 6 doors should successfully link to a wall.
    // (One door at hinge=(9800,1600) is on exterior wall W2 — should match.)
    assert!(linked >= 5, "expected at least 5 doors linked to walls, got {linked}");
}
