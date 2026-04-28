//! Integration test — wall extraction against the small floorplan corpus.

use std::fs::File;
use std::io::BufReader;

use cad_dxf_parser::parse_all;
use cad_extract::wall::ParallelLinePairExtractor;
use cad_extract::WallExtractor;

const SMALL_DXF_PATH: &str = "../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf";

#[test]
fn extracts_walls_from_small_floorplan() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();

    eprintln!("Extracted {} walls from small floorplan", walls.len());
    for w in &walls {
        eprintln!("  W{}: thickness={:.0}mm, kind={:?}", w.id, w.thickness, w.kind);
    }

    // Per golden JSON: 4 exterior walls (paired, 200mm thick) + 4 interior
    // partitions (single-line, 50mm default) = 8 walls total.
    assert!(
        walls.len() >= 6 && walls.len() <= 12,
        "expected 6-12 walls (golden = 8), got {}",
        walls.len()
    );
}

#[test]
fn small_floorplan_exterior_walls_have_200mm_thickness() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();

    let walls = ParallelLinePairExtractor::default()
        .extract(&doc.entities)
        .unwrap();

    let exterior: Vec<_> = walls
        .iter()
        .filter(|w| (w.thickness - 200.0).abs() < 1.0)
        .collect();

    assert_eq!(
        exterior.len(),
        4,
        "expected 4 exterior wall pairs at 200mm thickness, got {}",
        exterior.len()
    );
}
