use crate::parser::{ParseResult, Parser};
use techscript_ast::{
    Block, DSLBlock, DSLChild, DSLProperty, ExpressionStmt, ForStmt, IfStmt, ImportStmt,
    RepeatStmt, ReturnStmt, SayStmt, Statement, ThrowStmt, TryStmt, WhileStmt,
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
            && self.dsl_keywords.contains(&self.peek().lexeme)
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

        let condition = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
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
                let blk = self.parse_block_or_then_end(reporter)?;
                else_ifs.push((cond, blk));
            } else if self.check(TokenKind::Or) && self.peek_ahead(1).kind == TokenKind::When {
                self.advance(); // consume Or
                self.advance(); // consume When
                let cond = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
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
                    let blk = self.parse_block_or_then_end(reporter)?;
                    else_ifs.push((cond, blk));
                } else {
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
        let kind = self.parse_identifier(reporter)?.name;

        // Parse optional arguments until the statement terminator
        let mut args = Vec::new();
        while !self.check(TokenKind::Newline)
            && !self.check(TokenKind::Semicolon)
            && !self.is_at_end()
        {
            args.push(self.parse_expression(techscript_syntax::Precedence::None, reporter)?);
        }
        self.consume_terminator(reporter)?;

        // Parse body: properties and sub-blocks until `end`
        let (properties, children) = self.parse_dsl_body(reporter)?;

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

    /// Parse the body of a DSL block: zero or more properties and sub-blocks, terminated by `end`.
    fn parse_dsl_body(
        &mut self,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<(Vec<DSLProperty>, Vec<DSLChild>)> {
        let mut properties = Vec::new();
        let mut children = Vec::new();

        loop {
            while self.match_token(TokenKind::Newline) {}
            if self.check(TokenKind::End) || self.is_at_end() {
                break;
            }

            if !self.check(TokenKind::Identifier) {
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

            if name == "code" {
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

        self.consume(
            TokenKind::End,
            ErrorCode::E0105,
            "Expected 'end' to close DSL block",
            reporter,
        )?;

        Ok((properties, children))
    }

    /// Parse a DSL property: `name [value...]` terminated by newline/semicolon.
    pub fn parse_dsl_property(
        &mut self,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<DSLProperty> {
        let start_pos = self.peek().span.start;
        let name = self.parse_identifier(reporter)?.name;

        let mut value = None;
        // Check if there is a value expression on the same line
        if !self.check(TokenKind::Newline)
            && !self.check(TokenKind::Semicolon)
            && !self.check(TokenKind::End)
            && !self.check(TokenKind::RightBrace)
            && !self.is_at_end()
        {
            value = Some(self.parse_expression(techscript_syntax::Precedence::None, reporter)?);
        }
        self.consume_terminator(reporter)?;

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
}
