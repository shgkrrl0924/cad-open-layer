//! Streaming DXF parser state machine.
//!
//! Handles HEADER (variable extraction), ENTITIES (top-level entities), and
//! BLOCKS (block definitions with nested entities). TABLES, CLASSES, OBJECTS
//! sections are skipped silently — their contents are not yet promoted to
//! parse events.

use std::io::BufRead;

use cad_core::error::{CadError, ParseWarning, Result};
use cad_core::{BlockDef, Entity, EntityProps, Point};

use crate::builders::build_entity;
use crate::reader::{Pair, PairReader};

/// One semantic event from the DXF stream.
#[derive(Debug, Clone)]
pub enum ParseEvent {
    /// A SECTION started.
    SectionStart(SectionKind),
    /// The current SECTION ended.
    SectionEnd,
    /// HEADER variable: `$INSUNITS`, `$ACADVER`, etc.
    HeaderVariable { name: String, value: String },
    /// Top-level entity from the ENTITIES section.
    Entity(Entity),
    /// A block definition started in the BLOCKS section.
    /// The full block (with all nested entities) is also delivered as
    /// `BlockDefinition` after `BlockDefinitionEnd`.
    BlockDefinitionStart { name: String, base_point: Point },
    /// One entity nested inside the current block.
    EntityInBlock(Entity),
    /// A block definition ended. Carries the fully-assembled BlockDef.
    BlockDefinitionEnd(BlockDef),
    /// End-of-file marker.
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    Header,
    Classes,
    Tables,
    Blocks,
    Entities,
    Objects,
    Thumbnail,
    AcdsData,
    Unknown,
}

