//! Source file management for the TechScript 2.0 compiler.
//!
//! This module provides the infrastructure for tracking source files loaded
//! during compilation. It enables the error reporting system to resolve byte
//! offsets into human-readable file/line/column positions.
//!
//! # Architecture
//!
//! - [`FileId`] — Lightweight handle identifying a loaded source file.
//! - [`SourceFile`] — Immutable record of a file's path, contents, and
//!   precomputed line-start offsets.
//! - [`SourceManager`] — Registry that owns all loaded source files and
//!   provides shared access via `Arc<SourceFile>`.
//! - [`Position`] — Human-readable resolved position (file, line, column, offset).
//!
//! # Design Notes
//!
//! Source files are immutable after loading. The `SourceManager` stores files
//! behind `Arc` pointers so that the lexer, parser, diagnostics, and LSP can
//! all hold references to the same file data without cloning.
//!
//! Line and column numbers are **1-indexed** in `Position` (matching user
//! expectations and LSP conventions). Internal byte offsets remain 0-indexed.
//!
//! ## Future: String Interning
//!
//! A future version may introduce a string interner (`Symbol(u32)`) to deduplicate
//! repeated identifier strings across the compilation session. This would live
//! alongside `SourceManager` in this module. Phase 1 uses heap-allocated `String`
//! values, which is correct and sufficient for the tree-walking interpreter.

use crate::span::Span;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A unique identifier for a source file loaded by the compiler.
///
/// `FileId` values are assigned sequentially by the [`SourceManager`] and are
/// valid only within the lifetime of a single compilation session.
///
/// # Examples
///
/// ```
/// use techscript_common::FileId;
///
/// let id = FileId(0);
/// assert_eq!(id.as_u32(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub u32);

impl FileId {
    /// Returns the underlying `u32` value.
    #[inline]
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileId({})", self.0)
    }
}

/// An immutable record of a loaded source file.
///
/// Once constructed, a `SourceFile` never changes. It stores the file path,
/// full source text, and a precomputed table of line-start byte offsets for
/// efficient line/column resolution.
///
/// # Line Resolution
///
/// The `line_starts` vector maps line numbers (0-indexed internally) to the
/// byte offset where each line begins. This allows O(log n) line lookup via
/// binary search.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// The unique identifier for this file within the current compilation.
    pub id: FileId,
    /// The file system path (may be relative or absolute).
    pub path: PathBuf,
    /// The full UTF-8 source text.
    pub source: String,
    /// Byte offsets of the start of each line (0-indexed).
    /// The first entry is always 0.
    line_starts: Vec<usize>,
}

impl SourceFile {
    /// Creates a new `SourceFile`, precomputing line-start offsets.
    ///
    /// # Arguments
    ///
    /// * `id` — The file identifier assigned by the [`SourceManager`].
    /// * `path` — The file system path of this source file.
    /// * `source` — The full UTF-8 source text.
    pub fn new(id: FileId, path: PathBuf, source: String) -> Self {
        let line_starts = Self::compute_line_starts(&source);
        Self {
            id,
            path,
            source,
            line_starts,
        }
    }

    /// Resolves a byte offset to a 1-indexed `(line, column)` pair.
    ///
    /// Returns `None` if the offset is beyond the end of the source text.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_common::{FileId, SourceFile};
    /// use std::path::PathBuf;
    ///
    /// let file = SourceFile::new(
    ///     FileId(0),
    ///     PathBuf::from("test.txs"),
    ///     "make x = 42\nsay x\n".to_string(),
    /// );
    /// // 'say' starts at byte 12, which is line 2, column 1
    /// assert_eq!(file.line_col(12), Some((2, 1)));
    /// ```
    pub fn line_col(&self, byte_offset: usize) -> Option<(usize, usize)> {
        if byte_offset > self.source.len() {
            return None;
        }

        // Binary search for the line containing this offset.
        let line_index = match self.line_starts.binary_search(&byte_offset) {
            Ok(exact) => exact,
            Err(insert_pos) => insert_pos.saturating_sub(1),
        };

        let line_start = self.line_starts[line_index];
        let line = line_index + 1; // 1-indexed
        let column = byte_offset - line_start + 1; // 1-indexed

        Some((line, column))
    }

