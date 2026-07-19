//! # TechScript Errors Crate
//!
//! Unified diagnostic management and terminal error reporting.
//! Implements all ErrorCode registers and levels from specifications.

use serde::{Deserialize, Serialize};
use techscript_common::Span;

/// Unified categories of Diagnostic levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

/// Diagnostic code registration (E0001..E9999, W0001..W0099).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    // Lexer (E0001 - E0099)
    E0001, // Unexpected character
    E0010, // Trailing underscore in number
    E0011, // Empty numeric prefix
    E0012, // Invalid base digit
    E0021, // Unterminated string

    // Parser (E0100 - E0299)
    E0100, // Expected expression
    E0101, // Expected identifier
    E0104, // Expected left brace
    E0105, // Expected right brace
    E0107, // Expected statement terminator
    E0113, // Invalid assignment target

    // Semantic (E0300 - E0499)
    E0300, // Undefined variable
    E0301, // Duplicate variable declaration
    E0302, // Reassign constant
    E0310, // Arity low
    E0311, // Arity high
    E0312, // Return outside function
    E0313, // Mixed top-level statements with explicit main
    E0320, // Self outside method
    E0340, // Module not found
    E0350, // Non-exportable declaration

    // DSL validation (E0400 - E0499)
    E0400, // Duplicate property in DSL block
    E0401, // Unknown property for DSL block
    E0402, // Missing required property in DSL block
    E0403, // Invalid nested DSL block

    // Runtime (E1000 - E1999)
    E1010, // Div by zero
    E1011, // Type mismatch
    E1020, // Stack overflow
    E1030, // Not iterable
    E1041, // Member not found
    E1050, // Index out of bounds

    // Warnings (W0001 - W0099)
    W0001, // Reserved identifier naming
    W0010, // Shadowing variable
    W0011, // Unused variable
    W0015, // Deprecated 'fun' keyword
}

/// A structured diagnostic message emitted by compiler passes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: ErrorCode,
    pub message: String,
    pub span: Span,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn new(level: DiagnosticLevel, code: ErrorCode, message: String, span: Span) -> Self {
        Self {
            level,
            code,
            message,
            span,
            help: None,
        }
    }

    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
}

/// DiagnosticReporter collects and formats diagnostic warnings and errors.
#[derive(Debug, Default)]
pub struct DiagnosticReporter {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticReporter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn report(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Renders all diagnostics to standard output.
    pub fn print_diagnostics(&self, _source: &str, _file_name: &str) {
        for diag in &self.diagnostics {
            println!("{:?}[{:?}]: {}", diag.level, diag.code, diag.message);
        }
    }
}
