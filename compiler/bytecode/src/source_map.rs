use serde::{Deserialize, Serialize};
use techscript_common::Span;

/// Source code mappings resolving runtime offsets back to code spans.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SourceMap {
    pub mappings: Vec<(u32, Span)>,
}

impl SourceMap {
    /// Creates a new empty SourceMap.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a span location for the bytecode instruction offset.
    pub fn add(&mut self, offset: u32, span: Span) {
        self.mappings.push((offset, span));
    }

    /// Resolves instruction offset back to span.
    pub fn resolve(&self, offset: u32) -> Option<Span> {
        self.mappings
            .iter()
            .find(|&&(off, _)| off == offset)
            .map(|&(_, span)| span)
    }
}
