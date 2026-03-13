"""TechScript Parser — recursive-descent parser with precedence climbing.

Consumes a list of ``Token`` objects from the lexer and produces an AST
(``Program`` node) as defined in ``ast_nodes``.
"""

from __future__ import annotations

from techscript.tokens import Token, TokenType
from techscript.ast_nodes import *
from techscript.errors import ParseError


class Parser:
    """Recursive-descent parser for TechScript."""

    def __init__(self, tokens: list[Token]) -> None:
        self.tokens = tokens
        self.pos = 0

    # ------------------------------------------------------------------
    # Helpers
    # ------------------------------------------------------------------

    def _peek(self) -> Token:
        return self.tokens[self.pos]

    def _advance(self) -> Token:
        tok = self.tokens[self.pos]
        self.pos += 1
        return tok

    def _expect(self, tt: TokenType, value: str | None = None) -> Token:
        tok = self._peek()
        if tok.type != tt:
            raise ParseError(
                f"Expected {tt.name}, got {tok.type.name} ('{tok.value}')",
                line=tok.line, column=tok.column,
            )
        if value is not None and tok.value != value:
            raise ParseError(
                f"Expected '{value}', got '{tok.value}'",
                line=tok.line, column=tok.column,
            )
        return self._advance()

    def _match(self, tt: TokenType, value: str | None = None) -> Token | None:
        tok = self._peek()
        if tok.type == tt and (value is None or tok.value == value):
            return self._advance()
        return None

    def _kw(self, value: str) -> Token | None:
        """Match a keyword token with a specific value."""
        return self._match(TokenType.KEYWORD, value)

    def _skip_nl(self) -> None:
        while self._peek().type == TokenType.NEWLINE:
            self._advance()

    def _at_end(self) -> bool:
        return self._peek().type == TokenType.EOF

    # ------------------------------------------------------------------
    # Public entry point
    # ------------------------------------------------------------------

    def parse(self) -> Program:
        self._skip_nl()
        body: list[Any] = []
        while not self._at_end():
            stmt = self._parse_statement()
            if stmt is not None:
                body.append(stmt)
            self._skip_nl()
        return Program(body=body)

    # ------------------------------------------------------------------
    # Statement parsing
    # ------------------------------------------------------------------

    def _parse_statement(self) -> Any:
        tok = self._peek()

        if tok.type == TokenType.KEYWORD:
            handler = {
                "say": self._parse_say,
                "make": self._parse_set,
                "keep": self._parse_const,
                "when": self._parse_if,
                "unless": self._parse_unless,
                "each": self._parse_for,
                "repeat": self._parse_while,
                "until": self._parse_until,
                "build": self._parse_fn,
                "model": self._parse_class,
                "send": self._parse_return,
                "stop": lambda: (self._advance(), BreakStmt())[1],
                "skip": lambda: (self._advance(), SkipStmt())[1],
                "pass": lambda: (self._advance(), PassStmt())[1],
                "attempt": self._parse_try,
                "fail": self._parse_throw,
                "match": self._parse_match,
                "use": self._parse_import,
                "take": self._parse_from_import,
                "drop": self._parse_del,
                "defer": self._parse_defer,
            }.get(tok.value)
            if handler:
                return handler()

        return self._parse_expression_statement()

    # --- simple statements ---

    def _parse_say(self) -> SayStmt:
        self._advance()  # 'say'
        values = [self._parse_expression()]
        while self._match(TokenType.COMMA):
            values.append(self._parse_expression())
        return SayStmt(values=values)

    def _parse_set(self) -> SetStmt:
        self._advance()  # 'make'
        name = self._expect(TokenType.IDENTIFIER).value
        self._expect(TokenType.ASSIGN)
        value = self._parse_expression()
        return SetStmt(name=name, value=value)

    def _parse_const(self) -> ConstStmt:
        self._advance()  # 'keep'
        name = self._expect(TokenType.IDENTIFIER).value
        self._expect(TokenType.ASSIGN)
        value = self._parse_expression()
        return ConstStmt(name=name, value=value)

    def _parse_return(self) -> ReturnStmt:
        self._advance()  # 'send'
        value = None
        if self._peek().type not in (TokenType.NEWLINE, TokenType.EOF, TokenType.RBRACE):
            value = self._parse_expression()
        return ReturnStmt(value=value)

    def _parse_throw(self) -> ThrowStmt:
        self._advance()  # 'fail'
        return ThrowStmt(value=self._parse_expression())

    def _parse_del(self) -> DelStmt:
        self._advance()  # 'drop'
        return DelStmt(name=self._expect(TokenType.IDENTIFIER).value)

    def _parse_defer(self) -> DeferStmt:
        self._advance()  # 'defer'
        return DeferStmt(expression=self._parse_expression())

    def _parse_import(self) -> ImportStmt:
        self._advance()  # 'import'
        module = self._expect(TokenType.IDENTIFIER).value
        while self._match(TokenType.DOT):
            module += "." + self._expect(TokenType.IDENTIFIER).value
        alias = None
        if self._kw("as"):
            alias = self._expect(TokenType.IDENTIFIER).value
        return ImportStmt(module=module, alias=alias)

    def _parse_from_import(self) -> FromImportStmt:
        self._advance()  # 'from'
        module = self._expect(TokenType.IDENTIFIER).value
        while self._match(TokenType.DOT):
            module += "." + self._expect(TokenType.IDENTIFIER).value
        self._expect(TokenType.KEYWORD, "import")
        names = [self._expect(TokenType.IDENTIFIER).value]
        while self._match(TokenType.COMMA):
            names.append(self._expect(TokenType.IDENTIFIER).value)
        return FromImportStmt(module=module, names=names)

    def _parse_export(self) -> ExportStmt:
        self._advance()  # 'export'
        return ExportStmt(declaration=self._parse_statement())

    # --- compound statements ---

    def _parse_if(self) -> IfStmt:
        self._advance()  # 'when'
        condition = self._parse_expression()
        body = self._parse_block()
        elif_clauses: list[tuple[Any, list]] = []
        while self._kw("alt"):
            ec = self._parse_expression()
            eb = self._parse_block()
            elif_clauses.append((ec, eb))
        else_body = None
        if self._kw("else"):
            else_body = self._parse_block()
        return IfStmt(condition, body, elif_clauses, else_body)

    def _parse_unless(self) -> IfStmt:
        self._advance()  # 'unless'
        condition = self._parse_expression()
        body = self._parse_block()
        # unless X ≡ if not X
        return IfStmt(condition=UnaryOp("not", condition), body=body)

    def _parse_for(self) -> ForStmt:
        self._advance()  # 'each'
        var_name = self._expect(TokenType.IDENTIFIER).value
        self._expect(TokenType.KEYWORD, "in")
        iterable = self._parse_expression()
        body = self._parse_block()
        return ForStmt(var_name=var_name, iterable=iterable, body=body)

    def _parse_while(self) -> WhileStmt:
        self._advance()  # 'repeat'
        condition = self._parse_expression()
        body = self._parse_block()
        return WhileStmt(condition=condition, body=body)

    def _parse_until(self) -> WhileStmt:
        self._advance()  # 'until'
        condition = self._parse_expression()
        body = self._parse_block()
        # until X ≡ while not X
        return WhileStmt(condition=UnaryOp("not", condition), body=body)

    def _parse_fn(self) -> FnStmt:
        self._advance()  # 'build'
        name = self._expect(TokenType.IDENTIFIER).value
        self._expect(TokenType.LPAREN)
        params = self._parse_param_list()
        self._expect(TokenType.RPAREN)
        body = self._parse_block()
        return FnStmt(name=name, params=params, body=body)

    def _parse_class(self) -> ClassStmt:
        self._advance()  # 'model'
        name = self._expect(TokenType.IDENTIFIER).value
        parent = None
        if self._match(TokenType.LPAREN):
            parent = self._expect(TokenType.IDENTIFIER).value
            self._expect(TokenType.RPAREN)
        body = self._parse_block()
        return ClassStmt(name=name, parent=parent, body=body)

    def _parse_try(self) -> TryStmt:
        self._advance()  # 'attempt'
        body = self._parse_block()
        catch_var = None
        catch_body: list[Any] = []
        finally_body: list[Any] | None = None
        if self._kw("rescue"):
            if self._peek().type == TokenType.IDENTIFIER:
                catch_var = self._advance().value
            catch_body = self._parse_block()
        if self._kw("always"):
            finally_body = self._parse_block()
        return TryStmt(body, catch_var, catch_body, finally_body)

    def _parse_match(self) -> MatchStmt:
        self._advance()  # 'match'
        subject = self._parse_expression()
        self._expect(TokenType.LBRACE)
        self._skip_nl()
        cases: list[tuple[Any, list]] = []
        while self._kw("case"):
            pattern = self._parse_expression()
            case_body = self._parse_block()
            cases.append((pattern, case_body))
            self._skip_nl()
        self._expect(TokenType.RBRACE)
        return MatchStmt(subject=subject, cases=cases)

    def _parse_guard(self) -> GuardStmt:
        self._advance()  # 'guard'
        condition = self._parse_expression()
        self._expect(TokenType.KEYWORD, "else")
        body = self._parse_block()
        return GuardStmt(condition=condition, else_body=body)

    def _parse_with(self) -> WithStmt:
        self._advance()  # 'with'
        expr = self._parse_expression()
        self._expect(TokenType.KEYWORD, "as")
        var = self._expect(TokenType.IDENTIFIER).value
        body = self._parse_block()
        return WithStmt(expression=expr, var_name=var, body=body)

    # --- blocks ---

    def _parse_block(self) -> list[Any]:
        self._skip_nl()
        self._expect(TokenType.LBRACE)
        stmts: list[Any] = []
        while self._peek().type not in (TokenType.RBRACE, TokenType.EOF):
            self._skip_nl()
            if self._peek().type in (TokenType.RBRACE, TokenType.EOF):
                break
            stmt = self._parse_statement()
            if stmt is not None:
                stmts.append(stmt)
            self._skip_nl()
        if self._peek().type == TokenType.RBRACE:
            self._advance()
        return stmts

    def _parse_param_list(self) -> list[Param]:
        params: list[Param] = []
        if self._peek().type == TokenType.RPAREN:
            return params
        # skip 'self' as a pseudo-param (kept for class methods)
        if self._peek().type == TokenType.KEYWORD and self._peek().value == "self":
            self._advance()
            params.append(Param(name="self"))
            if not self._match(TokenType.COMMA):
                return params
        params.append(self._parse_one_param())
        while self._match(TokenType.COMMA):
            params.append(self._parse_one_param())
        return params

    def _parse_one_param(self) -> Param:
        name = self._expect(TokenType.IDENTIFIER).value
        default = None
        if self._match(TokenType.ASSIGN):
            default = self._parse_expression()
        return Param(name=name, default=default)

    # --- expression statement / assignment ---

    def _parse_expression_statement(self) -> Any:
        expr = self._parse_expression()
        assign_ops = {
            TokenType.ASSIGN, TokenType.PLUS_ASSIGN,
            TokenType.MINUS_ASSIGN, TokenType.STAR_ASSIGN,
            TokenType.SLASH_ASSIGN,
        }
        if self._peek().type in assign_ops:
            op_tok = self._advance()
            value = self._parse_expression()
            return AssignStmt(target=expr, op=op_tok.value, value=value)
        return ExpressionStmt(expression=expr)

    # ------------------------------------------------------------------
    # Expression parsing (precedence climbing)
    # ------------------------------------------------------------------

    def _parse_expression(self) -> Any:
        return self._parse_ternary()

    def _parse_ternary(self) -> Any:
        expr = self._parse_or()
        if self._kw("when"):
            condition = self._parse_or()
            self._expect(TokenType.KEYWORD, "else")
            false_val = self._parse_ternary()
            return TernaryExpr(true_val=expr, condition=condition, false_val=false_val)
        return expr

    def _parse_or(self) -> Any:
        left = self._parse_and()
        while self._kw("or"):
            left = BinaryOp(left, "or", self._parse_and())
        return left

    def _parse_and(self) -> Any:
        left = self._parse_not()
        while self._kw("and"):
            left = BinaryOp(left, "and", self._parse_not())
        return left

    def _parse_not(self) -> Any:
        if self._kw("not"):
            return UnaryOp("not", self._parse_not())
        return self._parse_comparison()

    def _parse_comparison(self) -> Any:
        left = self._parse_range()
        _comp = {
            TokenType.EQUAL, TokenType.NOT_EQUAL,
            TokenType.LESS, TokenType.GREATER,
            TokenType.LESS_EQUAL, TokenType.GREATER_EQUAL,
        }
        while True:
            if self._peek().type in _comp:
                op = self._advance()
                left = BinaryOp(left, op.value, self._parse_range())
            elif self._peek().type == TokenType.KEYWORD and self._peek().value in ("is", "in", "has"):
                op = self._advance()
                left = BinaryOp(left, op.value, self._parse_range())
            else:
                break
        return left

    def _parse_range(self) -> Any:
        left = self._parse_addition()
        if self._match(TokenType.DOTDOT_EQUAL):
            right = self._parse_addition()
            return RangeExpr(start=left, end=right, inclusive=True)
        if self._match(TokenType.DOTDOT):
            right = self._parse_addition()
            return RangeExpr(start=left, end=right, inclusive=False)
        return left

    def _parse_addition(self) -> Any:
        left = self._parse_multiplication()
        while self._peek().type in (TokenType.PLUS, TokenType.MINUS):
            op = self._advance()
            left = BinaryOp(left, op.value, self._parse_multiplication())
        return left

    def _parse_multiplication(self) -> Any:
        left = self._parse_unary()
        while self._peek().type in (TokenType.STAR, TokenType.SLASH, TokenType.DOUBLE_SLASH, TokenType.PERCENT):
            op = self._advance()
            left = BinaryOp(left, op.value, self._parse_unary())
        return left

    def _parse_unary(self) -> Any:
        if self._peek().type in (TokenType.MINUS, TokenType.PLUS):
            op = self._advance()
            return UnaryOp(op.value, self._parse_unary())
        return self._parse_power()

    def _parse_power(self) -> Any:
        base = self._parse_call()
        if self._match(TokenType.POWER):
            return BinaryOp(base, "**", self._parse_unary())
        return base

    def _parse_call(self) -> Any:
        expr = self._parse_primary()
        while True:
            if self._match(TokenType.LPAREN):
                args: list[Any] = []
                if self._peek().type != TokenType.RPAREN:
                    args.append(self._parse_expression())
                    while self._match(TokenType.COMMA):
                        args.append(self._parse_expression())
                self._expect(TokenType.RPAREN)
                expr = CallExpr(callee=expr, args=args)
            elif self._match(TokenType.LBRACKET):
                idx = self._parse_expression()
                self._expect(TokenType.RBRACKET)
                expr = IndexExpr(obj=expr, index=idx)
            elif self._match(TokenType.DOT):
                member = self._expect(TokenType.IDENTIFIER).value
                expr = MemberExpr(obj=expr, member=member)
            elif self._match(TokenType.PIPE):
                func = self._parse_primary()
                expr = CallExpr(callee=func, args=[expr])
            else:
                break
        return expr

    def _parse_primary(self) -> Any:
        tok = self._peek()

        if tok.type == TokenType.NUMBER_INT:
            self._advance()
            return NumberLit(value=int(tok.value, 0))

        if tok.type == TokenType.NUMBER_FLOAT:
            self._advance()
            return NumberLit(value=float(tok.value))

        if tok.type == TokenType.STRING:
            self._advance()
            return StringLit(value=tok.value)

        if tok.type == TokenType.FSTRING:
            self._advance()
            return FStringLit(raw=tok.value)

        if tok.type == TokenType.BOOL_TRUE:
            self._advance()
            return BoolLit(value=True)

        if tok.type == TokenType.BOOL_FALSE:
            self._advance()
            return BoolLit(value=False)

        if tok.type == TokenType.NONE:
            self._advance()
            return NoneLit()

        if tok.type == TokenType.IDENTIFIER:
            self._advance()
            return Identifier(name=tok.value)

        # ask / ?
        if tok.type == TokenType.KEYWORD and tok.value == "ask":
            self._advance()
            return AskExpr(prompt=self._parse_expression())
        if tok.type == TokenType.QUESTION:
            self._advance()
            return AskExpr(prompt=self._parse_expression())

        # new ClassName(args)
        if tok.type == TokenType.KEYWORD and tok.value == "new":
            self._advance()
            cls_name = self._expect(TokenType.IDENTIFIER).value
            self._expect(TokenType.LPAREN)
            args: list[Any] = []
            if self._peek().type != TokenType.RPAREN:
                args.append(self._parse_expression())
                while self._match(TokenType.COMMA):
                    args.append(self._parse_expression())
            self._expect(TokenType.RPAREN)
            return CallExpr(callee=Identifier(cls_name), args=args)

        # self
        if tok.type == TokenType.KEYWORD and tok.value == "self":
            self._advance()
            return Identifier(name="self")

        # super
        if tok.type == TokenType.KEYWORD and tok.value == "super":
            self._advance()
            return Identifier(name="super")

        # typeof(x)
        if tok.type == TokenType.KEYWORD and tok.value == "typeof":
            self._advance()
            self._expect(TokenType.LPAREN)
            arg = self._parse_expression()
            self._expect(TokenType.RPAREN)
            return CallExpr(callee=Identifier("typeof"), args=[arg])

        # List literal
        if tok.type == TokenType.LBRACKET:
            return self._parse_list_literal()

        # Map literal
        if tok.type == TokenType.LBRACE:
            return self._parse_map_literal()

        # Grouped expression or lambda
        if tok.type == TokenType.LPAREN:
            return self._parse_grouped_or_lambda()

        # Wildcard _ (used in match/case)
        if tok.type == TokenType.IDENTIFIER and tok.value == "_":
            self._advance()
            return Identifier(name="_")

        raise ParseError(
            f"Unexpected token: '{tok.value}' ({tok.type.name})",
            line=tok.line, column=tok.column,
        )

    # --- composite primaries ---

    def _parse_list_literal(self) -> ListLit:
        self._advance()  # [
        elements: list[Any] = []
        self._skip_nl()
        if self._peek().type != TokenType.RBRACKET:
            elements.append(self._parse_expression())
            while self._match(TokenType.COMMA):
                self._skip_nl()
                if self._peek().type == TokenType.RBRACKET:
                    break
                elements.append(self._parse_expression())
        self._skip_nl()
        self._expect(TokenType.RBRACKET)
        return ListLit(elements=elements)

    def _parse_map_literal(self) -> MapLit:
        self._advance()  # {
        entries: list[tuple[Any, Any]] = []
        self._skip_nl()
        if self._peek().type != TokenType.RBRACE:
            entries.append(self._parse_map_entry())
            while self._match(TokenType.COMMA):
                self._skip_nl()
                if self._peek().type == TokenType.RBRACE:
                    break
                entries.append(self._parse_map_entry())
        self._skip_nl()
        self._expect(TokenType.RBRACE)
        return MapLit(entries=entries)

    def _parse_map_entry(self) -> tuple[Any, Any]:
        self._skip_nl()
        # Allow identifier as shorthand key  {name: "Alice"}
        if self._peek().type == TokenType.IDENTIFIER and self.tokens[self.pos + 1].type == TokenType.COLON:
            key = StringLit(value=self._advance().value)
        else:
            key = self._parse_expression()
        self._expect(TokenType.COLON)
        value = self._parse_expression()
        return (key, value)

    def _parse_grouped_or_lambda(self) -> Any:
        self._advance()  # (

        # () => expr
        if self._peek().type == TokenType.RPAREN:
            self._advance()
            if self._match(TokenType.ARROW):
                body = self._parse_expression()
                return LambdaExpr(params=[], body=body)
            # empty parens — treat as none? shouldn't happen, error
            raise ParseError("Empty parentheses", line=self._peek().line, column=self._peek().column)

        # Try single-expression group first; peek ahead for comma or arrow
        save = self.pos
        try:
            expr = self._parse_expression()

            # (expr) — grouped
            if self._match(TokenType.RPAREN):
                # Check for => after )
                if self._match(TokenType.ARROW):
                    if isinstance(expr, Identifier):
                        body = self._parse_expression()
                        return LambdaExpr(params=[Param(name=expr.name)], body=body)
                return expr

            # (id, id, …) => expr — lambda
            if self._peek().type == TokenType.COMMA and isinstance(expr, Identifier):
                params = [Param(name=expr.name)]
                while self._match(TokenType.COMMA):
                    p_name = self._expect(TokenType.IDENTIFIER).value
                    params.append(Param(name=p_name))
                self._expect(TokenType.RPAREN)
                self._expect(TokenType.ARROW)
                body = self._parse_expression()
                return LambdaExpr(params=params, body=body)

            # Fallback: just a grouped expression
            self._expect(TokenType.RPAREN)
            return expr

        except ParseError:
            # Restore and try harder
            self.pos = save
            raise
