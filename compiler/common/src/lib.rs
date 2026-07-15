//! # TechScript Common Crate
//!
//! Shared structures and utilities used across different compiler passes.
//! This crate contains primitives like Spans, Node IDs, Identifiers,
//! and standard Diagnostics representations.

use serde::{Serialize, Deserialize};

/// Represents a source location (byte offset span).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Start byte offset (inclusive)
    pub start: usize,
    /// End byte offset (exclusive)
    pub end: usize,
}

impl Span {
    /// Create a new Span.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// A unique ID for AST nodes, allowing semantic metadata mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u32);

/// An identifier with its source location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    /// Create a new Ident.
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}
