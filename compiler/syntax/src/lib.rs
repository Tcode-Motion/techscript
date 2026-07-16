//! # TechScript Syntax Crate
//!
//! Language syntax constants, unified token kinds, and precedence mapping for Pratt parsing.
//!
//! This crate serves as the single source of truth for the TechScript 2.0 language syntax,
//! completely decoupled from lexing, parsing, AST generation, or semantic rules.
//!
//! ## Modules
//!
//! - [`TokenKind`] — Complete enum containing all canonical, alias, and reserved keyword tokens.
//! - [`LiteralKind`] & [`NumericLiteralKind`] — Logical categories for literals.
//! - [`Token`] — Scanned token struct combining kind, lexeme, and source span.
//! - [`Precedence`] & [`Associativity`] — Operator precedence levels and grouping rules.
//!
//! ## Helpers
//!
//! - [`lookup_keyword`] — Compile-time static lookup mapping strings to keyword token kinds.
//! - [`numeric_literal_kind`] — Analyzes a numeric literal string format (hex, binary, octal, decimal).

mod precedence;
mod token;
mod token_kind;

// Re-exports
pub use precedence::{Associativity, Precedence};
pub use token::Token;
pub use token_kind::{
    lookup_keyword, numeric_literal_kind, LiteralKind, NumericLiteralKind, TokenKind,
};
