use crate::function::BytecodeFunction;
use serde::{Deserialize, Serialize};
use techscript_ir::types::IRType;

/// A top-level compiled program module carrying functions and global definitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BytecodeModule {
    pub name: String,
    pub functions: Vec<BytecodeFunction>,
    pub globals: Vec<(String, IRType)>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub entry_idx: u32,
}

impl BytecodeModule {
    /// Creates a new BytecodeModule.
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            globals: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            entry_idx: 0,
        }
    }
}
