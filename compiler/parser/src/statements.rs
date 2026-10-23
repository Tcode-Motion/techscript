use crate::parser::{ParseResult, Parser};
use techscript_ast::{
    Block, CallExpr, DSLBlock, DSLChild, DSLProperty, Expression, ExpressionStmt, ForStmt, IfStmt,
    ImportStmt, LiteralExpr, LiteralVal, RepeatStmt, ReturnStmt, SayStmt, Statement, ThrowStmt,
    TryStmt, WhileStmt,
};
use techscript_common::{Ident, Span};
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
            || self.check(TokenKind::Keep)
            || self.check(TokenKind::Build)
            || self.check(TokenKind::Fun)
            || self.check(TokenKind::Function)
            || self.check(TokenKind::Async)
            || self.check(TokenKind::Do)
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
        } else if self.check(TokenKind::For)
            || self.check(TokenKind::Each)
            || self.check(TokenKind::In)
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
        } else if self.check(TokenKind::Send) && self.check_is_send_to() {
            // WebForge API send: `send NoteInput to /api/notes`
            let start_pos = self.peek().span.start;
            self.advance(); // consume 'send'
            let payload = if self.check(TokenKind::Identifier) && self.peek_ahead(1).lexeme == "to"
            {
                let id_tok = self.advance();
                Expression::Identifier(Ident::new(id_tok.lexeme.clone(), id_tok.span))
            } else if self.check(TokenKind::StringLiteral) && self.peek_ahead(1).lexeme == "to" {
                let str_tok = self.advance();
                Expression::Literal(LiteralExpr::new(
                    self.next_id(),
                    LiteralVal::Str(str_tok.lexeme.clone()),
                    str_tok.span,
                ))
            } else {
                self.parse_expression(techscript_syntax::Precedence::None, reporter)?
            };

            if self.peek().lexeme == "to" {
                self.advance(); // consume 'to'
            }
            let endpoint_expr = if self.check(TokenKind::StringLiteral) {
                self.parse_expression(techscript_syntax::Precedence::None, reporter)?
            } else {
                let ep_start = self.peek().span.start;
                let mut path_str = String::new();
                while self.check(TokenKind::Slash) || self.check(TokenKind::Identifier) {
                    path_str.push_str(&self.peek().lexeme);
                    self.advance();
                }
                let ep_span = Span::new(ep_start, self.previous().span.end);
                Expression::Literal(LiteralExpr::new(
                    self.next_id(),
                    LiteralVal::Str(path_str),
                    ep_span,
                ))
            };
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            let send_func = Expression::Identifier(Ident::new("web_api_send".to_string(), span));
            let call = CallExpr::new(
                self.next_id(),
                Box::new(send_func),
                vec![endpoint_expr, payload],
                span,
            );
            Ok(Statement::Expression(ExpressionStmt::new(
                self.next_id(),
                Expression::Call(call),
                span,
            )))
        } else if self.check(TokenKind::Return)
            || self.check(TokenKind::Give)
            || self.check(TokenKind::Send)
        {
            let start_pos = self.peek().span.start;
            let kw_token = self.peek();
            if kw_token.kind == TokenKind::Return {
                reporter.report(techscript_errors::Diagnostic::new(
                    techscript_errors::DiagnosticLevel::Warning,
                    techscript_errors::ErrorCode::TSW1003,
                    "Warning TSW1003: 'return' is deprecated. Use 'send' to return values."
                        .to_string(),
                    kw_token.span,
                ));
            } else if kw_token.kind == TokenKind::Give {
                reporter.report(techscript_errors::Diagnostic::new(
                    techscript_errors::DiagnosticLevel::Warning,
                    techscript_errors::ErrorCode::TSW1005,
                    "Warning TSW1005: 'give' is deprecated. Use 'send' to return values."
                        .to_string(),
                    kw_token.span,
                ));
            }
            self.advance();
            let mut value = None;
            // Check if there is an expression on the same line
            if !self.check(TokenKind::Semicolon)
                && !self.check(TokenKind::Newline)
                && !self.check(TokenKind::RightBrace)
                && !self.check(TokenKind::End)
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
        } else if self.check(TokenKind::Break) || self.check(TokenKind::Stop) {
            let start_pos = self.peek().span.start;
            self.advance();
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Break(techscript_ast::BreakStmt::new(
                self.next_id(),
                span,
            )))
        } else if self.check(TokenKind::Continue) || self.check(TokenKind::Skip) {
            let start_pos = self.peek().span.start;
            self.advance();
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Continue(techscript_ast::ContinueStmt::new(
                self.next_id(),
                span,
            )))
        } else if self.check(TokenKind::Use) {
            let stmt = self.parse_use_stmt(reporter)?;
            Ok(Statement::Import(stmt))
        } else if self.check(TokenKind::Import)
            || (self.peek().kind == TokenKind::Identifier && self.peek().lexeme == "from")
            || self.check(TokenKind::From)
        {
            let stmt = self.parse_import_stmt(reporter)?;
            Ok(Statement::Import(stmt))
        } else if self.check(TokenKind::LeftBrace) {
            let block = self.parse_block(reporter)?;
            Ok(Statement::Block(block))
        } else if self.check(TokenKind::Identifier)
            && self.peek().lexeme == "start"
            && self.peek_ahead(1).lexeme == "website"
        {
            let start_pos = self.peek().span.start;
            self.advance(); // start
            self.advance(); // website
            let mut port_val = None;
            if !self.check(TokenKind::Newline)
                && !self.check(TokenKind::Semicolon)
                && !self.is_at_end()
            {
                port_val =
                    Some(self.parse_expression(techscript_syntax::Precedence::None, reporter)?);
            }
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            let start_func =
                Expression::Identifier(Ident::new("web_start_website".to_string(), span));
            let mut call_args = Vec::new();
            if let Some(pv) = port_val {
                call_args.push(pv);
            }
            let call = CallExpr::new(self.next_id(), Box::new(start_func), call_args, span);
            Ok(Statement::Expression(ExpressionStmt::new(
                self.next_id(),
                Expression::Call(call),
                span,
            )))
        } else if self.check(TokenKind::Identifier)
            && self.peek().lexeme == "show"
            && self.peek_ahead(1).lexeme == "message"
        {
            let start_pos = self.peek().span.start;
            self.advance(); // show
            self.advance(); // message
            let val = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Say(SayStmt::new(self.next_id(), val, span)))
        } else if self.check(TokenKind::Identifier)
            && (self.peek().lexeme == "show" || self.dsl_keywords.contains(&self.peek().lexeme))
            && !matches!(
                self.peek_ahead(1).kind,
                TokenKind::Equal
                    | TokenKind::PlusEqual
                    | TokenKind::MinusEqual
                    | TokenKind::StarEqual
                    | TokenKind::SlashEqual
                    | TokenKind::PercentEqual
                    | TokenKind::LeftBracket
                    | TokenKind::Dot
                    | TokenKind::LeftParen
            )
        {
            let block = self.parse_dsl_block(reporter)?;
            Ok(Statement::DSL(block))
        } else {
            let start_pos = self.peek().span.start;
            let mut expr = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
            // v1.0.8 permits the readable one-argument call `greet "World"`.
            if matches!(expr, techscript_ast::Expression::Identifier(_))
                && matches!(
                    self.peek().kind,
                    TokenKind::StringLiteral
                        | TokenKind::FStringStart
                        | TokenKind::IntLiteral
                        | TokenKind::FloatLiteral
                        | TokenKind::True
                        | TokenKind::False
                        | TokenKind::LeftBracket
                        | TokenKind::LeftBrace
                )
            {
                let argument =
                    self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
                let span = Span::new(expr.span().start, argument.span().end);
                expr = techscript_ast::Expression::Call(techscript_ast::CallExpr::new(
                    self.next_id(),
                    Box::new(expr),
                    vec![argument],
                    span,
                ));
            }
            self.consume_terminator(reporter)?;
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Statement::Expression(ExpressionStmt::new(
                self.next_id(),
                expr,
                span,
            )))
        }
    }

    pub fn parse_block_or_then_end(
        &mut self,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<Block> {
        while self.match_token(TokenKind::Newline) {}
        let start_pos = self.peek().span.start;
        if self.check(TokenKind::LeftBrace) {
            self.parse_block(reporter)
        } else {
            let has_then = self.match_token(TokenKind::Then);
            let mut statements = Vec::new();
            loop {
                while self.match_token(TokenKind::Newline) {}
                if self.check(TokenKind::End)
                    || self.check(TokenKind::Else)
                    || self.check(TokenKind::Elif)
                    || self.check(TokenKind::Catch)
                    || self.is_at_end()
                    || (!has_then
                        && ((self.check(TokenKind::When)
                            && self.peek_ahead(1).kind == TokenKind::Identifier
                            && self.peek_ahead(2).lexeme == "is")
                            || self.check(TokenKind::Make)
                            || (self.check(TokenKind::Identifier)
                                && ["page", "database", "start", "website"]
                                    .contains(&self.peek().lexeme.as_str()))))
                {
                    break;
                }
                match self.parse_statement(reporter) {
                    Ok(stmt) => statements.push(stmt),
                    Err(_) => {
                        self.synchronize();
                    }
                }
            }
            if self.check(TokenKind::End) {
                self.advance(); // consume 'end'
            } else if has_then {
                self.consume(
                    TokenKind::End,
                    ErrorCode::E0105,
                    "Expected 'end' to end block starting with 'then'",
                    reporter,
                )?;
            }
            let span = Span::new(start_pos, self.previous().span.end);
            Ok(Block::new(self.next_id(), statements, span))
        }
    }

    /// Parse block ` { statement* } `
    pub fn parse_block(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<Block> {
        let start_pos = self.peek().span.start;
        if self.check(TokenKind::LeftBrace) {
            reporter.report(techscript_errors::Diagnostic::new(
                techscript_errors::DiagnosticLevel::Warning,
                techscript_errors::ErrorCode::TSW1006,
                "Warning TSW1006: Braces are deprecated. Use 'then ... end' or canonical end blocks instead.".to_string(),
                self.peek().span,
            ));
        }
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

        if self.check(TokenKind::RightBrace) {
            reporter.report(techscript_errors::Diagnostic::new(
                techscript_errors::DiagnosticLevel::Warning,
                techscript_errors::ErrorCode::TSW1006,
                "Warning TSW1006: Braces are deprecated. Use 'then ... end' or canonical end blocks instead.".to_string(),
                self.peek().span,
            ));
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
        if self.check(TokenKind::If) {
            reporter.report(techscript_errors::Diagnostic::new(
                techscript_errors::DiagnosticLevel::Warning,
                techscript_errors::ErrorCode::TSW1007,
                "Warning TSW1007: 'if' is deprecated. Use 'when' instead.".to_string(),
                self.peek().span,
            ));
        }
        self.advance(); // consume if/when

        // Support English-like WebForge event syntax: when <TargetId> is <clicked/submitted/...>:
        let is_event_syntax =
            self.peek().kind == TokenKind::Identifier && self.peek_ahead(1).lexeme == "is";

        let condition = if is_event_syntax {
            let target_token = self.advance();
            let target_name = target_token.lexeme.clone();
            self.advance(); // consume "is"
            let event_token = self.advance(); // consume event (e.g. "clicked")
            let event_name = event_token.lexeme.clone();
            let span = Span::new(target_token.span.start, event_token.span.end);
            techscript_ast::Expression::Binary(techscript_ast::BinaryExpr::new(
                self.next_id(),
                Box::new(techscript_ast::Expression::Identifier(Ident::new(
                    target_name,
                    target_token.span,
                ))),
                "is".to_string(),
                Box::new(techscript_ast::Expression::Identifier(Ident::new(
                    event_name,
                    event_token.span,
                ))),
                span,
            ))
        } else {
            self.parse_expression(techscript_syntax::Precedence::None, reporter)?
        };

        self.match_token(TokenKind::Colon);
        let body = self.parse_block_or_then_end(reporter)?;

        let mut else_ifs = Vec::new();
        let mut else_body = None;

        while !self.is_at_end() {
            // Skip newlines before checking elif/else
            while self.check(TokenKind::Newline) {
                self.advance();
            }

            if self.match_token(TokenKind::Elif) {
                let cond = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
                self.match_token(TokenKind::Colon);
                let blk = self.parse_block_or_then_end(reporter)?;
                else_ifs.push((cond, blk));
            } else if self.check(TokenKind::Or) && self.peek_ahead(1).kind == TokenKind::When {
                self.advance(); // consume Or
                self.advance(); // consume When
                let cond = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
                self.match_token(TokenKind::Colon);
                let blk = self.parse_block_or_then_end(reporter)?;
                else_ifs.push((cond, blk));
            } else if self.match_token(TokenKind::Else) {
                // Check if followed by "if" or "when" for backward compatibility else-ifs
                if self.check(TokenKind::If) || self.check(TokenKind::When) {
                    if self.check(TokenKind::If) {
                        reporter.report(techscript_errors::Diagnostic::new(
                            techscript_errors::DiagnosticLevel::Warning,
                            techscript_errors::ErrorCode::TSW1007,
                            "Warning TSW1007: 'if' is deprecated. Use 'when' instead.".to_string(),
                            self.peek().span,
                        ));
                    }
                    self.advance(); // consume if/when
                    let cond =
                        self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
                    self.match_token(TokenKind::Colon);
                    let blk = self.parse_block_or_then_end(reporter)?;
                    else_ifs.push((cond, blk));
                } else {
                    self.match_token(TokenKind::Colon);
                    else_body = Some(self.parse_block_or_then_end(reporter)?);
                    break;
                }
            } else {
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
        let body = self.parse_block_or_then_end(reporter)?;

        let span = Span::new(start_pos, body.span.end);
        Ok(ForStmt::new(self.next_id(), item, iterable, body, span))
    }

    /// while condition { body }
    fn parse_while_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<WhileStmt> {
        let start_pos = self.peek().span.start;
        reporter.report(techscript_errors::Diagnostic::new(
            techscript_errors::DiagnosticLevel::Warning,
            techscript_errors::ErrorCode::TSW1008,
            "Warning TSW1008: 'while' is deprecated. Use 'repeat' instead.".to_string(),
            self.peek().span,
        ));
        self.advance(); // consume while

        let condition = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        let body = self.parse_block_or_then_end(reporter)?;

        let span = Span::new(start_pos, body.span.end);
        Ok(WhileStmt::new(self.next_id(), condition, body, span))
    }

    /// repeat count { body }
    fn parse_repeat_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<RepeatStmt> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume repeat

        let count = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        let body = self.parse_block_or_then_end(reporter)?;

        let span = Span::new(start_pos, body.span.end);
        Ok(RepeatStmt::new(self.next_id(), count, body, span))
    }

    /// try/attempt { body } catch error { catch_body }
    fn parse_try_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<TryStmt> {
        let start_pos = self.peek().span.start;
        if self.check(TokenKind::Attempt) {
            reporter.report(techscript_errors::Diagnostic::new(
                techscript_errors::DiagnosticLevel::Warning,
                techscript_errors::ErrorCode::TSW1004,
                "Warning TSW1004: 'attempt' is deprecated. Use 'try' instead.".to_string(),
                self.peek().span,
            ));
        }
        self.advance(); // consume try/attempt

        let body = self.parse_block_or_then_end(reporter)?;
        self.consume(
            TokenKind::Catch,
            ErrorCode::E0100,
            "Expected catch block after try",
            reporter,
        )?;
        let catch_var = self.parse_identifier(reporter)?;
        let catch_body = self.parse_block_or_then_end(reporter)?;

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

    /// Parse the v1.0.8 `use module` form into the shared import AST node.
    /// Also registers the module's DSL keywords for subsequent declarative block parsing.
    fn parse_use_stmt(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<ImportStmt> {
        let start_pos = self.peek().span.start;
        self.advance(); // use
        let mut path = vec![self.parse_identifier(reporter)?];
        while self.match_token(TokenKind::Dot) {
            path.push(self.parse_identifier(reporter)?);
        }
        self.consume_terminator(reporter)?;

        // Register DSL keywords from the imported module
        let module_name = path
            .iter()
            .map(|i| i.name.as_str())
            .collect::<Vec<_>>()
            .join(".");
        self.register_dsl_keywords(&module_name);

        let span = Span::new(start_pos, self.previous().span.end);
        Ok(ImportStmt::new(self.next_id(), path, None, span))
    }

    /// Parse a declarative DSL block: `keyword [args...]` newline `[body...]` `end`
    pub fn parse_dsl_block(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<DSLBlock> {
        let start_pos = self.peek().span.start;
        let mut kind = self.parse_identifier(reporter)?.name;

        if kind == "site" {
            kind = "website".to_string();
        } else if kind == "data" {
            kind = "database".to_string();
        } else if kind == "model3D" || kind == "model3d" || kind == "scene3d" || kind == "scene3D" {
            kind = "scene3d".to_string();
        } else if kind == "3D" || kind == "3d" {
            if self.peek().lexeme == "scene" {
                self.advance();
            }
            kind = "scene3d".to_string();
        } else if kind == "show" || kind == "make" {
            if self.peek().lexeme == "3D" || self.peek().lexeme == "3d" {
                self.advance();
                if self.peek().lexeme == "scene" || self.peek().lexeme == "model" {
                    self.advance();
                }
                kind = "scene3d".to_string();
            } else if (self.peek().kind == TokenKind::IntLiteral && self.peek().lexeme == "3")
                && (self.peek_ahead(1).lexeme == "D" || self.peek_ahead(1).lexeme == "d")
            {
                self.advance(); // 3
                self.advance(); // D
                if self.peek().lexeme == "scene" || self.peek().lexeme == "model" {
                    self.advance();
                }
                kind = "scene3d".to_string();
            } else if self.check(TokenKind::Identifier) {
                kind = self.advance().lexeme.clone();
            }
        }

        // Parse optional arguments until the statement terminator, colon, or left brace
        let mut args = Vec::new();
        let mut properties = Vec::new();
        while !self.check(TokenKind::Newline)
            && !self.check(TokenKind::Semicolon)
            && !self.check(TokenKind::Colon)
            && !self.check(TokenKind::LeftBrace)
            && !self.is_at_end()
        {
            if self.match_token(TokenKind::Arrow) {
                let action_start = self.peek().span.start;
                let mut action_str = String::new();
                while !self.check(TokenKind::Newline)
                    && !self.check(TokenKind::Semicolon)
                    && !self.is_at_end()
                {
                    if !action_str.is_empty() {
                        action_str.push(' ');
                    }
                    action_str.push_str(&self.peek().lexeme);
                    self.advance();
                }
                let action_span = Span::new(action_start, self.previous().span.end);
                properties.push(DSLProperty::new(
                    self.next_id(),
                    "action".to_string(),
                    Some(Expression::Literal(LiteralExpr::new(
                        self.next_id(),
                        LiteralVal::Str(action_str),
                        action_span,
                    ))),
                    action_span,
                ));
                break;
            } else if self.peek().lexeme == "called" || self.peek().lexeme == "as" {
                self.advance();
                let id_start = self.peek().span.start;
                let id_name =
                    if self.check(TokenKind::Identifier) || self.check(TokenKind::StringLiteral) {
                        let s = self.peek().lexeme.clone();
                        self.advance();
                        s
                    } else {
                        "element".to_string()
                    };
                let id_span = Span::new(id_start, self.previous().span.end);
                properties.push(DSLProperty::new(
                    self.next_id(),
                    "id".to_string(),
                    Some(Expression::Literal(LiteralExpr::new(
                        self.next_id(),
                        LiteralVal::Str(id_name),
                        id_span,
                    ))),
                    id_span,
                ));
            } else {
                args.push(self.parse_expression(techscript_syntax::Precedence::Call, reporter)?);
            }
        }
        let has_brace = self.match_token(TokenKind::LeftBrace);
        let has_colon = if !has_brace {
            self.match_token(TokenKind::Colon)
        } else {
            false
        };
        let mut body_props = Vec::new();
        let mut children = Vec::new();

        if has_colon && !self.check(TokenKind::Newline) && !self.check(TokenKind::Semicolon) {
            // Inline properties on the same line after ':'
            while !self.check(TokenKind::Newline)
                && !self.check(TokenKind::Semicolon)
                && !self.check(TokenKind::End)
                && !self.check(TokenKind::RightBrace)
                && !self.is_at_end()
            {
                if self.check(TokenKind::Identifier) {
                    let prop = self.parse_dsl_property(reporter)?;
                    body_props.push(prop);
                } else {
                    break;
                }
            }
            if self.check(TokenKind::End) || self.check(TokenKind::RightBrace) {
                self.advance();
            }
            if !self.is_at_end()
                && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
            {
                self.consume_terminator(reporter)?;
            }
        } else if has_brace {
            if !self.is_at_end()
                && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
            {
                self.consume_terminator(reporter)?;
            }
            let (bp, ch) = self.parse_dsl_body_delimited(&kind, true, reporter)?;
            body_props = bp;
            children = ch;
        } else if has_colon {
            if !self.is_at_end()
                && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
            {
                self.consume_terminator(reporter)?;
            }
            let (bp, ch) = self.parse_dsl_body_delimited(&kind, false, reporter)?;
            body_props = bp;
            children = ch;
        } else {
            // Neither brace nor colon.
            // If the next line has 'end', parse body as classic block. Otherwise it's a standalone element (e.g. `button "Get Started"`).
            let mut is_classic_end_block = false;
            let mut scan = 0;
            while self.pos + scan < self.tokens.len() {
                let tk = &self.tokens[self.pos + scan];
                if tk.kind == TokenKind::End {
                    is_classic_end_block = true;
                    break;
                }
                if tk.kind == TokenKind::RightBrace || tk.kind == TokenKind::Eof {
                    break;
                }
                scan += 1;
            }

            if is_classic_end_block {
                if !self.is_at_end()
                    && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
                {
                    self.consume_terminator(reporter)?;
                }
                let (bp, ch) = self.parse_dsl_body_delimited(&kind, false, reporter)?;
                body_props = bp;
                children = ch;
            } else {
                if !self.is_at_end()
                    && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
                {
                    self.consume_terminator(reporter)?;
                }
            }
        }
        properties.append(&mut body_props);

        let span = Span::new(start_pos, self.previous().span.end);
        Ok(DSLBlock::new(
            self.next_id(),
            kind,
            args,
            properties,
            children,
            span,
        ))
    }

    /// Parse the body of a DSL block: zero or more properties and sub-blocks, terminated by `end` or `}`.
    fn parse_dsl_body(
        &mut self,
        parent_kind: &str,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<(Vec<DSLProperty>, Vec<DSLChild>)> {
        self.parse_dsl_body_delimited(parent_kind, false, reporter)
    }

    fn parse_dsl_body_delimited(
        &mut self,
        parent_kind: &str,
        has_brace: bool,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<(Vec<DSLProperty>, Vec<DSLChild>)> {
        let mut properties = Vec::new();
        let mut children = Vec::new();

        loop {
            while self.match_token(TokenKind::Newline) {}
            if self.check(TokenKind::End) || self.is_at_end() {
                break;
            }
            if has_brace && self.check(TokenKind::RightBrace) {
                self.advance();
                return Ok((properties, children));
            }
            let is_top_break = match parent_kind {
                "link" => {
                    self.peek().lexeme == "link"
                        || self.peek().lexeme == "nav"
                        || self.peek().lexeme == "header"
                        || self.peek().lexeme == "footer"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.peek().lexeme == "show"
                        || self.peek().lexeme == "card"
                        || self.peek().lexeme == "section"
                        || self.peek().lexeme == "button"
                        || self.check(TokenKind::When)
                }
                "nav" => {
                    self.peek().lexeme == "nav"
                        || self.peek().lexeme == "header"
                        || self.peek().lexeme == "footer"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "header" => {
                    self.peek().lexeme == "header"
                        || self.peek().lexeme == "footer"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "footer" => {
                    self.peek().lexeme == "header"
                        || self.peek().lexeme == "footer"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "button" => {
                    self.peek().lexeme == "button"
                        || self.peek().lexeme == "show"
                        || self.peek().lexeme == "card"
                        || self.peek().lexeme == "section"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "hero" => {
                    self.peek().lexeme == "hero"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "heading"
                                || self.peek_ahead(1).lexeme == "hero"
                                || self.peek_ahead(1).lexeme == "3D"
                                || self.peek_ahead(1).lexeme == "3d"
                                || self.peek_ahead(1).lexeme == "scene"
                                || (self.peek_ahead(1).kind == TokenKind::IntLiteral
                                    && self.peek_ahead(1).lexeme == "3")))
                        || self.check(TokenKind::When)
                }
                "page" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "database" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "table" => {
                    self.peek().lexeme == "table"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "cube" => {
                    self.peek().lexeme == "show"
                        || self.peek().lexeme == "enable"
                        || self.peek().lexeme == "add"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "scene" | "scene3d" => {
                    self.peek().lexeme == "show"
                        || self.peek().lexeme == "page"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                }
                "section" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "heading"
                                || self.peek_ahead(1).lexeme == "button"
                                || self.peek_ahead(1).lexeme == "input"
                                || self.peek_ahead(1).lexeme == "3D"
                                || self.peek_ahead(1).lexeme == "3d"
                                || self.peek_ahead(1).lexeme == "scene"
                                || (self.peek_ahead(1).kind == TokenKind::IntLiteral
                                    && self.peek_ahead(1).lexeme == "3")))
                }
                "card" => {
                    self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "heading"
                                || self.peek_ahead(1).lexeme == "button"
                                || self.peek_ahead(1).lexeme == "input"
                                || self.peek_ahead(1).lexeme == "3D"
                                || self.peek_ahead(1).lexeme == "3d"
                                || self.peek_ahead(1).lexeme == "scene"
                                || (self.peek_ahead(1).kind == TokenKind::IntLiteral
                                    && self.peek_ahead(1).lexeme == "3")))
                }
                "item" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "item"
                                || self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "step" | "milestone" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "step"
                                || self.peek_ahead(1).lexeme == "milestone"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "tab" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "tab"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "stat" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "stat"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "alert" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "stats" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "timeline" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "accordion" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                "tabs" => {
                    self.peek().lexeme == "page"
                        || self.peek().lexeme == "database"
                        || self.peek().lexeme == "start"
                        || self.check(TokenKind::When)
                        || (self.peek().lexeme == "show"
                            && (self.peek_ahead(1).lexeme == "section"
                                || self.peek_ahead(1).lexeme == "card"
                                || self.peek_ahead(1).lexeme == "timeline"
                                || self.peek_ahead(1).lexeme == "accordion"
                                || self.peek_ahead(1).lexeme == "tabs"
                                || self.peek_ahead(1).lexeme == "stats"
                                || self.peek_ahead(1).lexeme == "alert"
                                || self.peek_ahead(1).lexeme == "heading"))
                }
                _ => false,
            };
            let is_global_break = !has_brace
                && parent_kind != "website"
                && (self.check(TokenKind::When)
                    || (self.check(TokenKind::Make)
                        && ((self.peek_ahead(1).lexeme == "a"
                            && self.peek_ahead(2).lexeme == "website")
                            || self.peek_ahead(1).lexeme == "website"))
                    || (parent_kind == "page" && self.peek().lexeme == "page")
                    || self.peek().lexeme == "database"
                    || self.peek().lexeme == "start");
            if (!has_brace && is_top_break) || is_global_break {
                break;
            }

            if !self.check(TokenKind::Identifier)
                && !self.check(TokenKind::Make)
                && !self.check(TokenKind::Model)
            {
                let diag = techscript_errors::Diagnostic::new(
                    techscript_errors::DiagnosticLevel::Error,
                    ErrorCode::E0100,
                    "Expected property name or sub-block keyword inside DSL block".to_string(),
                    self.peek().span,
                );
                reporter.report(diag);
                self.synchronize();
                continue;
            }

            let name = self.peek().lexeme.clone();

            if name == "code"
                && (self.peek_ahead(1).kind == TokenKind::Newline
                    || self.peek_ahead(1).kind == TokenKind::Colon)
            {
                // Code block: parse raw TechScript statements until `end`
                self.advance();
                self.consume_terminator(reporter)?;
                let code_start = self.peek().span.start;
                let mut statements = Vec::new();
                loop {
                    while self.match_token(TokenKind::Newline) {}
                    if self.check(TokenKind::End) || self.is_at_end() {
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
                    TokenKind::End,
                    ErrorCode::E0105,
                    "Expected 'end' to close code block",
                    reporter,
                )?;
                let span = Span::new(code_start, self.previous().span.end);
                children.push(DSLChild::Code(Block::new(self.next_id(), statements, span)));
            } else if name == "show" || name == "make" {
                self.advance(); // consume 'show' or 'make'
                let elem_start = self.peek().span.start;
                let elem_kind = if self.peek().lexeme == "3D" || self.peek().lexeme == "3d" {
                    self.advance();
                    if self.peek().lexeme == "scene" || self.peek().lexeme == "model" {
                        self.advance();
                    }
                    "scene3d".to_string()
                } else if (self.peek().kind == TokenKind::IntLiteral && self.peek().lexeme == "3")
                    && (self.peek_ahead(1).lexeme == "D" || self.peek_ahead(1).lexeme == "d")
                {
                    self.advance(); // 3
                    self.advance(); // D
                    if self.peek().lexeme == "scene" || self.peek().lexeme == "model" {
                        self.advance();
                    }
                    "scene3d".to_string()
                } else if self.check(TokenKind::Identifier) {
                    self.advance().lexeme.clone()
                } else {
                    "element".to_string()
                };

                let mut elem_args = Vec::new();
                let mut elem_props = Vec::new();
                while !self.check(TokenKind::Newline)
                    && !self.check(TokenKind::Semicolon)
                    && !self.check(TokenKind::Colon)
                    && !self.is_at_end()
                {
                    if self.match_token(TokenKind::Arrow) {
                        let action_start = self.peek().span.start;
                        let mut action_str = String::new();
                        while !self.check(TokenKind::Newline)
                            && !self.check(TokenKind::Semicolon)
                            && !self.is_at_end()
                        {
                            if !action_str.is_empty() {
                                action_str.push(' ');
                            }
                            action_str.push_str(&self.peek().lexeme);
                            self.advance();
                        }
                        let action_span = Span::new(action_start, self.previous().span.end);
                        elem_props.push(DSLProperty::new(
                            self.next_id(),
                            "action".to_string(),
                            Some(Expression::Literal(LiteralExpr::new(
                                self.next_id(),
                                LiteralVal::Str(action_str),
                                action_span,
                            ))),
                            action_span,
                        ));
                        break;
                    } else if self.peek().lexeme == "called" || self.peek().lexeme == "as" {
                        self.advance();
                        let id_start = self.peek().span.start;
                        let id_name = if self.check(TokenKind::Identifier)
                            || self.check(TokenKind::StringLiteral)
                        {
                            let s = self.peek().lexeme.clone();
                            self.advance();
                            s
                        } else {
                            "item".to_string()
                        };
                        let id_span = Span::new(id_start, self.previous().span.end);
                        elem_props.push(DSLProperty::new(
                            self.next_id(),
                            "id".to_string(),
                            Some(Expression::Literal(LiteralExpr::new(
                                self.next_id(),
                                LiteralVal::Str(id_name),
                                id_span,
                            ))),
                            id_span,
                        ));
                    } else {
                        let arg_expr =
                            self.parse_expression(techscript_syntax::Precedence::Call, reporter)?;
                        if elem_kind == "button" && !elem_props.iter().any(|p| p.name == "label") {
                            let arg_span = arg_expr.span();
                            elem_props.push(DSLProperty::new(
                                self.next_id(),
                                "label".to_string(),
                                Some(arg_expr.clone()),
                                arg_span,
                            ));
                        }
                        elem_args.push(arg_expr);
                    }
                }

                let has_elem_brace = self.match_token(TokenKind::LeftBrace);
                let has_colon = if !has_elem_brace {
                    self.match_token(TokenKind::Colon)
                } else {
                    false
                };
                let (elem_props_extra, elem_children) = if has_elem_brace {
                    self.consume_terminator(reporter)?;
                    self.parse_dsl_body_delimited(&elem_kind, true, reporter)?
                } else if has_colon {
                    if self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon) {
                        self.consume_terminator(reporter)?;
                        let (sub_props, sub_children) =
                            self.parse_dsl_body(&elem_kind, reporter)?;
                        (sub_props, sub_children)
                    } else {
                        // Inline properties on the same line after ':'
                        let mut sub_props = Vec::new();
                        while !self.check(TokenKind::Newline)
                            && !self.check(TokenKind::Semicolon)
                            && !self.check(TokenKind::End)
                            && !self.is_at_end()
                        {
                            if self.check(TokenKind::Identifier) {
                                let prop = self.parse_dsl_property(reporter)?;
                                sub_props.push(prop);
                            } else {
                                break;
                            }
                        }
                        if self.check(TokenKind::End) {
                            self.advance();
                        }
                        if !self.is_at_end()
                            && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
                        {
                            self.consume_terminator(reporter)?;
                        }
                        (sub_props, Vec::new())
                    }
                } else {
                    if !self.is_at_end()
                        && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
                    {
                        self.consume_terminator(reporter)?;
                    }
                    (Vec::new(), Vec::new())
                };
                elem_props.extend(elem_props_extra);

                let elem_span = Span::new(elem_start, self.previous().span.end);
                let elem_block = DSLBlock::new(
                    self.next_id(),
                    elem_kind,
                    elem_args,
                    elem_props,
                    elem_children,
                    elem_span,
                );
                children.push(DSLChild::Block(elem_block));
            } else if name == "add" {
                self.advance(); // consume 'add'
                let item_start = self.peek().span.start;
                let item_kind = if self.check(TokenKind::Identifier) {
                    self.advance().lexeme.clone()
                } else {
                    "item".to_string()
                };

                let mut item_args = Vec::new();
                let mut item_props = Vec::new();
                while !self.check(TokenKind::Newline)
                    && !self.check(TokenKind::Semicolon)
                    && !self.check(TokenKind::Colon)
                    && !self.is_at_end()
                {
                    if self.peek().lexeme == "at" {
                        self.advance();
                        let at_val =
                            self.parse_expression(techscript_syntax::Precedence::Call, reporter)?;
                        let at_span = at_val.span();
                        item_props.push(DSLProperty::new(
                            self.next_id(),
                            "at".to_string(),
                            Some(at_val),
                            at_span,
                        ));
                    } else if self.peek().lexeme == "size" {
                        self.advance();
                        let size_val =
                            self.parse_expression(techscript_syntax::Precedence::Call, reporter)?;
                        let size_span = size_val.span();
                        item_props.push(DSLProperty::new(
                            self.next_id(),
                            "size".to_string(),
                            Some(size_val),
                            size_span,
                        ));
                    } else if self.peek().lexeme == "color" {
                        self.advance();
                        if self.check(TokenKind::Is) || self.peek().lexeme == "is" {
                            self.advance();
                        }
                        let color_val =
                            self.parse_expression(techscript_syntax::Precedence::Call, reporter)?;
                        let color_span = color_val.span();
                        item_props.push(DSLProperty::new(
                            self.next_id(),
                            "color".to_string(),
                            Some(color_val),
                            color_span,
                        ));
                    } else if self.peek().lexeme == "material" {
                        self.advance();
                        if self.check(TokenKind::Is) || self.peek().lexeme == "is" {
                            self.advance();
                        }
                        let mat_val =
                            self.parse_expression(techscript_syntax::Precedence::Call, reporter)?;
                        let mat_span = mat_val.span();
                        item_props.push(DSLProperty::new(
                            self.next_id(),
                            "material".to_string(),
                            Some(mat_val),
                            mat_span,
                        ));
                    } else {
                        item_args.push(
                            self.parse_expression(techscript_syntax::Precedence::Call, reporter)?,
                        );
                    }
                }

                let has_colon = self.match_token(TokenKind::Colon);
                let (extra_props, item_children) = if has_colon {
                    if self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon) {
                        self.consume_terminator(reporter)?;
                        self.parse_dsl_body(&item_kind, reporter)?
                    } else {
                        // inline properties
                        let mut sub_props = Vec::new();
                        while !self.check(TokenKind::Newline)
                            && !self.check(TokenKind::Semicolon)
                            && !self.check(TokenKind::End)
                            && !self.is_at_end()
                        {
                            if self.check(TokenKind::Identifier) {
                                let prop = self.parse_dsl_property(reporter)?;
                                sub_props.push(prop);
                            } else {
                                break;
                            }
                        }
                        if self.check(TokenKind::End) {
                            self.advance();
                        }
                        if !self.is_at_end()
                            && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
                        {
                            self.consume_terminator(reporter)?;
                        }
                        (sub_props, Vec::new())
                    }
                } else {
                    if !self.is_at_end()
                        && (self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon))
                    {
                        self.consume_terminator(reporter)?;
                    }
                    (Vec::new(), Vec::new())
                };
                item_props.extend(extra_props);

                let item_span = Span::new(item_start, self.previous().span.end);
                let item_block = DSLBlock::new(
                    self.next_id(),
                    item_kind,
                    item_args,
                    item_props,
                    item_children,
                    item_span,
                );
                children.push(DSLChild::Block(item_block));
            } else if name == "enable" {
                self.advance(); // consume 'enable'
                let prop_start = self.previous().span.start;
                let mut prop_name = String::new();
                while !self.check(TokenKind::Newline)
                    && !self.check(TokenKind::Semicolon)
                    && !self.is_at_end()
                {
                    if !prop_name.is_empty() {
                        prop_name.push('_');
                    }
                    prop_name.push_str(&self.peek().lexeme);
                    self.advance();
                }
                self.consume_terminator(reporter)?;
                let prop_span = Span::new(prop_start, self.previous().span.end);
                properties.push(DSLProperty::new(
                    self.next_id(),
                    prop_name,
                    Some(Expression::Literal(LiteralExpr::new(
                        self.next_id(),
                        LiteralVal::Bool(true),
                        prop_span,
                    ))),
                    prop_span,
                ));
            } else if self.dsl_sub_blocks.contains(&name) {
                // Nested DSL sub-block
                let block = self.parse_dsl_block(reporter)?;
                children.push(DSLChild::Block(block));
            } else {
                // Property: `name [value...]`
                let prop = self.parse_dsl_property(reporter)?;
                properties.push(prop);
            }
        }

        if !has_brace && self.check(TokenKind::End) {
            self.advance();
        }

        Ok((properties, children))
    }

    /// Parse a DSL property: `name [value...]` terminated by newline/semicolon.
    pub fn parse_dsl_property(
        &mut self,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<DSLProperty> {
        let start_pos = self.peek().span.start;
        let mut name = self.parse_identifier(reporter)?.name;

        // Support multi-word property names like "rotation speed"
        while self.check(TokenKind::Identifier) && self.peek().lexeme != "is" {
            name.push('_');
            name.push_str(&self.advance().lexeme);
        }

        // Support `is`, `from`, `by`, `to` prepositions in property
        if self.check(TokenKind::Is)
            || self.peek().lexeme == "is"
            || self.peek().lexeme == "from"
            || self.peek().lexeme == "by"
            || self.peek().lexeme == "to"
        {
            self.advance();
        }

        let mut value = None;
        // Check if there is a value expression on the same line
        if !self.check(TokenKind::Newline)
            && !self.check(TokenKind::Semicolon)
            && !self.check(TokenKind::End)
            && !self.check(TokenKind::RightBrace)
            && !self.check(TokenKind::Comma)
            && !self.is_at_end()
        {
            value = Some(self.parse_expression(techscript_syntax::Precedence::None, reporter)?);
        }
        if self.match_token(TokenKind::Comma) {
            // Consumed comma separator between inline properties
        } else if !self.check(TokenKind::End)
            && !self.check(TokenKind::RightBrace)
            && !self.is_at_end()
        {
            self.consume_terminator(reporter)?;
        }

        let span = Span::new(start_pos, self.previous().span.end);
        Ok(DSLProperty::new(self.next_id(), name, value, span))
    }

    /// Consumes the required statement terminator (newline, semicolon, or implicit).
    pub fn consume_terminator(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<()> {
        let mut has_semicolon = false;
        let start_pos = self.peek().span;
        if self.check(TokenKind::Semicolon) {
            has_semicolon = true;
        }
        if self.match_token(TokenKind::Semicolon) || self.match_token(TokenKind::Newline) {
            // consume multiple consecutive terminators
            while self.check(TokenKind::Semicolon) || self.check(TokenKind::Newline) {
                if self.check(TokenKind::Semicolon) {
                    has_semicolon = true;
                }
                self.advance();
            }
            if has_semicolon {
                reporter.report(techscript_errors::Diagnostic::new(
                    techscript_errors::DiagnosticLevel::Warning,
                    techscript_errors::ErrorCode::TSW1006,
                    "Warning TSW1006: Semicolons are deprecated. Use newlines instead.".to_string(),
                    start_pos,
                ));
            }
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

    /// Checks if a `send` keyword is part of a WebForge `send <val> to <endpoint>` construct.
    fn check_is_send_to(&self) -> bool {
        let mut i = 1;
        while self.pos + i < self.tokens.len() {
            let tk = &self.tokens[self.pos + i];
            if tk.kind == TokenKind::Newline
                || tk.kind == TokenKind::Semicolon
                || tk.kind == TokenKind::Eof
            {
                break;
            }
            if tk.lexeme == "to" {
                return true;
            }
            i += 1;
        }
        false
    }
}
