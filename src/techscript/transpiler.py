"""TechScript → Python Transpiler.

Walks the AST and emits equivalent, safe Python source code.  The generated
Python can then be exec'd or written to a ``.py`` file.

Usage::

    from techscript.transpiler import transpile, transpile_and_run
    py_src = transpile(program_ast)
    transpile_and_run(program_ast)
"""

from __future__ import annotations

from typing import Any

from techscript.ast_nodes import *
from techscript.builtins import _to_str


# Preamble injected at the top of every transpiled file
_PREAMBLE = '''\
# --- TechScript transpiled output ---
import math, os, json, random, time as _time

def _ts_to_str(v):
    if v is None: return "none"
    if isinstance(v, bool): return "true" if v else "false"
    return str(v)

# Built-in aliases
to_int = int
to_float = float
to_str = _ts_to_str
to_bool = bool
to_list = list
typeof = lambda v: type(v).__name__
sqrt = math.sqrt
ceil = math.ceil
floor = math.floor
randint = random.randint
choice = random.choice
pi = math.pi
e = math.e

def ask(prompt=""):
    return input(prompt)

'''


class Transpiler:
    """Convert a ``Program`` AST into Python source code."""

    def __init__(self) -> None:
        self._indent = 0

    def transpile(self, program: Program) -> str:
        lines = [_PREAMBLE]
        for stmt in program.body:
            lines.append(self._stmt(stmt))
        return "\n".join(lines)

    # ------------------------------------------------------------------
    # indentation
    # ------------------------------------------------------------------

    def _ind(self) -> str:
        return "    " * self._indent

    def _block(self, stmts: list) -> str:
        self._indent += 1
        lines = []
        if not stmts:
            lines.append(f"{self._ind()}pass")
        for s in stmts:
            lines.append(self._stmt(s))
        self._indent -= 1
        return "\n".join(lines)

    # ------------------------------------------------------------------
    # statements
    # ------------------------------------------------------------------

    def _stmt(self, node: Any) -> str:
        ind = self._ind()

        if isinstance(node, SayStmt):
            args = ", ".join(self._expr(v) for v in node.values)
            return f"{ind}print({args})"

        if isinstance(node, SetStmt):
            return f"{ind}{node.name} = {self._expr(node.value)}"

        if isinstance(node, ConstStmt):
            return f"{ind}{node.name} = {self._expr(node.value)}  # const"

        if isinstance(node, AssignStmt):
            target = self._expr(node.target)
            return f"{ind}{target} {node.op} {self._expr(node.value)}"

        if isinstance(node, ExpressionStmt):
            return f"{ind}{self._expr(node.expression)}"

        if isinstance(node, IfStmt):
            lines = [f"{ind}if {self._expr(node.condition)}:"]
            lines.append(self._block(node.body))
            for ec, eb in node.elif_clauses:
                lines.append(f"{ind}elif {self._expr(ec)}:")
                lines.append(self._block(eb))
            if node.else_body:
                lines.append(f"{ind}else:")
                lines.append(self._block(node.else_body))
            return "\n".join(lines)

        if isinstance(node, ForStmt):
            lines = [f"{ind}for {node.var_name} in {self._expr(node.iterable)}:"]
            lines.append(self._block(node.body))
            return "\n".join(lines)

        if isinstance(node, WhileStmt):
            lines = [f"{ind}while {self._expr(node.condition)}:"]
            lines.append(self._block(node.body))
            return "\n".join(lines)

        if isinstance(node, FnStmt):
            params = ", ".join(self._param(p) for p in node.params)
            lines = [f"{ind}def {node.name}({params}):"]
            lines.append(self._block(node.body))
            return "\n".join(lines)

        if isinstance(node, ReturnStmt):
            if node.value:
                return f"{ind}return {self._expr(node.value)}"
            return f"{ind}return"

        if isinstance(node, BreakStmt):
            return f"{ind}break"

        if isinstance(node, SkipStmt):
            return f"{ind}continue"

        if isinstance(node, PassStmt):
            return f"{ind}pass"

        if isinstance(node, ClassStmt):
            parent = f"({node.parent})" if node.parent else ""
            lines = [f"{ind}class {node.name}{parent}:"]
            lines.append(self._block(node.body))
            return "\n".join(lines)

        if isinstance(node, TryStmt):
            lines = [f"{ind}try:"]
            lines.append(self._block(node.body))
            var = f" as {node.catch_var}" if node.catch_var else ""
            lines.append(f"{ind}except Exception{var}:")
            lines.append(self._block(node.catch_body))
            if node.finally_body:
                lines.append(f"{ind}finally:")
                lines.append(self._block(node.finally_body))
            return "\n".join(lines)

        if isinstance(node, ThrowStmt):
            return f"{ind}raise Exception({self._expr(node.value)})"

        if isinstance(node, MatchStmt):
            subj = self._expr(node.subject)
            lines = [f"{ind}_match_val = {subj}"]
            first = True
            for pattern, body in node.cases:
                kw = "if" if first else "elif"
                if isinstance(pattern, Identifier) and pattern.name == "_":
                    lines.append(f"{ind}else:")
                else:
                    lines.append(f"{ind}{kw} _match_val == {self._expr(pattern)}:")
                lines.append(self._block(body))
                first = False
            return "\n".join(lines)

        if isinstance(node, GuardStmt):
            lines = [f"{ind}if not ({self._expr(node.condition)}):"]
            lines.append(self._block(node.else_body))
            return "\n".join(lines)

        if isinstance(node, DelStmt):
            return f"{ind}del {node.name}"

        if isinstance(node, (ImportStmt, FromImportStmt, ExportStmt, DeferStmt, WithStmt)):
            return f"{ind}pass  # {type(node).__name__} (stub)"

        return f"{ind}pass  # unknown: {type(node).__name__}"

    # ------------------------------------------------------------------
    # expressions
    # ------------------------------------------------------------------

    def _expr(self, node: Any) -> str:
        if node is None:
            return "None"

        if isinstance(node, NumberLit):
            return repr(node.value)

        if isinstance(node, StringLit):
            return repr(node.value)

        if isinstance(node, FStringLit):
            return 'f"' + node.raw + '"'

        if isinstance(node, BoolLit):
            return "True" if node.value else "False"

        if isinstance(node, NoneLit):
            return "None"

        if isinstance(node, Identifier):
            return node.name

        if isinstance(node, ListLit):
            elems = ", ".join(self._expr(e) for e in node.elements)
            return f"[{elems}]"

        if isinstance(node, MapLit):
            entries = ", ".join(f"{self._expr(k)}: {self._expr(v)}" for k, v in node.entries)
            return "{" + entries + "}"

        if isinstance(node, BinaryOp):
            left = self._expr(node.left)
            right = self._expr(node.right)
            op = node.op
            if op == "and":
                return f"({left} and {right})"
            if op == "or":
                return f"({left} or {right})"
            return f"({left} {op} {right})"

        if isinstance(node, UnaryOp):
            if node.op == "not":
                return f"(not {self._expr(node.operand)})"
            return f"({node.op}{self._expr(node.operand)})"

        if isinstance(node, CallExpr):
            callee = self._expr(node.callee)
            args = ", ".join(self._expr(a) for a in node.args)
            return f"{callee}({args})"

        if isinstance(node, IndexExpr):
            return f"{self._expr(node.obj)}[{self._expr(node.index)}]"

        if isinstance(node, MemberExpr):
            return f"{self._expr(node.obj)}.{node.member}"

        if isinstance(node, LambdaExpr):
            params = ", ".join(p.name for p in node.params)
            return f"(lambda {params}: {self._expr(node.body)})"

        if isinstance(node, AskExpr):
            return f"input({self._expr(node.prompt)})"

        if isinstance(node, TernaryExpr):
            return f"({self._expr(node.true_val)} if {self._expr(node.condition)} else {self._expr(node.false_val)})"

        if isinstance(node, RangeExpr):
            start = self._expr(node.start)
            end = self._expr(node.end)
            if node.inclusive:
                return f"list(range({start}, {end} + 1))"
            return f"list(range({start}, {end}))"

        return repr(node)

    def _param(self, p: Param) -> str:
        if p.default is not None:
            return f"{p.name}={self._expr(p.default)}"
        return p.name


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

def transpile(program: Program) -> str:
    """Return Python source code equivalent to the given TechScript AST."""
    return Transpiler().transpile(program)


def transpile_and_run(program: Program) -> None:
    """Transpile to Python and execute it immediately."""
    py_code = transpile(program)
    exec(py_code, {"__name__": "__main__"})
