"""TechScript REPL — interactive read-eval-print loop."""

from __future__ import annotations

from techscript.lexer import Lexer
from techscript.parser import Parser
from techscript.interpreter import Interpreter
from techscript.errors import TechScriptError, LexerError, ParseError, format_error


BANNER = """\
╭──────────────────────────────────────╮
│  TechScript v1.0.0 Interactive REPL  │
│  Type 'help' for commands, 'exit'    │
│  to quit.                            │
╰──────────────────────────────────────╯
"""

HELP_TEXT = """\
REPL Commands:
  help          Show this help
  exit          Exit the REPL

Quick Reference:
  say "Hello"              Print output
  name = ask "Name? "      Read input
  x = 42                   Set variable
  if x > 0: ...            Conditional
  for i in 1..5: ...       Loop
  fn add(a, b): return a+b Define function"""


def start_repl() -> None:
    """Launch the interactive TechScript REPL."""
    print(BANNER)

    interp = Interpreter()
    buffer = ""

    while True:
        try:
            prompt = "... " if buffer else ">>> "
            line = input(prompt)
        except (EOFError, KeyboardInterrupt):
            print("\nGoodbye! 👋")
            break

        stripped = line.strip()

        # Meta commands (only at top level)
        if not buffer:
            if stripped in ("exit", "quit"):
                print("Goodbye! 👋")
                break
            if stripped == "help":
                print(HELP_TEXT)
                continue

        buffer += line + "\n"

        # If line ends with ':', expect more input
        if stripped.endswith(":"):
            continue

        # In multi-line mode, blank line finishes the block
        if buffer.count("\n") > 1 and not stripped:
            pass  # fall through to execute
        elif buffer.count("\n") > 1 and stripped:
            continue

        source = buffer.strip()
        buffer = ""

        if not source:
            continue

        try:
            tokens = Lexer(source, "<repl>").tokenize()
            program = Parser(tokens).parse()
            interp.run(program)
        except (LexerError, ParseError) as e:
            print(format_error(e))
        except TechScriptError as e:
            print(format_error(e))
        except Exception as e:
            print(f"[internal] {e}")
