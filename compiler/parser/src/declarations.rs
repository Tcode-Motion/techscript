use crate::parser::{ParseResult, Parser};
use techscript_ast::{
    ConstDecl, EnumDecl, EnumVariant, ExportDecl, FieldSpec, FuncDecl, MethodDecl, MethodKeyword,
    ModelDecl, Parameter, Pattern, Statement, StructDecl, TypeSpec, VarDecl,
};
use techscript_common::{Ident, Span};
use techscript_errors::{DiagnosticReporter, ErrorCode};
use techscript_syntax::TokenKind;

impl<'a> Parser<'a> {
    /// Parse any declaration node.
    pub fn parse_declaration(
        &mut self,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<Statement> {
        if self.check(TokenKind::Make) || self.check(TokenKind::Let) || self.check(TokenKind::Var) {
            let decl = self.parse_variable_decl(reporter)?;
            Ok(Statement::VarDecl(decl))
        } else if self.check(TokenKind::Const) {
            let decl = self.parse_constant_decl(reporter)?;
            Ok(Statement::ConstDecl(decl))
        } else if self.check(TokenKind::Build)
            || self.check(TokenKind::Fun)
            || self.check(TokenKind::Function)
            || self.check(TokenKind::Async)
        {
            let decl = self.parse_function_decl(reporter)?;
            Ok(Statement::FuncDecl(decl))
        } else if self.check(TokenKind::Struct) {
            let decl = self.parse_struct_decl(reporter)?;
            Ok(Statement::StructDecl(decl))
        } else if self.check(TokenKind::Enum) {
            let decl = self.parse_enum_decl(reporter)?;
            Ok(Statement::EnumDecl(decl))
        } else if self.check(TokenKind::Model) || self.check(TokenKind::Class) {
            let decl = self.parse_model_decl(reporter)?;
            Ok(Statement::ModelDecl(decl))
        } else if self.check(TokenKind::Export) {
            let decl = self.parse_export_decl(reporter)?;
            Ok(Statement::ExportDecl(decl))
        } else {
            Err(())
        }
    }

    /// make/let/var pattern[: type] = expression
    fn parse_variable_decl(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<VarDecl> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume keyword

        let pattern = self.parse_pattern(reporter)?;

        let mut type_ann = None;
        if self.match_token(TokenKind::Colon) {
            type_ann = Some(self.parse_type_spec(reporter)?);
        }

        self.consume(
            TokenKind::Equal,
            ErrorCode::E0100,
            "Expected '=' after variable pattern",
            reporter,
        )?;
        let initializer = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        self.consume_terminator(reporter)?;

        let span = Span::new(start_pos, self.previous().span.end);
        Ok(VarDecl::new(
            self.next_id(),
            pattern,
            type_ann,
            initializer,
            span,
        ))
    }

    /// const pattern[: type] = expression
    fn parse_constant_decl(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<ConstDecl> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume const

        let pattern = self.parse_pattern(reporter)?;

        let mut type_ann = None;
        if self.match_token(TokenKind::Colon) {
            type_ann = Some(self.parse_type_spec(reporter)?);
        }

        self.consume(
            TokenKind::Equal,
            ErrorCode::E0100,
            "Expected '=' after constant pattern",
            reporter,
        )?;
        let initializer = self.parse_expression(techscript_syntax::Precedence::None, reporter)?;
        self.consume_terminator(reporter)?;

        let span = Span::new(start_pos, self.previous().span.end);
        Ok(ConstDecl::new(
            self.next_id(),
            pattern,
            type_ann,
            initializer,
            span,
        ))
    }

    /// build/fun/function name<T>(params) -> Ret { body }
    fn parse_function_decl(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<FuncDecl> {
        let start_pos = self.peek().span.start;
        let mut async_kw = false;
        if self.match_token(TokenKind::Async) {
            async_kw = true;
        }

        if !self.check(TokenKind::Build)
            && !self.check(TokenKind::Fun)
            && !self.check(TokenKind::Function)
        {
            let span = self.peek().span;
            reporter.report(techscript_errors::Diagnostic::new(
                techscript_errors::DiagnosticLevel::Error,
                ErrorCode::E0100,
                "Expected build or fun keyword in function declaration".to_string(),
                span,
            ));
            return Err(());
        }
        self.advance(); // consume keyword

        let name = self.parse_identifier(reporter)?;

        let mut generic_params = None;
        if self.match_token(TokenKind::Less) {
            let mut list = Vec::new();
            loop {
                list.push(self.parse_identifier(reporter)?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(
                TokenKind::Greater,
                ErrorCode::E0105,
                "Expected '>' after generic parameters",
                reporter,
            )?;
            generic_params = Some(list);
        }

        self.consume(
            TokenKind::LeftParen,
            ErrorCode::E0104,
            "Expected '(' before function parameters",
            reporter,
        )?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                params.push(self.parse_parameter(reporter)?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(
            TokenKind::RightParen,
            ErrorCode::E0105,
            "Expected ')' after function parameters",
            reporter,
        )?;

        let mut return_type = None;
        if self.match_token(TokenKind::Arrow) {
            return_type = Some(self.parse_type_spec(reporter)?);
        }

        let body = self.parse_block(reporter)?;
        let span = Span::new(start_pos, body.span.end);

        Ok(FuncDecl::new(
            self.next_id(),
            async_kw,
            name,
            generic_params,
            params,
            return_type,
            body,
            span,
        ))
    }

    /// struct Name { fields }
    fn parse_struct_decl(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<StructDecl> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume struct

        let name = self.parse_identifier(reporter)?;
        self.consume(
            TokenKind::LeftBrace,
            ErrorCode::E0104,
            "Expected '{' before struct body",
            reporter,
        )?;

        let mut fields = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            while self.match_token(TokenKind::Newline) || self.match_token(TokenKind::Semicolon) {}
            if self.check(TokenKind::RightBrace) {
                break;
            }
            let field_start = self.peek().span.start;
            let field_name = self.parse_identifier(reporter)?;
            self.consume(
                TokenKind::Colon,
                ErrorCode::E0100,
                "Expected ':' after struct field name",
                reporter,
            )?;
            let field_type = self.parse_type_spec(reporter)?;
            self.consume_terminator(reporter)?;
            let field_span = Span::new(field_start, self.previous().span.end);
            fields.push(FieldSpec::new(field_name, field_type, field_span));
        }

        self.consume(
            TokenKind::RightBrace,
            ErrorCode::E0105,
            "Expected '}' after struct body",
            reporter,
        )?;
        let span = Span::new(start_pos, self.previous().span.end);

        Ok(StructDecl::new(self.next_id(), name, fields, span))
    }

    /// enum Name { variants }
    fn parse_enum_decl(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<EnumDecl> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume enum

        let name = self.parse_identifier(reporter)?;
        self.consume(
            TokenKind::LeftBrace,
            ErrorCode::E0104,
            "Expected '{' before enum body",
            reporter,
        )?;

        let mut variants = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            while self.match_token(TokenKind::Newline) || self.match_token(TokenKind::Semicolon) {}
            if self.check(TokenKind::RightBrace) {
                break;
            }
            let var_start = self.peek().span.start;
            let var_name = self.parse_identifier(reporter)?;

            let mut payload = None;
            if self.match_token(TokenKind::LeftParen) {
                let mut list = Vec::new();
                loop {
                    list.push(self.parse_type_spec(reporter)?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(
                    TokenKind::RightParen,
                    ErrorCode::E0105,
                    "Expected ')' after enum payload",
                    reporter,
                )?;
                payload = Some(list);
            }

            self.consume_terminator(reporter)?;
            let var_span = Span::new(var_start, self.previous().span.end);
            variants.push(EnumVariant::new(var_name, payload, var_span));
        }

        self.consume(
            TokenKind::RightBrace,
            ErrorCode::E0105,
            "Expected '}' after enum body",
            reporter,
        )?;
        let span = Span::new(start_pos, self.previous().span.end);

        Ok(EnumDecl::new(self.next_id(), name, variants, span))
    }

    /// model Name [extends Parent] { members }
    fn parse_model_decl(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<ModelDecl> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume model/class

        let name = self.parse_identifier(reporter)?;

        let mut parent = None;
        if self.match_token(TokenKind::Identifier) && self.previous().lexeme == "extends" {
            parent = Some(self.parse_identifier(reporter)?);
        }

        self.consume(
            TokenKind::LeftBrace,
            ErrorCode::E0104,
            "Expected '{' before model body",
            reporter,
        )?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            while self.match_token(TokenKind::Newline) || self.match_token(TokenKind::Semicolon) {}
            if self.check(TokenKind::RightBrace) {
                break;
            }
            // Check model members
            if self.check(TokenKind::Make)
                || self.check(TokenKind::Let)
                || self.check(TokenKind::Var)
            {
                fields.push(self.parse_variable_decl(reporter)?);
            } else if self.check(TokenKind::Const) {
                // Constants are treated as fields in the AST
                let c_decl = self.parse_constant_decl(reporter)?;
                let f_decl = VarDecl::new(
                    c_decl.id,
                    c_decl.pattern,
                    c_decl.type_ann,
                    c_decl.initializer,
                    c_decl.span,
                );
                fields.push(f_decl);
            } else if self.check(TokenKind::Build)
                || self.check(TokenKind::Fun)
                || self.check(TokenKind::Function)
            {
                let keyword = if self.check(TokenKind::Fun) {
                    MethodKeyword::Fun
                } else {
                    MethodKeyword::Build
                };
                let fn_decl = self.parse_function_decl(reporter)?;
                methods.push(MethodDecl::new(
                    fn_decl.id,
                    keyword,
                    fn_decl.name,
                    fn_decl.generic_params,
                    fn_decl.params,
                    fn_decl.return_type,
                    fn_decl.body,
                    fn_decl.span,
                ));
            } else {
                // recovery inside model
                self.advance();
            }
        }

        self.consume(
            TokenKind::RightBrace,
            ErrorCode::E0105,
            "Expected '}' after model body",
            reporter,
        )?;
        let span = Span::new(start_pos, self.previous().span.end);

        Ok(ModelDecl::new(
            self.next_id(),
            name,
            parent,
            fields,
            methods,
            span,
        ))
    }

    /// export <declaration>
    fn parse_export_decl(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<ExportDecl> {
        let start_pos = self.peek().span.start;
        self.advance(); // consume export

        let decl = self.parse_declaration(reporter)?;
        let span = Span::new(start_pos, decl.span().end);

        Ok(ExportDecl::new(self.next_id(), Box::new(decl), span))
    }

    /// Parse pattern inside make / const declarations.
    fn parse_pattern(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<Pattern> {
        if self.match_token(TokenKind::LeftParen) {
            let mut list = Vec::new();
            loop {
                list.push(self.parse_identifier(reporter)?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(
                TokenKind::RightParen,
                ErrorCode::E0105,
                "Expected ')' after pattern list",
                reporter,
            )?;
            Ok(Pattern::Tuple(list))
        } else if self.match_token(TokenKind::LeftBracket) {
            let mut list = Vec::new();
            loop {
                list.push(self.parse_identifier(reporter)?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(
                TokenKind::RightBracket,
                ErrorCode::E0105,
                "Expected ']' after pattern list",
                reporter,
            )?;
            Ok(Pattern::List(list))
        } else if self.match_token(TokenKind::LeftBrace) {
            let mut list = Vec::new();
            loop {
                list.push(self.parse_identifier(reporter)?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(
                TokenKind::RightBrace,
                ErrorCode::E0105,
                "Expected '}' after pattern list",
                reporter,
            )?;
            Ok(Pattern::Struct(list))
        } else {
            let name = self.parse_identifier(reporter)?;
            Ok(Pattern::Single(name))
        }
    }

    /// Parse type annotations (e.g. `Int` or `List<String>`).
    pub fn parse_type_spec(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<TypeSpec> {
        let name = self.parse_identifier(reporter)?;
        let mut generic_args = None;
        if self.match_token(TokenKind::Less) {
            let mut args = Vec::new();
            loop {
                args.push(self.parse_type_spec(reporter)?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(
                TokenKind::Greater,
                ErrorCode::E0105,
                "Expected '>' after generic arguments",
                reporter,
            )?;
            generic_args = Some(args);
        }
        let span = Span::new(name.span.start, self.previous().span.end);
        Ok(TypeSpec::new(name, generic_args, span))
    }

    /// Parses function parameters: `IDENTIFIER [ : type ] [ = default ]`.
    fn parse_parameter(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<Parameter> {
        let name = self.parse_identifier(reporter)?;
        let mut type_ann = None;
        if self.match_token(TokenKind::Colon) {
            type_ann = Some(self.parse_type_spec(reporter)?);
        }
        let mut default = None;
        if self.match_token(TokenKind::Equal) {
            default = Some(self.parse_expression(techscript_syntax::Precedence::None, reporter)?);
        }
        let span = Span::new(name.span.start, self.previous().span.end);
        Ok(Parameter::new(name, type_ann, default, span))
    }

    /// Expects and consumes an Identifier.
    pub fn parse_identifier(&mut self, reporter: &mut DiagnosticReporter) -> ParseResult<Ident> {
        let token = self.consume(
            TokenKind::Identifier,
            ErrorCode::E0101,
            "Expected identifier",
            reporter,
        )?;
        Ok(Ident {
            name: token.lexeme.clone(),
            span: token.span,
        })
    }
}