    /// Returns the byte offset where the given 1-indexed line begins.
    ///
    /// Returns `None` if the line number is 0 or exceeds the number of lines.
    pub fn line_start(&self, line: usize) -> Option<usize> {
        if line == 0 || line > self.line_starts.len() {
            return None;
        }
        Some(self.line_starts[line - 1])
    }

    /// Returns the full text content of the given 1-indexed line (without
    /// the trailing newline).
    ///
    /// Returns `None` if the line number is 0 or exceeds the number of lines.
    pub fn line_content(&self, line: usize) -> Option<&str> {
        if line == 0 || line > self.line_starts.len() {
            return None;
        }

        let start = self.line_starts[line - 1];
        let end = if line < self.line_starts.len() {
            self.line_starts[line]
        } else {
            self.source.len()
        };

        // Trim trailing newline characters (\n or \r\n).
        let slice = &self.source[start..end];
        Some(slice.trim_end_matches(['\n', '\r']))
    }

    /// Returns the total number of lines in this source file.
    ///
    /// An empty file has 1 line (the empty line).
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Returns the full source text.
    #[inline]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the file path.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Computes line-start byte offsets for the given source text.
    fn compute_line_starts(source: &str) -> Vec<usize> {
        let mut starts = vec![0];
        for (i, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                starts.push(i + 1);
            }
        }
        starts
    }
}

/// A human-readable resolved source position.
///
/// Contains all the information needed to display a diagnostic location:
/// the file, the 1-indexed line and column, and the raw byte offset.
///
/// # Examples
///
/// ```
/// use techscript_common::{FileId, Position};
///
/// let pos = Position::new(FileId(0), 5, 12, 42);
/// assert_eq!(format!("{pos}"), "FileId(0):5:12");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// The file this position belongs to.
    pub file_id: FileId,
    /// The 1-indexed line number.
    pub line: usize,
    /// The 1-indexed column number (byte offset within the line).
    pub column: usize,
    /// The absolute 0-indexed byte offset in the source file.
    pub offset: usize,
}

impl Position {
    /// Creates a new `Position`.
    #[inline]
    pub fn new(file_id: FileId, line: usize, column: usize, offset: usize) -> Self {
        Self {
            file_id,
            line,
            column,
            offset,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file_id, self.line, self.column)
    }
}

/// Registry of all source files loaded during a compilation session.
///
/// The `SourceManager` owns all source file data and provides shared access
/// via `Arc<SourceFile>`. This allows the lexer, parser, diagnostics, and LSP
/// to hold references to source files without cloning the full text.
///
/// Files are assigned sequential [`FileId`] values starting from 0.
///
/// # Examples
///
/// ```
/// use techscript_common::SourceManager;
/// use std::path::PathBuf;
///
/// let mut manager = SourceManager::new();
/// let id = manager.add_file(PathBuf::from("main.txs"), "say \"hello\"".to_string());
/// let file = manager.get_file(id).unwrap();
/// assert_eq!(file.source(), "say \"hello\"");
/// ```
#[derive(Debug, Default)]
pub struct SourceManager {
    files: Vec<Arc<SourceFile>>,
}

impl SourceManager {
    /// Creates an empty `SourceManager`.
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Registers a source file and returns its assigned [`FileId`].
    ///
    /// The file contents become immutable once added.
    pub fn add_file(&mut self, path: PathBuf, source: String) -> FileId {
        let id = FileId(self.files.len() as u32);
        let file = SourceFile::new(id, path, source);
        self.files.push(Arc::new(file));
        id
    }

    /// Retrieves a source file by its [`FileId`].
    ///
    /// Returns `None` if the ID does not correspond to a loaded file.
    pub fn get_file(&self, id: FileId) -> Option<Arc<SourceFile>> {
        self.files.get(id.0 as usize).cloned()
    }

    /// Returns the total number of loaded source files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Resolves a [`Span`] within a given file to a [`Position`].
    ///
    /// Returns `None` if the file ID is invalid or the span's start offset
    /// is out of bounds.
    pub fn resolve_position(&self, file_id: FileId, span: &Span) -> Option<Position> {
        let file = self.get_file(file_id)?;
        let (line, column) = file.line_col(span.start)?;
        Some(Position::new(file_id, line, column, span.start))
    }
}
