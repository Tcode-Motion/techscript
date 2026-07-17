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
            Ok(Program::new(self.next_id(), statements, span))
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
        Program::new(self.next_id(), statements, span)
    }
}

pub fn parse_recovered(
    tokens: &[Token],
    reporter: &mut DiagnosticReporter,
) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse_recovered(reporter)
}

