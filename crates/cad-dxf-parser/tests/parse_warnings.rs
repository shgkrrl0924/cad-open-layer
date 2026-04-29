//! Verify that malformed numeric DXF values surface as `ParseWarning` instead
//! of being silently coerced to 0.0/0.
//!
//! Codex adversarial review (medium): "Invalid numeric DXF values are
//! silently converted to zero" — bad coordinates become origin geometry.

use std::io::Cursor;

use cad_dxf_parser::parse_all;

const MALFORMED_LINE_DXF: &str = "\
0
SECTION
2
ENTITIES
0
LINE
8
WALLS
10
not-a-number
20
0.0
11
1000.0
21
0.0
0
ENDSEC
0
EOF
";

#[test]
fn malformed_coordinate_records_warning() {
    let doc = parse_all(Cursor::new(MALFORMED_LINE_DXF)).unwrap();

    // Parse succeeds (lenient mode), but a warning records the bad coordinate.
    assert_eq!(doc.entities.len(), 1, "LINE entity is still produced");
    assert!(
        !doc.parse_warnings.is_empty(),
        "must record at least one ParseWarning for the malformed group 10 value"
    );

    let bad = doc
        .parse_warnings
        .iter()
        .find(|w| w.code == 10 && w.value == "not-a-number")
        .expect("warning for the malformed group 10 must be present");
    assert_eq!(bad.entity, "LINE");
    assert_eq!(bad.kind, "f64");
}

#[test]
fn well_formed_dxf_has_no_warnings() {
    const CLEAN: &str = "\
0
SECTION
2
ENTITIES
0
LINE
8
WALLS
10
0.0
20
0.0
11
1000.0
21
0.0
0
ENDSEC
0
EOF
";
    let doc = parse_all(Cursor::new(CLEAN)).unwrap();
    assert_eq!(doc.entities.len(), 1);
    assert!(
        doc.parse_warnings.is_empty(),
        "clean DXF must produce zero warnings, got {:?}",
        doc.parse_warnings
    );
}
