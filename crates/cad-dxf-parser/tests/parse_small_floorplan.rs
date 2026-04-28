//! Integration test — parse the small synthetic floorplan DXF.

use std::fs::File;
use std::io::BufReader;

use cad_core::Entity;
use cad_dxf_parser::{parse_all, ParseEvent, SectionKind, StreamingParser};

const SMALL_DXF_PATH: &str = "../../tests/corpus/synthetic/small_floorplan_simple_r2000.dxf";

#[test]
fn parses_small_floorplan_without_error() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).expect("parse should succeed");

    assert!(!doc.entities.is_empty(), "should produce at least one entity");
    assert!(doc.header.contains_key("$ACADVER"), "should capture $ACADVER variable");
}

#[test]
fn small_floorplan_has_expected_acadver() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    let acadver = doc.header.get("$ACADVER").expect("$ACADVER must exist");
    assert_eq!(acadver, "AC1015", "DXF version should be R2000 (AC1015)");
}

#[test]
fn small_floorplan_has_expected_units() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    let units = doc.header.get("$INSUNITS").expect("$INSUNITS must exist");
    // 4 = Millimeter (per cad_core::DxfUnits::from_dxf_code)
    assert_eq!(units, "4", "units should be 4 (Millimeter)");
}

#[test]
fn small_floorplan_entity_counts_match_golden() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    let mut lines = 0;
    let mut arcs = 0;
    let mut texts = 0;
    let mut mtexts = 0;
    let mut circles = 0;
    let mut lwpolys = 0;
    let mut inserts = 0;
    let mut dimensions = 0;
    let mut other = 0;

    for e in &doc.entities {
        match e {
            Entity::Line { .. } => lines += 1,
            Entity::Arc { .. } => arcs += 1,
            Entity::Text { .. } => texts += 1,
            Entity::MText { .. } => mtexts += 1,
            Entity::Circle { .. } => circles += 1,
            Entity::LwPolyline { .. } => lwpolys += 1,
            Entity::Insert { .. } => inserts += 1,
            Entity::Dimension { .. } => dimensions += 1,
            Entity::Raw(_) => other += 1,
        }
    }

    assert_eq!(mtexts, 0, "small DXF has no MTEXT");
    assert_eq!(dimensions, 0, "small DXF uses LINE+ARC for dimension drawing, not DIMENSION entity");

    // From manual analysis of the small DXF (see golden JSON):
    // - Plenty of LINEs: 4 outer walls + 4 inner walls + 4 partition lines + 18 tick marks
    //   + 12 window lines (6 windows × 2) + 6 door leaves + 12 dimension extension lines
    //   + 18 furniture lines + 8 grid lines = many. Just sanity-check it's substantial.
    // - 6 ARCs (door swings).
    // - 14 TEXTs (5 room labels + 2 dimension texts + 1 dim text + 8 grid labels).
    // - 3 CIRCLEs (furniture: stove burners + table).
    assert!(lines > 50, "expected many LINE entities (got {lines})");
    assert_eq!(arcs, 6, "expected 6 ARC entities (door swings)");
    assert!(texts >= 13, "expected at least 13 TEXT entities (got {texts})");
    assert_eq!(circles, 3, "expected 3 CIRCLE entities (furniture)");
    assert_eq!(lwpolys, 0, "small DXF has no LWPOLYLINE");
    assert_eq!(inserts, 0, "small DXF has no INSERT (uses LINE+ARC for doors)");
    assert_eq!(other, 0, "no unrecognized entity types");
}

#[test]
fn small_floorplan_first_wall_line_coords() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus DXF must exist");
    let reader = BufReader::new(file);
    let doc = parse_all(reader).unwrap();

    // First entity should be the bottom exterior wall: (0,0,0) → (10000,0,0).
    let first = doc.entities.first().expect("at least one entity");
    if let Entity::Line { p1, p2, layer, .. } = first {
        assert_eq!(layer, "WALLS", "first entity should be on WALLS layer");
        assert!((p1.x - 0.0).abs() < 1e-6);
        assert!((p1.y - 0.0).abs() < 1e-6);
        assert!((p2.x - 10000.0).abs() < 1e-6);
        assert!((p2.y - 0.0).abs() < 1e-6);
    } else {
        panic!("first entity should be a LINE, got {first:?}");
    }
}

#[test]
fn streaming_iteration_yields_section_events() {
    let file = File::open(SMALL_DXF_PATH).expect("test corpus DXF must exist");
    let reader = BufReader::new(file);
    let mut parser = StreamingParser::new(reader);

    let mut header_seen = false;
    let mut entities_seen = false;
    let mut eof_seen = false;

    while let Some(event) = parser.next_event().unwrap() {
        match event {
            ParseEvent::SectionStart(SectionKind::Header) => header_seen = true,
            ParseEvent::SectionStart(SectionKind::Entities) => entities_seen = true,
            ParseEvent::Eof => {
                eof_seen = true;
                break;
            }
            _ => {}
        }
    }

    assert!(header_seen, "should see HEADER section");
    assert!(entities_seen, "should see ENTITIES section");
    assert!(eof_seen, "should reach EOF");
}
