//! Round-trip test: build a Floorplan-ish set of entities, emit to DXF,
//! then parse it back and verify equivalence.

use std::io::Cursor;

use cad_core::{Entity, EntityProps, Point};
use cad_dxf_parser::parse_all;
use cad_dxf_writer::{layer::standard_arch_layers, write_dxf};

fn line(p1: Point, p2: Point, layer: &str) -> Entity {
    Entity::Line {
        p1,
        p2,
        layer: layer.into(),
        props: EntityProps::default(),
    }
}

fn arc(c: Point, r: f64, start_deg: f64, end_deg: f64, layer: &str) -> Entity {
    Entity::Arc {
        center: c,
        radius: r,
        start_angle: start_deg.to_radians(),
        end_angle: end_deg.to_radians(),
        layer: layer.into(),
        props: EntityProps::default(),
    }
}

fn text(pos: Point, value: &str, layer: &str) -> Entity {
    Entity::Text {
        position: pos,
        value: value.into(),
        height: 250.0,
        rotation: 0.0,
        layer: layer.into(),
        props: EntityProps::default(),
    }
}

#[test]
fn round_trip_basic_entities() {
    let entities = vec![
        line(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10000.0, 0.0, 0.0),
            "WALLS",
        ),
        line(
            Point::new(10000.0, 0.0, 0.0),
            Point::new(10000.0, 6500.0, 0.0),
            "WALLS",
        ),
        arc(Point::new(4000.0, 1200.0, 0.0), 800.0, 0.0, 90.0, "DOORS"),
        text(Point::new(800.0, 1200.0, 0.0), "LIVING", "TEXT"),
    ];

    let mut buf: Vec<u8> = vec![];
    write_dxf(
        &mut buf,
        &[("$ACADVER", "AC1015"), ("$INSUNITS", "4")],
        &standard_arch_layers(),
        &[],
        &entities,
    )
    .unwrap();

    eprintln!("DXF output size: {} bytes", buf.len());

    let parsed = parse_all(Cursor::new(&buf)).unwrap();

    assert_eq!(parsed.entities.len(), 4, "should round-trip 4 entities");
    assert_eq!(
        parsed.header.get("$ACADVER").map(String::as_str),
        Some("AC1015")
    );

    // Verify first entity is the LINE.
    if let Entity::Line { p1, p2, layer, .. } = &parsed.entities[0] {
        assert_eq!(layer, "WALLS");
        assert!((p1.x - 0.0).abs() < 1e-6);
        assert!((p2.x - 10000.0).abs() < 1e-6);
    } else {
        panic!(
            "first entity should be a LINE, got {:?}",
            parsed.entities[0]
        );
    }

    // Verify ARC angles preserved (degrees → radians → degrees).
    if let Entity::Arc {
        radius,
        start_angle,
        end_angle,
        layer,
        ..
    } = &parsed.entities[2]
    {
        assert_eq!(layer, "DOORS");
        assert!((radius - 800.0).abs() < 1e-6);
        assert!((start_angle - 0.0).abs() < 1e-6);
        assert!((end_angle - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
    } else {
        panic!(
            "third entity should be an ARC, got {:?}",
            parsed.entities[2]
        );
    }

    // Verify TEXT value preserved.
    if let Entity::Text { value, layer, .. } = &parsed.entities[3] {
        assert_eq!(layer, "TEXT");
        assert_eq!(value, "LIVING");
    } else {
        panic!("fourth entity should be TEXT");
    }
}

#[test]
fn round_trip_with_block_definitions() {
    use cad_core::BlockDef;

    let block = BlockDef {
        name: "DOOR_800".into(),
        layer: "DOORS".into(),
        base_point: Point::new(0.0, 0.0, 0.0),
        flags: 0,
        entities: vec![
            line(
                Point::new(0.0, 0.0, 0.0),
                Point::new(800.0, 0.0, 0.0),
                "DOORS",
            ),
            arc(Point::new(0.0, 0.0, 0.0), 800.0, 0.0, 90.0, "DOORS"),
        ],
        props: EntityProps::default(),
    };

    let entities = vec![Entity::Insert {
        block_name: "DOOR_800".into(),
        position: Point::new(4000.0, 1200.0, 0.0),
        scale_x: 1.0,
        scale_y: 1.0,
        scale_z: 1.0,
        rotation: 0.0,
        layer: "DOORS".into(),
        props: EntityProps::default(),
    }];

    let mut buf: Vec<u8> = vec![];
    write_dxf(
        &mut buf,
        &[("$ACADVER", "AC1015"), ("$INSUNITS", "4")],
        &standard_arch_layers(),
        &[block],
        &entities,
    )
    .unwrap();

    let parsed = parse_all(Cursor::new(&buf)).unwrap();

    assert_eq!(parsed.entities.len(), 1, "should round-trip 1 INSERT");
    // Filter out built-in *Model_Space / *Paper_Space which the writer always emits.
    let user_blocks: Vec<_> = parsed
        .blocks
        .iter()
        .filter(|b| !b.name.starts_with('*'))
        .collect();
    assert_eq!(
        user_blocks.len(),
        1,
        "should round-trip 1 user BLOCK definition"
    );
    assert_eq!(user_blocks[0].name, "DOOR_800");
    assert_eq!(user_blocks[0].entities.len(), 2);

    if let Entity::Insert {
        block_name,
        position,
        ..
    } = &parsed.entities[0]
    {
        assert_eq!(block_name, "DOOR_800");
        assert!((position.x - 4000.0).abs() < 1e-6);
        assert!((position.y - 1200.0).abs() < 1e-6);
    } else {
        panic!("entity should be INSERT");
    }
}

#[test]
fn raw_extras_survive_round_trip() {
    // A LINE with an unrecognized group code (here: 999, a TEXT-style alignment
    // we don't model). The parser must capture it into raw_extras, and the
    // writer must re-emit it so a parse → write → parse cycle preserves it.
    //
    // Codex flagged this as a [high] defect: emit_extras was dropping
    // raw_extras entirely.
    let mut props = EntityProps::default();
    props.raw_extras.push((999, "preserved-marker".into()));

    let entities = vec![Entity::Line {
        p1: Point::new(0.0, 0.0, 0.0),
        p2: Point::new(1000.0, 0.0, 0.0),
        layer: "WALLS".into(),
        props,
    }];

    let mut buf: Vec<u8> = vec![];
    write_dxf(
        &mut buf,
        &[("$ACADVER", "AC1015"), ("$INSUNITS", "4")],
        &standard_arch_layers(),
        &[],
        &entities,
    )
    .unwrap();

    let parsed = parse_all(Cursor::new(&buf)).unwrap();
    assert_eq!(parsed.entities.len(), 1);

    let extras = match &parsed.entities[0] {
        Entity::Line { props, .. } => &props.raw_extras,
        _ => panic!("expected LINE"),
    };
    let has_marker = extras
        .iter()
        .any(|(c, v)| *c == 999 && v == "preserved-marker");
    assert!(
        has_marker,
        "raw_extras (999) must survive write/re-parse, got {extras:?}"
    );
}

#[test]
fn dxf_output_parses_as_valid_r2000() {
    // Smoke test: minimal valid DXF with no entities still parses.
    let mut buf: Vec<u8> = vec![];
    write_dxf(
        &mut buf,
        &[("$ACADVER", "AC1015"), ("$INSUNITS", "4")],
        &standard_arch_layers(),
        &[],
        &[],
    )
    .unwrap();

    let parsed = parse_all(Cursor::new(&buf)).unwrap();
    assert_eq!(parsed.entities.len(), 0);
    assert_eq!(
        parsed.header.get("$ACADVER").map(String::as_str),
        Some("AC1015")
    );
}
