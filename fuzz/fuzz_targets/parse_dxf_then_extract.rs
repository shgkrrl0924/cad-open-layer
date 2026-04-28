#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

use cad_dxf_parser::parse_all;
use cad_extract::extract_floorplan;

// Fuzz the full Layer 1 → Layer 3 pipeline. Many algorithms (DCEL,
// parallel-line-pair, room polygonization) operate on parsed geometry —
// they must tolerate adversarial entity shapes (zero-length lines,
// collinear pairs, NaN-from-malformed-coords, etc.) without panic.
fuzz_target!(|data: &[u8]| {
    let Ok(doc) = parse_all(Cursor::new(data)) else { return };
    let _ = extract_floorplan(&doc.entities);
});
