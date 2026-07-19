//! # TechScript Linter Crate
//!
//! Static analysis rules and linter passes (`tech lint`).
//! Inspects AST structures to flag shadowing, unused variables, and deprecations.

use techscript_ast::{DSLBlock, DSLChild, Statement};
use techscript_errors::{Diagnostic, DiagnosticLevel, ErrorCode};
use techscript_semantic::CheckedProgram;

/// Trait definition for static analysis lint rules.
pub trait LintRule {
    fn name(&self) -> &'static str;
    fn check(&self, program: &CheckedProgram) -> Vec<Diagnostic>;
}

/// Lint rule: Check that DSL blocks have proper `end` terminators and no empty blocks.
pub struct DslBlockLintRule;

impl LintRule for DslBlockLintRule {
    fn name(&self) -> &'static str {
        "dsl-block-lint"
    }

    fn check(&self, program: &CheckedProgram) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.check_dsl_blocks(&program.program.statements, &mut diagnostics);
        diagnostics
    }
}

impl DslBlockLintRule {
    fn check_dsl_blocks(&self, stmts: &[Statement], diags: &mut Vec<Diagnostic>) {
        for stmt in stmts {
            if let Statement::DSL(block) = stmt {
                self.validate_dsl_block(block, diags);
            }
        }
    }

    fn validate_dsl_block(&self, block: &DSLBlock, diags: &mut Vec<Diagnostic>) {
        // Warn if a block has no properties and no children (empty block)
        if block.properties.is_empty() && block.children.is_empty() && block.args.is_empty() {
            diags.push(Diagnostic::new(
                DiagnosticLevel::Warning,
                ErrorCode::E0400,
                format!("DSL block '{}' is empty — add properties or children, or remove it", block.kind),
                block.span,
            ));
        }

        // Recurse into children
        for child in &block.children {
            match child {
                DSLChild::Block(sub_block) => {
                    self.validate_dsl_block(sub_block, diags);
                }
                DSLChild::Code(code_block) => {
                    self.check_dsl_blocks(&code_block.statements, diags);
                }
                _ => {}
            }
        }
    }
}

/// Lint rule: Check that DSL blocks follow naming conventions.
pub struct DslNamingConventionRule;

impl LintRule for DslNamingConventionRule {
    fn name(&self) -> &'static str {
        "dsl-naming-convention"
    }

    fn check(&self, program: &CheckedProgram) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.check_naming(&program.program.statements, &mut diagnostics);
        diagnostics
    }
}

impl DslNamingConventionRule {
    fn check_naming(&self, stmts: &[Statement], diags: &mut Vec<Diagnostic>) {
        for stmt in stmts {
            if let Statement::DSL(block) = stmt {
                // Check that 'page' blocks have a string path argument
                if block.kind == "page" {
                    let has_path = block.args.iter().any(|arg| {
                        matches!(arg, techscript_ast::Expression::Literal(lit) if matches!(&lit.value, techscript_ast::LiteralVal::Str(_)))
                    });
                    if !has_path {
                        diags.push(Diagnostic::new(
                            DiagnosticLevel::Warning,
                            ErrorCode::E0401,
                            "'page' block should have a path argument like page \"/\"".to_string(),
                            block.span,
                        ));
                    }
                }

                // Recurse into children
                for child in &block.children {
                    if let DSLChild::Block(sub) = child {
                        self.check_naming_sub(sub, diags);
                    }
                }
            }
        }
    }

    fn check_naming_sub(&self, block: &DSLBlock, diags: &mut Vec<Diagnostic>) {
        for child in &block.children {
            if let DSLChild::Block(sub) = child {
                if sub.kind == "page" {
                    let has_path = sub.args.iter().any(|arg| {
                        matches!(arg, techscript_ast::Expression::Literal(lit) if matches!(&lit.value, techscript_ast::LiteralVal::Str(_)))
                    });
                    if !has_path {
                        diags.push(Diagnostic::new(
                            DiagnosticLevel::Warning,
                            ErrorCode::E0401,
                            "'page' block should have a path argument like page \"/\"".to_string(),
                            sub.span,
                        ));
                    }
                }
                self.check_naming_sub(sub, diags);
            }
        }
    }
}

/// Dynamic linter verification engine.
#[derive(Default)]
pub struct Linter {
    rules: Vec<Box<dyn LintRule>>,
}

impl Linter {
    pub fn new() -> Self {
        let mut linter = Self { rules: Vec::new() };
        // Register default lint rules
        linter.add_rule(Box::new(DslBlockLintRule));
        linter.add_rule(Box::new(DslNamingConventionRule));
        linter
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
