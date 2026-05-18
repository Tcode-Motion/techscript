// ── TechScript Error Handling ────────────────────────────────────────
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Lexer,
    Parse,
    Compile,
    Runtime,
}

#[derive(Debug, Clone)]
pub struct TechError {
    pub kind: ErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub file: String,
}

impl TechError {
    pub fn new(kind: ErrorKind, message: String, line: usize, column: usize, file: &str) -> Self {
        TechError { kind, message, line, column, file: file.to_string() }
    }

    pub fn lexer(msg: impl Into<String>, line: usize, col: usize, file: &str) -> Self {
        Self::new(ErrorKind::Lexer, msg.into(), line, col, file)
    }

    pub fn parse(msg: impl Into<String>, line: usize, col: usize, file: &str) -> Self {
        Self::new(ErrorKind::Parse, msg.into(), line, col, file)
    }

    pub fn compile(msg: impl Into<String>, line: usize, col: usize) -> Self {
        Self::new(ErrorKind::Compile, msg.into(), line, col, "<unknown>")
    }

    pub fn runtime(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::Runtime, msg.into(), 0, 0, "<unknown>")
    }

    pub fn runtime_at(msg: impl Into<String>, line: usize) -> Self {
        Self::new(ErrorKind::Runtime, msg.into(), line, 0, "<unknown>")
    }
}

impl fmt::Display for TechError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind_str = match self.kind {
            ErrorKind::Lexer => "Syntax error",
            ErrorKind::Parse => "Syntax error",
            ErrorKind::Compile => "Compile error",
            ErrorKind::Runtime => "Runtime error",
        };
        write!(f, "{} in {}:{}: {}", kind_str, self.file, self.line, self.message)
    }
}

pub type TechResult<T> = Result<T, TechError>;

/// Return close matches for "Did you mean?" suggestions.
pub fn suggest_correction(unknown: &str, known_words: &[&str]) -> Vec<String> {
    known_words
        .iter()
        .filter(|w| strsim::jaro_winkler(unknown, w) > 0.75)
        .map(|s| s.to_string())
        .take(3)
        .collect()
}

/// Format an error with source code context and optional suggestions.
pub fn format_error(error: &TechError, source_lines: &[&str]) -> String {
    format_error_with_hints(error, source_lines, &[])
}

pub fn format_error_with_hints(error: &TechError, source_lines: &[&str], known_names: &[&str]) -> String {
    let mut out = String::new();

    let kind_str = match error.kind {
        ErrorKind::Lexer | ErrorKind::Parse => "Syntax Error",
        ErrorKind::Compile => "Compile Error",
        ErrorKind::Runtime => "Runtime Error",
    };

    out.push_str(&format!("{} in {}:{}\n", kind_str, error.file, error.line));
    out.push_str(&format!("  {}\n", error.message));

    if error.message.contains("Undefined") || error.message.contains("Unknown") {
        if let Some(word) = error.message.split('\'').nth(1) {
            let suggestions = suggest_correction(word, known_names);
            if let Some(first) = suggestions.first() {
                out.push_str(&format!("  Did you mean: {}?\n", first));
            }
        }
    }
    out.push('\n');

    if error.line > 0 && error.line <= source_lines.len() {
        let line_idx = error.line - 1;

        let start_line = if line_idx >= 2 { line_idx - 2 } else { 0 };
        let end_line = (line_idx + 2).min(source_lines.len().saturating_sub(1));

        for i in start_line..=end_line {
            let prefix = if i == line_idx { "> " } else { "  " };
            out.push_str(&format!("{} {:4} | {}\n", prefix, i + 1, source_lines[i]));

            if i == line_idx && error.column > 0 {
                let padding = " ".repeat(7 + error.column);
                out.push_str(&format!("{}^\n", padding));
            }
        }
    }
    out
}
