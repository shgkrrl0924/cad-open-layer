//! Streaming DXF file parser.
//!
//! Layer 1 of CAD Open Layer. Reads ASCII DXF format and emits a stream of
//! `ParseEvent` values. Use [`parse_all`] for the convenience all-in-one
//! call, or [`StreamingParser`] for incremental processing.
//!
//! See `docs/algorithms.md` and `docs/deep-dive.md` §2 for the FSM design.

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

mod builders;
mod parser;
mod reader;

pub use parser::{ParseEvent, SectionKind, StreamingParser};

use std::collections::HashMap;
use std::io::BufRead;

use cad_core::error::{ParseWarning, Result};
use cad_core::{BlockDef, Entity};

/// In-memory representation of a fully-parsed DXF file.
#[derive(Debug, Default)]
pub struct DxfDocument {
    /// HEADER section variables. Keys are variable names like `$INSUNITS`.
    pub header: HashMap<String, String>,
    /// All top-level entities from the ENTITIES section.
    pub entities: Vec<Entity>,
    /// Block definitions from the BLOCKS section.
    pub blocks: Vec<BlockDef>,
    /// Non-fatal parse warnings (malformed numeric values that fell back to
    /// 0.0/0). Empty for well-formed files. Inspect this to detect silent
    /// corruption that would otherwise become origin geometry.
    pub parse_warnings: Vec<ParseWarning>,
}

impl DxfDocument {
    /// Apply a single parse event to the in-memory document.
    pub fn apply(&mut self, event: ParseEvent) {
        match event {
            ParseEvent::HeaderVariable { name, value } => {
                self.header.insert(name, value);
            }
            ParseEvent::Entity(e) => self.entities.push(e),
            ParseEvent::BlockDefinitionEnd(b) => self.blocks.push(b),
            _ => {}
        }
    }
}

/// Parse a DXF file in one shot, returning an in-memory document.
pub fn parse_all<R: BufRead>(reader: R) -> Result<DxfDocument> {
    let mut parser = StreamingParser::new(reader);
    let mut doc = DxfDocument::default();
    while let Some(event) = parser.next_event()? {
        doc.apply(event);
    }
    doc.parse_warnings = parser.take_warnings();
    Ok(doc)
}
