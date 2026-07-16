//! Unique AST node identifiers for the TechScript 2.0 compiler.
//!
//! Every AST node, expression, and statement receives a unique [`NodeId`] during
//! parsing. The semantic analyzer uses these IDs to attach resolved information
//! (scopes, types, symbol references) without mutating the AST itself.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};

/// A unique identifier for an AST node.
///
/// `NodeId` values are assigned sequentially by a [`NodeIdGenerator`] during parsing.
/// They allow downstream compiler passes (semantic analysis, interpretation) to associate
/// metadata with specific nodes via side tables, without modifying the AST structure.
///
/// # Examples
///
/// ```
/// use techscript_common::NodeId;
///
/// let id = NodeId(0);
/// assert_eq!(id.as_u32(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u32);

impl NodeId {
    /// Creates a dummy `NodeId` with the sentinel value `u32::MAX`.
    ///
    /// Used for synthetic nodes that are not part of the original source
    /// (e.g., compiler-generated constructs in tests).
    #[inline]
    pub fn dummy() -> Self {
        Self(u32::MAX)
    }

    /// Returns the underlying `u32` value.
    #[inline]
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", self.0)
    }
}

/// Thread-safe sequential generator for [`NodeId`] values.
///
/// Each compiler invocation creates a fresh generator starting from 0.
/// The generator uses atomic operations, making it safe to share across
/// threads if parallel parsing is ever introduced.
///
/// # Examples
///
/// ```
/// use techscript_common::NodeIdGenerator;
///
/// let gen = NodeIdGenerator::new();
/// let first = gen.next();
/// let second = gen.next();
/// assert_eq!(first.as_u32(), 0);
/// assert_eq!(second.as_u32(), 1);
/// ```
pub struct NodeIdGenerator {
    counter: AtomicU32,
}

impl NodeIdGenerator {
    /// Creates a new generator starting from 0.
    pub fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
        }
    }

    /// Generates and returns the next sequential `NodeId`.
    ///
    /// This method is thread-safe and can be called concurrently.
    pub fn next(&self) -> NodeId {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        NodeId(id)
    }

    /// Returns the next `NodeId` that would be generated without advancing
    /// the counter.
    ///
    /// Useful for inspecting the current state of the generator.
    pub fn peek(&self) -> NodeId {
        NodeId(self.counter.load(Ordering::Relaxed))
    }

    /// Returns the most recently generated `NodeId`.
    ///
    /// Returns `NodeId(0)` if no IDs have been generated yet; note that this
    /// is also the value of the *first* generated ID. Use [`peek`](Self::peek)
    /// to distinguish between "no IDs generated" (peek returns 0) and
    /// "one ID generated" (peek returns 1).
    pub fn current(&self) -> NodeId {
        let val = self.counter.load(Ordering::Relaxed);
        NodeId(val.saturating_sub(1))
    }

    /// Resets the generator back to 0.
    ///
    /// Primarily useful in test harnesses where deterministic node IDs are
    /// required across multiple test cases.
    pub fn reset(&self) {
        self.counter.store(0, Ordering::Relaxed);
    }
}

impl Default for NodeIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NodeIdGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeIdGenerator")
            .field("next_id", &self.counter.load(Ordering::Relaxed))
            .finish()
    }
}
