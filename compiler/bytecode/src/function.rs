use crate::chunk::Chunk;
use crate::debug::DebugSymbols;
use crate::source_map::SourceMap;
use serde::{Deserialize, Serialize};

/// Compiled bytecode function signature carrying stack height limits and chunks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BytecodeFunction {
    pub name: String,
    pub param_count: u32,
    pub local_count: u32,
    pub max_stack_size: u32,
    pub chunk: Chunk,
    pub source_map: SourceMap,
    pub debug_symbols: DebugSymbols,
}

impl BytecodeFunction {
    /// Creates a new BytecodeFunction.
    pub fn new(name: String, param_count: u32) -> Self {
        Self {
            name,
            param_count,
            local_count: 0,
            max_stack_size: 0,
            chunk: Chunk::new(),
            source_map: SourceMap::new(),
            debug_symbols: DebugSymbols::new(),
        }
    }
}
