"""Quick smoke test to find which tests fail."""
import sys
from io import StringIO
from unittest.mock import patch
from techscript.lexer import Lexer
from techscript.parser import Parser
from techscript.interpreter import Interpreter

def run(src):
    tokens = Lexer(src).tokenize()
    program = Parser(tokens).parse()
    interp = Interpreter()
    buf = StringIO()
    with patch("sys.stdout", buf):
        interp.run(program)
    return buf.getvalue().strip()

tests = {
    "say_hello": ('say "Hello, World!"', "Hello, World!"),
    "set_var": ('set name = "Alice"\nsay name', "Alice"),
    "assign": ("x = 10\nsay x", "10"),
    "compound_assign": ("x = 10\nx += 5\nsay x", "15"),
    "if_true": ("if true:\n    say 1\n", "1"),
    "if_false": ("if false:\n    say 1\n", ""),
    "if_else": ("if false:\n    say 1\nelse:\n    say 2\n", "2"),
    "for_range": ("for i in 1..=3:\n    say i\n", "1\n2\n3"),
    "while": ("x = 3\nwhile x > 0:\n    say x\n    x -= 1\n", "3\n2\n1"),
    "fn": ("fn add(a, b):\n    return a + b\nsay add(3, 4)\n", "7"),
    "lambda": ("double = (x) => x * 2\nsay double(5)\n", "10"),
    "abs": ("say abs(-5)", "5"),
    "len": ('say len("hello")', "5"),
    "upper": ('say "hello".upper()', "HELLO"),
    "list_map": ("say [1, 2, 3].map((x) => x * 2)", "[2, 4, 6]"),
    "fstring": ('x = 42\nsay f"val={x}"', "val=42"),
    "range_excl": ("say 1..5", "[1, 2, 3, 4]"),
    "range_incl": ("say 1..=5", "[1, 2, 3, 4, 5]"),
    "class": (
        'class Dog:\n    fn init(self, name):\n        self.name = name\n    fn speak(self):\n        say self.name\ndog = Dog("Buddy")\ndog.speak()\n',
        "Buddy",
    ),
    "recursion": (
        "fn fact(n):\n    if n <= 1:\n        return 1\n    return n * fact(n - 1)\nsay fact(5)\n",
        "120",
    ),
}

passed = 0
failed = 0
for name, (src, expected) in tests.items():
    try:
        result = run(src)
        if result == expected:
            print(f"  [PASS] {name}")
            passed += 1
        else:
            print(f"  [FAIL] {name}: expected={expected!r}, got={result!r}")
            failed += 1
    except Exception as e:
        print(f"  [ERR]  {name}: {type(e).__name__}: {e}")
        failed += 1

print(f"\n{passed} passed, {failed} failed")
