"""TechScript Abstract Syntax Tree node definitions.

Every node is a frozen-ish ``@dataclass``.  Statements and expressions are
plain classes — no common base is needed because we pattern-match on type.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any


# ===================================================================
# Programme root
# ===================================================================

@dataclass
class Program:
    body: list[Any]


@dataclass
class Param:
    name: str
    default: Any = None


# ===================================================================
# Statements
# ===================================================================

@dataclass
class SayStmt:
    """``say <expr>, …``"""
    values: list[Any]

@dataclass
class SetStmt:
    """``set <name> = <expr>``"""
    name: str
    value: Any

@dataclass
class AssignStmt:
    """``<target> = | += | -= | *= | /= <expr>``"""
    target: Any
    op: str
    value: Any

@dataclass
class ExpressionStmt:
    """A bare expression used as a statement (e.g. function call)."""
    expression: Any

@dataclass
class IfStmt:
    condition: Any
    body: list[Any]
    elif_clauses: list[tuple[Any, list[Any]]] = field(default_factory=list)
    else_body: list[Any] | None = None

@dataclass
class ForStmt:
    var_name: str
    iterable: Any
    body: list[Any]

@dataclass
class WhileStmt:
    condition: Any
    body: list[Any]

@dataclass
class FnStmt:
    name: str
    params: list[Param]
    body: list[Any]

@dataclass
class ClassStmt:
    name: str
    parent: str | None = None
    body: list[Any] = field(default_factory=list)

@dataclass
class ReturnStmt:
    value: Any = None

@dataclass
class BreakStmt:
    pass

@dataclass
class SkipStmt:
    pass

@dataclass
class PassStmt:
    pass

@dataclass
class TryStmt:
    body: list[Any]
    catch_var: str | None = None
    catch_body: list[Any] = field(default_factory=list)
    finally_body: list[Any] | None = None

@dataclass
class ThrowStmt:
    value: Any

@dataclass
class MatchStmt:
    subject: Any
    cases: list[tuple[Any, list[Any]]] = field(default_factory=list)

@dataclass
class ImportStmt:
    module: str
    names: list[str] | None = None
    alias: str | None = None

@dataclass
class FromImportStmt:
    module: str
    names: list[str]

@dataclass
class DelStmt:
    name: str

@dataclass
class DeferStmt:
    expression: Any

@dataclass
class GuardStmt:
    condition: Any
    else_body: list[Any]

@dataclass
class WithStmt:
    expression: Any
    var_name: str
    body: list[Any]

@dataclass
class ConstStmt:
    name: str
    value: Any

@dataclass
class ExportStmt:
    declaration: Any


# ===================================================================
# Expressions
# ===================================================================

@dataclass
class NumberLit:
    value: int | float

@dataclass
class StringLit:
    value: str

@dataclass
class FStringLit:
    """Stored as raw template string; the interpreter handles ``{…}`` parts."""
    raw: str

@dataclass
class BoolLit:
    value: bool

@dataclass
class NoneLit:
    pass

@dataclass
class ListLit:
    elements: list[Any]

@dataclass
class MapLit:
    entries: list[tuple[Any, Any]]

@dataclass
class Identifier:
    name: str

@dataclass
class BinaryOp:
    left: Any
    op: str
    right: Any

@dataclass
class UnaryOp:
    op: str
    operand: Any

@dataclass
class CallExpr:
    callee: Any
    args: list[Any]

@dataclass
class IndexExpr:
    obj: Any
    index: Any

@dataclass
class MemberExpr:
    obj: Any
    member: str

@dataclass
class LambdaExpr:
    params: list[Param]
    body: Any           # single expression

@dataclass
class AskExpr:
    prompt: Any

@dataclass
class TernaryExpr:
    true_val: Any
    condition: Any
    false_val: Any

@dataclass
class RangeExpr:
    start: Any
    end: Any
    inclusive: bool = False
