//! Error types for CAD Open Layer.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CadError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid DXF group code at line {line}: {got}")]
    InvalidGroupCode { line: usize, got: String },

    #[error("Unexpected EOF: expected {expected}, line {line}")]
    UnexpectedEof { expected: &'static str, line: usize },

    #[error("Invalid value for group code {code} at line {line}: got {value}, expected {expected}")]
    InvalidValue {
        code: i32,
        value: String,
        line: usize,
        expected: &'static str,
    },

    #[error("Unsupported DXF version: {found}")]
    UnsupportedVersion { found: String },

    #[error("Semantic inconsistency: {0}")]
    SemanticInconsistency(String),

    #[error("Extraction error: {0}")]
    Extract(String),

    #[error("Synthesis error: {0}")]
    Synthesize(String),
}

pub type Result<T> = std::result::Result<T, CadError>;

/// A non-fatal parsing issue. Recorded into `DxfDocument::parse_warnings` so
/// the caller can detect silent corruption (e.g., a malformed coordinate that
/// would otherwise default to `0.0`). The parser is intentionally lenient
/// because real-world DXFs are messy, but every lenience must be visible.
#[derive(Debug, Clone)]
pub struct ParseWarning {
    pub code: i32,
    pub value: String,
    pub kind: &'static str,
    pub entity: &'static str,
}
