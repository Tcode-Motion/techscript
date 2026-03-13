"""TechScript CLI — the ``tech`` command.

Subcommands::

    tech run <file.txs>                Interpret a .txs file
    tech transpile <file.txs>          Transpile to Python & run
    tech --mode transpile <file.txs>   Alias for transpile
    tech check <file.txs>              Syntax-check only
    tech repl                          Start interactive REPL
    tech version                       Print version
"""

from __future__ import annotations

import argparse
import os
import sys

from techscript import __version__
from techscript.lexer import Lexer
from techscript.parser import Parser
from techscript.interpreter import Interpreter
from techscript.transpiler import transpile, transpile_and_run
from techscript.repl import start_repl
from techscript.errors import TechScriptError, LexerError, ParseError, format_error


def _read_source(filepath: str) -> str:
    if not os.path.exists(filepath):
        print(f"Error: File not found: {filepath}", file=sys.stderr)
        sys.exit(1)
    with open(filepath, encoding="utf-8") as f:
        return f.read()


def _parse_source(source: str, filepath: str = "<stdin>"):
    """Lex + parse, returning (program, source_lines)."""
    tokens = Lexer(source, filepath).tokenize()
    program = Parser(tokens).parse()
    return program, source.splitlines()


# -----------------------------------------------------------------------
# Sub-command handlers
# -----------------------------------------------------------------------

def cmd_run(args: argparse.Namespace) -> None:
    source = _read_source(args.file)
    try:
        program, src_lines = _parse_source(source, args.file)
        if args.debug:
            from techscript.lexer import Lexer as L
            for t in L(source, args.file).tokenize():
                print(f"  {t}")
            print("---")
        interp = Interpreter()
        interp.run(program)
    except (LexerError, ParseError, TechScriptError) as e:
        print(format_error(e, source.splitlines()), file=sys.stderr)
        sys.exit(1)
    except KeyboardInterrupt:
        print("\nInterrupted.", file=sys.stderr)
        sys.exit(130)


def cmd_transpile(args: argparse.Namespace) -> None:
    source = _read_source(args.file)
    try:
        program, _ = _parse_source(source, args.file)
        if args.output:
            py_code = transpile(program)
            with open(args.output, "w", encoding="utf-8") as f:
                f.write(py_code)
            print(f"Transpiled to: {args.output}")
        else:
            transpile_and_run(program)
    except (LexerError, ParseError, TechScriptError) as e:
        print(format_error(e, source.splitlines()), file=sys.stderr)
        sys.exit(1)


def cmd_check(args: argparse.Namespace) -> None:
    source = _read_source(args.file)
    try:
        _parse_source(source, args.file)
        print(f"✓ {args.file}: No syntax errors found.")
    except (LexerError, ParseError) as e:
        print(f"✗ {args.file}: {e}", file=sys.stderr)
        sys.exit(1)


def cmd_repl(_args: argparse.Namespace) -> None:
    start_repl()


def cmd_version(_args: argparse.Namespace) -> None:
    print(f"TechScript v{__version__}")


# -----------------------------------------------------------------------
# Argument parser
# -----------------------------------------------------------------------

def _build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="tech",
        description="TechScript — a simple, friendly programming language (.txs)",
    )
    p.add_argument("--version", "-V", action="store_true", help="Show version")
    p.add_argument("--mode", choices=["run", "transpile"], default=None,
                   help="Execution mode (alternative to subcommands)")

    sub = p.add_subparsers(dest="command")

    # run
    r = sub.add_parser("run", help="Run a .txs file (interpreter mode)")
    r.add_argument("file", help="Path to .txs file")
    r.add_argument("--debug", action="store_true", help="Show debug info")

    # transpile
    t = sub.add_parser("transpile", help="Transpile .txs to Python and run")
    t.add_argument("file", help="Path to .txs file")
    t.add_argument("-o", "--output", help="Write Python to file instead of running")

    # check
    c = sub.add_parser("check", help="Check syntax without running")
    c.add_argument("file", help="Path to .txs file")

    # repl
    sub.add_parser("repl", help="Start interactive REPL")

    # version
    sub.add_parser("version", help="Show version")

    return p


def main() -> None:
    argv = sys.argv[1:]

    # Quick intercepts BEFORE argparse
    if not argv or argv[0] in ("help", "--help", "-h"):
        _build_parser().print_help()
        return

    if argv[0] in ("--version", "-V"):
        print(f"TechScript v{__version__}")
        return

    # --mode <mode> <file> shorthand (bypass argparse to avoid subparser conflict)
    if argv[0] == "--mode" and len(argv) >= 3:
        mode = argv[1]
        filepath = argv[2]
        ns = argparse.Namespace(file=filepath, debug=False, output=None)
        if mode == "run":
            cmd_run(ns)
        elif mode == "transpile":
            cmd_transpile(ns)
        else:
            print(f"Unknown mode: {mode}. Use 'run' or 'transpile'.", file=sys.stderr)
            sys.exit(1)
        return

    # Bare file argument: `tech hello.txs`
    if argv[0].endswith((".txs", ".tx")) and os.path.isfile(argv[0]):
        ns = argparse.Namespace(file=argv[0], debug="--debug" in argv)
        cmd_run(ns)
        return

    # Standard argparse subcommand dispatch
    parser = _build_parser()
    args = parser.parse_args()

    dispatch = {
        "run": cmd_run,
        "transpile": cmd_transpile,
        "check": cmd_check,
        "repl": cmd_repl,
        "version": cmd_version,
    }

    if args.command in dispatch:
        dispatch[args.command](args)
        return

    parser.print_help()


if __name__ == "__main__":
    main()
