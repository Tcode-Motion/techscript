//! # TechScript Lexer Crate
//!
//! Scans raw UTF-8 source strings and tokenizes them into a vector of Tokens.
//! Uses logos DFA definitions for maximum character processing performance,
//! combined with robust manual scanners for strings and f-strings.

use logos::Logos;
use techscript_common::Span;
use techscript_errors::{Diagnostic, DiagnosticLevel, DiagnosticReporter, ErrorCode};
use techscript_syntax::{lookup_keyword, Token, TokenKind};

/// Private token enumeration used internally by Logos for scanning.
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(skip r"[ \t\r]+")] // Skip spaces, tabs, and carriage returns
#[allow(dead_code)]
enum LogosToken {
    #[token("\n")]
    #[token("\r\n")]
    Newline,

    // Block Comments (nestable)
    #[token("/*", block_comment)]
    BlockComment,

    // Identifiers and Potential Keywords
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    IdentifierOrKeyword,

    // Numeric Literals
    #[regex(r"0[xX][0-9a-fA-F_]+")]
    IntHex,
    #[regex(r"0[bB][01_]+")]
    IntBinary,
    #[regex(r"0[oO][0-7_]+")]
    IntOctal,
    #[regex(r"[0-9][0-9_]*")]
    IntDecimal,

    #[regex(
        r"[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?|[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*"
    )]
    Float,

    // Three-character Operators
    #[token("===")]
    TripleEqual,
    #[token("!==")]
    BangEqualEqual,

    // Two-character Operators
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    BangEqual,
    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("*=")]
    StarEqual,
    #[token("/=")]
    SlashEqual,
    #[token("%=")]
    PercentEqual,
    #[token("**")]
    DoubleStar,
    #[token("//")]
    DoubleSlash,
    #[token("..=")]
    DotDotEqual,
    #[token("..")]
    DotDot,
    #[token("?.")]
    QuestionDot,
    #[token("??")]
    QuestionQuestion,
    #[token("->")]
    Arrow,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("&&")]
    DoubleAmpersand,
    #[token("||")]
    DoublePipe,

    // One-character Operators
    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("!")]
    Bang,

    // Delimiters and Separators
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
}

