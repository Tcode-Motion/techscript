use crate::symbol_table::SymbolTable;
use crate::types::TypeInterner;
use std::collections::HashMap;
use techscript_ast::NodeId;
use techscript_errors::Diagnostic;

/// External semantic mapping registry keeping AST nodes immutable.
pub struct SemanticContext {
    pub interner: TypeInterner,
    pub symbol_table: SymbolTable,
    pub node_types: HashMap<NodeId, crate::types::TypeId>,
    pub diagnostics: Vec<Diagnostic>,
    pub loop_depth: usize,
    pub function_depth: usize,
    pub current_model: Option<String>,
}

impl Default for SemanticContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticContext {
    pub fn new() -> Self {
        Self {
            interner: TypeInterner::new(),
            symbol_table: SymbolTable::new(),
            node_types: HashMap::new(),
            diagnostics: Vec::new(),
            loop_depth: 0,
            function_depth: 0,
            current_model: None,
        }
    }
}
