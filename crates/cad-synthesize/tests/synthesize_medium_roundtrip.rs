//! Medium-scale round-trip: 1.8MB / 23k entities multifamily DXF.
//! parse → extract → synthesize → parse → extract, then compare counts.

use std::fs::File;
use std::io::{BufReader, Cursor};
use std::time::Instant;

use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;
use cad_semantic::{OpeningKind, WallKind};
use cad_synthesize::{floorplan_to_dxf, SynthesizeConfig};

const MEDIUM_DXF_PATH: &str =
    "../../tests/corpus/synthetic/medium_multifamily_floorplan_synthetic_r2000.dxf";

#[test]
fn medium_round_trip_pipeline_completes() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus must exist");
    let original = extract_floorplan(&parse_all(BufReader::new(file)).unwrap().entities).unwrap();

    let synth_start = Instant::now();
    let mut buf: Vec<u8> = vec![];
    floorplan_to_dxf(&original, &mut buf, &SynthesizeConfig::default()).unwrap();
    let synth_elapsed = synth_start.elapsed();

    eprintln!(
        "MEDIUM SYNTHESIZE: {} bytes in {:?}",
        buf.len(),
        synth_elapsed
    );

    let parse_start = Instant::now();
    let parsed = parse_all(Cursor::new(&buf)).unwrap();
    let parse_elapsed = parse_start.elapsed();

    let extract_start = Instant::now();
    let regenerated = extract_floorplan(&parsed.entities).unwrap();
    let extract_elapsed = extract_start.elapsed();

    eprintln!("MEDIUM ROUND-TRIP: parse {parse_elapsed:?}, extract {extract_elapsed:?}");
    eprintln!(
        "  ORIG:  {} walls, {} openings, {} rooms, {} dimensions",
        original.walls.len(),
        original.openings.len(),
        original.rooms.len(),
        original.dimensions.len()
    );
    eprintln!(
        "  REGEN: {} walls, {} openings, {} rooms, {} dimensions",
        regenerated.walls.len(),
        regenerated.openings.len(),
        regenerated.rooms.len(),
        regenerated.dimensions.len()
    );

    // Wall count must be exactly preserved (parallel-line-pair → centerline → 2 lines → re-pair).
    assert_eq!(
        original.walls.len(),
        regenerated.walls.len(),
        "wall count must round-trip"
    );

    // Wall classification must round-trip. NOTE: synthesis writes all walls to
    // the single "WALLS" layer, so this currently relies on thickness-based
    // classification reproducing the same kind. Korean/English layer-hint
    // overrides on the original side would NOT survive — flag any mismatch.
    let mut orig_kinds: std::collections::HashMap<WallKind, usize> = Default::default();
    let mut regen_kinds: std::collections::HashMap<WallKind, usize> = Default::default();
    for w in &original.walls {
        *orig_kinds.entry(w.kind).or_insert(0) += 1;
    }
    for w in &regenerated.walls {
        *regen_kinds.entry(w.kind).or_insert(0) += 1;
    }
    eprintln!("Wall kinds:  orig={orig_kinds:?}, regen={regen_kinds:?}");
    assert_eq!(
        orig_kinds, regen_kinds,
        "wall kind histogram must round-trip"
    );
}

#[test]
fn medium_round_trip_preserves_door_window_counts() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus must exist");
    let original = extract_floorplan(&parse_all(BufReader::new(file)).unwrap().entities).unwrap();

    let mut buf: Vec<u8> = vec![];
    floorplan_to_dxf(&original, &mut buf, &SynthesizeConfig::default()).unwrap();
    let regenerated = extract_floorplan(&parse_all(Cursor::new(&buf)).unwrap().entities).unwrap();

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

    eprintln!("Doors:   orig={orig_doors}, regen={regen_doors}");
    eprintln!("Windows: orig={orig_windows}, regen={regen_windows}");

    // INSERT-based detection should preserve all openings exactly.
    assert_eq!(orig_doors, regen_doors, "door count must round-trip");
    assert_eq!(orig_windows, regen_windows, "window count must round-trip");
}

#[test]
fn medium_round_trip_preserves_grid() {
    let file = File::open(MEDIUM_DXF_PATH).expect("medium corpus must exist");
    let original = extract_floorplan(&parse_all(BufReader::new(file)).unwrap().entities).unwrap();

    let mut buf: Vec<u8> = vec![];
    floorplan_to_dxf(&original, &mut buf, &SynthesizeConfig::default()).unwrap();
    let regenerated = extract_floorplan(&parse_all(Cursor::new(&buf)).unwrap().entities).unwrap();

    assert!(!original.grids.is_empty());
    assert!(!regenerated.grids.is_empty());
    let og = &original.grids[0];
    let rg = &regenerated.grids[0];
    eprintln!(
        "Grid: orig X={}/Y={}, regen X={}/Y={}",
        og.x_axes.len(),
        og.y_axes.len(),
        rg.x_axes.len(),
        rg.y_axes.len()
    );
    assert_eq!(
        og.x_axes.len(),
        rg.x_axes.len(),
        "X-axes count must round-trip"
    );
    assert_eq!(
        og.y_axes.len(),
        rg.y_axes.len(),
        "Y-axes count must round-trip"
    );
}
