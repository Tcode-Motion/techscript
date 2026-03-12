"""Tests for the TechScript → Python transpiler."""
from techscript.lexer import Lexer
from techscript.parser import Parser
from techscript.transpiler import transpile


def tx(src: str) -> str:
    tokens = Lexer(src).tokenize()
    program = Parser(tokens).parse()
    return transpile(program)


class TestTranspile:
    def test_say(self):
        py = tx('say "hello"')
        assert 'print("hello")' in py or "print('hello')" in py

    def test_variable(self):
        py = tx("x = 42")
        assert "x = 42" in py

    def test_if(self):
        py = tx("when true {\n    say 1\n}\n")
        assert "if True:" in py

    def test_for(self):
        py = tx("each i in items {\n    say i\n}\n")
        assert "for i in items:" in py

    def test_fn(self):
        py = tx("build add(a, b) {\n    send a + b\n}\n")
        assert "def add(a, b):" in py

    def test_class(self):
        py = tx("model Dog {\n    pass\n}\n")
        assert "class Dog:" in py

    def test_lambda(self):
        py = tx("f = (x) => x * 2")
        assert "lambda x:" in py

    def test_range_inclusive(self):
        py = tx("say 1..=5")
        assert "range(1, 5 + 1)" in py

    def test_preamble(self):
        py = tx('say "hi"')
        assert "TechScript transpiled output" in py
        assert "import math" in py

    def test_match(self):
        py = tx('match x {\n    case "a" {\n        say 1\n    }\n}\n')
        assert "_match_val" in py

    def test_try_catch(self):
        py = tx("attempt {\n    say 1\n} rescue err {\n    say 2\n}\n")
        assert "try:" in py
        assert "except" in py
