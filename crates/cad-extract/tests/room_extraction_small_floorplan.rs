//! Integration test — room detection on the small floorplan corpus.

use std::fs::File;
use std::io::BufReader;

use cad_dxf_parser::parse_all;
use cad_extract::room::{detect_rooms, RoomConfig};
use cad_extract::wall::ParallelLinePairExtractor;
use cad_extract::WallExtractor;

const SMALL_DXF_PATH: &str = "../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf";

#[test]
fn detects_rooms_in_small_floorplan() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();

    let rooms = detect_rooms(&walls, &doc.entities, &RoomConfig::default()).unwrap();

    eprintln!("Detected {} rooms:", rooms.len());
    for r in &rooms {
        eprintln!(
            "  R{}: label={:?}, area={:.1}m², bounding_walls={:?}",
            r.id, r.label, r.area_sq_m, r.bounding_walls
        );
    }

    // Per golden: 5 rooms expected (LIVING/KITCHEN/BED 1/BED 2/BATH).
    // BATH label is inside KITCHEN polygon (golden anomaly), so we expect
    // 4-5 rooms with labels and possibly one unlabeled face.
    assert!(
        rooms.len() >= 4 && rooms.len() <= 7,
        "expected 4-7 rooms, got {}",
        rooms.len()
    );
}

#[test]
fn small_floorplan_has_living_room_label() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();
    let rooms = detect_rooms(&walls, &doc.entities, &RoomConfig::default()).unwrap();

    let labels: Vec<String> = rooms.iter().filter_map(|r| r.label.clone()).collect();
    eprintln!("Detected room labels: {labels:?}");

    assert!(
        labels.iter().any(|l| l == "LIVING"),
        "should detect LIVING room, found: {labels:?}"
    );
}

#[test]
fn small_floorplan_room_areas_are_reasonable() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();
    let rooms = detect_rooms(&walls, &doc.entities, &RoomConfig::default()).unwrap();

    let total_area: f64 = rooms.iter().map(|r| r.area_sq_m).sum();
    eprintln!("Total room area: {total_area:.1}m²");

    // Building footprint is 10m × 6.5m = 65m². Room areas should sum to less
    // than that (walls take up some space), but at least 80% of footprint.
    assert!(
        total_area >= 50.0,
        "total room area should be at least 50m² (got {total_area})"
    );
    assert!(
        total_area <= 65.0,
        "total room area should not exceed building footprint 65m² (got {total_area})"
    );
}
