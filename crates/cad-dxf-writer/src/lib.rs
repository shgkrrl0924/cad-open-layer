//! DXF file emitter. Layer 1 (write side) of CAD Open Layer.
//!
//! Writes ASCII DXF format compatible with `AutoCAD` R2000 (AC1015) onward.
//!
//! Two APIs:
//! - [`DxfWriter`] — incremental streaming writer. Caller controls section
//!   order: HEADER → TABLES → BLOCKS → ENTITIES → EOF.
//! - [`write_dxf`] — convenience all-in-one function.

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

mod entity_emit;
pub mod layer;

use std::io::{BufWriter, Write};

use cad_core::error::Result;
use cad_core::{BlockDef, Entity};

pub use layer::LayerDef;

/// Streaming DXF writer.
pub struct DxfWriter<W: Write> {
    inner: BufWriter<W>,
}

impl<W: Write> DxfWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            inner: BufWriter::new(writer),
        }
    }

    /// Write the HEADER section with the given variable list. Common vars:
    /// `("$ACADVER", "AC1015")`, `("$INSUNITS", "4")`.
    pub fn write_header(&mut self, vars: &[(&str, &str)]) -> Result<()> {
        self.write_pair(0, "SECTION")?;
        self.write_pair(2, "HEADER")?;
        for (name, value) in vars {
            self.write_pair(9, name)?;
            // Group code for the value depends on the variable. For string
            // variables ($ACADVER) it's 1; for integer variables ($INSUNITS)
            // it's 70. Heuristic: try int first, fall back to string.
            if value.parse::<i64>().is_ok() {
                self.write_pair(70, value)?;
            } else {
                self.write_pair(1, value)?;
            }
        }
        self.write_pair(0, "ENDSEC")?;
        Ok(())
    }

    /// Write the TABLES section containing LAYER table and `BLOCK_RECORD` table.
    pub fn write_tables(&mut self, layers: &[LayerDef], block_names: &[&str]) -> Result<()> {
        self.write_pair(0, "SECTION")?;
        self.write_pair(2, "TABLES")?;

        // LAYER table
        self.write_pair(0, "TABLE")?;
        self.write_pair(2, "LAYER")?;
        self.write_pair(70, &format!("{}", layers.len()))?;
        for layer in layers {
            self.write_pair(0, "LAYER")?;
            self.write_pair(2, &layer.name)?;
            self.write_pair(70, &format!("{}", layer.flags))?;
            self.write_pair(62, &format!("{}", layer.color))?;
            self.write_pair(6, &layer.linetype)?;
        }
        self.write_pair(0, "ENDTAB")?;

        // BLOCK_RECORD table — one record per block.
        self.write_pair(0, "TABLE")?;
        self.write_pair(2, "BLOCK_RECORD")?;
        self.write_pair(70, &format!("{}", block_names.len() + 2))?;
        // Required built-in records
        for name in &["*Model_Space", "*Paper_Space"] {
            self.write_pair(0, "BLOCK_RECORD")?;
            self.write_pair(2, name)?;
        }
        for name in block_names {
            self.write_pair(0, "BLOCK_RECORD")?;
            self.write_pair(2, name)?;
        }
        self.write_pair(0, "ENDTAB")?;

        self.write_pair(0, "ENDSEC")?;
        Ok(())
    }

    /// Write the BLOCKS section. Each block becomes a BLOCK ... ENDBLK record
    /// containing its nested entities.
    pub fn write_blocks(&mut self, blocks: &[BlockDef]) -> Result<()> {
        self.write_pair(0, "SECTION")?;
        self.write_pair(2, "BLOCKS")?;

        // Built-in *Model_Space and *Paper_Space.
        for name in &["*Model_Space", "*Paper_Space"] {
            self.write_pair(0, "BLOCK")?;
            self.write_pair(8, "0")?;
            self.write_pair(2, name)?;
            self.write_pair(70, "0")?;
            self.write_pair(10, "0.000")?;
            self.write_pair(20, "0.000")?;
            self.write_pair(30, "0.000")?;
            self.write_pair(3, name)?;
            self.write_pair(1, "")?;
            self.write_pair(0, "ENDBLK")?;
            self.write_pair(8, "0")?;
        }

        for block in blocks {
            self.write_pair(0, "BLOCK")?;
            self.write_pair(8, &block.layer)?;
            self.write_pair(2, &block.name)?;
            self.write_pair(70, &format!("{}", block.flags))?;
            self.write_pair(10, &format_f(block.base_point.x))?;
            self.write_pair(20, &format_f(block.base_point.y))?;
            self.write_pair(30, &format_f(block.base_point.z))?;
            self.write_pair(3, &block.name)?;
            self.write_pair(1, "")?;
            for e in &block.entities {
                self.write_entity(e)?;
            }
            self.write_pair(0, "ENDBLK")?;
            self.write_pair(8, &block.layer)?;
        }

        self.write_pair(0, "ENDSEC")?;
        Ok(())
    }

    /// Begin the ENTITIES section. Caller then calls `write_entity` repeatedly,
    /// then `end_entities`.
    pub fn begin_entities(&mut self) -> Result<()> {
        self.write_pair(0, "SECTION")?;
        self.write_pair(2, "ENTITIES")?;
        Ok(())
    }

    pub fn end_entities(&mut self) -> Result<()> {
        self.write_pair(0, "ENDSEC")?;
        Ok(())
    }

    /// Write a single entity to the current section.
    pub fn write_entity(&mut self, entity: &Entity) -> Result<()> {
        entity_emit::emit_entity(&mut self.inner, entity)
    }

    /// Finalize the file with the EOF marker and flush.
    pub fn finish(mut self) -> Result<()> {
        self.write_pair(0, "EOF")?;
        self.inner.flush()?;
        Ok(())
    }

    pub(crate) fn write_pair(&mut self, code: i32, value: &str) -> Result<()> {
        write_pair_to(&mut self.inner, code, value)
    }
}

pub(crate) fn write_pair_to<W: Write>(w: &mut W, code: i32, value: &str) -> Result<()> {
    writeln!(w, "{code}")?;
    writeln!(w, "{value}")?;
    Ok(())
}

pub(crate) fn format_f(value: f64) -> String {
    // 3 decimal places matches the synthetic corpus convention; readers
    // accept any precision so this is purely cosmetic.
    format!("{value:.3}")
}

/// Convenience: write a complete DXF file in one call.
pub fn write_dxf<W: Write>(
    writer: W,
    header_vars: &[(&str, &str)],
    layers: &[LayerDef],
    blocks: &[BlockDef],
    entities: &[Entity],
) -> Result<()> {
    let mut w = DxfWriter::new(writer);
    w.write_header(header_vars)?;
    let block_names: Vec<&str> = blocks.iter().map(|b| b.name.as_str()).collect();
    w.write_tables(layers, &block_names)?;
    w.write_blocks(blocks)?;
    w.begin_entities()?;
    for e in entities {
        w.write_entity(e)?;
    }
    w.end_entities()?;
    w.finish()?;
    Ok(())
}
