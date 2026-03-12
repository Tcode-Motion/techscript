"""Tests for the TechScript lexer."""
import pytest
from techscript.lexer import Lexer
from techscript.tokens import TokenType
from techscript.errors import LexerError


def lex(src: str) -> list:
    return Lexer(src).tokenize()


def types(src: str) -> list[TokenType]:
    return [t.type for t in lex(src) if t.type != TokenType.EOF]


def values(src: str) -> list[str]:
    return [t.value for t in lex(src) if t.type not in (TokenType.EOF, TokenType.NEWLINE)]


# --- Numbers ---

class TestNumbers:
    def test_integer(self):
        toks = lex("42")
        assert toks[0].type == TokenType.NUMBER_INT
        assert toks[0].value == "42"

    def test_float(self):
        toks = lex("3.14")
        assert toks[0].type == TokenType.NUMBER_FLOAT
        assert toks[0].value == "3.14"

    def test_hex(self):
        toks = lex("0xFF")
        assert toks[0].type == TokenType.NUMBER_INT
        assert toks[0].value == "0xFF"

    def test_binary(self):
        toks = lex("0b1010")
        assert toks[0].type == TokenType.NUMBER_INT

    def test_underscore_separator(self):
        toks = lex("1_000_000")
        assert toks[0].value == "1000000"

    def test_scientific(self):
        toks = lex("2.5e10")
        assert toks[0].type == TokenType.NUMBER_FLOAT


# --- Strings ---

class TestStrings:
    def test_double_quote(self):
        toks = lex('"hello"')
        assert toks[0].type == TokenType.STRING
        assert toks[0].value == "hello"

    def test_single_quote(self):
        toks = lex("'world'")
        assert toks[0].type == TokenType.STRING
        assert toks[0].value == "world"

    def test_escape_newline(self):
        toks = lex(r'"line1\nline2"')
        assert "\n" in toks[0].value

    def test_fstring(self):
        toks = lex('f"hello {name}"')
        assert toks[0].type == TokenType.FSTRING
        assert "name" in toks[0].value

    def test_unterminated_string(self):
        with pytest.raises(LexerError, match="Unterminated"):
            lex('"hello')


# --- Keywords & Identifiers ---

class TestKeywords:
    def test_keyword(self):
        toks = lex("say")
        assert toks[0].type == TokenType.KEYWORD
        assert toks[0].value == "say"

    def test_identifier(self):
        toks = lex("my_var")
        assert toks[0].type == TokenType.IDENTIFIER

    def test_true_false_none(self):
        t = types("true false none")
        assert TokenType.BOOL_TRUE in t
        assert TokenType.BOOL_FALSE in t
        assert TokenType.NONE in t


# --- Operators ---

class TestOperators:
    def test_arithmetic(self):
        t = types("+ - * / // % **")
        assert TokenType.PLUS in t
        assert TokenType.POWER in t
        assert TokenType.DOUBLE_SLASH in t

    def test_comparison(self):
        t = types("== != < > <= >=")
        assert TokenType.EQUAL in t
        assert TokenType.LESS_EQUAL in t

    def test_assignment(self):
        t = types("+= -= *= /=")
        assert TokenType.PLUS_ASSIGN in t

    def test_arrow(self):
        t = types("=>")
        assert TokenType.ARROW in t

    def test_pipe(self):
        t = types("|>")
        assert TokenType.PIPE in t

    def test_range(self):
        t = types("..")
        assert TokenType.DOTDOT in t

    def test_range_inclusive(self):
        t = types("..=")
        assert TokenType.DOTDOT_EQUAL in t

    def test_spread(self):
        t = types("...")
        assert TokenType.SPREAD in t


# --- Comments ---

class TestComments:
    def test_single_line(self):
        toks = lex("42 # comment\n")
        vals = [t for t in toks if t.type == TokenType.NUMBER_INT]
        assert len(vals) == 1

    def test_block_comment(self):
        toks = lex("#[ block ]# 42")
        vals = [t for t in toks if t.type == TokenType.NUMBER_INT]
        assert len(vals) == 1
