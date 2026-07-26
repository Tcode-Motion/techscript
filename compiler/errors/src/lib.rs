//! # TechScript Errors Crate
//!
//! Unified diagnostic management and terminal error reporting.
//! Implements all ErrorCode registers and levels from the TechScript 2.0 specification.
//!
//! # Error Code Namespaces
//!
//! | Prefix | Category | Phase |
//! |---|---|---|
//! | `TSE0xxx` | Compile-time errors | Lexer / Parser / Semantic |
//! | `TSW1xxx` | Deprecation warnings | Parser |
//! | `TSW2xxx` | Style / lint warnings | Semantic |
//! | `TSI3xxx` | Informational hints | Semantic |

use serde::{Deserialize, Serialize};
use techscript_common::Span;

/// Unified categories of Diagnostic levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

/// Diagnostic code registration for TechScript 2.0.
///
/// # Stability
/// All `E0xxx` / `W0xxx` codes are stable — they are never removed, only deprecated.
/// New `TSW`/`TSI` codes may be added in minor releases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    // ── Lexer Errors (TSE0001 – TSE0099) ─────────────────────────────────────
    E0001, // Unexpected character (e.g. `@`)
    E0010, // Trailing underscore in number literal (e.g. `42_`)
    E0011, // Empty numeric literal after base prefix (e.g. `0x`)
    E0012, // Invalid digit for numeric base (e.g. `0b102`)
    E0021, // Unterminated string literal

    // ── Parser Errors (TSE0100 – TSE0299) ────────────────────────────────────
    E0100, // Expected expression
    E0101, // Expected identifier
    E0104, // Expected `end` to close block (old: Expected `{`)
    E0105, // Expected block body (old: Expected `}`)
    E0107, // Expected statement terminator (missing newline)
    E0113, // Invalid assignment target (e.g. `42 = x`)

    // ── Semantic Errors (TSE0300 – TSE0499) ──────────────────────────────────
    E0300, // Undefined variable (used before declaration)
    E0301, // Duplicate declaration in same scope
    E0302, // Cannot reassign `const` (TSE0302)
    E0303, // Variable used before assignment
    E0310, // Wrong argument count (too few)
    E0311, // Wrong argument count (too many)
    E0312, // `send` outside function body (TSE0312)
    E0313, // Mixed top-level statements with explicit main
    E0320, // `self` used outside method declaration
    E0340, // Module not found — cannot resolve import
    E0350, // Cannot export non-exportable declaration

    // ── DSL Validation Errors (TSE0400 – TSE0499) ────────────────────────────
    E0400, // Duplicate property in DSL block
    E0401, // Unknown property for DSL block type
    E0402, // Missing required property in DSL block
    E0403, // Invalid nested DSL block

    // ── Runtime Errors (TSE1000 – TSE1999) ───────────────────────────────────
    E1010, // Division by zero
    E1011, // Type mismatch in operation (e.g. `"hello" - 5`)
    E1020, // Stack overflow (recursion limit exceeded)
    E1030, // Value not iterable in `for` loop
    E1041, // Field or method not found on object
    E1050, // Index out of bounds

    // ── Legacy Warnings (W0001 – W0099) ──────────────────────────────────────
    W0001, // Reserved identifier naming
    W0010, // Shadowing variable
    W0011, // Unused variable
    W0015, // Deprecated `fun` keyword (superseded by TSW1002)

    // ── Deprecation Warnings — Parser Phase (TSW1001 – TSW1099) ──────────────
    /// `make x = 5` → plain assignment `x = 5`
    TSW1001,
    /// `build fn()` → `do fn()` (also covers `fun`, `function`)
    TSW1002,
    /// `return x` → `send x`
    TSW1003,
    /// `attempt { }` → `try`
    TSW1004,
    /// `give x` → `send x`
    TSW1005,
    /// `{ }` block delimiters → `end`; `;` → (removed)
    TSW1006,
    /// `if cond` → `when cond`
    TSW1007,
    /// `while cond` → `repeat cond`
    TSW1008,
    /// `import mod` or `from mod import x` → `use mod`
    TSW1009,
    /// `each x in y` → `for x in y`
    TSW1010,
    /// `none` → `null`
    TSW1011,
    /// `f"..."` interpolated string → `$"..."`
    TSW1012,
    /// `model Name` → `class Name`
    TSW1013,
    /// `std.io.println(x)` → `say x`
    TSW1014,

    // ── Style / Lint Warnings — Semantic Phase (TSW2001 – TSW2099) ───────────
    /// Variable declared (or auto-created) but never read
    TSW2001,
    /// Variable name shadows an identifier in an outer scope
    TSW2002,

    // ── Informational Hints (TSI3001 – TSI3099) ───────────────────────────────
    /// String concatenation with `+` — consider using `$"..."` interpolation
    TSI3001,
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
