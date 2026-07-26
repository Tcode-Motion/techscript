use crate::parser::{ParseResult, Parser};
use techscript_ast::{
    AskExpr, AssignmentExpr, BinaryExpr, CallExpr, Expression, FStringExpr, FStringPart, IndexExpr,
    ListExpr, LiteralExpr, LiteralVal, MapExpr, MemberExpr, NewExpr, RangeExpr,
};
use techscript_common::{Ident, Span};
use techscript_errors::{Diagnostic, DiagnosticLevel, DiagnosticReporter, ErrorCode};
use techscript_syntax::{Associativity, Precedence, Token, TokenKind};

impl<'a> Parser<'a> {
    /// Pratt parser top-level expression entry point.
    pub fn parse_expression(
        &mut self,
        precedence: Precedence,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<Expression> {
        let token = self.advance().clone();
        let mut left = self.parse_prefix(&token, reporter)?;

        while !self.is_at_end() {
            let next_prec = self.peek().kind.precedence();

            // Check for implicit call (e.g. `env "PATH"`, `say x` etc.)
            if precedence < Precedence::Call 
                && self.can_start_implicit_call_arg(self.peek().kind)
                && self.is_callable(&left)
            {
                // Parse argument(s)
                let mut args = Vec::new();
                let arg = self.parse_expression(Precedence::Call, reporter)?;
                args.push(arg);
                while self.match_token(TokenKind::Comma) {
                    args.push(self.parse_expression(Precedence::Call, reporter)?);
                }
                let span = Span::new(left.span().start, self.previous().span.end);
                left = Expression::Call(CallExpr::new(
                    self.next_id(),
                    Box::new(left),
                    args,
                    span,
                ));
                continue;
            }

            if precedence >= next_prec {
                break;
            }

            let next_token = self.advance().clone();
            left = self.parse_infix(left, &next_token, reporter)?;
        }

        Ok(left)
    }

    fn can_start_implicit_call_arg(&self, kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::IntLiteral
                | TokenKind::FloatLiteral
                | TokenKind::StringLiteral
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Null
                | TokenKind::None
                | TokenKind::Identifier
                | TokenKind::SelfKw
                | TokenKind::FStringStart
                | TokenKind::Ask
                | TokenKind::New
        )
    }

    fn is_callable(&self, expr: &Expression) -> bool {
        matches!(
            expr,
            Expression::Identifier(_)
                | Expression::Member(_)
                | Expression::Index(_)
                | Expression::Call(_)
        )
    }

