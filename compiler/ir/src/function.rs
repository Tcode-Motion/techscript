use crate::block::BasicBlock;
use crate::types::{FunctionId, IRType, LocalId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An IR function containing its signature, parameters, and basic blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub id: FunctionId,
    pub name: String,
    pub params: Vec<(LocalId, String, IRType)>,
    pub blocks: Vec<BasicBlock>,
    pub local_types: HashMap<LocalId, IRType>,
    pub return_type: IRType,
}

impl Function {
    /// Creates a new function.
    pub fn new(id: FunctionId, name: String, return_type: IRType) -> Self {
        Self {
            id,
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            local_types: HashMap::new(),
            return_type,
        }
    }
}
