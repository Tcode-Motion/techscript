//! # TechScript Lexer Crate
//!
//! Scans raw UTF-8 source strings and tokenizes them into a vector of Tokens.
//! Uses logos DFA definitions for maximum character processing performance.

#![allow(dead_code, unused)]

use techscript_syntax::{Token, TokenKind};
use techscript_errors::{Diagnostic, DiagnosticReporter};

/// Lexical analyzer that parses source text into a token stream.
pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer for the given source code.
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    /// Tokenizes the source string, logging failures to the reporter.
    pub fn lex(&mut self, _reporter: &mut DiagnosticReporter) -> Result<Vec<Token>, Vec<Diagnostic>> {
        // Skeletal implementation: returns EOF token
        let tokens = vec![
            Token::new(
                TokenKind::Eof,
                "".to_string(),
                techscript_common::Span::new(self.source.len(), self.source.len()),
            )
        ];
        Ok(tokens)
    }
}

/// Helper function to scan source code directly.
pub fn lex(source: &str, reporter: &mut DiagnosticReporter) -> Result<Vec<Token>, Vec<Diagnostic>> {
    let mut lexer = Lexer::new(source);
    lexer.lex(reporter)
}
