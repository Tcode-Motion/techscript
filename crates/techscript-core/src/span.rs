// ── TechScript Span — Source Location Tracking ──────────────────────
// Every token, AST node, and error carries a Span for precise diagnostics.

use std::sync::Arc;

/// A span in source code, tracking file, line, column, and length.
#[derive(Debug, Clone)]
pub struct Span {
    pub file: Arc<str>,
    pub line: usize,
    pub col: usize,
    pub length: usize,
}

impl Span {
    pub fn new(file: Arc<str>, line: usize, col: usize, length: usize) -> Self {
        Span { file, line, col, length }
    }

    /// Create a dummy span for generated code / builtins.
    pub fn dummy() -> Self {
        Span {
            file: Arc::from("<builtin>"),
            line: 0,
            col: 0,
            length: 0,
        }
    }

    /// Merge two spans into one that covers both.
    pub fn merge(&self, other: &Span) -> Span {
        let start_line = self.line.min(other.line);
        let end_line = self.line.max(other.line);
        let start_col = if self.line <= other.line { self.col } else { other.col };
        let end_col = if self.line >= other.line {
            self.col + self.length
        } else {
            other.col + other.length
        };
        Span {
            file: self.file.clone(),
            line: start_line,
            col: start_col,
            length: if start_line == end_line { end_col - start_col } else { self.length },
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Span::dummy()
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}