    /// Handles parsing of prefix / leaf expressions.
    fn parse_prefix(
        &mut self,
        token: &Token,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<Expression> {
        match token.kind {
            TokenKind::IntLiteral => {
                let val = match self.parse_int_value(&token.lexeme) {
                    Ok(i) => LiteralVal::Int(i),
                    Err(_) => LiteralVal::Int(0),
                };
                Ok(Expression::Literal(LiteralExpr::new(
                    self.next_id(),
                    val,
                    token.span,
                )))
            }
            TokenKind::FloatLiteral => {
                let clean = token.lexeme.replace('_', "");
                let val = clean
                    .parse::<f64>()
                    .map(LiteralVal::Float)
                    .unwrap_or(LiteralVal::Float(0.0));
                Ok(Expression::Literal(LiteralExpr::new(
                    self.next_id(),
                    val,
                    token.span,
                )))
            }
            TokenKind::StringLiteral => {
                let val = LiteralVal::Str(self.clean_string_literal(&token.lexeme));
                Ok(Expression::Literal(LiteralExpr::new(
                    self.next_id(),
                    val,
                    token.span,
                )))
            }
            TokenKind::True => Ok(Expression::Literal(LiteralExpr::new(
                self.next_id(),
                LiteralVal::Bool(true),
                token.span,
            ))),
            TokenKind::False => Ok(Expression::Literal(LiteralExpr::new(
                self.next_id(),
                LiteralVal::Bool(false),
                token.span,
            ))),
            TokenKind::Null | TokenKind::None => Ok(Expression::Literal(LiteralExpr::new(
                self.next_id(),
                LiteralVal::None,
                token.span,
            ))),
            TokenKind::Identifier => {
                let ident = Ident {
                    name: token.lexeme.clone(),
                    span: token.span,
                };
                Ok(Expression::Identifier(ident))
            }
            TokenKind::SelfKw => {
                let ident = Ident {
                    name: "self".to_string(),
                    span: token.span,
                };
                Ok(Expression::Identifier(ident))
            }
            TokenKind::Minus | TokenKind::Plus | TokenKind::Not | TokenKind::Await => {
                let right = self.parse_expression(Precedence::Unary, reporter)?;
                let span = Span::new(token.span.start, right.span().end);
                Ok(Expression::Unary(techscript_ast::UnaryExpr::new(
                    self.next_id(),
                    token.lexeme.clone(),
                    Box::new(right),
                    span,
                )))
            }
            TokenKind::LeftParen => {
                let expr = self.parse_expression(Precedence::None, reporter)?;
                self.consume(
                    TokenKind::RightParen,
                    ErrorCode::E0105,
                    "Expected ')' after grouping expression",
                    reporter,
                )?;
                Ok(Expression::Group(Box::new(expr)))
            }
            TokenKind::LeftBracket => {
                let start_pos = token.span.start;
                let mut items = Vec::new();
                while self.match_token(TokenKind::Newline) {}
                if !self.check(TokenKind::RightBracket) {
                    loop {
                        while self.match_token(TokenKind::Newline) {}
                        items.push(self.parse_expression(Precedence::None, reporter)?);
                        while self.match_token(TokenKind::Newline) {}
                        if !self.match_token(TokenKind::Comma) {
                            break;
                        }
                        while self.match_token(TokenKind::Newline) {}
                    }
                }
                while self.match_token(TokenKind::Newline) {}
                self.consume(
                    TokenKind::RightBracket,
                    ErrorCode::E0105,
                    "Expected ']' after list elements",
                    reporter,
                )?;
                let span = Span::new(start_pos, self.previous().span.end);
                Ok(Expression::List(ListExpr::new(self.next_id(), items, span)))
            }
            TokenKind::LeftBrace => {
                let start_pos = token.span.start;
                let mut entries = Vec::new();
                while self.match_token(TokenKind::Newline) {}
                if !self.check(TokenKind::RightBrace) {
                    loop {
                        while self.match_token(TokenKind::Newline) {}
                        let key = self.parse_expression(Precedence::None, reporter)?;
                        self.consume(
                            TokenKind::Colon,
                            ErrorCode::E0100,
                            "Expected ':' after map key",
                            reporter,
                        )?;
                        while self.match_token(TokenKind::Newline) {}
                        let val = self.parse_expression(Precedence::None, reporter)?;
                        entries.push((key, val));
                        while self.match_token(TokenKind::Newline) {}
                        if !self.match_token(TokenKind::Comma) {
                            break;
                        }
                        while self.match_token(TokenKind::Newline) {}
                    }
                }
                while self.match_token(TokenKind::Newline) {}
                self.consume(
                    TokenKind::RightBrace,
                    ErrorCode::E0105,
                    "Expected '}' after map entries",
                    reporter,
                )?;
                let span = Span::new(start_pos, self.previous().span.end);
                Ok(Expression::Map(MapExpr::new(self.next_id(), entries, span)))
            }
            TokenKind::Ask => {
                let prompt = self.parse_expression(Precedence::Unary, reporter)?;
                let span = Span::new(token.span.start, prompt.span().end);
                Ok(Expression::Ask(AskExpr::new(
                    self.next_id(),
                    Box::new(prompt),
                    span,
                )))
            }
            TokenKind::New => {
                let class_name = self.parse_identifier(reporter)?;
                // Parse optional generic args
                if self.match_token(TokenKind::Less) {
                    loop {
                        let _ = self.parse_type_spec(reporter)?;
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
                }
                self.consume(
                    TokenKind::LeftParen,
                    ErrorCode::E0104,
                    "Expected '(' after class name in instantiation",
                    reporter,
                )?;
                let mut args = Vec::new();
                if !self.check(TokenKind::RightParen) {
                    loop {
                        args.push(self.parse_expression(Precedence::None, reporter)?);
                        if !self.match_token(TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.consume(
                    TokenKind::RightParen,
                    ErrorCode::E0105,
                    "Expected ')' after constructor arguments",
                    reporter,
                )?;
                let span = Span::new(token.span.start, self.previous().span.end);
                Ok(Expression::New(NewExpr::new(
                    self.next_id(),
                    class_name,
                    args,
                    span,
                )))
            }
            TokenKind::FStringStart => {
                let start_pos = token.span.start;
                if token.lexeme.starts_with('f') {
                    reporter.report(techscript_errors::Diagnostic::new(
                        techscript_errors::DiagnosticLevel::Warning,
                        techscript_errors::ErrorCode::TSW1012,
                        "Warning TSW1012: 'f\"' prefix is deprecated. Use '$\"' for string interpolation.".to_string(),
                        token.span,
                    ));
                }
                let mut parts = Vec::new();
                while !self.check(TokenKind::FStringEnd) && !self.is_at_end() {
                    if self.match_token(TokenKind::FStringText) {
                        parts.push(FStringPart::Literal(self.previous().lexeme.clone()));
                    } else if self.match_token(TokenKind::FStringExprStart) {
                        let expr = self.parse_expression(Precedence::None, reporter)?;
                        self.consume(
                            TokenKind::FStringExprEnd,
                            ErrorCode::E0105,
                            "Expected '}' after f-string expression",
                            reporter,
                        )?;
                        parts.push(FStringPart::Expr(expr));
                    } else {
                        self.advance();
                    }
                }
                self.consume(
                    TokenKind::FStringEnd,
                    ErrorCode::E0105,
                    "Expected closing quote for f-string",
                    reporter,
                )?;
                let span = Span::new(start_pos, self.previous().span.end);
                Ok(Expression::FString(FStringExpr::new(
                    self.next_id(),
                    parts,
                    span,
                )))
            }
            TokenKind::Typeof => {
                let right = self.parse_expression(Precedence::Unary, reporter)?;
                let callee = Expression::Identifier(techscript_common::Ident::new("type_of".to_string(), token.span));
                let span = Span::new(token.span.start, right.span().end);
                Ok(Expression::Call(techscript_ast::CallExpr::new(
                    self.next_id(),
                    Box::new(callee),
                    vec![right],
                    span,
                )))
            }
            _ => {
                let diag = Diagnostic::new(
                    DiagnosticLevel::Error,
                    ErrorCode::E0100,
                    format!("Expected expression, found: '{}'", token.lexeme),
                    token.span,
                );
                reporter.report(diag);
                Err(())
            }
        }
    }

    /// Handles parsing of infix / postfix expressions.
    fn parse_infix(
        &mut self,
        left: Expression,
        token: &Token,
        reporter: &mut DiagnosticReporter,
    ) -> ParseResult<Expression> {
        let precedence = token.kind.precedence();
        let assoc = token.kind.associativity();

        match token.kind {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::DoubleSlash
            | TokenKind::Percent
            | TokenKind::DoubleStar
            | TokenKind::EqualEqual
            | TokenKind::BangEqual
            | TokenKind::TripleEqual
            | TokenKind::BangEqualEqual
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Is
            | TokenKind::In
            | TokenKind::Equals
            | TokenKind::QuestionQuestion => {
                let right_prec = if assoc == Associativity::Right {
                    next_lower_precedence(precedence)
                } else {
                    precedence
                };
                let right = self.parse_expression(right_prec, reporter)?;
                let span = Span::new(left.span().start, right.span().end);
                let lexeme = if token.kind == TokenKind::Equals {
                    "==".to_string()
                } else {
                    token.lexeme.clone()
                };
                Ok(Expression::Binary(BinaryExpr::new(
                    self.next_id(),
                    Box::new(left),
                    lexeme,
                    Box::new(right),
                    span,
                )))
            }
            TokenKind::QuestionDot => {
                let right = self.parse_expression(Precedence::Call, reporter)?;
                let span = Span::new(left.span().start, right.span().end);
                Ok(Expression::Binary(BinaryExpr::new(
                    self.next_id(),
                    Box::new(left),
                    "?.".to_string(),
                    Box::new(right),
                    span,
                )))
            }
            TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual => {
                if !self.is_valid_assignment_target(&left) {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0113,
                        "Invalid assignment target".to_string(),
                        left.span(),
                    );
                    reporter.report(diag);
                    return Err(());
                }
                let right = self.parse_expression(Precedence::Assignment, reporter)?;
                let span = Span::new(left.span().start, right.span().end);
                Ok(Expression::Assignment(AssignmentExpr::new(
                    self.next_id(),
                    Box::new(left),
                    token.lexeme.clone(),
                    Box::new(right),
                    span,
                )))
            }
            TokenKind::DotDot | TokenKind::DotDotEqual => {
                let inclusive = token.kind == TokenKind::DotDotEqual;
                let right = self.parse_expression(Precedence::Range, reporter)?;
                let span = Span::new(left.span().start, right.span().end);
                Ok(Expression::Range(RangeExpr::new(
                    self.next_id(),
                    Box::new(left),
                    inclusive,
                    Box::new(right),
                    span,
                )))
            }
            TokenKind::Dot => {
                let member = self.parse_identifier(reporter)?;
                let span = Span::new(left.span().start, member.span.end);
                Ok(Expression::Member(MemberExpr::new(
                    self.next_id(),
                    Box::new(left),
                    member,
                    span,
                )))
            }
            TokenKind::LeftBracket => {
                let index = self.parse_expression(Precedence::None, reporter)?;
                self.consume(
                    TokenKind::RightBracket,
                    ErrorCode::E0105,
                    "Expected ']' after index access",
                    reporter,
                )?;
                let span = Span::new(left.span().start, self.previous().span.end);
                Ok(Expression::Index(IndexExpr::new(
                    self.next_id(),
                    Box::new(left),
                    Box::new(index),
                    span,
                )))
            }
            TokenKind::LeftParen => {
                let mut args = Vec::new();
                while self.match_token(TokenKind::Newline) {}
                if !self.check(TokenKind::RightParen) {
                    loop {
                        while self.match_token(TokenKind::Newline) {}
                        args.push(self.parse_expression(Precedence::None, reporter)?);
                        while self.match_token(TokenKind::Newline) {}
                        if !self.match_token(TokenKind::Comma) {
                            break;
                        }
                        while self.match_token(TokenKind::Newline) {}
                    }
                }
                while self.match_token(TokenKind::Newline) {}
                self.consume(
                    TokenKind::RightParen,
                    ErrorCode::E0105,
                    "Expected ')' after call arguments",
                    reporter,
                )?;
                let span = Span::new(left.span().start, self.previous().span.end);
                Ok(Expression::Call(CallExpr::new(
                    self.next_id(),
                    Box::new(left),
                    args,
                    span,
                )))
            }
            _ => Ok(left),
        }
    }

    /// Verifies if the left-hand side expression is a valid target for assignment.
    fn is_valid_assignment_target(&self, expr: &Expression) -> bool {
        matches!(
            expr,
            Expression::Identifier(_) | Expression::Member(_) | Expression::Index(_)
        )
    }

    /// Helper to parse string literal base prefixes (e.g. hex, octal, binary).
    fn parse_int_value(&self, lexeme: &str) -> Result<i64, ()> {
        let clean = lexeme.replace('_', "");
        if let Some(stripped) = clean.strip_prefix("0x") {
            i64::from_str_radix(stripped, 16).map_err(|_| ())
        } else if let Some(stripped) = clean.strip_prefix("0X") {
            i64::from_str_radix(stripped, 16).map_err(|_| ())
        } else if let Some(stripped) = clean.strip_prefix("0b") {
            i64::from_str_radix(stripped, 2).map_err(|_| ())
        } else if let Some(stripped) = clean.strip_prefix("0B") {
            i64::from_str_radix(stripped, 2).map_err(|_| ())
        } else if let Some(stripped) = clean.strip_prefix("0o") {
            i64::from_str_radix(stripped, 8).map_err(|_| ())
        } else if let Some(stripped) = clean.strip_prefix("0O") {
            i64::from_str_radix(stripped, 8).map_err(|_| ())
        } else {
            clean.parse::<i64>().map_err(|_| ())
        }
    }

    /// Removes bounding quotes from standard string literals.
    fn clean_string_literal(&self, lexeme: &str) -> String {
        if lexeme.len() >= 2 && lexeme.starts_with('"') && lexeme.ends_with('"') {
            lexeme[1..lexeme.len() - 1].to_string()
        } else {
            lexeme.to_string()
        }
    }
}

/// Helper function to return the next lower precedence level.
fn next_lower_precedence(prec: Precedence) -> Precedence {
    match prec {
        Precedence::None => Precedence::None,
        Precedence::Assignment => Precedence::None,
        Precedence::NullCoalescing => Precedence::Assignment,
        Precedence::Or => Precedence::NullCoalescing,
        Precedence::And => Precedence::Or,
        Precedence::Equality => Precedence::And,
        Precedence::Comparison => Precedence::Equality,
        Precedence::Range => Precedence::Comparison,
        Precedence::BitwiseOr => Precedence::Range,
        Precedence::BitwiseXor => Precedence::BitwiseOr,
        Precedence::BitwiseAnd => Precedence::BitwiseXor,
        Precedence::Shift => Precedence::BitwiseAnd,
        Precedence::Term => Precedence::Shift,
        Precedence::Factor => Precedence::Term,
        Precedence::Exponent => Precedence::Factor,
        Precedence::Unary => Precedence::Exponent,
        Precedence::Call => Precedence::Unary,
    }
}
