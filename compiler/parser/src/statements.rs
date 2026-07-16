use crate::parser::{ParseResult, Parser};
use techscript_ast::{
    Block, ExpressionStmt, ForStmt, IfStmt, ImportStmt, RepeatStmt, ReturnStmt, SayStmt, Statement,
    ThrowStmt, TryStmt, WhileStmt,
};
use techscript_common::Span;
use techscript_errors::{DiagnosticReporter, ErrorCode};
use techscript_syntax::TokenKind;

impl<'a> Parser<'a> {
    /// Parse any statement node.
    pub fn parse_statement(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<Statement> {
        // Clear leading newlines before starting a statement
        while self.match_token(TokenKind::Newline) {}

        if self.is_at_end() {
            return Err(());
        }

        // Check if it is a declaration
        if self.check(TokenKind::Make)
            || self.check(TokenKind::Let)
            || self.check(TokenKind::Var)
            || self.check(TokenKind::Const)
            || self.check(TokenKind::Build)
            || self.check(TokenKind::Fun)
            || self.check(TokenKind::Function)
            || self.check(TokenKind::Async)
            || self.check(TokenKind::Struct)
            || self.check(TokenKind::Enum)
            || self.check(TokenKind::Model)
            || self.check(TokenKind::Class)
        {
            return self.parse_declaration(reporter);
        }

        // Standard statement nodes
        if self.check(TokenKind::If) || self.check(TokenKind::When) {
            let stmt = self.parse_if_stmt(reporter)?;
            Ok(Statement::If(stmt))
        } else if self.check(TokenKind::For) || self.check(TokenKind::In) {
            // EBNF uses "for" or "each" (Note: "each" is an identifier, handled below or keyword)
            let stmt = self.parse_for_stmt(reporter)?;
            Ok(Statement::For(stmt))
        } else if self.peek().kind == TokenKind::Identifier
            && (self.peek().lexeme == "each" || self.peek().lexeme == "for")
        {
            let stmt = self.parse_for_stmt(reporter)?;
            Ok(Statement::For(stmt))
        } else if self.check(TokenKind::While) {
            let stmt = self.parse_while_stmt(reporter)?;
            Ok(Statement::While(stmt))
        } else if self.check(TokenKind::Repeat) {
            let stmt = self.parse_repeat_stmt(reporter)?;
            Ok(Statement::Repeat(stmt))
        } else if self.check(TokenKind::Try) || self.check(TokenKind::Attempt) {
            let stmt = self.parse_try_stmt(reporter)?;
            Ok(Statement::Try(stmt))
        } else if self.check(TokenKind::Say) {
            let start_pos = self.peek().span.start;
            self.advance();
            let value = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Say(SayStmt::new(self.next_id(), value, span)))
        } else if self.check(TokenKind::Return) {
            let start_pos = self.peek().span.start;
            self.advance();
            let mut value = None;
            // Check if there is an expression on the same line
            if !self.check(TokenKind::Semicolon)
                && !self.check(TokenKind::Newline)
                && !self.check(TokenKind::RightBrace)
                && !self.is_at_end()
            {
                value = Some(self.parse_expression(techscript_syntax::Precedence::None, reporter)?);
            }
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Return(ReturnStmt::new(
                self.next_id(),
                value,
                span,
            )))
        } else if self.check(TokenKind::Throw) {
            let start_pos = self.peek().span.start;
            self.advance();
            let value = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Throw(ThrowStmt::new(
                self.next_id(),
                value,
                span,
            )))
        } else if self.check(TokenKind::Break) {
            let start_pos = self.peek().span.start;
            self.advance();
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Break(techscript_ast::BreakStmt::new(
                self.next_id(),
                span,
            )))
        } else if self.check(TokenKind::Continue) {
            let start_pos = self.peek().span.start;
            self.advance();
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Continue(techscript_ast::ContinueStmt::new(
                self.next_id(),
                span,
            )))
        } else if self.check(TokenKind::Import)
            || (self.peek().kind == TokenKind::Identifier && self.peek().lexeme == "from")
            || self.check(TokenKind::From)
        {
            let stmt = self.parse_import_stmt(reporter)?;
            Ok(Statement::Import(stmt))
        } else if self.check(TokenKind::LeftBrace) {
            let block = self.parse_block(reporter)?;
            Ok(Statement::Block(block))
        } else {
            // Expression Statement
            let start_pos = self.peek().span.start;
            let expr = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Expression(ExpressionStmt::new(
                self.next_id(),
                expr,
                span,
            )))
        }
    }

    /// Parse block ` { statement* } `
    pub fn parse_block(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<Block> {
        let start_pos = self.peek().span.start;
        self.consume(
            TokenKind::LeftBrace,
            ErrorCode::E0104,
            "Expected '{' to start block",
            reporter,
        )?;

        let mut statements = Vec::new();
        loop {
            while self.match_token(TokenKind::Newline) {}
            if self.check(TokenKind::RightBrace) || self.is_at_end() {
                break;
            }
            match self.parse_statement(reporter) {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                    self.synchronize();
                }
            }
        }

        self.consume(
            TokenKind::RightBrace,
            ErrorCode::E0105,
            "Expected '}' to end block",
            reporter,
        )?;
        let span = Span::new(start_pos, self.previous().span.end);

        Ok(Block::new(self.next_id(), statements, span))
    }

    /// if/when condition { body } elif condition { body } else { body }
    fn parse_if_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<IfStmt> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume if/when

        let condition = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        let body = self.parse_block(reporter)?;

        let mut else_ifs = Vec::new();
        let mut else_body = None;

        while !self.is_at_end() {
            // Skip newlines before checking elif/else
            let mut skipped_newlines = false;
            while self.check(TokenKind::Newline) {
                self.advance();
                skipped_newlines = true;
            }

            if self.match_token(TokenKind::Elif) {
                let cond = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
                let blk = self.parse_block(reporter)?;
                else_ifs.push((cond, blk));
            } else if self.match_token(TokenKind::Else) {
                // Check if followed by "if" or "when" for backward compatibility else-ifs
                if self.check(TokenKind::If) || self.check(TokenKind::When) {
                    self.advance(); // consume if/when
                    let cond =
                        self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
                    let blk = self.parse_block(reporter)?;
                    else_ifs.push((cond, blk));
                } else {
                    else_body = Some(self.parse_block(reporter)?);
                    break;
                }
            } else {
                // If we didn't find elif/else, put back a newline if we skipped any,
                // so that the caller can parse it as a terminator.
                if skipped_newlines {
                    // Note: Instead of actually rewinding the token stream (which Parser does not support directly),
                    // we can just backtrack by 1 token if the last skipped token was a newline.
                    // But wait, the parser has self.current.
                    // If we just do not backtrack, does the caller care?
                    // In parse_block:
                    // while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
                    //     match self.parse_statement(reporter) { ... }
                    // }
                    // Inside parse_statement, the first thing it does is:
                    // while self.match_token(TokenKind::Newline) {}
                    // So any leading newlines are skipped anyway!
                    // Thus, not backtracking is perfectly fine and safe, because newlines are skipped!
                }
                break;
            }
        }

        let span = Span::new(start_pos, self.previous().span.end);
        Ok(IfStmt::new(
            self.next_id(),
            condition,
            body,
            else_ifs,
            else_body,
            span,
        ))
    }

    /// for/each item in iterable { body }
    fn parse_for_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<ForStmt> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume for/each

        let item = self.parse_identifier(reporter)?;
        self.consume(
            TokenKind::In,
            ErrorCode::E0100,
            "Expected 'in' after for loop variable",
            reporter,
        )?;
        let iterable = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        let body = self.parse_block(reporter)?;

        let span = Span::new(start_pos, body.span.end);
        Ok(ForStmt::new(self.next_id(), item, iterable, body, span))
    }

    /// while condition { body }
    fn parse_while_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<WhileStmt> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume while

        let condition = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        let body = self.parse_block(reporter)?;

        let span = Span::new(start_pos, body.span.end);
        Ok(WhileStmt::new(self.next_id(), condition, body, span))
    }

    /// repeat count { body }
    fn parse_repeat_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<RepeatStmt> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume repeat

        let count = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        let body = self.parse_block(reporter)?;

        let span = Span::new(start_pos, body.span.end);
        Ok(RepeatStmt::new(self.next_id(), count, body, span))
    }

    /// try/attempt { body } catch error { catch_body }
    fn parse_try_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<TryStmt> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume try/attempt

        let body = self.parse_block(reporter)?;
        self.consume(
            TokenKind::Catch,
            ErrorCode::E0100,
            "Expected catch block after try",
            reporter,
        )?;
        let catch_var = self.parse_identifier(reporter)?;
        let catch_body = self.parse_block(reporter)?;

        let span = Span::new(start_pos, catch_body.span.end);
        Ok(TryStmt::new(
            self.next_id(),
            body,
            catch_var,
            catch_body,
            span,
        ))
    }

    /// import path [as alias]
    /// from path import (symbols | *)
    fn parse_import_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<ImportStmt> {
        let start_pos = self.peek().span.start;

        if self.match_token(TokenKind::Import) {
            let mut path = Vec::new();
            path.push(self.parse_identifier(reporter)?);
            while self.match_token(TokenKind::Dot) {
                path.push(self.parse_identifier(reporter)?);
            }

            let mut symbols = None;
            if self.check(TokenKind::Identifier) && self.peek().lexeme == "as" {
                self.advance(); // consume "as"
                let alias = self.parse_identifier(reporter)?;
                symbols = Some(vec![alias]);
            }

            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(ImportStmt::new(self.next_id(), path, symbols, span))
        } else {
            // from ... import ...
            self.advance(); // consume from
            let mut path = Vec::new();
            path.push(self.parse_identifier(reporter)?);
            while self.match_token(TokenKind::Dot) {
                path.push(self.parse_identifier(reporter)?);
            }

            self.consume(
                TokenKind::Import,
                ErrorCode::E0100,
                "Expected import after from module path",
                reporter,
            )?;

            let mut symbols = Vec::new();
            if self.match_token(TokenKind::Star) {
                let star_span = self.previous().span;
                symbols.push(techscript_ast::Ident::new("*".to_string(), star_span));
            } else {
                loop {
                    let mut sym = self.parse_identifier(reporter)?;
                    if self.check(TokenKind::Identifier) && self.peek().lexeme == "as" {
                        self.advance(); // consume "as"
                        let alias = self.parse_identifier(reporter)?;
                        sym.name = format!("{}:{}", sym.name, alias.name);
                    }
                    symbols.push(sym);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(ImportStmt::new(self.next_id(), path, Some(symbols), span))
        }
    }

    /// Consumes the required statement terminator (newline, semicolon, or implicit).
    pub fn consume_terminator(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<()> {
        if self.match_token(TokenKind::Semicolon) || self.match_token(TokenKind::Newline) {
            // consume multiple consecutive terminators
            while self.match_token(TokenKind::Semicolon) || self.match_token(TokenKind::Newline) {}
            Ok(())
        } else if self.check(TokenKind::RightBrace) || self.is_at_end() {
            // Implicit terminator permitted
            Ok(())
        } else {
            let span = self.peek().span;
            let diag = techscript_errors::Diagnostic::new(
                techscript_errors::DiagnosticLevel::Error,
                ErrorCode::E0107,
                "Expected statement terminator (semicolon or newline)".to_string(),
                span,
            );
            reporter.report(diag);
            Err(())
        }
    }
}