impl SectionKind {
    fn from_name(s: &str) -> Self {
        match s {
            "HEADER" => Self::Header,
            "CLASSES" => Self::Classes,
            "TABLES" => Self::Tables,
            "BLOCKS" => Self::Blocks,
            "ENTITIES" => Self::Entities,
            "OBJECTS" => Self::Objects,
            "THUMBNAILIMAGE" => Self::Thumbnail,
            "ACDSDATA" => Self::AcdsData,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Initial,
    InSection(SectionKind),
    /// In ENTITIES section, accumulating a top-level entity's group codes.
    AccumulatingEntity,
    /// In BLOCKS section, accumulating a block header (after `0/BLOCK`).
    InBlockHeader,
    /// In BLOCKS section, accumulating a nested entity inside a block.
    InBlockEntity,
    Done,
}

/// Streaming DXF parser. Yields one `ParseEvent` per call to `next_event`.
pub struct StreamingParser<R: BufRead> {
    reader: PairReader<R>,
    state: State,

    /// Header section: holds the variable name from a `9/$VAR` pair so the
    /// next pair (the value) can be combined.
    pending_header_var: Option<String>,

    /// Entity accumulation (used for both top-level and in-block entities).
    entity_kind: Option<String>,
    entity_groups: Vec<Pair>,

    /// Block header accumulator (between `0/BLOCK` and the first entity or
    /// `0/ENDBLK`).
    block_header_groups: Vec<Pair>,

    /// Currently-being-assembled block. Populated when block header is
    /// finalized; entities inside are appended.
    current_block: Option<BlockDef>,

    /// Non-fatal parse warnings (e.g., malformed numeric values that fell
    /// back to 0.0 / 0). Surfaced on the `DxfDocument` so callers can detect
    /// silent corruption (codex medium finding).
    parse_warnings: Vec<ParseWarning>,
}

impl<R: BufRead> StreamingParser<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: PairReader::new(reader),
            state: State::Initial,
            pending_header_var: None,
            entity_kind: None,
            entity_groups: vec![],
            block_header_groups: vec![],
            current_block: None,
            parse_warnings: vec![],
        }
    }

    /// Drain accumulated parse warnings. Callers should consume this after
    /// the stream has been fully parsed; `parse_all` wires it onto
    /// `DxfDocument::parse_warnings` automatically.
    pub fn take_warnings(&mut self) -> Vec<ParseWarning> {
        std::mem::take(&mut self.parse_warnings)
    }

    /// Read until the next semantic event. Returns `None` only after EOF
    /// has been delivered.
    pub fn next_event(&mut self) -> Result<Option<ParseEvent>> {
        if self.state == State::Done {
            return Ok(None);
        }

        loop {
            let pair = match self.reader.read_pair()? {
                Some(p) => p,
                None => {
                    if let Some(event) = self.flush_anything()? {
                        return Ok(Some(event));
                    }
                    self.state = State::Done;
                    return Ok(Some(ParseEvent::Eof));
                }
            };

            // 0/EOF marker — short-circuit.
            if pair.code == 0 && pair.value == "EOF" {
                if let Some(event) = self.flush_anything()? {
                    self.reader.push_back(pair);
                    return Ok(Some(event));
                }
                self.state = State::Done;
                return Ok(Some(ParseEvent::Eof));
            }

            // Section open/close. Section transitions can happen from any
            // accumulating state — flush first.
            if pair.code == 0 && pair.value == "SECTION" {
                if let Some(event) = self.flush_anything()? {
                    self.reader.push_back(pair);
                    return Ok(Some(event));
                }
                let name_pair = self.reader.read_pair()?.ok_or(CadError::UnexpectedEof {
                    expected: "section name (group 2)",
                    line: self.reader.line_no(),
                })?;
                if name_pair.code != 2 {
                    return Err(CadError::InvalidValue {
                        code: name_pair.code,
                        value: name_pair.value,
                        line: name_pair.line_no,
                        expected: "section name (group code 2)",
                    });
                }
                let kind = SectionKind::from_name(&name_pair.value);
                self.state = State::InSection(kind);
                return Ok(Some(ParseEvent::SectionStart(kind)));
            }

            if pair.code == 0 && pair.value == "ENDSEC" {
                if let Some(event) = self.flush_anything()? {
                    self.reader.push_back(pair);
                    return Ok(Some(event));
                }
                self.state = State::Initial;
                return Ok(Some(ParseEvent::SectionEnd));
            }

            // BLOCKS section: 0/BLOCK starts a new block definition.
            if pair.code == 0
                && pair.value == "BLOCK"
                && matches!(self.state, State::InSection(SectionKind::Blocks))
            {
                self.block_header_groups.clear();
                self.state = State::InBlockHeader;
                continue;
            }

            // BLOCKS section: 0/ENDBLK closes the current block.
            if pair.code == 0 && pair.value == "ENDBLK" {
                // Flush any in-progress block entity first.
                if let Some(event) = self.flush_block_entity()? {
                    self.reader.push_back(pair);
                    return Ok(Some(event));
                }
                if let Some(block_event) = self.close_block()? {
                    return Ok(Some(block_event));
                }
                continue;
            }

            // Dispatch by current state.
            match self.state {
                State::InSection(SectionKind::Header) => {
                    if pair.code == 9 {
                        self.pending_header_var = Some(pair.value);
                    } else if let Some(name) = self.pending_header_var.take() {
                        return Ok(Some(ParseEvent::HeaderVariable {
                            name,
                            value: pair.value,
                        }));
                    }
                    // Otherwise drop (group code outside expected pattern).
                }

                State::InSection(SectionKind::Entities) => {
                    if pair.code == 0 {
                        // 0/<EntityType> starts a new top-level entity.
                        let prev_event = self.flush_top_entity();
                        self.entity_kind = Some(pair.value);
                        self.entity_groups.clear();
                        self.state = State::AccumulatingEntity;
                        if let Some(event) = prev_event {
                            return Ok(Some(event));
                        }
                    }
                    // Other group codes outside an accumulating entity are dropped.
                }
                State::AccumulatingEntity => {
                    if pair.code == 0 {
                        let prev_event = self.flush_top_entity();
                        self.entity_kind = Some(pair.value);
                        self.entity_groups.clear();
                        if let Some(event) = prev_event {
                            return Ok(Some(event));
                        }
                    } else {
                        self.entity_groups.push(pair);
                    }
                }

                State::InBlockHeader => {
                    if pair.code == 0 {
                        // First entity inside the block. Finalize header.
                        self.finalize_block_header()?;
                        self.entity_kind = Some(pair.value);
                        self.entity_groups.clear();
                        self.state = State::InBlockEntity;
                        // Emit BlockDefinitionStart event before accumulating entities.
                        if let Some(b) = &self.current_block {
                            return Ok(Some(ParseEvent::BlockDefinitionStart {
                                name: b.name.clone(),
                                base_point: b.base_point,
                            }));
                        }
                    } else {
                        self.block_header_groups.push(pair);
                    }
                }

                State::InBlockEntity => {
                    if pair.code == 0 {
                        let prev_event = self.flush_block_entity()?;
                        self.entity_kind = Some(pair.value);
                        self.entity_groups.clear();
                        if let Some(event) = prev_event {
                            return Ok(Some(event));
                        }
                    } else {
                        self.entity_groups.push(pair);
                    }
                }

                State::InSection(_) => {
                    // TABLES / CLASSES / OBJECTS / THUMBNAIL / ACDSDATA — silent skip.
                }

                State::Initial | State::Done => {
                    // Skip stray pairs between sections / after EOF.
                }
            }
        }
    }

    /// Flush whatever is currently being accumulated in any state. Used at
    /// section boundaries and EOF.
    fn flush_anything(&mut self) -> Result<Option<ParseEvent>> {
        if !self.entity_groups.is_empty() || self.entity_kind.is_some() {
            match self.state {
                State::AccumulatingEntity => return Ok(self.flush_top_entity()),
                State::InBlockEntity => return self.flush_block_entity(),
                _ => {}
            }
        }
        Ok(None)
    }

    fn flush_top_entity(&mut self) -> Option<ParseEvent> {
        let kind = self.entity_kind.take()?;
        let groups = std::mem::take(&mut self.entity_groups);
        let entity = build_entity(&kind, &groups, &mut self.parse_warnings);
        Some(ParseEvent::Entity(entity))
    }

    fn flush_block_entity(&mut self) -> Result<Option<ParseEvent>> {
        let kind = match self.entity_kind.take() {
            Some(k) => k,
            None => return Ok(None),
        };
        let groups = std::mem::take(&mut self.entity_groups);
        let entity = build_entity(&kind, &groups, &mut self.parse_warnings);
        if let Some(b) = self.current_block.as_mut() {
            b.entities.push(entity.clone());
        }
        Ok(Some(ParseEvent::EntityInBlock(entity)))
    }

    fn finalize_block_header(&mut self) -> Result<()> {
        let groups = std::mem::take(&mut self.block_header_groups);
        let block = block_def_from_groups(&groups, &mut self.parse_warnings);
        self.current_block = Some(block);
        Ok(())
    }

    fn close_block(&mut self) -> Result<Option<ParseEvent>> {
        // It's possible we hit ENDBLK directly after 0/BLOCK without any
        // entities inside. In that case, finalize the header now.
        if self.state == State::InBlockHeader {
            self.finalize_block_header()?;
            // Emit BlockDefinitionStart for empty block.
            // Fall through to also emit End immediately on next call.
            if let Some(b) = self.current_block.take() {
                let event = ParseEvent::BlockDefinitionEnd(b);
                self.state = State::InSection(SectionKind::Blocks);
                return Ok(Some(event));
            }
        }

        if let Some(b) = self.current_block.take() {
            self.state = State::InSection(SectionKind::Blocks);
            return Ok(Some(ParseEvent::BlockDefinitionEnd(b)));
        }
        Ok(None)
    }
}

