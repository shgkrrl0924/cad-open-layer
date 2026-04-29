//! End-to-end round trip test:
//! `parse(small_dxf)` → extract → synthesize → parse → extract
//! and verify the two extracted Floorplans are semantically equivalent.

use std::fs::File;
use std::io::{BufReader, Cursor};

use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;
use cad_synthesize::{floorplan_to_dxf, SynthesizeConfig};

const SMALL_DXF_PATH: &str = "../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf";

#[test]
fn round_trip_preserves_wall_count() {
    let original = read_floorplan(SMALL_DXF_PATH);
    let regenerated = round_trip(&original);

    eprintln!(
        "Walls: original={}, regenerated={}",
        original.walls.len(),
        regenerated.walls.len()
    );
    eprintln!(
        "Openings: original={}, regenerated={}",
        original.openings.len(),
        regenerated.openings.len()
    );
    eprintln!(
        "Rooms: original={}, regenerated={}",
        original.rooms.len(),
        regenerated.rooms.len()
    );

    assert_eq!(
        original.walls.len(),
        regenerated.walls.len(),
        "wall count must be preserved"
    );
}

#[test]
fn round_trip_preserves_room_labels() {
    let original = read_floorplan(SMALL_DXF_PATH);
    let regenerated = round_trip(&original);

    let original_labels: std::collections::HashSet<_> = original
        .rooms
        .iter()
        .filter_map(|r| r.label.clone())
        .collect();
    let regen_labels: std::collections::HashSet<_> = regenerated
        .rooms
        .iter()
        .filter_map(|r| r.label.clone())
        .collect();

    eprintln!("Original labels: {original_labels:?}");
    eprintln!("Regenerated labels: {regen_labels:?}");

    // The four named rooms (LIVING / KITCHEN / BED 1 / BED 2) should
    // round-trip; BATH gets absorbed into KITCHEN per source DXF anomaly.
    for expected in &["LIVING", "KITCHEN", "BED 1", "BED 2"] {
        assert!(
            regen_labels.iter().any(|l| l == expected),
            "regenerated should contain {expected}, got {regen_labels:?}"
        );
    }
}

#[test]
fn round_trip_preserves_opening_count() {
    let original = read_floorplan(SMALL_DXF_PATH);
    let regenerated = round_trip(&original);

    use cad_semantic::OpeningKind;
    let orig_doors = original
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Door)
        .count();
    let orig_windows = original
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Window)
        .count();
    let regen_doors = regenerated
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Door)
        .count();
    let regen_windows = regenerated
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Window)
        .count();

    eprintln!("Doors: orig={orig_doors}, regen={regen_doors}");
    eprintln!("Windows: orig={orig_windows}, regen={regen_windows}");

    assert_eq!(orig_doors, 6, "small floorplan has 6 doors");
    assert_eq!(orig_windows, 6, "small floorplan has 6 windows");

    // After Task #29, INSERT-based opening detection is supported, so
    // synthesis output should round-trip the full opening count.
    assert_eq!(
        regen_doors, 6,
        "round-trip should preserve 6 doors via INSERT detection"
    );
    assert_eq!(regen_windows, 6, "round-trip should preserve 6 windows");
}

#[test]
fn round_trip_dxf_is_parseable() {
    let original = read_floorplan(SMALL_DXF_PATH);

    let mut buf: Vec<u8> = vec![];
    floorplan_to_dxf(&original, &mut buf, &SynthesizeConfig::default()).unwrap();

    eprintln!("Synthesized DXF size: {} bytes", buf.len());
    assert!(buf.len() > 100, "DXF should be non-trivial");

    // Just verify it parses without error.
    let parsed = parse_all(Cursor::new(&buf)).unwrap();
    assert!(!parsed.entities.is_empty(), "should produce entities");
}

fn read_floorplan(path: &str) -> cad_semantic::Floorplan {
    let file = File::open(path).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    extract_floorplan(&doc.entities).unwrap()
}

fn round_trip(plan: &cad_semantic::Floorplan) -> cad_semantic::Floorplan {
    let mut buf: Vec<u8> = vec![];
    floorplan_to_dxf(plan, &mut buf, &SynthesizeConfig::default()).unwrap();
    let parsed = parse_all(Cursor::new(&buf)).unwrap();
    extract_floorplan(&parsed.entities).unwrap()
}
