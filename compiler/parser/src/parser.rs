use techscript_common::{NodeIdGenerator, Span};
use techscript_errors::{Diagnostic, DiagnosticLevel, DiagnosticReporter, ErrorCode};
use techscript_syntax::{Token, TokenKind};

/// Unified Result type for parser functions, allowing recovery and early return.
pub type ParseResult<T> = Result<T, ()>;

/// Core parsing state, holding tokens, cursor position, and ID generator.
pub struct Parser<'a> {
    pub(crate) tokens: &'a [Token],
    pub(crate) pos: usize,
    pub(crate) node_id_gen: NodeIdGenerator,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    /// Create a new Parser instance.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            pos: 0,
            node_id_gen: NodeIdGenerator::new(),
        }
    }

    /// Returns `true` if the cursor is at the end of the token stream.
    pub(crate) fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    /// Peeks at the current token without advancing the cursor.
    pub(crate) fn peek(&self) -> &Token {
        if self.pos >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.pos]
        }
    }

    /// Peeks at the token `n` positions ahead of the cursor.
    pub(crate) fn peek_ahead(&self, n: usize) -> &Token {
        if self.pos + n >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.pos + n]
        }
    }

    /// Advances the cursor and returns the previous token.
    pub(crate) fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    /// Returns the previous token.
    pub(crate) fn previous(&self) -> &Token {
        &self.tokens[self.pos - 1]
    }

    /// Checks if the current token matches the given kind.
    pub(crate) fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    /// Matches the current token against the given kind, advancing if matched.
    pub(crate) fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consumes the expected token kind, reporting an error on failure.
    pub(crate) fn consume(
        &mut self,
        kind: TokenKind,
        code: ErrorCode,
        msg: &str,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<&Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let span = self.peek().span;
            let diag = Diagnostic::new(DiagnosticLevel::Error, code, msg.to_string(), span);
            reporter.report(diag);
            Err(())
        }
    }

    /// Generates the next sequential `NodeId`.
    pub(crate) fn next_id(&self) -> techscript_common::NodeId {
        self.node_id_gen.next()
    }

    /// Returns the span of the current token.
    pub(crate) fn current_span(&self) -> Span {
        self.peek().span
    }

    /// Returns the span of the previous token.
    pub(crate) fn prev_span(&self) -> Span {
        self.previous().span
    }

    /// Synchronizes the parser boundary to recover from syntax errors.
    pub(crate) fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon
                || self.previous().kind == TokenKind::Newline
            {
                return;
            }

            match self.peek().kind {
                TokenKind::Make
                | TokenKind::Const
                | TokenKind::Build
                | TokenKind::Struct
                | TokenKind::Enum
                | TokenKind::Model
                | TokenKind::Export
                | TokenKind::If
                | TokenKind::For
                | TokenKind::While
                | TokenKind::Repeat
                | TokenKind::Try
                | TokenKind::Import
                | TokenKind::Say
                | TokenKind::Return
                | TokenKind::Throw
                | TokenKind::Let
                | TokenKind::Var
                | TokenKind::Fun
                | TokenKind::Function
                | TokenKind::When
                | TokenKind::Attempt
                | TokenKind::Class => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }
}
