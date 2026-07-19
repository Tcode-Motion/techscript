//! # TechScript Parser Crate
//!
//! Parses token streams and builds an Abstract Syntax Tree (AST).
//! Combines recursive descent for statements and Pratt parsing for expressions.

#![allow(clippy::result_unit_err)]

mod declarations;
mod expressions;
mod parser;
mod statements;

pub use parser::{ParseResult, Parser};
use techscript_ast::{Program, Span};
use techscript_errors::{Diagnostic, DiagnosticReporter};
use techscript_syntax::{Token, TokenKind};

impl<'a> Parser<'a> {
    /// Evaluates structural bounds and parses program nodes.
    pub fn parse(&mut self, reporter: &mut DiagnosticReporter) -> Result<Program, Vec<Diagnostic>> {
        let start_pos = self.peek().span.start;
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip leading newlines/semicolons before statements
            while self.match_token(TokenKind::Newline) || self.match_token(TokenKind::Semicolon) {}
            if self.is_at_end() {
                break;
            }

            match self.parse_statement(reporter) {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                    self.synchronize();
                }
            }
        }

        if reporter.has_errors() {
            Err(reporter.get_diagnostics().to_vec())
        } else {
            let end_pos = self.peek().span.end;
            let span = Span::new(start_pos, end_pos);
            let final_statements = statements;
            Ok(Program::new(self.next_id(), final_statements, span))
        }
    }
}

/// Helper function to parse a token list.
pub fn parse(
    tokens: &[Token],
    reporter: &mut DiagnosticReporter,
) -> Result<Program, Vec<Diagnostic>> {
    let mut parser = Parser::new(tokens);
    parser.parse(reporter)
}

impl<'a> Parser<'a> {
    /// Parses the token stream and returns the program AST even if errors are found,
    /// by employing compiler statement synchronization.
    pub fn parse_recovered(&mut self, reporter: &mut DiagnosticReporter) -> Program {
        let start_pos = self.peek().span.start;
        let mut statements = Vec::new();

        while !self.is_at_end() {
            while self.match_token(TokenKind::Newline) || self.match_token(TokenKind::Semicolon) {}
            if self.is_at_end() {
                break;
            }

            match self.parse_statement(reporter) {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                    self.synchronize();
                }
            }
        }

        let end_pos = self.peek().span.end;
        let span = Span::new(start_pos, end_pos);
        let final_statements = statements;
        Program::new(self.next_id(), final_statements, span)
    }

    /// Wraps all top-level execution statements into a synthetic `main` function.
    fn wrap_top_level_statements(&mut self, statements: Vec<techscript_ast::Statement>) -> Vec<techscript_ast::Statement> {
        use techscript_ast::{Statement, FuncDecl, Block, Ident, Span};

        let mut has_explicit_main = false;
        for stmt in &statements {
            if let Statement::FuncDecl(f) = stmt {
                if f.name.name == "main" {
                    has_explicit_main = true;
                    break;
                }
            }
        }

        // If there is an explicit main, DO NOT WRAP.
        // The semantic resolver (resolve.rs) will iterate over the AST and correctly emit E0313
        // when it finds top-level execution statements alongside an explicit main.
        if has_explicit_main {
            return statements;
        }

        let mut executable_stmts = Vec::new();
        let mut declarations = Vec::new();

        for stmt in statements {
            match &stmt {
                Statement::FuncDecl(_)
                | Statement::StructDecl(_)
                | Statement::EnumDecl(_)
                | Statement::ModelDecl(_)
                | Statement::ExportDecl(_) => {
                    // These are definitive declarations that stay at the top level
                    declarations.push(stmt);
                }
                Statement::DSL(_) => {
                    // DSL blocks are executable statements (they have side effects)
                    executable_stmts.push(stmt);
                }
                Statement::VarDecl(_) | Statement::ConstDecl(_) => {
                    // We keep global variables at the top level so they can be accessed globally
                    declarations.push(stmt);
                }
                _ => {
                    // All other statements (If, For, Expression, Say, etc.) are executed as script logic
                    executable_stmts.push(stmt);
                }
            }
        }

        if executable_stmts.is_empty() {
            return declarations;
        }

        // We have top-level executable statements. Wrap them into a synthetic main() function.
        let start = executable_stmts.first().unwrap().span().start;
        let end = executable_stmts.last().unwrap().span().end;
        let span = Span::new(start, end);

        let main_body = Block::new(self.next_id(), executable_stmts, span);
        let main_func = FuncDecl::new(
            self.next_id(),
            false,
            Ident::new("main".to_string(), span),
            None,
            vec![],
            None,
            main_body,
            span,
        );

        declarations.push(Statement::FuncDecl(main_func));
        declarations
    }
}

pub fn parse_recovered(
    tokens: &[Token],
    reporter: &mut DiagnosticReporter,
) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse_recovered(reporter)
}

