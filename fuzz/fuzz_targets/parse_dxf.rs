#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

use cad_dxf_parser::parse_all;

// Fuzz the streaming parser with arbitrary byte input. Goal: prove the
// parser never panics, only returns Result::Err on malformed input. ASCII
// DXF is a structured text format, so most random byte streams are
// rejected; the rare valid-shaped ones must round-trip cleanly without
// abort/UB. The parser must not be vulnerable to crash via:
//   - non-UTF8 sequences
//   - truncated mid-pair input
//   - mismatched group code/value pairs
//   - deeply nested SECTION/BLOCK/ENTITY constructs
fuzz_target!(|data: &[u8]| {
    let _ = parse_all(Cursor::new(data));
});
