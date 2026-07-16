//! # TechScript Common Crate
//!
//! Foundational types and utilities shared across all TechScript 2.0 compiler,
//! runtime, and tooling crates. This crate sits at the bottom of the dependency
//! graph — every other crate depends on it, and it depends on nothing except
//! `serde` for serialization.
//!
//! ## Core Types
//!
//! - [`Span`] — Byte-offset range tracking source locations.
//! - [`NodeId`] — Unique identifier for AST nodes.
//! - [`NodeIdGenerator`] — Thread-safe sequential generator for [`NodeId`] values.
//! - [`Ident`] — Named identifier paired with its source [`Span`].
//!
//! ## Source Management
//!
//! - [`FileId`] — Handle identifying a loaded source file.
//! - [`SourceFile`] — Immutable record of a file's path, contents, and line offsets.
//! - [`SourceManager`] — Registry of all source files in a compilation session.
//! - [`Position`] — Resolved human-readable position (file, line, column, offset).
//!
//! ## File Validation
//!
//! - [`is_techscript_file`] — Checks whether a path has the `.txs` extension.
//! - [`validate_extension`] — Returns a typed error for invalid extensions.
//! - [`CommonError`] — Error type for common crate operations.
//!
//! ## Constants
//!
//! - [`TECHSCRIPT_VERSION`] — Compiler version from `Cargo.toml`.
//! - [`MAX_RECURSION_DEPTH`] — Stack overflow guard limit.
//! - [`MAX_SOURCE_FILE_SIZE`] — Maximum source file size in bytes.
//! - [`TECHSCRIPT_EXTENSION`] / [`TECHSCRIPT_DOT_EXTENSION`] — File extension constants.

// ── Module declarations ──────────────────────────────────────────────────────

mod constants;
mod file_ext;
mod ident;
mod node_id;
mod source;
mod span;

// ── Public re-exports ────────────────────────────────────────────────────────

// Core types
pub use ident::Ident;
pub use node_id::{NodeId, NodeIdGenerator};
pub use span::Span;

// Source management
pub use source::{FileId, Position, SourceFile, SourceManager};

// File validation
pub use file_ext::{
    is_techscript_file, validate_extension, CommonError, TECHSCRIPT_DOT_EXTENSION,
    TECHSCRIPT_EXTENSION,
};

// Constants
pub use constants::{MAX_RECURSION_DEPTH, MAX_SOURCE_FILE_SIZE, TECHSCRIPT_VERSION};
