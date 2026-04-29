//! Low-level group-code pair reader.
//!
//! Reads pairs of lines from the underlying `BufRead`:
//! - Line 1: integer group code
//! - Line 2: string value (interpretation depends on group code)
//!
//! Supports a single-pair pushback so the state machine can lookahead.

use std::io::BufRead;

use cad_core::error::CadError;

/// One line pair: (group code, value as &str). Value is borrowed, lifetime
/// bound to the [`PairReader`]'s internal buffer.
#[derive(Debug, Clone)]
pub struct Pair {
    pub code: i32,
    pub value: String,
    pub line_no: usize,
}

pub struct PairReader<R: BufRead> {
    reader: R,
    line_no: usize,
    pushback: Option<Pair>,
}

impl<R: BufRead> PairReader<R> {
    pub const fn new(reader: R) -> Self {
        Self {
            reader,
            line_no: 0,
            pushback: None,
        }
    }

    pub const fn line_no(&self) -> usize {
        self.line_no
    }

    /// Push a pair back so the next `read_pair` returns it.
    /// At most one pair can be buffered.
    pub fn push_back(&mut self, pair: Pair) {
        debug_assert!(self.pushback.is_none(), "PairReader pushback overflow");
        self.pushback = Some(pair);
    }

    /// Read the next group-code pair, or return `None` at EOF.
    pub fn read_pair(&mut self) -> Result<Option<Pair>, CadError> {
        if let Some(p) = self.pushback.take() {
            return Ok(Some(p));
        }

        let code_line = match self.read_line_trimmed()? {
            Some(s) => s,
            None => return Ok(None),
        };
        let code: i32 = code_line.parse().map_err(|_| CadError::InvalidGroupCode {
            line: self.line_no,
            got: code_line.clone(),
        })?;
        let line_of_pair = self.line_no;

        let value_line = self.read_line_trimmed()?.ok_or(CadError::UnexpectedEof {
            expected: "group value",
            line: self.line_no,
        })?;

        Ok(Some(Pair {
            code,
            value: value_line,
            line_no: line_of_pair,
        }))
    }

    fn read_line_trimmed(&mut self) -> Result<Option<String>, CadError> {
        let mut buf = String::new();
        let n = self.reader.read_line(&mut buf)?;
        if n == 0 {
            return Ok(None);
        }
        self.line_no += 1;
        // Strip trailing CR/LF and any surrounding whitespace.
        let s = buf.trim().to_string();
        Ok(Some(s))
    }
}
