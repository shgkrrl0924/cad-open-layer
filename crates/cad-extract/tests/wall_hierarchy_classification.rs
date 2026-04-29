//! Wall hierarchy classification — verify Exterior/Interior/Partition labels.

use std::fs::File;
use std::io::BufReader;

use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;
use cad_semantic::WallKind;

const SMALL_DXF_PATH: &str = "../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf";

#[test]
fn small_floorplan_classifies_4_exterior_4_partition() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus must exist");
    let doc = parse_all(BufReader::new(file)).unwrap();
    let plan = extract_floorplan(&doc.entities).unwrap();

    let by_kind: std::collections::HashMap<WallKind, usize> =
        plan.walls
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, w| {
                *acc.entry(w.kind).or_insert(0) += 1;
                acc
            });

    eprintln!("Wall hierarchy classification:");
    for (kind, count) in &by_kind {
        eprintln!("  {kind:?}: {count}");
    }

    // Per golden + thickness rule (200mm ≥ 180mm exterior threshold,
    // 50mm < 100mm interior threshold → Partition):
    // 4 exterior wall pairs → 4 Exterior
    // 4 single-line partitions @ 50mm → 4 Partition
    assert_eq!(
        by_kind.get(&WallKind::Exterior).copied().unwrap_or(0),
        4,
        "expected 4 exterior walls (200mm)"
    );
    assert_eq!(
        by_kind.get(&WallKind::Partition).copied().unwrap_or(0),
        4,
        "expected 4 partition walls (50mm)"
    );
}
