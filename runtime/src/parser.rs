// ── TechScript Parser ────────────────────────────────────────────────
// Port of parser.py — recursive-descent parser with precedence climbing.

use crate::token::{Token, TokenType};
use crate::ast::*;
use crate::error::{TechError, TechResult};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    filename: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, filename: &str) -> Self {
        Parser { tokens, pos: 0, filename: filename.to_string() }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, tt: TokenType, val: Option<&str>) -> TechResult<Token> {
        let tok = self.peek().clone();
        if tok.token_type == tt {
            if let Some(v) = val {
                if tok.value != v {
                    return Err(TechError::parse(
                        format!("Expected '{}', got '{}'", v, tok.value),
                        tok.line, tok.column, &self.filename
                    ));
                }
            }
            Ok(self.advance())
        } else {
            Err(TechError::parse(
                format!("Expected {:?}, got {:?} ('{}')", tt, tok.token_type, tok.value),
                tok.line, tok.column, &self.filename
            ))
        }
    }

    fn match_tok(&mut self, tt: TokenType, val: Option<&str>) -> bool {
        let tok = self.peek();
        if tok.token_type == tt {
            if let Some(v) = val {
                if tok.value != v { return false; }
            }
            self.advance();
            true
        } else {
            false
        }
    }

    fn kw(&mut self, value: &str) -> bool {
        self.match_tok(TokenType::Keyword, Some(value))
    }

    /// Variable names may be identifiers; DSL keywords like `page` are allowed as bindings.
    fn parse_member_name(&mut self) -> TechResult<String> {
        match &self.peek().token_type {
            TokenType::Identifier | TokenType::Keyword => Ok(self.advance().value),
            other => Err(TechError::parse(
                format!("Expected property name, got {:?}", other),
                self.peek().line,
                self.peek().column,
                &self.filename,
            )),
        }
    }

    fn parse_binding_name(&mut self) -> TechResult<String> {
        match &self.peek().token_type {
            TokenType::Identifier => Ok(self.advance().value),
            TokenType::Keyword => {
                let name = self.peek().value.clone();
                let allowed = [
                    "page", "state", "component", "window", "scene", "timeline", "route", "api",
                ];
                if allowed.contains(&name.as_str()) {
                    Ok(self.advance().value)
                } else {
                    Err(TechError::parse(
                        format!("Expected variable name, got keyword '{}'", name),
                        self.peek().line,
                        self.peek().column,
                        &self.filename,
                    ))
                }
            }
            other => Err(TechError::parse(
                format!("Expected variable name, got {:?}", other),
                self.peek().line,
                self.peek().column,
                &self.filename,
            )),
        }
    }

    fn skip_nl(&mut self) {
        while self.peek().token_type == TokenType::Newline {
            self.advance();
        }
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn next_token_is(&self, tt: TokenType) -> bool {
        self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].token_type == tt
    }

    // ─── Public Entry Point ──────────────────────────────────────────

    pub fn parse(mut self) -> TechResult<Program> {
        self.skip_nl();
        let mut body = Vec::new();
        while !self.at_end() {
            body.push(self.parse_statement()?);
            self.skip_nl();
        }
        Ok(Program { body })
    }

    // ─── Statements ─────────────────────────────────────────────────

    fn parse_statement(&mut self) -> TechResult<Stmt> {
        self.skip_nl();
        let tok = self.peek().clone();

        if tok.token_type == TokenType::Keyword {
            match tok.value.as_str() {
                "say"     => return self.parse_say(),
                "make"    => return self.parse_set(),
                "keep" | "const" => return self.parse_const(),
                "send" | "return" => return self.parse_return(),
                "fail" | "throw" => return self.parse_throw(),
                "drop"    => return self.parse_del(),
                "defer"   => return self.parse_defer(),
                "use"     => return self.parse_import(),
                "take"    => return self.parse_from_import(),
                "share"   => return self.parse_export(),
                "when"    => return self.parse_if(),
                "unless"  => return self.parse_unless(),
                "each"    => return self.parse_for(),
                "repeat"  => return self.parse_while(),
                "loop"    => return self.parse_loop(),
                "until"   => return self.parse_until(),
                "build" | "do" => return self.parse_fn(),
                "model" | "class" => return self.parse_class(),
                "attempt" | "try" => return self.parse_try(),
                "match"   => return self.parse_match(),
                "guard"   => return self.parse_guard(),
                "with"    => return self.parse_with(),
                "state" if self.next_token_is(TokenType::Identifier) => return self.parse_state(),
                "component" if self.next_token_is(TokenType::Identifier) => return self.parse_component(),
                "page" if self.next_token_is(TokenType::Identifier) => return self.parse_page(),
                "api" if self.next_token_is(TokenType::Identifier) => return self.parse_api(),
                "window" if self.next_token_is(TokenType::String) => return self.parse_window(),
                "scene" if self.next_token_is(TokenType::Identifier) => return self.parse_scene(),
                "timeline" if self.next_token_is(TokenType::Identifier) => return self.parse_timeline(),
                "render" if self.next_token_is(TokenType::String) => return self.parse_render(),
                "button" if self.next_token_is(TokenType::String) => return self.parse_button(),
                "input" if self.next_token_is(TokenType::Identifier) => return self.parse_input(),
                "label" if self.next_token_is(TokenType::String) => return self.parse_label(),
                "camera"  => return self.parse_camera(),
                "light"   => return self.parse_light(),
                "mesh"    => return self.parse_mesh(),
                "move" if self.next_token_is(TokenType::Identifier) => return self.parse_anime_move(),
                "fade" if self.next_token_is(TokenType::Identifier) => return self.parse_anime_fade(),
                "stop" | "break" => { self.advance(); return Ok(Stmt::Break); }
                "skip" | "continue" => { self.advance(); return Ok(Stmt::Skip); }
                "pass"    => { self.advance(); return Ok(Stmt::Pass); }
                _ => {}
            }
        }

        self.parse_expression_statement()
    }

    fn parse_say(&mut self) -> TechResult<Stmt> {
        self.advance(); // "say"
        let mut values = vec![self.parse_expression()?];
        while self.match_tok(TokenType::Comma, None) {
            values.push(self.parse_expression()?);
        }
        Ok(Stmt::Say { values })
    }

    fn parse_set(&mut self) -> TechResult<Stmt> {
        self.advance(); // "make"
        let name = self.parse_binding_name()?;
        self.expect(TokenType::Assign, None)?;
        let value = self.parse_expression()?;
        Ok(Stmt::Set { name, value })
    }

    fn parse_const(&mut self) -> TechResult<Stmt> {
        self.advance(); // "keep"
        let name = self.parse_binding_name()?;
        self.expect(TokenType::Assign, None)?;
        let value = self.parse_expression()?;
        Ok(Stmt::Const { name, value })
    }

    fn parse_return(&mut self) -> TechResult<Stmt> {
        self.advance(); // "send"
        let tok = self.peek();
        if tok.token_type == TokenType::Newline || tok.token_type == TokenType::Eof || tok.token_type == TokenType::RBrace {
            Ok(Stmt::Return { value: None })
        } else {
            Ok(Stmt::Return { value: Some(self.parse_expression()?) })
        }
    }

    fn parse_throw(&mut self) -> TechResult<Stmt> {
        self.advance(); // "fail"
        Ok(Stmt::Throw { value: self.parse_expression()? })
    }

    fn parse_del(&mut self) -> TechResult<Stmt> {
        self.advance(); // "drop"
        let name_tok = self.expect(TokenType::Identifier, None)?;
        Ok(Stmt::Del { name: name_tok.value })
    }

    fn parse_defer(&mut self) -> TechResult<Stmt> {
        self.advance(); // "defer"
        Ok(Stmt::Defer { expression: self.parse_expression()? })
    }

    /// Module names may be identifiers or the special token sequence `3` + `d`.
    fn parse_module_name(&mut self) -> TechResult<String> {
        if self.peek().token_type == TokenType::NumberInt && self.peek().value == "3" {
            self.advance();
            if self.peek().token_type == TokenType::Identifier && self.peek().value == "d" {
                self.advance();
                return Ok("3d".into());
            }
            return Err(TechError::parse(
                "Expected module name after 'use'",
                self.peek().line,
                self.peek().column,
                &self.filename,
            ));
        }
        Ok(self.expect(TokenType::Identifier, None)?.value)
    }

    fn parse_import(&mut self) -> TechResult<Stmt> {
        self.advance(); // "use"
        let module_name = self.parse_module_name()?;
        let alias = if self.kw("as") {
            Some(self.expect(TokenType::Identifier, None)?.value)
        } else {
            None
        };
        Ok(Stmt::Import { module: module_name, names: None, alias })
    }

    fn parse_from_import(&mut self) -> TechResult<Stmt> {
        self.advance(); // "take"
        let mut names = vec![self.expect(TokenType::Identifier, None)?.value];
        while self.match_tok(TokenType::Comma, None) {
            names.push(self.expect(TokenType::Identifier, None)?.value);
        }
        self.expect(TokenType::Keyword, Some("in"))?;
        let module_tok = self.expect(TokenType::Identifier, None)?;
        Ok(Stmt::FromImport { module: module_tok.value, names })
    }

    fn parse_export(&mut self) -> TechResult<Stmt> {
        self.advance(); // "share"
        let decl = self.parse_statement()?;
        Ok(Stmt::Export { declaration: Box::new(decl) })
    }

    fn parse_if(&mut self) -> TechResult<Stmt> {
        self.advance(); // "when"
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        let mut elif_clauses = Vec::new();
        let mut else_body = None;

        while self.peek().token_type == TokenType::Keyword {
            let kw = self.peek().value.as_str();
            if kw == "or" {
                self.advance();
                self.expect(TokenType::Keyword, Some("when"))?;
                let cond = self.parse_expression()?;
                let b = self.parse_block()?;
                elif_clauses.push((cond, b));
            } else if kw == "alt" {
                self.advance();
                let cond = self.parse_expression()?;
                let b = self.parse_block()?;
                elif_clauses.push((cond, b));
            } else {
                break;
            }
        }

        if self.kw("else") {
            else_body = Some(self.parse_block()?);
        }

        Ok(Stmt::If { condition, body, elif_clauses, else_body })
    }

    fn parse_unless(&mut self) -> TechResult<Stmt> {
        self.advance(); // "unless"
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::Unless { condition, body })
    }

    fn parse_for(&mut self) -> TechResult<Stmt> {
        self.advance(); // "each"
        let var_tok = self.expect(TokenType::Identifier, None)?;
        self.expect(TokenType::Keyword, Some("in"))?;
        let iterable = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::For { var_name: var_tok.value, iterable, body })
    }

    fn parse_while(&mut self) -> TechResult<Stmt> {
        self.advance(); // "repeat"
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_loop(&mut self) -> TechResult<Stmt> {
        self.advance(); // "loop"
        if self.kw("while") {
            let condition = self.parse_expression()?;
            let body = self.parse_block()?;
            return Ok(Stmt::While { condition, body });
        }
        let first = self.parse_expression()?;
        let body = self.parse_block()?;
        if let Expr::NumberInt(n) = first {
            Ok(Stmt::For {
                var_name: "_loop".into(),
                iterable: Expr::Range {
                    start: Box::new(Expr::NumberInt(1)),
                    end: Box::new(Expr::NumberInt(n)),
                    inclusive: true,
                },
                body,
            })
        } else {
            Ok(Stmt::While { condition: first, body })
        }
    }

    fn parse_until(&mut self) -> TechResult<Stmt> {
        self.advance(); // "until"
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::Until { condition, body })
    }

    fn parse_fn(&mut self) -> TechResult<Stmt> {
        self.advance(); // "build"
        let name_tok = self.expect(TokenType::Identifier, None)?;
        let params = self.parse_param_list()?;
        let body = self.parse_block()?;
        Ok(Stmt::Fn { name: name_tok.value, params, body })
    }

    fn parse_class(&mut self) -> TechResult<Stmt> {
        self.advance(); // "model"
        let name_tok = self.expect(TokenType::Identifier, None)?;
        let parent = if self.match_tok(TokenType::LParen, None) {
            let p = self.expect(TokenType::Identifier, None)?.value;
            self.expect(TokenType::RParen, None)?;
            Some(p)
        } else {
            None
        };
        let body_stmts = self.parse_block()?;
        Ok(Stmt::Class { name: name_tok.value, parent, body: body_stmts })
    }

    fn parse_try(&mut self) -> TechResult<Stmt> {
        self.advance(); // "attempt"
        let body = self.parse_block()?;
        let mut catch_var = None;
        let mut catch_body = Vec::new();
        let mut finally_body = None;

        if self.kw("rescue") || self.kw("catch") {
            if self.peek().token_type == TokenType::Identifier {
                catch_var = Some(self.advance().value);
            }
            catch_body = self.parse_block()?;
        }

        if self.kw("always") || self.kw("finally") {
            finally_body = Some(self.parse_block()?);
        }

        Ok(Stmt::Try { body, catch_var, catch_body, finally_body })
    }

    fn parse_match(&mut self) -> TechResult<Stmt> {
        self.advance(); // "match"
        let subject = self.parse_expression()?;
        self.expect(TokenType::LBrace, None)?;
        self.skip_nl();
        let mut cases = Vec::new();
        while self.kw("case") {
            let pattern = self.parse_expression()?;
            let case_body = self.parse_block()?;
            cases.push((pattern, case_body));
            self.skip_nl();
        }
        self.expect(TokenType::RBrace, None)?;
        Ok(Stmt::Match { subject, cases })
    }

    fn parse_guard(&mut self) -> TechResult<Stmt> {
        self.advance(); // "guard"
        let condition = self.parse_expression()?;
        self.expect(TokenType::Keyword, Some("else"))?;
        let else_body = self.parse_block()?;
        Ok(Stmt::Guard { condition, else_body })
    }

    fn parse_with(&mut self) -> TechResult<Stmt> {
        self.advance(); // "with"
        let expression = self.parse_expression()?;
        self.expect(TokenType::Keyword, Some("as"))?;
        let var_tok = self.expect(TokenType::Identifier, None)?;
        let body = self.parse_block()?;
        Ok(Stmt::With { expression, var_name: var_tok.value, body })
    }

    fn parse_state(&mut self) -> TechResult<Stmt> {
        self.advance(); // "state"
        let name_tok = self.expect(TokenType::Identifier, None)?;
        self.expect(TokenType::Assign, None)?;
        let value = self.parse_expression()?;
        Ok(Stmt::State { name: name_tok.value, value })
    }

    fn parse_component(&mut self) -> TechResult<Stmt> {
        self.advance();
        let name_tok = self.expect(TokenType::Identifier, None)?;
        let body = self.parse_block()?;
        Ok(Stmt::Component { name: name_tok.value, body })
    }

    fn parse_page(&mut self) -> TechResult<Stmt> {
        self.advance();
        let name_tok = self.expect(TokenType::Identifier, None)?;
        let body = self.parse_block()?;
        Ok(Stmt::Page { name: name_tok.value, body })
    }

    fn parse_api(&mut self) -> TechResult<Stmt> {
        self.advance();
        let name_tok = self.expect(TokenType::Identifier, None)?;
        self.expect(TokenType::LBrace, None)?;
        self.skip_nl();
        let mut routes = Vec::new();
        while self.kw("route") {
            let path_expr = self.parse_expression()?;
            let path = match path_expr {
                Expr::String(s) => s,
                _ => return Err(TechError::parse("Route path must be a string", self.peek().line, self.peek().column, &self.filename)),
            };
            let body = self.parse_block()?;
            routes.push(("GET".into(), path, body));
            self.skip_nl();
        }
        self.expect(TokenType::RBrace, None)?;
        Ok(Stmt::Api { name: name_tok.value, routes })
    }

    fn parse_window(&mut self) -> TechResult<Stmt> {
        self.advance();
        let title_expr = self.parse_expression()?;
        let title = match title_expr {
            Expr::String(s) => s,
            _ => return Err(TechError::parse("Window title must be a string", self.peek().line, self.peek().column, &self.filename)),
        };
        let body = self.parse_block()?;
        Ok(Stmt::Window { title, body })
    }

    fn parse_scene(&mut self) -> TechResult<Stmt> {
        self.advance();
        let name_tok = self.expect(TokenType::Identifier, None)?;
        let body = self.parse_block()?;
        Ok(Stmt::Scene { name: name_tok.value, body })
    }

    fn parse_timeline(&mut self) -> TechResult<Stmt> {
        self.advance();
        let name_tok = self.expect(TokenType::Identifier, None)?;
        let body = self.parse_block()?;
        Ok(Stmt::Timeline { name: name_tok.value, body })
    }

    fn parse_render(&mut self) -> TechResult<Stmt> {
        self.advance();
        let tag_expr = self.parse_expression()?;
        let tag = match tag_expr {
            Expr::String(s) => s,
            _ => return Err(TechError::parse("Render tag must be a string", self.peek().line, self.peek().column, &self.filename)),
        };
        let body = self.parse_block()?;
        Ok(Stmt::Render { tag, body })
    }

    fn parse_button(&mut self) -> TechResult<Stmt> {
        self.advance();
        let label_expr = self.parse_expression()?;
        let label = match label_expr {
            Expr::String(s) => s,
            _ => return Err(TechError::parse("Button label must be a string", self.peek().line, self.peek().column, &self.filename)),
        };
        let body = self.parse_block()?;
        Ok(Stmt::Button { label, body })
    }

    fn parse_input(&mut self) -> TechResult<Stmt> {
        self.advance();
        let name_tok = self.expect(TokenType::Identifier, None)?;
        let placeholder = if self.kw("placeholder") {
            match self.parse_expression()? {
                Expr::String(s) => s,
                _ => String::new(),
            }
        } else {
            String::new()
        };
        Ok(Stmt::Input { name: name_tok.value, placeholder })
    }

    fn parse_label(&mut self) -> TechResult<Stmt> {
        self.advance();
        let text_expr = self.parse_expression()?;
        let text = match text_expr {
            Expr::String(s) => s,
            _ => return Err(TechError::parse("Label text must be a string", self.peek().line, self.peek().column, &self.filename)),
        };
        Ok(Stmt::Label { text })
    }

    fn parse_camera(&mut self) -> TechResult<Stmt> {
        self.advance();
        let _ = self.kw("pos");
        let list = self.parse_list_lit()?;
        Ok(Stmt::Camera { coords: list })
    }

    fn parse_light(&mut self) -> TechResult<Stmt> {
        self.advance();
        let kind = if self.peek().token_type == TokenType::Identifier {
            self.advance().value
        } else {
            "ambient".into()
        };
        Ok(Stmt::Light { kind })
    }

    fn parse_anime_move(&mut self) -> TechResult<Stmt> {
        self.advance();
        let target = self.expect(TokenType::Identifier, None)?.value;
        let _ = self.kw("to");
        let coords = self.parse_list_lit()?;
        let duration = if self.kw("over") {
            self.parse_expression()?
        } else {
            Expr::NumberFloat(1.0)
        };
        let ease = if self.kw("ease") {
            match self.parse_expression()? {
                Expr::String(s) => s,
                Expr::Identifier(s) => s,
                _ => "linear".into(),
            }
        } else {
            "linear".into()
        };
        Ok(Stmt::AnimeMove {
            target,
            coords,
            duration,
            ease,
        })
    }

    fn parse_anime_fade(&mut self) -> TechResult<Stmt> {
        self.advance();
        let target = self.expect(TokenType::Identifier, None)?.value;
        let _ = self.kw("to");
        let opacity = self.parse_expression()?;
        let duration = if self.kw("over") {
            self.parse_expression()?
        } else {
            Expr::NumberFloat(0.5)
        };
        Ok(Stmt::AnimeFade {
            target,
            opacity,
            duration,
        })
    }

    fn parse_mesh(&mut self) -> TechResult<Stmt> {
        self.advance();
        let shape = if self.peek().token_type == TokenType::Identifier {
            self.advance().value
        } else {
            "cube".into()
        };
        let color = if self.kw("color") {
            match self.parse_expression()? {
                Expr::String(s) => s,
                _ => "#7c3aed".into(),
            }
        } else {
            "#7c3aed".into()
        };
        Ok(Stmt::Mesh { shape, color })
    }

    fn parse_list_lit(&mut self) -> TechResult<Vec<Expr>> {
        self.expect(TokenType::LBracket, None)?;
        let mut items = Vec::new();
        if self.peek().token_type != TokenType::RBracket {
            items.push(self.parse_expression()?);
            while self.match_tok(TokenType::Comma, None) {
                items.push(self.parse_expression()?);
            }
        }
        self.expect(TokenType::RBracket, None)?;
        Ok(items)
    }

    // ─── Block ──────────────────────────────────────────────────────

    fn parse_block(&mut self) -> TechResult<Vec<Stmt>> {
        self.skip_nl();
        self.expect(TokenType::LBrace, None)?;
        self.skip_nl();
        let mut stmts = Vec::new();
        while self.peek().token_type != TokenType::RBrace && !self.at_end() {
            stmts.push(self.parse_statement()?);
            self.skip_nl();
        }
        self.expect(TokenType::RBrace, None)?;
        Ok(stmts)
    }

    fn parse_param_list(&mut self) -> TechResult<Vec<Param>> {
        self.expect(TokenType::LParen, None)?;
        let mut params = Vec::new();
        if self.peek().token_type != TokenType::RParen {
            params.push(self.parse_one_param()?);
            while self.match_tok(TokenType::Comma, None) {
                params.push(self.parse_one_param()?);
            }
        }
        self.expect(TokenType::RParen, None)?;
        Ok(params)
    }

    fn parse_one_param(&mut self) -> TechResult<Param> {
        let name_tok = if self.peek().token_type == TokenType::Keyword && self.peek().value == "self" {
            self.advance()
        } else {
            self.expect(TokenType::Identifier, None)?
        };
        let default = if self.match_tok(TokenType::Assign, None) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Param { name: name_tok.value, default })
    }

    // ─── Expression Statement ───────────────────────────────────────

    fn parse_expression_statement(&mut self) -> TechResult<Stmt> {
        let expr = self.parse_expression()?;

        // Check for assignment operators
        let tok = self.peek().clone();
        match tok.token_type {
            TokenType::Assign | TokenType::PlusAssign | TokenType::MinusAssign |
            TokenType::StarAssign | TokenType::SlashAssign => {
                let op = self.advance().value;
                let val = self.parse_expression()?;
                return Ok(Stmt::Assign { target: expr, op, value: val });
            }
            _ => {}
        }

        Ok(Stmt::Expression { expression: expr })
    }

    // ─── Expressions (precedence climbing) ──────────────────────────

    fn parse_expression(&mut self) -> TechResult<Expr> {
        self.parse_ternary()
    }

    fn parse_ternary(&mut self) -> TechResult<Expr> {
        let expr = self.parse_or()?;
        if self.kw("when") {
            let condition = self.parse_or()?;
            self.expect(TokenType::Keyword, Some("else"))?;
            let false_val = self.parse_or()?;
            return Ok(Expr::Ternary {
                true_val: Box::new(expr),
                condition: Box::new(condition),
                false_val: Box::new(false_val),
            });
        }
        Ok(expr)
    }

    fn parse_or(&mut self) -> TechResult<Expr> {
        let mut left = self.parse_and()?;
        while self.kw("or") {
            let right = self.parse_and()?;
            left = Expr::BinaryOp { left: Box::new(left), op: "or".into(), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> TechResult<Expr> {
        let mut left = self.parse_not()?;
        while self.kw("and") {
            let right = self.parse_not()?;
            left = Expr::BinaryOp { left: Box::new(left), op: "and".into(), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> TechResult<Expr> {
        if self.kw("not") {
            let operand = self.parse_not()?;
            return Ok(Expr::UnaryOp { op: "not".into(), operand: Box::new(operand) });
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> TechResult<Expr> {
        let mut left = self.parse_pipe()?;
        loop {
            let tok = self.peek().clone();
            match tok.token_type {
                TokenType::Equal | TokenType::NotEqual |
                TokenType::Less | TokenType::Greater |
                TokenType::LessEqual | TokenType::GreaterEqual => {
                    let op = self.advance().value;
                    let right = self.parse_pipe()?;
                    left = Expr::BinaryOp { left: Box::new(left), op, right: Box::new(right) };
                }
                TokenType::Keyword if tok.value == "is" => {
                    self.advance();
                    let right = self.parse_pipe()?;
                    left = Expr::BinaryOp { left: Box::new(left), op: "is".into(), right: Box::new(right) };
                }
                TokenType::Keyword if tok.value == "in" => {
                    self.advance();
                    let right = self.parse_pipe()?;
                    left = Expr::BinaryOp { left: Box::new(left), op: "in".into(), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_pipe(&mut self) -> TechResult<Expr> {
        let mut left = self.parse_range()?;
        while self.match_tok(TokenType::Pipe, None) {
            let right = self.parse_range()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: "|>".into(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_range(&mut self) -> TechResult<Expr> {
        let left = self.parse_addition()?;
        if self.match_tok(TokenType::DotDotEqual, None) {
            let right = self.parse_addition()?;
            return Ok(Expr::Range { start: Box::new(left), end: Box::new(right), inclusive: true });
        }
        if self.match_tok(TokenType::DotDot, None) {
            let right = self.parse_addition()?;
            return Ok(Expr::Range { start: Box::new(left), end: Box::new(right), inclusive: false });
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> TechResult<Expr> {
        let mut left = self.parse_multiplication()?;
        loop {
            let tok = self.peek().clone();
            if tok.token_type == TokenType::Plus || tok.token_type == TokenType::Minus {
                let op = self.advance().value;
                let right = self.parse_multiplication()?;
                left = Expr::BinaryOp { left: Box::new(left), op, right: Box::new(right) };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> TechResult<Expr> {
        let mut left = self.parse_unary()?;
        loop {
            let tok = self.peek().clone();
            if tok.token_type == TokenType::Star || tok.token_type == TokenType::Slash ||
               tok.token_type == TokenType::DoubleSlash || tok.token_type == TokenType::Percent {
                let op = self.advance().value;
                let right = self.parse_unary()?;
                left = Expr::BinaryOp { left: Box::new(left), op, right: Box::new(right) };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> TechResult<Expr> {
        let tok = self.peek().clone();
        if tok.token_type == TokenType::Minus {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp { op: "-".into(), operand: Box::new(operand) });
        }
        if tok.token_type == TokenType::Keyword && tok.value == "typeof" {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp { op: "typeof".into(), operand: Box::new(operand) });
        }
        self.parse_power()
    }

    fn parse_power(&mut self) -> TechResult<Expr> {
        let base = self.parse_call()?;
        if self.match_tok(TokenType::Power, None) {
            let exp = self.parse_unary()?;
            return Ok(Expr::BinaryOp { left: Box::new(base), op: "**".into(), right: Box::new(exp) });
        }
        Ok(base)
    }

    fn parse_call(&mut self) -> TechResult<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_tok(TokenType::LParen, None) {
                // Function call
                let mut args = Vec::new();
                if self.peek().token_type != TokenType::RParen {
                    args.push(self.parse_expression()?);
                    while self.match_tok(TokenType::Comma, None) {
                        args.push(self.parse_expression()?);
                    }
                }
                self.expect(TokenType::RParen, None)?;
                expr = Expr::Call { callee: Box::new(expr), args };
            } else if self.match_tok(TokenType::Dot, None) {
                // Member access
                let member = self.parse_member_name()?;
                expr = Expr::Member { obj: Box::new(expr), member };
            } else if self.match_tok(TokenType::LBracket, None) {
                // Index access
                let index = self.parse_expression()?;
                self.expect(TokenType::RBracket, None)?;
                expr = Expr::Index { obj: Box::new(expr), index: Box::new(index) };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> TechResult<Expr> {
        let tok = self.peek().clone();

        match tok.token_type {
            TokenType::NumberInt => {
                self.advance();
                let v: i64 = if tok.value.starts_with("0x") || tok.value.starts_with("0X") {
                    i64::from_str_radix(&tok.value[2..], 16).unwrap_or(0)
                } else if tok.value.starts_with("0b") || tok.value.starts_with("0B") {
                    i64::from_str_radix(&tok.value[2..], 2).unwrap_or(0)
                } else if tok.value.starts_with("0o") || tok.value.starts_with("0O") {
                    i64::from_str_radix(&tok.value[2..], 8).unwrap_or(0)
                } else {
                    tok.value.parse().unwrap_or(0)
                };
                Ok(Expr::NumberInt(v))
            }
            TokenType::NumberFloat => {
                self.advance();
                let v: f64 = tok.value.parse().unwrap_or(0.0);
                Ok(Expr::NumberFloat(v))
            }
            TokenType::String => {
                self.advance();
                Ok(Expr::String(tok.value))
            }
            TokenType::FString => {
                self.advance();
                Ok(Expr::FString(tok.value))
            }
            TokenType::BoolTrue => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenType::BoolFalse => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenType::None => {
                self.advance();
                Ok(Expr::None)
            }
            TokenType::Identifier => {
                self.advance();
                Ok(Expr::Identifier(tok.value))
            }
            TokenType::Keyword if tok.value == "self" => {
                self.advance();
                Ok(Expr::Identifier("self".into()))
            }
            TokenType::Keyword
                if matches!(
                    tok.value.as_str(),
                    "page" | "state" | "component" | "window" | "scene" | "timeline" | "run"
                ) =>
            {
                self.advance();
                Ok(Expr::Identifier(tok.value))
            }
            TokenType::Keyword if tok.value == "ask" => {
                self.advance();
                let prompt = self.parse_expression()?;
                Ok(Expr::Ask { prompt: Box::new(prompt) })
            }
            TokenType::Keyword if tok.value == "new" => {
                self.advance();
                // `new ClassName(args)`
                let class_expr = self.parse_call()?;
                Ok(class_expr)
            }
            TokenType::LParen => {
                self.parse_grouped_or_lambda()
            }
            TokenType::LBracket => {
                self.parse_list_literal()
            }
            TokenType::LBrace => {
                self.parse_map_literal()
            }
            TokenType::Minus => {
                // Negative number
                self.advance();
                let operand = self.parse_primary()?;
                Ok(Expr::UnaryOp { op: "-".into(), operand: Box::new(operand) })
            }
            _ => {
                Err(TechError::parse(
                    format!("Unexpected token: {:?} '{}'", tok.token_type, tok.value),
                    tok.line, tok.column, &self.filename
                ))
            }
        }
    }

    fn parse_list_literal(&mut self) -> TechResult<Expr> {
        self.advance(); // [
        self.skip_nl();
        let mut elements = Vec::new();
        if self.peek().token_type != TokenType::RBracket {
            elements.push(self.parse_expression()?);
            while self.match_tok(TokenType::Comma, None) {
                self.skip_nl();
                if self.peek().token_type == TokenType::RBracket { break; }
                elements.push(self.parse_expression()?);
            }
        }
        self.skip_nl();
        self.expect(TokenType::RBracket, None)?;
        Ok(Expr::List(elements))
    }

    fn parse_map_literal(&mut self) -> TechResult<Expr> {
        self.advance(); // {
        self.skip_nl();
        let mut entries = Vec::new();
        if self.peek().token_type != TokenType::RBrace {
            let (k, v) = self.parse_map_entry()?;
            entries.push((k, v));
            while self.match_tok(TokenType::Comma, None) {
                self.skip_nl();
                if self.peek().token_type == TokenType::RBrace { break; }
                let (k, v) = self.parse_map_entry()?;
                entries.push((k, v));
            }
        }
        self.skip_nl();
        self.expect(TokenType::RBrace, None)?;
        Ok(Expr::Map(entries))
    }

    fn parse_map_entry(&mut self) -> TechResult<(Expr, Expr)> {
        self.skip_nl();
        let key = self.parse_expression()?;
        self.expect(TokenType::Colon, None)?;
        self.skip_nl();
        let value = self.parse_expression()?;
        Ok((key, value))
    }

    fn parse_grouped_or_lambda(&mut self) -> TechResult<Expr> {
        // Lookahead to decide: is this a lambda `(x) => expr` or a grouped expression `(expr)`?
        let save_pos = self.pos;

        // Try to parse as lambda
        if let Ok(lambda) = self.try_parse_lambda() {
            return Ok(lambda);
        }

        // Backtrack and parse as grouped expression
        self.pos = save_pos;
        self.advance(); // (
        self.skip_nl();
        let expr = self.parse_expression()?;
        self.skip_nl();
        self.expect(TokenType::RParen, None)?;
        Ok(expr)
    }

    fn try_parse_lambda(&mut self) -> TechResult<Expr> {
        self.advance(); // (
        let mut params = Vec::new();

        if self.peek().token_type != TokenType::RParen {
            let name = self.expect(TokenType::Identifier, None)?;
            let default = if self.match_tok(TokenType::Assign, None) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            params.push(Param { name: name.value, default });

            while self.match_tok(TokenType::Comma, None) {
                let name = self.expect(TokenType::Identifier, None)?;
                let default = if self.match_tok(TokenType::Assign, None) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                params.push(Param { name: name.value, default });
            }
        }

        self.expect(TokenType::RParen, None)?;
        self.expect(TokenType::Arrow, None)?;
        let body = self.parse_expression()?;

        Ok(Expr::Lambda { params, body: Box::new(body) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(src: &str) -> Program {
        let tokens = Lexer::new(src, "<test>").tokenize().unwrap();
        Parser::new(tokens, "<test>").parse().unwrap()
    }

    #[test]
    fn test_say() {
        let prog = parse("say \"Hello\"");
        assert_eq!(prog.body.len(), 1);
        match &prog.body[0] {
            Stmt::Say { values } => assert_eq!(values.len(), 1),
            _ => panic!("Expected Say statement"),
        }
    }

    #[test]
    fn test_make_variable() {
        let prog = parse("make x = 42");
        match &prog.body[0] {
            Stmt::Set { name, .. } => assert_eq!(name, "x"),
            _ => panic!("Expected Set statement"),
        }
    }

    #[test]
    fn test_function() {
        let prog = parse("build greet(name) {\n  say name\n}");
        match &prog.body[0] {
            Stmt::Fn { name, params, .. } => {
                assert_eq!(name, "greet");
                assert_eq!(params.len(), 1);
            }
            _ => panic!("Expected Fn statement"),
        }
    }

    #[test]
    fn test_if_else() {
        let prog = parse("when x > 10 {\n  say \"big\"\n} else {\n  say \"small\"\n}");
        match &prog.body[0] {
            Stmt::If { else_body, .. } => assert!(else_body.is_some()),
            _ => panic!("Expected If statement"),
        }
    }

    #[test]
    fn test_for_loop() {
        let prog = parse("each i in 1..10 {\n  say i\n}");
        match &prog.body[0] {
            Stmt::For { var_name, .. } => assert_eq!(var_name, "i"),
            _ => panic!("Expected For statement"),
        }
    }
}
