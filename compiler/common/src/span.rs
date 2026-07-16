//! Source location tracking for the TechScript 2.0 compiler.
//!
//! A [`Span`] represents a contiguous range of bytes in source code, used by every
//! AST node and diagnostic message to track where constructs originate. Spans use
//! half-open byte-offset intervals `[start, end)`.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a contiguous byte-offset range in source code.
///
/// Spans are the primary mechanism for tracking source locations across the
/// compiler pipeline. Every AST node, token, and diagnostic message carries
/// a `Span` to enable precise error reporting.
///
/// Byte offsets use a half-open interval: `start` is inclusive, `end` is exclusive.
///
/// # Examples
///
/// ```
/// use techscript_common::Span;
///
/// let span = Span::new(0, 5);
/// assert_eq!(span.len(), 5);
/// assert!(!span.is_empty());
/// assert!(span.contains(3));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Start byte offset (inclusive).
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
}

impl Span {
    /// Creates a new `Span` from the given byte offsets.
    ///
    /// # Arguments
    ///
    /// * `start` — Inclusive start byte offset.
    /// * `end` — Exclusive end byte offset.
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Creates a zero-length dummy span at offset 0.
    ///
    /// Useful for synthetic AST nodes that do not correspond to any real source
    /// text (e.g., compiler-generated nodes or test fixtures).
    #[inline]
    pub fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }

    /// Returns the byte length of this span.
    #[inline]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns `true` if this span covers zero bytes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Returns `true` if the given byte offset falls within this span.
    ///
    /// The check is half-open: `start <= offset < end`.
    #[inline]
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }

    /// Merges two spans into the smallest span that covers both.
    ///
    /// The resulting span starts at the minimum of the two starts and ends at
    /// the maximum of the two ends.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_common::Span;
    ///
    /// let a = Span::new(0, 5);
    /// let b = Span::new(10, 15);
    /// let merged = a.merge(b);
    /// assert_eq!(merged, Span::new(0, 15));
    /// ```
    #[inline]
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Extracts the source text slice corresponding to this span.
    ///
    /// Returns `None` if the span is out of bounds or does not align with
    /// valid UTF-8 character boundaries in the source string.
    ///
    /// # Arguments
    ///
    /// * `source` — The full source code string.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_common::Span;
    ///
    /// let source = "make x = 42";
    /// let span = Span::new(0, 4);
    /// assert_eq!(span.source_text(source), Some("make"));
    /// ```
    pub fn source_text<'a>(&self, source: &'a str) -> Option<&'a str> {
        if self.start > self.end || self.end > source.len() {
            return None;
        }
        if !source.is_char_boundary(self.start) || !source.is_char_boundary(self.end) {
            return None;
        }
        Some(&source[self.start..self.end])
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