fn block_def_from_groups(groups: &[Pair], warnings: &mut Vec<ParseWarning>) -> BlockDef {
    let mut name = String::new();
    let mut layer = String::from("0");
    let mut base = Point::new(0.0, 0.0, 0.0);
    let mut flags: i64 = 0;
    let mut props = EntityProps::default();
    let entity = "BLOCK";

    let warn_f = |s: &str, code: i32, warnings: &mut Vec<ParseWarning>| -> f64 {
        match s.trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                warnings.push(ParseWarning {
                    code,
                    value: s.to_string(),
                    kind: "f64",
                    entity,
                });
                0.0
            }
        }
    };
    let warn_i = |s: &str, code: i32, warnings: &mut Vec<ParseWarning>| -> i64 {
        match s.trim().parse::<i64>() {
            Ok(v) => v,
            Err(_) => {
                warnings.push(ParseWarning {
                    code,
                    value: s.to_string(),
                    kind: "i64",
                    entity,
                });
                0
            }
        }
    };

    for g in groups {
        match g.code {
            8 => layer = g.value.clone(),
            2 => name = g.value.clone(),
            10 => base.x = warn_f(&g.value, g.code, warnings),
            20 => base.y = warn_f(&g.value, g.code, warnings),
            30 => base.z = warn_f(&g.value, g.code, warnings),
            70 => flags = warn_i(&g.value, g.code, warnings),
            5 => props.handle = Some(g.value.clone()),
            62 => props.color = Some(warn_i(&g.value, g.code, warnings) as i16),
            6 => props.linetype = Some(g.value.clone()),
            100 => {}
            1001..=1071 => props.xdata.push((g.code, g.value.clone())),
            _ => props.raw_extras.push((g.code, g.value.clone())),
        }
    }

    BlockDef {
        name,
        layer,
        base_point: base,
        flags,
        entities: vec![],
        props,
    }
}
