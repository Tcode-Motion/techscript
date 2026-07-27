use crate::context::SemanticContext;
use crate::pipeline::Pass;
use crate::symbol_table::Symbol;
use crate::types::Type;
use techscript_ast::{Program, Statement};
use techscript_errors::{Diagnostic, DiagnosticLevel, ErrorCode};

/// First semantic compiler pass collecting top-level hoisted symbols.
pub struct CollectDecls;

impl Pass for CollectDecls {
    fn run(&mut self, program: &Program, context: &mut SemanticContext) {
        for stmt in &program.statements {
            self.collect_statement(stmt, context);
        }
    }
}

impl CollectDecls {
    fn collect_statement(&self, stmt: &Statement, context: &mut SemanticContext) {
        match stmt {
            Statement::FuncDecl(decl) => {
                let name = decl.name.name.clone();
                let is_builtin = [
                    "len",
                    "range",
                    "ask",
                    "push",
                    "insert",
                    "parse",
                    "write_file",
                    "read_file",
                    "spawn_async",
                    "print",
                    "println",
                    "std",
                    "get",
                    "set",
                ]
                .contains(&name.as_str());
                if context.symbol_table.scopes[0].symbols.contains_key(&name) && !is_builtin {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0301,
                        format!("Duplicate declaration of function '{}'", name),
                        decl.name.span,
                    );
                    context.diagnostics.push(diag);
                } else {
                    let type_id = context.interner.intern(Type::Function {
                        params: vec![],
                        ret_ty: context.interner.any(),
                    });
                    let symbol = Symbol::new(name.clone(), true, true, false, type_id);
                    context.symbol_table.register(name, symbol);
                }
            }
            Statement::StructDecl(decl) => {
                let name = decl.name.name.clone();
                if context.symbol_table.scopes[0].symbols.contains_key(&name) {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0301,
                        format!("Duplicate declaration of struct '{}'", name),
                        decl.name.span,
                    );
                    context.diagnostics.push(diag);
                } else {
                    let type_id = context.interner.intern(Type::Struct(name.clone()));
                    let symbol = Symbol::new(name.clone(), true, false, true, type_id);
                    context.symbol_table.register(name, symbol);
                }
            }
            Statement::EnumDecl(decl) => {
                let name = decl.name.name.clone();
                if context.symbol_table.scopes[0].symbols.contains_key(&name) {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0301,
                        format!("Duplicate declaration of enum '{}'", name),
                        decl.name.span,
                    );
                    context.diagnostics.push(diag);
                } else {
                    let type_id = context.interner.intern(Type::Enum(name.clone()));
                    let symbol = Symbol::new(name.clone(), true, false, true, type_id);
                    context.symbol_table.register(name, symbol);
                }
            }
            Statement::ModelDecl(decl) => {
                let name = decl.name.name.clone();
                if context.symbol_table.scopes[0].symbols.contains_key(&name) {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0301,
                        format!("Duplicate declaration of model '{}'", name),
                        decl.name.span,
                    );
                    context.diagnostics.push(diag);
                } else {
                    let type_id = context.interner.intern(Type::Model(name.clone()));
                    let symbol = Symbol::new(name.clone(), true, false, true, type_id);
                    context.symbol_table.register(name, symbol);
                }
            }
            Statement::ExportDecl(decl) => {
                self.collect_statement(&decl.declaration, context);
            }
            _ => {}
        }
    }
}
