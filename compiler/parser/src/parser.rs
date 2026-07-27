use std::collections::HashSet;
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
    /// Registry of known DSL block keywords for declarative syntax.
    /// When an identifier matches this set at the statement level, it triggers DSL block parsing.
    pub(crate) dsl_keywords: HashSet<String>,
    /// Sub-block keywords that can contain children (vs bare properties).
    pub(crate) dsl_sub_blocks: HashSet<String>,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    /// Create a new Parser instance with DSL keyword registry initialized.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            pos: 0,
            node_id_gen: NodeIdGenerator::new(),
            dsl_keywords: Self::init_dsl_keywords(),
            dsl_sub_blocks: Self::init_dsl_sub_blocks(),
        }
    }

    /// Initialize the set of known DSL block keywords.
    /// These are keywords that start a declarative DSL block at the statement level.
    fn init_dsl_keywords() -> HashSet<String> {
        let mut s = HashSet::new();
        // Web module blocks
        s.insert("website".to_string());
        s.insert("page".to_string());
        s.insert("hero".to_string());
        s.insert("section".to_string());
        s.insert("card".to_string());
        s.insert("footer".to_string());
        s.insert("button".to_string());
        s.insert("link".to_string());
        s.insert("input".to_string());
        s.insert("form".to_string());
        s.insert("nav".to_string());
        s.insert("header".to_string());
        s.insert("main".to_string());
        s.insert("aside".to_string());
        s.insert("start".to_string());
        // Canvas module blocks
        s.insert("logo".to_string());
        s.insert("rings".to_string());
        s.insert("emblem".to_string());
        s.insert("core".to_string());
        s.insert("letter".to_string());
        s.insert("circuits".to_string());
        s.insert("title".to_string());
        s.insert("subtitle".to_string());
        s.insert("tagline".to_string());
        s.insert("theme".to_string());
        s.insert("animation".to_string());
        s.insert("export".to_string());
        // Generic DSL blocks
        s.insert("window".to_string());
        s.insert("dialog".to_string());
        s.insert("menu".to_string());
        s
    }

    /// Initialize the set of DSL keywords that are sub-blocks (can contain children).
    /// All other DSL keywords are treated as properties within their parent block.
    fn init_dsl_sub_blocks() -> HashSet<String> {
        let mut s = HashSet::new();
        s.insert("website".to_string());
        s.insert("page".to_string());
        s.insert("hero".to_string());
        s.insert("section".to_string());
        s.insert("card".to_string());
        s.insert("footer".to_string());
        s.insert("button".to_string());
        s.insert("link".to_string());
        s.insert("input".to_string());
        s.insert("form".to_string());
        s.insert("nav".to_string());
        s.insert("header".to_string());
        s.insert("main".to_string());
        s.insert("aside".to_string());
        s.insert("window".to_string());
        s.insert("dialog".to_string());
        s.insert("menu".to_string());
        s
    }

    /// Register DSL keywords from a module. Called when parsing `use module` statements.
    pub(crate) fn register_dsl_keywords(&mut self, module: &str) {
        match module {
            "canvas" | "std.canvas" => {
                for kw in &[
                    "logo",
                    "rings",
                    "emblem",
                    "core",
                    "letter",
                    "circuits",
                    "title",
                    "subtitle",
                    "tagline",
                    "theme",
                    "animation",
                    "export",
                ] {
                    self.dsl_keywords.insert(kw.to_string());
                }
                // Only blocks that can nest children go into dsl_sub_blocks
                for kw in &["logo", "rings", "emblem", "core", "letter", "circuits"] {
                    self.dsl_sub_blocks.insert(kw.to_string());
                }
            }
            "web" | "std.web" => {
                for kw in &[
                    "website", "page", "hero", "section", "card", "footer", "button", "link",
                    "input", "form", "nav", "header", "main", "aside", "start",
                ] {
                    self.dsl_keywords.insert(kw.to_string());
                }
                for kw in &[
                    "website", "page", "hero", "section", "card", "footer", "button", "link",
                    "input", "form", "nav", "header", "main", "aside", "start",
                ] {
                    self.dsl_sub_blocks.insert(kw.to_string());
                }
            }
            _ => {}
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
