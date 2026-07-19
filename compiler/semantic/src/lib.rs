//! # TechScript Semantic Crate
//!
//! Handles name resolution, scope checking, type checking, and symbol table generation.
//! Resolves shadowing and issues warning notes for deprecated keywords.

pub mod context;
pub mod dsl_schema;
pub mod passes;
pub mod pipeline;
pub mod symbol_table;
pub mod types;

pub use symbol_table::{Scope, Symbol, SymbolTable};

use context::SemanticContext;
use passes::collect::CollectDecls;
use passes::resolve::ResolveSymbols;
use passes::validate_dsl::ValidateDSL;
use pipeline::PassPipeline;
use serde::{Deserialize, Serialize};
use techscript_ast::Program;
use techscript_errors::{Diagnostic, DiagnosticReporter};

/// AST program annotated with semantic metadata and resolved scope links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckedProgram {
    pub program: Program,
    pub symbols: SymbolTable,
}

/// The main semantic validation controller.
#[derive(Default)]
pub struct SemanticAnalyzer {
    context: SemanticContext,
}

impl SemanticAnalyzer {
    /// Creates a new SemanticAnalyzer instance.
    pub fn new() -> Self {
        Self {
            context: SemanticContext::new(),
        }
    }

    /// Performs scope and keyword check passes, returning the CheckedProgram.
    pub fn analyze(
        &mut self,
        program: Program,
        reporter: &mut DiagnosticReporter,
    ) -> Result<CheckedProgram, Vec<Diagnostic>> {
        let mut pipeline = PassPipeline::new();
        pipeline.add_pass(Box::new(CollectDecls));
        pipeline.add_pass(Box::new(ResolveSymbols));
        pipeline.add_pass(Box::new(ValidateDSL));

        pipeline.execute(&program, &mut self.context);

        // Copy gathered diagnostics to reporter
        for diag in &self.context.diagnostics {
            reporter.report(diag.clone());
        }

        if reporter.has_errors() {
            Err(reporter.get_diagnostics().to_vec())
        } else {
            let checked = CheckedProgram {
                program,
                symbols: self.context.symbol_table.clone(),
            };
            Ok(checked)
        }
    }
}

/// Helper function to perform semantic checks.
pub fn analyze(
    program: Program,
    reporter: &mut DiagnosticReporter,
) -> Result<CheckedProgram, Vec<Diagnostic>> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(program, reporter)
}
