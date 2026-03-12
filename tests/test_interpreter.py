"""End-to-end tests for the TechScript interpreter."""
import pytest
from io import StringIO
from unittest.mock import patch

from techscript.lexer import Lexer
from techscript.parser import Parser
from techscript.interpreter import Interpreter
from techscript.errors import TechScriptError


def run(src: str) -> str:
    """Run TechScript source and capture stdout."""
    tokens = Lexer(src).tokenize()
    program = Parser(tokens).parse()
    interp = Interpreter()
    buf = StringIO()
    with patch("sys.stdout", buf):
        interp.run(program)
    return buf.getvalue().strip()


class TestSay:
    def test_hello(self):
        assert run('say "Hello, World!"') == "Hello, World!"

    def test_number(self):
        assert run("say 42") == "42"

    def test_multiple_values(self):
        assert run('say "a", "b", "c"') == "a b c"

    def test_expression(self):
        assert run("say 2 + 3") == "5"


class TestVariables:
    def test_set(self):
        assert run('make name = "Alice"\nsay name') == "Alice"

    def test_assign(self):
        assert run("x = 10\nsay x") == "10"

    def test_compound_assign(self):
        assert run("x = 10\nx += 5\nsay x") == "15"

    def test_const(self):
        with pytest.raises(TechScriptError, match="constant"):
            run("keep X = 5\nX = 10")


class TestControlFlow:
    def test_if_true(self):
        assert run("when true {\n    say 1\n}\n") == "1"

    def test_if_false(self):
        assert run("when false {\n    say 1\n}\n") == ""

    def test_if_else(self):
        assert run("when false {\n    say 1\n} else {\n    say 2\n}\n") == "2"

    def test_if_elif(self):
        src = "x = 5\nwhen x > 10 {\n    say 1\n} alt x > 3 {\n    say 2\n} else {\n    say 3\n}\n"
        assert run(src) == "2"

    def test_for_range(self):
        out = run("each i in 1..=3 {\n    say i\n}\n")
        assert out == "1\n2\n3"

    def test_for_list(self):
        out = run('each x in ["a", "b"] {\n    say x\n}\n')
        assert out == "a\nb"

    def test_while(self):
        src = "x = 3\nrepeat x > 0 {\n    say x\n    x -= 1\n}\n"
        assert run(src) == "3\n2\n1"

    def test_break(self):
        src = "each i in 1..=10 {\n    when i > 3 {\n        stop\n    }\n    say i\n}\n"
        assert run(src) == "1\n2\n3"

    def test_skip(self):
        src = "each i in 1..=5 {\n    when i == 3 {\n        skip\n    }\n    say i\n}\n"
        assert run(src) == "1\n2\n4\n5"

    def test_match(self):
        src = 'x = "b"\nmatch x {\n    case "a" {\n        say 1\n    }\n    case "b" {\n        say 2\n    }\n}\n'
        assert run(src) == "2"


class TestFunctions:
    def test_fn_call(self):
        src = "build add(a, b) {\n    send a + b\n}\nsay add(3, 4)\n"
        assert run(src) == "7"

    def test_default_param(self):
        src = 'build greet(name, g = "Hi") {\n    say g\n    say name\n}\ngreet("A")\n'
        assert run(src) == "Hi\nA"

    def test_lambda(self):
        src = "double = (x) => x * 2\nsay double(5)\n"
        assert run(src) == "10"

    def test_closure(self):
        src = (
            "build create() {\n"
            "    n = 0\n"
            "    build inc() {\n"
            "        n += 1\n"
            "        send n\n"
            "    }\n"
            "    send inc\n"
            "}\n"
            "c = create()\n"
            "say c()\n"
            "say c()\n"
        )
        # Closures capture the environment by reference
        assert run(src) == "1\n2"

    def test_recursion(self):
        src = (
            "build fact(n) {\n"
            "    when n <= 1 {\n"
            "        send 1\n"
            "    }\n"
            "    send n * fact(n - 1)\n"
            "}\n"
            "say fact(5)\n"
        )
        assert run(src) == "120"


class TestClasses:
    def test_class_basic(self):
        src = (
            "model Dog {\n"
            "    build init(self, name) {\n"
            "        self.name = name\n"
            "    }\n"
            "    build speak(self) {\n"
            "        say self.name\n"
            "    }\n"
            "}\n"
            'dog = Dog("Buddy")\n'
            "dog.speak()\n"
        )
        assert run(src) == "Buddy"

    def test_inheritance(self):
        src = (
            "model Animal {\n"
            "    build init(self, name) {\n"
            "        self.name = name\n"
            "    }\n"
            "    build speak(self) {\n"
            "        say self.name\n"
            "    }\n"
            "}\n"
            "model Cat(Animal) {\n"
            "    build purr(self) {\n"
            '        say "purr"\n'
            "    }\n"
            "}\n"
            'c = Cat("Kitty")\n'
            "c.speak()\n"
            "c.purr()\n"
        )
        assert run(src) == "Kitty\npurr"


class TestBuiltins:
    def test_abs(self):
        assert run("say abs(-5)") == "5"

    def test_len(self):
        assert run('say len("hello")') == "5"

    def test_typeof(self):
        assert run("say typeof(42)") == "int"

    def test_to_int(self):
        assert run('say to_int("42")') == "42"

    def test_range(self):
        assert run("say range(1, 4)") == "[1, 2, 3]"

    def test_sqrt(self):
        assert run("say sqrt(16)") == "4.0"

    def test_is_even(self):
        assert run("say is_even(4)") == "true"


class TestStringMethods:
    def test_upper(self):
        assert run('say "hello".upper()') == "HELLO"

    def test_split(self):
        assert run('say "a,b".split(",")') == "['a', 'b']"

    def test_length(self):
        assert run('say "hi".length') == "2"

    def test_contains(self):
        assert run('say "hello".contains("ell")') == "true"


class TestListMethods:
    def test_length(self):
        assert run("say [1, 2, 3].length") == "3"

    def test_map(self):
        assert run("say [1, 2, 3].map((x) => x * 2)") == "[2, 4, 6]"

    def test_filter(self):
        assert run("say [1, 2, 3, 4].filter((x) => x > 2)") == "[3, 4]"


class TestErrorHandling:
    def test_try_catch(self):
        src = (
            "attempt {\n"
            '    fail "oops"\n'
            "} rescue err {\n"
            '    say "caught"\n'
            "}\n"
        )
        assert run(src) == "caught"

    def test_undefined_var(self):
        with pytest.raises(TechScriptError, match="Undefined"):
            run("say undefined_variable")


class TestFString:
    def test_basic(self):
        assert run('x = 42\nsay f"val={x}"') == "val=42"

    def test_expression(self):
        assert run('say f"sum={2 + 3}"') == "sum=5"


class TestRange:
    def test_exclusive(self):
        assert run("say 1..5") == "[1, 2, 3, 4]"

    def test_inclusive(self):
        assert run("say 1..=5") == "[1, 2, 3, 4, 5]"
