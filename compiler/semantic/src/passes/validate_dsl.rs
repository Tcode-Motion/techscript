use crate::context::SemanticContext;
use crate::dsl_schema::build_dsl_registry;
use crate::pipeline::Pass;
use techscript_ast::{DSLBlock, DSLChild, Statement};
use techscript_errors::{Diagnostic, DiagnosticLevel, ErrorCode};
use techscript_ast::Program;

pub struct ValidateDSL;

impl Pass for ValidateDSL {
    fn run(&mut self, program: &Program, context: &mut SemanticContext) {
        let registry = build_dsl_registry();
        for stmt in &program.statements {
            if let Statement::DSL(block) = stmt {
                validate_dsl_block(block, &registry, context);
            }
        }
    }
}

fn validate_dsl_block(
    block: &DSLBlock,
    registry: &std::collections::HashMap<String, crate::dsl_schema::DSLSchema>,
    context: &mut SemanticContext,
) {
    let schema = registry.get(&block.kind);

    // Collect property names for duplicate check
    let mut prop_names: std::collections::HashSet<String> = std::collections::HashSet::new();

    for prop in &block.properties {
        // Duplicate property check
        if !prop_names.insert(prop.name.clone()) {
            let diag = Diagnostic::new(
                DiagnosticLevel::Error,
                ErrorCode::E0400,
                format!(
                    "Duplicate property '{}' in '{}' block",
                    prop.name, block.kind
                ),
                prop.span,
            );
            context.diagnostics.push(diag);
            continue;
        }

        // Unknown property check
        if let Some(s) = schema {
            if !s.is_valid_property(&prop.name) {
                let diag = Diagnostic::new(
                    DiagnosticLevel::Warning,
                    ErrorCode::E0401,
                    format!(
                        "Unknown property '{}' for '{}' block",
                        prop.name, block.kind
                    ),
                    prop.span,
                );
                context.diagnostics.push(diag);
            }
        }
    }

    // Required properties check
    if let Some(s) = schema {
        let present: std::collections::HashSet<String> =
            block.properties.iter().map(|p| p.name.clone()).collect();
        for required in &s.required_properties {
            if !present.contains(required) {
                let diag = Diagnostic::new(
                    DiagnosticLevel::Error,
                    ErrorCode::E0402,
                    format!(
                        "Missing required property '{}' in '{}' block",
                        required, block.kind
                    ),
                    block.span,
                );
                context.diagnostics.push(diag);
            }
        }
    }

    // Recurse into children
    for child in &block.children {
        match child {
            DSLChild::Block(sub_block) => {
                // Check that child kind is allowed in parent
                if let Some(s) = schema {
                    if !s.is_allowed_child(&sub_block.kind) {
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Warning,
                            ErrorCode::E0403,
                            format!(
                                "'{}' block is not allowed inside '{}' block",
                                sub_block.kind, block.kind
                            ),
                            sub_block.span,
                        );
                        context.diagnostics.push(diag);
                    }
                }
                validate_dsl_block(sub_block, registry, context);
            }
            DSLChild::Code(block) => {
                for stmt in &block.statements {
                    if let Statement::DSL(dsl) = stmt {
                        validate_dsl_block(dsl, registry, context);
                    }
                }
            }
            DSLChild::Property(prop) => {
                if let Some(s) = schema {
                    if !s.is_valid_property(&prop.name) {
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Warning,
                            ErrorCode::E0401,
                            format!(
                                "Unknown property '{}' for '{}' block",
                                prop.name, block.kind
                            ),
                            prop.span,
                        );
                        context.diagnostics.push(diag);
                    }
                }
            }
        }
    }
}
