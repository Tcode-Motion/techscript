use crate::function::Function;
use crate::types::{BlockId, FunctionId, GlobalId, IRType, ValueId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use techscript_ast::LiteralVal;

/// A top-level compiler module holding functions, globals, constants, and optimization hooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    pub globals: Vec<(GlobalId, String, IRType)>,
    pub constants: Vec<(ValueId, LiteralVal)>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,

    // Optimization/CFG analysis metadata hooks for Phase 10 Optimizer
    pub dominator_tree: HashMap<BlockId, Vec<BlockId>>,
    pub post_dominator_tree: HashMap<BlockId, Vec<BlockId>>,
    pub loop_info: Vec<Vec<BlockId>>,
    pub call_graph: HashMap<FunctionId, Vec<FunctionId>>,
}

impl Module {
    /// Creates a new Module with empty analysis structures.
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            globals: Vec::new(),
            constants: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            dominator_tree: HashMap::new(),
            post_dominator_tree: HashMap::new(),
            loop_info: Vec::new(),
            call_graph: HashMap::new(),
        }
    }
}
