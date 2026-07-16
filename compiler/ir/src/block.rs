use crate::instruction::{Instruction, Terminator};
use crate::types::BlockId;
use serde::{Deserialize, Serialize};

/// A basic block containing sequential instructions ending in a control-flow terminator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: BlockId,
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
    pub predecessors: Vec<BlockId>,
    pub successors: Vec<BlockId>,
}

impl BasicBlock {
    /// Creates a new basic block.
    pub fn new(id: BlockId, label: String) -> Self {
        Self {
            id,
            label,
            instructions: Vec::new(),
            terminator: None,
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }
}