/// Logos callback function to handle nested block comments `/* ... */`.
fn block_comment(lex: &mut logos::Lexer<LogosToken>) -> Result<(), ()> {
    let mut depth = 1;
    let mut chars = lex.remainder().char_indices().peekable();
    let mut bump_len = 0;

    while let Some((i, c)) = chars.next() {
        bump_len = i + c.len_utf8();
        if c == '/' {
            if let Some((_, '*')) = chars.peek() {
                chars.next();
                depth += 1;
            }
        } else if c == '*' {
            if let Some((_, '/')) = chars.peek() {
                chars.next();
                bump_len += 2;
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
        }
    }

    lex.bump(bump_len);
    if depth == 0 {
        Ok(())
    } else {
        Err(())
    }
}

/// Helper function to check if the substring starting with `//` is a comment or a double slash operator.
fn is_comment_start(remaining: &str) -> bool {
    if remaining.starts_with("///") || remaining.starts_with("//!") {
        return true;
    }
    if let Some(after_slash) = remaining.strip_prefix("//") {
        if let Some(next_char) = after_slash.chars().next() {
            if next_char == '\n' || next_char == '\r' {
                return true; // empty comment at end of line
            }
        } else {
            return true; // empty comment at EOF
        }
        let trimmed = after_slash.trim_start_matches([' ', '\t']);
        if let Some(first_char) = trimmed.chars().next() {
            if first_char.is_ascii_digit() || "%*=<>&|!([{},.;?+-".contains(first_char) {
                return false; // DoubleSlash operator
            }
        }
        true
    } else {
        false
    }
}

/// Lexical analyzer that parses source text into a token stream.
pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    token_queue: Vec<Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer for the given source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            token_queue: Vec::new(),
        }
    }

    /// Tokenizes the source string, logging failures to the reporter.
    pub fn lex(
        &mut self,
        reporter: &mut DiagnosticReporter,
    ) -> Result<Vec<Token>, Vec<Diagnostic>> {
        let mut tokens = Vec::new();

        while self.pos < self.source.len() || !self.token_queue.is_empty() {
            // 1. Pop from queue if not empty
            if !self.token_queue.is_empty() {
                tokens.push(self.token_queue.remove(0));
                continue;
            }

            // Skip spaces, tabs, and carriage returns (matching logos skip rule)
            let remaining = &self.source[self.pos..];
            let ws_len = remaining
                .chars()
                .take_while(|c| *c == ' ' || *c == '\t' || *c == '\r')
                .map(|c| c.len_utf8())
                .sum::<usize>();

            if ws_len > 0 {
                self.pos += ws_len;
                continue;
            }

            if self.pos >= self.source.len() {
                break;
            }

            let remaining = &self.source[self.pos..];

            // 2. Intercept and skip line comments before calling Logos
            if remaining.starts_with('#') {
                let len = remaining.find('\n').unwrap_or(remaining.len());
                self.pos += len;
                continue;
            }
            if remaining.starts_with("//") && is_comment_start(remaining) {
                let len = remaining.find('\n').unwrap_or(remaining.len());
                self.pos += len;
                continue;
            }

            // 3. Check for string / f-string starts
            if remaining.starts_with("f\"") || remaining.starts_with("$\"") {
                if self.scan_string(true, reporter).is_err() {
                    return Err(reporter.get_diagnostics().to_vec());
                }
                continue;
            } else if remaining.starts_with('"') {
                if self.scan_string(false, reporter).is_err() {
                    return Err(reporter.get_diagnostics().to_vec());
                }
                continue;
            }

            // 4. Fallback to Logos lexer for standard tokens
            let mut logos_lexer = LogosToken::lexer(remaining);
            if let Some(token_res) = logos_lexer.next() {
                let matched_text = logos_lexer.slice();
                let matched_span = Span::new(
                    self.pos + logos_lexer.span().start,
                    self.pos + logos_lexer.span().end,
                );

                self.pos += logos_lexer.span().end;

                match token_res {
                    Ok(logos_token) => {
                        let kind =
                            self.map_logos_token(logos_token, matched_text, matched_span, reporter);
                        if logos_token != LogosToken::BlockComment {
                            tokens.push(Token::new(kind, matched_text.to_string(), matched_span));
                        }
                    }
                    Err(_) => {
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Error,
                            ErrorCode::E0001,
                            format!("Unexpected character: '{}'", matched_text),
                            matched_span,
                        );
                        reporter.report(diag);
                        return Err(reporter.get_diagnostics().to_vec());
                    }
                }
            } else {
                let matched_span = Span::new(self.pos, self.pos + 1);
                let char_str = remaining
                    .chars()
                    .next()
                    .map_or("EOF".to_string(), |c| c.to_string());
                let diag = Diagnostic::new(
                    DiagnosticLevel::Error,
                    ErrorCode::E0001,
                    format!("Unexpected character: '{}'", char_str),
                    matched_span,
                );
                reporter.report(diag);
                return Err(reporter.get_diagnostics().to_vec());
            }
        }

        if reporter.has_errors() {
            return Err(reporter.get_diagnostics().to_vec());
        }

        // Collapse consecutive newlines to a single Newline token
        let mut collapsed_tokens = Vec::new();
        let mut last_was_newline = false;
        for token in tokens {
            if token.kind == TokenKind::Newline {
                if !last_was_newline {
                    collapsed_tokens.push(token);
                    last_was_newline = true;
                }
            } else {
                collapsed_tokens.push(token);
                last_was_newline = false;
            }
        }

        // Add EOF token
        collapsed_tokens.push(Token::new(
            TokenKind::Eof,
            "".to_string(),
            Span::new(self.source.len(), self.source.len()),
        ));

        Ok(collapsed_tokens)
    }

    /// Maps internal `LogosToken` to public `TokenKind` and handles validation.
    fn map_logos_token(
        &self,
        token: LogosToken,
        lexeme: &str,
        span: Span,
        reporter: &mut DiagnosticReporter,
    ) -> TokenKind {
        match token {
            LogosToken::Newline => TokenKind::Newline,
            LogosToken::IdentifierOrKeyword => {
                if lexeme.starts_with("__") {
                    let warning = Diagnostic::new(
                        DiagnosticLevel::Warning,
                        ErrorCode::W0001,
                        format!(
                            "Identifier '{}' starts with reserved double underscore prefix",
                            lexeme
                        ),
                        span,
                    );
                    reporter.report(warning);
                }
                lookup_keyword(lexeme).unwrap_or(TokenKind::Identifier)
            }
            LogosToken::IntHex => {
                self.validate_numeric_suffix(lexeme, span, reporter);
                TokenKind::IntLiteral
            }
            LogosToken::IntBinary => {
                self.validate_numeric_suffix(lexeme, span, reporter);
                TokenKind::IntLiteral
            }
            LogosToken::IntOctal => {
                self.validate_numeric_suffix(lexeme, span, reporter);
                TokenKind::IntLiteral
            }
            LogosToken::IntDecimal => {
                self.validate_numeric_suffix(lexeme, span, reporter);
                TokenKind::IntLiteral
            }
            LogosToken::Float => {
                self.validate_numeric_suffix(lexeme, span, reporter);
                TokenKind::FloatLiteral
            }
            LogosToken::TripleEqual => TokenKind::TripleEqual,
            LogosToken::BangEqualEqual => TokenKind::BangEqualEqual,
            LogosToken::EqualEqual => TokenKind::EqualEqual,
            LogosToken::BangEqual => TokenKind::BangEqual,
            LogosToken::PlusEqual => TokenKind::PlusEqual,
            LogosToken::MinusEqual => TokenKind::MinusEqual,
            LogosToken::StarEqual => TokenKind::StarEqual,
            LogosToken::SlashEqual => TokenKind::SlashEqual,
            LogosToken::PercentEqual => TokenKind::PercentEqual,
            LogosToken::DoubleStar => TokenKind::DoubleStar,
            LogosToken::DoubleSlash => TokenKind::DoubleSlash,
            LogosToken::DotDotEqual => TokenKind::DotDotEqual,
            LogosToken::DotDot => TokenKind::DotDot,
            LogosToken::QuestionDot => TokenKind::QuestionDot,
            LogosToken::QuestionQuestion => TokenKind::QuestionQuestion,
            LogosToken::Arrow => TokenKind::Arrow,
            LogosToken::LessEqual => TokenKind::LessEqual,
            LogosToken::GreaterEqual => TokenKind::GreaterEqual,
            LogosToken::DoubleAmpersand => TokenKind::And,
            LogosToken::DoublePipe => TokenKind::Or,
            LogosToken::Equal => TokenKind::Equal,
            LogosToken::Plus => TokenKind::Plus,
            LogosToken::Minus => TokenKind::Minus,
            LogosToken::Star => TokenKind::Star,
            LogosToken::Slash => TokenKind::Slash,
            LogosToken::Percent => TokenKind::Percent,
            LogosToken::Less => TokenKind::Less,
            LogosToken::Greater => TokenKind::Greater,
            LogosToken::Bang => TokenKind::Not,
            LogosToken::LeftParen => TokenKind::LeftParen,
            LogosToken::RightParen => TokenKind::RightParen,
            LogosToken::LeftBrace => TokenKind::LeftBrace,
            LogosToken::RightBrace => TokenKind::RightBrace,
            LogosToken::LeftBracket => TokenKind::LeftBracket,
            LogosToken::RightBracket => TokenKind::RightBracket,
            LogosToken::Comma => TokenKind::Comma,
            LogosToken::Dot => TokenKind::Dot,
            LogosToken::Colon => TokenKind::Colon,
            LogosToken::Semicolon => TokenKind::Semicolon,
            LogosToken::BlockComment => TokenKind::Error, // skipped in lex loop
        }
    }

    /// Validates that numeric literals do not end in trailing underscores.
    fn validate_numeric_suffix(&self, lexeme: &str, span: Span, reporter: &mut DiagnosticReporter) {
        if lexeme.ends_with('_') {
            let diag = Diagnostic::new(
                DiagnosticLevel::Error,
                ErrorCode::E0010,
                format!(
                    "Numeric literal '{}' cannot have a trailing underscore",
                    lexeme
                ),
                span,
            );
            reporter.report(diag);
        }
    }

    /// Scans standard string literals and recursive f-strings.
    fn scan_string(
        &mut self,
        is_fstring: bool,
        reporter: &mut DiagnosticReporter,
    ) -> Result<(), ()> {
        let start_pos = self.pos;
        if is_fstring {
            let lexeme = self.source[start_pos..start_pos + 2].to_string();
            self.pos += 2; // skip `f"` or `$"`
            self.token_queue.push(Token::new(
                TokenKind::FStringStart,
                lexeme,
                Span::new(start_pos, start_pos + 2),
            ));

            let mut text_start = self.pos;
            let mut text_buf = String::new();

            while self.pos < self.source.len() {
                let remaining = &self.source[self.pos..];
                if remaining.starts_with('"') {
                    if !text_buf.is_empty() {
                        self.token_queue.push(Token::new(
                            TokenKind::FStringText,
                            text_buf.clone(),
                            Span::new(text_start, self.pos),
                        ));
                    }
                    self.token_queue.push(Token::new(
                        TokenKind::FStringEnd,
                        "\"".to_string(),
                        Span::new(self.pos, self.pos + 1),
                    ));
                    self.pos += 1;
                    return Ok(());
                }

                if remaining.starts_with('{') {
                    if !text_buf.is_empty() {
                        self.token_queue.push(Token::new(
                            TokenKind::FStringText,
                            text_buf.clone(),
                            Span::new(text_start, self.pos),
                        ));
                        text_buf.clear();
                    }

                    let expr_start_pos = self.pos;
                    self.token_queue.push(Token::new(
                        TokenKind::FStringExprStart,
                        "{".to_string(),
                        Span::new(expr_start_pos, expr_start_pos + 1),
                    ));
                    self.pos += 1;

                    let mut brace_count = 1;
                    let expr_text_start = self.pos;
                    while self.pos < self.source.len() {
                        let c = self.source[self.pos..].chars().next().unwrap();
                        if c == '{' {
                            brace_count += 1;
                        } else if c == '}' {
                            brace_count -= 1;
                            if brace_count == 0 {
                                break;
                            }
                        }
                        self.pos += c.len_utf8();
                    }

                    if brace_count > 0 {
                        let span = Span::new(expr_start_pos, self.source.len());
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Error,
                            ErrorCode::E0021,
                            "Unterminated expression brace inside f-string".to_string(),
                            span,
                        );
                        reporter.report(diag);
                        return Err(());
                    }

                    let expr_substring = &self.source[expr_text_start..self.pos];
                    let mut inner_lexer = Lexer::new(expr_substring);
                    let mut inner_reporter = DiagnosticReporter::new();
                    match inner_lexer.lex(&mut inner_reporter) {
                        Ok(inner_tokens) => {
                            for mut token in inner_tokens {
                                if token.kind != TokenKind::Eof {
                                    token.span = Span::new(
                                        expr_text_start + token.span.start,
                                        expr_text_start + token.span.end,
                                    );
                                    self.token_queue.push(token);
                                }
                            }
                        }
                        Err(inner_diags) => {
                            for diag in inner_diags {
                                let mut adjusted = diag.clone();
                                adjusted.span = Span::new(
                                    expr_text_start + diag.span.start,
                                    expr_text_start + diag.span.end,
                                );
                                reporter.report(adjusted);
                            }
                            return Err(());
                        }
                    }

                    self.token_queue.push(Token::new(
                        TokenKind::FStringExprEnd,
                        "}".to_string(),
                        Span::new(self.pos, self.pos + 1),
                    ));
                    self.pos += 1; // skip `}`
                    text_start = self.pos;
                    continue;
                }

                let c = remaining.chars().next().unwrap();
                if c == '\\' {
                    if remaining.len() < 2 {
                        let span = Span::new(self.pos, self.pos + 1);
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Error,
                            ErrorCode::E0021,
                            "Trailing backslash escape inside f-string".to_string(),
                            span,
                        );
                        reporter.report(diag);
                        return Err(());
                    }
                    let next_c = remaining.chars().nth(1).unwrap();
                    text_buf.push('\\');
                    text_buf.push(next_c);
                    self.pos += 2;
                } else {
                    text_buf.push(c);
                    self.pos += c.len_utf8();
                }
            }
        } else {
            self.pos += 1; // skip starting quote `"`
            while self.pos < self.source.len() {
                let remaining = &self.source[self.pos..];
                if remaining.starts_with('"') {
                    self.pos += 1; // skip ending quote `"`
                    let raw_lexeme = &self.source[start_pos..self.pos];
                    self.token_queue.push(Token::new(
                        TokenKind::StringLiteral,
                        raw_lexeme.to_string(),
                        Span::new(start_pos, self.pos),
                    ));
                    return Ok(());
                }

                let c = remaining.chars().next().unwrap();
                if c == '\\' {
                    if remaining.len() < 2 {
                        let span = Span::new(self.pos, self.pos + 1);
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Error,
                            ErrorCode::E0021,
                            "Trailing backslash escape inside string".to_string(),
                            span,
                        );
                        reporter.report(diag);
                        return Err(());
                    }
                    self.pos += 2;
                } else {
                    self.pos += c.len_utf8();
                }
            }
        }

        let span = Span::new(start_pos, self.source.len());
        let diag = Diagnostic::new(
            DiagnosticLevel::Error,
            ErrorCode::E0021,
            "Unterminated string literal".to_string(),
            span,
        );
        reporter.report(diag);
        Err(())
    }
}

