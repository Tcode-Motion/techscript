use serde::{Deserialize, Serialize};
use techscript_ast::LiteralVal;

/// Deduplicated storage for literal constants, strings, integers, and floats.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConstantPool {
    pub constants: Vec<LiteralVal>,
}

impl ConstantPool {
    /// Creates an empty ConstantPool.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a constant value to the pool and returns its index. Reuses matches.
    pub fn add(&mut self, val: LiteralVal) -> u32 {
        if let Some(pos) = self.constants.iter().position(|c| c == &val) {
            pos as u32
        } else {
            let idx = self.constants.len() as u32;
            self.constants.push(val);
            idx
        }
    }

    /// Retrieves a constant from the pool by index.
    pub fn get(&self, idx: u32) -> Option<&LiteralVal> {
        self.constants.get(idx as usize)
    }
}
