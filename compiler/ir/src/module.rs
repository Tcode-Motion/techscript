use crate::function::Function;
use crate::types::{BlockId, DslBlockId, FunctionId, GlobalId, IRType, ValueId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use techscript_ast::LiteralVal;

/// A lowered DSL block stored in the IR module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslBlockIR {
    pub id: DslBlockId,
    pub kind: String,
    pub args: Vec<LiteralVal>,
    pub properties: Vec<(String, Option<LiteralVal>)>,
    pub children: Vec<(DslBlockId, String)>,
    pub span: (u32, u32),
}

/// A top-level compiler module holding functions, globals, constants, and optimization hooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    pub globals: Vec<(GlobalId, String, IRType)>,
    pub dsl_blocks: Vec<DslBlockIR>,
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
            dsl_blocks: Vec::new(),
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