/// Helper function to scan source code directly.
pub fn lex(source: &str, reporter: &mut DiagnosticReporter) -> Result<Vec<Token>, Vec<Diagnostic>> {
    let mut lexer = Lexer::new(source);
    lexer.lex(reporter)
}

impl<'a> Lexer<'a> {
    /// Tokenizes the source string, recovering from error states instead of failing.
    pub fn lex_recovered(
        &mut self,
        reporter: &mut DiagnosticReporter,
    ) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.source.len() || !self.token_queue.is_empty() {
            if !self.token_queue.is_empty() {
                tokens.push(self.token_queue.remove(0));
                continue;
            }

            let remaining = &self.source[self.pos..];
            let ws_len = remaining
                .chars()
                .take_while(|c| *c == ' ' || *c == '\t' || *c == '\r')
                .map(|c| c.len_utf8())
                .sum::<usize>();

            if ws_len > 0 {
                self.pos += ws_len;
                continue;
            }

            if self.pos >= self.source.len() {
                break;
            }

            let remaining = &self.source[self.pos..];

            if remaining.starts_with('#') {
                let len = remaining.find('\n').unwrap_or(remaining.len());
                self.pos += len;
                continue;
            }
            if remaining.starts_with("//") && is_comment_start(remaining) {
                let len = remaining.find('\n').unwrap_or(remaining.len());
                self.pos += len;
                continue;
            }

            if remaining.starts_with("f\"") {
                if self.scan_string(true, reporter).is_err() {
                    self.pos += 2; // skip start of f-string
                }
                continue;
            } else if remaining.starts_with('"') {
                if self.scan_string(false, reporter).is_err() {
                    self.pos += 1; // skip start of string
                }
                continue;
            }

            let mut logos_lexer = LogosToken::lexer(remaining);
            if let Some(token_res) = logos_lexer.next() {
                let matched_text = logos_lexer.slice();
                let matched_span = Span::new(
                    self.pos + logos_lexer.span().start,
                    self.pos + logos_lexer.span().end,
                );

                self.pos += logos_lexer.span().end;

                match token_res {
                    Ok(logos_token) => {
                        let kind =
                            self.map_logos_token(logos_token, matched_text, matched_span, reporter);
                        if logos_token != LogosToken::BlockComment {
                            tokens.push(Token::new(kind, matched_text.to_string(), matched_span));
                        }
                    }
                    Err(_) => {
                        let span = Span::new(self.pos - matched_text.len(), self.pos);
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Error,
                            ErrorCode::E0001,
                            format!("Unexpected character: '{}'", matched_text),
                            span,
                        );
                        reporter.report(diag);
                    }
                }
            } else {
                self.pos += 1;
            }
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            String::new(),
            Span::new(self.source.len(), self.source.len()),
        ));

        tokens
    }
}

pub fn lex_recovered(source: &str, reporter: &mut DiagnosticReporter) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    lexer.lex_recovered(reporter)
}

