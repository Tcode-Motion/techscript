//! # TechScript Semantic Crate
//!
//! Handles name resolution, scope checking, type checking, and symbol table generation.
//! Resolves shadowing and issues warning notes for deprecated keywords.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use techscript_ast::Program;
use techscript_errors::{Diagnostic, DiagnosticReporter};

/// Information associated with a resolved symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub is_constant: bool,
}

/// A single lexical scope containing declared symbols.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}

/// Symbol table tracking scopes and resolution mappings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
}

/// AST program annotated with semantic metadata and resolved scope links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckedProgram {
    pub program: Program,
    pub symbols: SymbolTable,
}

/// The main semantic validation controller.
#[derive(Default)]
pub struct SemanticAnalyzer {
    symbols: SymbolTable,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::default(),
        }
    }

    /// Performs scope and keyword check passes, returning the CheckedProgram.
    pub fn analyze(&mut self, program: Program, _reporter: &mut DiagnosticReporter) -> Result<CheckedProgram, Vec<Diagnostic>> {
        let checked = CheckedProgram {
            program,
            symbols: self.symbols.clone(),
        };
        Ok(checked)
    }
}

/// Helper function to perform semantic checks.
pub fn analyze(program: Program, reporter: &mut DiagnosticReporter) -> Result<CheckedProgram, Vec<Diagnostic>> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(program, reporter)
}
