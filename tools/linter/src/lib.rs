//! # TechScript Linter Crate
//!
//! Static analysis rules and linter passes (`tech lint`).
//! Inspects AST structures to flag shadowing, unused variables, and deprecations.

use techscript_errors::Diagnostic;
use techscript_semantic::CheckedProgram;

/// Trait definition for static analysis lint rules.
pub trait LintRule {
    fn name(&self) -> &'static str;
    fn check(&self, program: &CheckedProgram) -> Vec<Diagnostic>;
}

/// Dynamic linter verification engine.
#[derive(Default)]
pub struct Linter {
    rules: Vec<Box<dyn LintRule>>,
}

impl Linter {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<dyn LintRule>) {
        self.rules.push(rule);
    }

    /// Evaluates all configured rules against the checked AST.
    pub fn lint(&self, program: &CheckedProgram) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for rule in &self.rules {
            diagnostics.extend(rule.check(program));
        }
        diagnostics
    }
}
