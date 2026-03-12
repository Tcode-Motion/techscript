"""Tests for the TechScript parser."""
import pytest
from techscript.lexer import Lexer
from techscript.parser import Parser
from techscript.ast_nodes import *


def parse(src: str) -> Program:
    tokens = Lexer(src).tokenize()
    return Parser(tokens).parse()


def first_stmt(src: str):
    return parse(src).body[0]


class TestSimpleStatements:
    def test_say(self):
        stmt = first_stmt('say "hello"')
        assert isinstance(stmt, SayStmt)
        assert isinstance(stmt.values[0], StringLit)

    def test_say_multiple(self):
        stmt = first_stmt('say "a", "b"')
        assert len(stmt.values) == 2

    def test_set(self):
        stmt = first_stmt("make x = 42")
        assert isinstance(stmt, SetStmt)
        assert stmt.name == "x"
        assert isinstance(stmt.value, NumberLit)

    def test_assign(self):
        stmt = first_stmt("x = 10")
        assert isinstance(stmt, AssignStmt)

    def test_const(self):
        stmt = first_stmt("keep PI = 3.14")
        assert isinstance(stmt, ConstStmt)
        assert stmt.name == "PI"

    def test_return(self):
        stmt = first_stmt("send 42")
        assert isinstance(stmt, ReturnStmt)

    def test_break(self):
        assert isinstance(first_stmt("stop"), BreakStmt)

    def test_skip(self):
        assert isinstance(first_stmt("skip"), SkipStmt)

    def test_pass(self):
        assert isinstance(first_stmt("pass"), PassStmt)

    def test_del(self):
        stmt = first_stmt("drop x")
        assert isinstance(stmt, DelStmt)
        assert stmt.name == "x"

    def test_throw(self):
        stmt = first_stmt('fail "error"')
        assert isinstance(stmt, ThrowStmt)


class TestCompoundStatements:
    def test_if(self):
        stmt = first_stmt("when true { say 1 }")
        assert isinstance(stmt, IfStmt)
        assert len(stmt.body) == 1

    def test_if_else(self):
        src = "when true { say 1 } else { say 2 }"
        stmt = first_stmt(src)
        assert isinstance(stmt, IfStmt)
        assert stmt.else_body is not None

    def test_for(self):
        src = "each i in items { say i }"
        stmt = first_stmt(src)
        assert isinstance(stmt, ForStmt)
        assert stmt.var_name == "i"

    def test_while(self):
        src = "repeat true { say 1 }"
        stmt = first_stmt(src)
        assert isinstance(stmt, WhileStmt)

    def test_fn(self):
        src = "build add(a, b) { send a }"
        stmt = first_stmt(src)
        assert isinstance(stmt, FnStmt)
        assert stmt.name == "add"
        assert len(stmt.params) == 2

    def test_class(self):
        src = "model Dog { pass }"
        stmt = first_stmt(src)
        assert isinstance(stmt, ClassStmt)
        assert stmt.name == "Dog"

    def test_try_catch(self):
        src = "attempt { say 1 } rescue err { say 2 }"
        stmt = first_stmt(src)
        assert isinstance(stmt, TryStmt)
        assert stmt.catch_var == "err"

    def test_match(self):
        src = 'match x { case "a" { say 1 } }'
        stmt = first_stmt(src)
        assert isinstance(stmt, MatchStmt)


class TestExpressions:
    def test_number(self):
        stmt = first_stmt("42")
        assert isinstance(stmt, ExpressionStmt)
        assert isinstance(stmt.expression, NumberLit)
        assert stmt.expression.value == 42

    def test_string(self):
        stmt = first_stmt('"hello"')
        assert stmt.expression.value == "hello"

    def test_binary_op(self):
        stmt = first_stmt("1 + 2")
        assert isinstance(stmt.expression, BinaryOp)
        assert stmt.expression.op == "+"

    def test_unary_minus(self):
        stmt = first_stmt("-5")
        assert isinstance(stmt.expression, UnaryOp)

    def test_call(self):
        stmt = first_stmt("foo(1, 2)")
        assert isinstance(stmt.expression, CallExpr)

    def test_member_access(self):
        stmt = first_stmt("obj.name")
        assert isinstance(stmt.expression, MemberExpr)

    def test_index(self):
        stmt = first_stmt("arr[0]")
        assert isinstance(stmt.expression, IndexExpr)

    def test_list_literal(self):
        stmt = first_stmt("[1, 2, 3]")
        assert isinstance(stmt.expression, ListLit)
        assert len(stmt.expression.elements) == 3

    def test_map_literal(self):
        stmt = first_stmt('{name: "Alice"}')
        assert isinstance(stmt.expression, MapLit)

    def test_lambda(self):
        stmt = first_stmt("(x) => x")
        assert isinstance(stmt.expression, LambdaExpr)

    def test_range_exclusive(self):
        stmt = first_stmt("1..10")
        assert isinstance(stmt.expression, RangeExpr)
        assert stmt.expression.inclusive is False

    def test_range_inclusive(self):
        stmt = first_stmt("1..=10")
        assert isinstance(stmt.expression, RangeExpr)
        assert stmt.expression.inclusive is True

    def test_ask(self):
        stmt = first_stmt('ask "name? "')
        assert isinstance(stmt.expression, AskExpr)

    def test_precedence(self):
        # 2 + 3 * 4 should parse as 2 + (3 * 4)
        stmt = first_stmt("2 + 3 * 4")
        expr = stmt.expression
        assert isinstance(expr, BinaryOp)
        assert expr.op == "+"
        assert isinstance(expr.right, BinaryOp)
        assert expr.right.op == "*"

    def test_pipe(self):
        stmt = first_stmt('"hi" |> upper')
        expr = stmt.expression
        assert isinstance(expr, CallExpr)
