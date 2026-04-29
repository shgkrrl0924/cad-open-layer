//! Verify that wall layer names round-trip through synthesis.
//!
//! Codex+advisor flagged: `floorplan_to_dxf` was collapsing every wall onto
//! `config.wall_layer`, so Korean ("외벽"/"내벽") or English ("EXTR-..."/"INTR-...")
//! layer hints would be lost on a synthesize → reparse cycle, breaking the
//! `WallKind` classification round-trip.

use std::io::Cursor;

use cad_core::Point;
use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;
use cad_geometry::polyline::Polyline;
use cad_semantic::{Floorplan, Wall, WallKind};
use cad_synthesize::{floorplan_to_dxf, SynthesizeConfig};

fn wall(id: u32, layer: &str, p1: Point, p2: Point) -> Wall {
    Wall {
        id,
        centerline: Polyline::new(vec![p1, p2], false),
        thickness: 50.0, // sub-partition thickness — only layer hint can save it
        height: None,
        layer: layer.into(),
        kind: WallKind::Unknown, // re-classified by extract_floorplan
        openings: vec![],
    }
}

#[test]
fn korean_layer_hint_survives_synthesis() {
    // Two walls with thickness 50mm (would be Partition by thickness rule),
    // but their Korean layer names should override to Exterior / Interior.
    let plan = Floorplan {
        walls: vec![
            wall(
                1,
                "외벽",
                Point::new(0.0, 0.0, 0.0),
                Point::new(10000.0, 0.0, 0.0),
            ),
            wall(
                2,
                "내벽",
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 5000.0, 0.0),
            ),
        ],
        ..Default::default()
    };

    let mut buf: Vec<u8> = vec![];
    floorplan_to_dxf(&plan, &mut buf, &SynthesizeConfig::default()).unwrap();

    let parsed = parse_all(Cursor::new(&buf)).unwrap();
    let regen = extract_floorplan(&parsed.entities).unwrap();

    // Re-extracted walls should pick up the original Korean layer names and
    // be classified accordingly. (Note: extract_floorplan re-pairs LINEs into
    // walls, so wall identity is by geometry, not original WallId.)
    let exterior_count = regen
        .walls
        .iter()
        .filter(|w| w.kind == WallKind::Exterior)
        .count();
    let interior_count = regen
        .walls
        .iter()
        .filter(|w| w.kind == WallKind::Interior)
        .count();

    eprintln!(
        "Layers in regen: {:?}",
        regen
            .walls
            .iter()
            .map(|w| w.layer.as_str())
            .collect::<Vec<_>>()
    );

    assert!(
        exterior_count >= 1,
        "외벽 layer hint must survive synthesis → at least 1 Exterior wall expected, got {exterior_count}"
    );
    assert!(
        interior_count >= 1,
        "내벽 layer hint must survive synthesis → at least 1 Interior wall expected, got {interior_count}"
    );
}
