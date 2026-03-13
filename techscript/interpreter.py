"""TechScript Interpreter — AST-walking evaluator.

This is the execution engine.  It walks the AST produced by ``parser.py``
and evaluates every node, maintaining runtime state via ``Environment``
scopes and using Python-native values internally.
"""

from __future__ import annotations

import re
from typing import Any

from techscript.ast_nodes import *
from techscript.environment import Environment
from techscript.builtins import register_builtins, _to_str
from techscript.errors import (
    TechScriptError, NameErr, TypeErr, ValueErr, RuntimeErr,
    format_error, suggest_correction,
)
from techscript.tokens import KEYWORDS


# ===================================================================
# Control-flow signals (raised as exceptions, caught by loops / fns)
# ===================================================================

class _ReturnSignal(Exception):
    __slots__ = ("value",)
    def __init__(self, value: Any = None):
        self.value = value

class _BreakSignal(Exception):
    pass

class _SkipSignal(Exception):
    pass


# ===================================================================
# Runtime value types
# ===================================================================

class TechFunction:
    """A user-defined function (closure)."""
    __slots__ = ("name", "params", "body", "closure")

    def __init__(self, name: str, params: list[Param], body: list, closure: Environment):
        self.name = name
        self.params = params
        self.body = body
        self.closure = closure

    def __repr__(self) -> str:
        return f"<fn {self.name}>"


class TechClass:
    """A user-defined class."""
    __slots__ = ("name", "methods", "parent")

    def __init__(self, name: str, methods: dict[str, TechFunction], parent: TechClass | None = None):
        self.name = name
        self.methods = methods
        self.parent = parent

    def find_method(self, name: str) -> TechFunction | None:
        if name in self.methods:
            return self.methods[name]
        if self.parent:
            return self.parent.find_method(name)
        return None

    def __repr__(self) -> str:
        return f"<class {self.name}>"


class TechInstance:
    """An instance of ``TechClass``."""
    __slots__ = ("klass", "fields")

    def __init__(self, klass: TechClass):
        self.klass = klass
        self.fields: dict[str, Any] = {}

    def get(self, name: str) -> Any:
        if name in self.fields:
            return self.fields[name]
        method = self.klass.find_method(name)
        if method:
            return self._bind(method)
        raise NameErr(f"'{self.klass.name}' has no attribute '{name}'")

    def _bind(self, method: TechFunction) -> TechFunction:
        env = Environment(parent=method.closure)
        env.set("self", self)
        # Remove 'self' from params for the caller
        params = [p for p in method.params if p.name != "self"]
        return TechFunction(method.name, params, method.body, env)

    def __repr__(self) -> str:
        return f"<{self.klass.name} instance>"


# ===================================================================
# Interpreter
# ===================================================================

class Interpreter:
    """Walk an AST ``Program`` and execute it."""

    def __init__(self) -> None:
        self.global_env = Environment()
        register_builtins(self.global_env)

    def run(self, program: Program) -> None:
        self._exec_block(program.body, self.global_env)

    # ------------------------------------------------------------------
    # execute statements
    # ------------------------------------------------------------------

    def _exec_block(self, stmts: list, env: Environment) -> None:
        for stmt in stmts:
            self._exec(stmt, env)

    def _exec(self, node: Any, env: Environment) -> None:  # noqa: C901 — complexity is inherent
        if isinstance(node, SayStmt):
            parts = [_to_str(self._eval(v, env)) for v in node.values]
            print(" ".join(parts))

        elif isinstance(node, SetStmt):
            env.set(node.name, self._eval(node.value, env))

        elif isinstance(node, ConstStmt):
            env.set_const(node.name, self._eval(node.value, env))

        elif isinstance(node, AssignStmt):
            val = self._eval(node.value, env)
            self._do_assign(node.target, node.op, val, env)

        elif isinstance(node, ExpressionStmt):
            self._eval(node.expression, env)

        elif isinstance(node, IfStmt):
            if self._truthy(self._eval(node.condition, env)):
                self._exec_block(node.body, Environment(parent=env))
            else:
                done = False
                for ec, eb in node.elif_clauses:
                    if self._truthy(self._eval(ec, env)):
                        self._exec_block(eb, Environment(parent=env))
                        done = True
                        break
                if not done and node.else_body:
                    self._exec_block(node.else_body, Environment(parent=env))

        elif isinstance(node, ForStmt):
            items = self._eval(node.iterable, env)
            if not hasattr(items, "__iter__"):
                raise TypeErr(f"Cannot iterate over {_to_str(items)}")
            for item in items:
                loop_env = Environment(parent=env)
                loop_env.set(node.var_name, item)
                try:
                    self._exec_block(node.body, loop_env)
                except _BreakSignal:
                    break
                except _SkipSignal:
                    continue

        elif isinstance(node, WhileStmt):
            while self._truthy(self._eval(node.condition, env)):
                try:
                    self._exec_block(node.body, Environment(parent=env))
                except _BreakSignal:
                    break
                except _SkipSignal:
                    continue

        elif isinstance(node, FnStmt):
            fn = TechFunction(node.name, node.params, node.body, env)
            env.set(node.name, fn)

        elif isinstance(node, ClassStmt):
            parent = None
            if node.parent:
                parent = env.get(node.parent)
                if not isinstance(parent, TechClass):
                    raise TypeErr(f"'{node.parent}' is not a class")
            methods: dict[str, TechFunction] = {}
            for member in node.body:
                if isinstance(member, FnStmt):
                    methods[member.name] = TechFunction(member.name, member.params, member.body, env)
            klass = TechClass(node.name, methods, parent)
            env.set(node.name, klass)

        elif isinstance(node, ReturnStmt):
            raise _ReturnSignal(self._eval(node.value, env) if node.value else None)

        elif isinstance(node, BreakStmt):
            raise _BreakSignal()

        elif isinstance(node, SkipStmt):
            raise _SkipSignal()

        elif isinstance(node, PassStmt):
            pass

        elif isinstance(node, TryStmt):
            try:
                self._exec_block(node.body, Environment(parent=env))
            except TechScriptError as e:
                catch_env = Environment(parent=env)
                if node.catch_var:
                    catch_env.set(node.catch_var, {"message": str(e), "type": type(e).__name__})
                self._exec_block(node.catch_body, catch_env)
            finally:
                if node.finally_body:
                    self._exec_block(node.finally_body, Environment(parent=env))

        elif isinstance(node, ThrowStmt):
            val = self._eval(node.value, env)
            if isinstance(val, TechScriptError):
                raise val
            raise TechScriptError(str(val))

        elif isinstance(node, MatchStmt):
            val = self._eval(node.subject, env)
            for pattern, case_body in node.cases:
                if isinstance(pattern, Identifier) and pattern.name == "_":
                    self._exec_block(case_body, Environment(parent=env))
                    break
                pat_val = self._eval(pattern, env)
                if val == pat_val:
                    self._exec_block(case_body, Environment(parent=env))
                    break

        elif isinstance(node, GuardStmt):
            if not self._truthy(self._eval(node.condition, env)):
                self._exec_block(node.else_body, Environment(parent=env))

        elif isinstance(node, DelStmt):
            env.delete(node.name)

        elif isinstance(node, (ImportStmt, FromImportStmt)):
            if node.module == "web":
                from techscript.web import WebPageNative
                env.set("WebPage", WebPageNative)
            else:
                pass  # Module system stub for others

        elif isinstance(node, ExportStmt):
            self._exec(node.declaration, env)

        elif isinstance(node, DeferStmt):
            pass  # Defer stub

        elif isinstance(node, WithStmt):
            pass  # Context manager stub

        else:
            raise RuntimeErr(f"Unknown statement: {type(node).__name__}")

    # ------------------------------------------------------------------
    # assignment helper
    # ------------------------------------------------------------------

    def _do_assign(self, target: Any, op: str, val: Any, env: Environment) -> None:
        if isinstance(target, Identifier):
            if op == "=":
                env.update(target.name, val)
            else:
                cur = env.get(target.name)
                _ops = {"+=": lambda a, b: a + b, "-=": lambda a, b: a - b,
                        "*=": lambda a, b: a * b, "/=": lambda a, b: a / b}
                env.update(target.name, _ops[op](cur, val))
        elif isinstance(target, IndexExpr):
            obj = self._eval(target.obj, env)
            idx = self._eval(target.index, env)
            if op == "=":
                obj[idx] = val
            else:
                cur = obj[idx]
                _ops = {"+=": lambda a, b: a + b, "-=": lambda a, b: a - b,
                        "*=": lambda a, b: a * b, "/=": lambda a, b: a / b}
                obj[idx] = _ops[op](cur, val)
        elif isinstance(target, MemberExpr):
            obj = self._eval(target.obj, env)
            if isinstance(obj, TechInstance):
                obj.fields[target.member] = val
            elif isinstance(obj, dict):
                obj[target.member] = val

    # ------------------------------------------------------------------
    # evaluate expressions
    # ------------------------------------------------------------------

    def _eval(self, node: Any, env: Environment) -> Any:  # noqa: C901
        if node is None:
            return None

        if isinstance(node, NumberLit):
            return node.value

        if isinstance(node, StringLit):
            return node.value

        if isinstance(node, FStringLit):
            return self._eval_fstring(node.raw, env)

        if isinstance(node, BoolLit):
            return node.value

        if isinstance(node, NoneLit):
            return None

        if isinstance(node, ListLit):
            return [self._eval(e, env) for e in node.elements]

        if isinstance(node, MapLit):
            result: dict[str, Any] = {}
            for k_expr, v_expr in node.entries:
                key = self._eval(k_expr, env)
                result[key] = self._eval(v_expr, env)
            return result

        if isinstance(node, Identifier):
            try:
                return env.get(node.name)
            except NameErr:
                # Try suggestion
                all_names = list(KEYWORDS) + env.all_names()
                suggestions = suggest_correction(node.name, all_names)
                hint = f" Did you mean: {suggestions[0]}?" if suggestions else ""
                raise NameErr(f"Undefined variable: '{node.name}'.{hint}")

        if isinstance(node, BinaryOp):
            return self._eval_binary(node, env)

        if isinstance(node, UnaryOp):
            val = self._eval(node.operand, env)
            if node.op == "-":
                return -val
            if node.op == "+":
                return +val
            if node.op == "not":
                return not self._truthy(val)
            raise TypeErr(f"Unknown unary operator: {node.op}")

        if isinstance(node, CallExpr):
            return self._eval_call(node, env)

        if isinstance(node, IndexExpr):
            obj = self._eval(node.obj, env)
            idx = self._eval(node.index, env)
            try:
                return obj[idx]
            except (IndexError, KeyError) as e:
                raise ValueErr(str(e))

        if isinstance(node, MemberExpr):
            return self._eval_member(node, env)

        if isinstance(node, LambdaExpr):
            return TechFunction("<lambda>", node.params, [ReturnStmt(node.body)], env)

        if isinstance(node, AskExpr):
            prompt = _to_str(self._eval(node.prompt, env))
            return input(prompt)

        if isinstance(node, TernaryExpr):
            if self._truthy(self._eval(node.condition, env)):
                return self._eval(node.true_val, env)
            return self._eval(node.false_val, env)

        if isinstance(node, RangeExpr):
            start = self._eval(node.start, env)
            end = self._eval(node.end, env)
            if node.inclusive:
                return list(range(int(start), int(end) + 1))
            return list(range(int(start), int(end)))

        raise TypeErr(f"Unknown expression: {type(node).__name__}")

    # ------------------------------------------------------------------
    # binary ops
    # ------------------------------------------------------------------

    def _eval_binary(self, node: BinaryOp, env: Environment) -> Any:
        op = node.op

        # Short-circuit
        if op == "and":
            left = self._eval(node.left, env)
            return left if not self._truthy(left) else self._eval(node.right, env)
        if op == "or":
            left = self._eval(node.left, env)
            return left if self._truthy(left) else self._eval(node.right, env)

        left = self._eval(node.left, env)
        right = self._eval(node.right, env)

        try:
            if op == "+":
                return left + right
            if op == "-":
                return left - right
            if op == "*":
                return left * right
            if op == "/":
                if right == 0:
                    raise RuntimeErr("Division by zero")
                return left / right
            if op == "//":
                if right == 0:
                    raise RuntimeErr("Division by zero")
                return left // right
            if op == "%":
                return left % right
            if op == "**":
                return left ** right
            if op == "==":
                return left == right
            if op == "!=":
                return left != right
            if op == "<":
                return left < right
            if op == ">":
                return left > right
            if op == "<=":
                return left <= right
            if op == ">=":
                return left >= right
            if op == "in":
                return left in right
            if op == "has":
                return right in left
            if op == "is":
                return _to_str(type(left).__name__) == right or isinstance(left, type(right))
        except TypeError as e:
            raise TypeErr(str(e))

        raise TypeErr(f"Unknown operator: {op}")

    # ------------------------------------------------------------------
    # member access
    # ------------------------------------------------------------------

    def _eval_member(self, node: MemberExpr, env: Environment) -> Any:
        obj = self._eval(node.obj, env)
        m = node.member

        if isinstance(obj, TechInstance):
            return obj.get(m)

        if isinstance(obj, dict):
            # Properties
            if m == "length":
                return len(obj)
            if m in obj:
                return obj[m]
            return self._dict_method(obj, m)

        if isinstance(obj, list):
            if m == "length":
                return len(obj)
            if m == "first":
                return obj[0] if obj else None
            if m == "last":
                return obj[-1] if obj else None
            return self._list_method(obj, m)

        if isinstance(obj, str):
            if m == "length":
                return len(obj)
            return self._str_method(obj, m)

        # Allow calling methods on injected native Python objects (like WebPageNative)
        if hasattr(obj, m):
            attr = getattr(obj, m)
            if callable(attr):
                return attr
            return attr

        raise TypeErr(f"Cannot access '.{m}' on {_to_str(obj)}")

    # --  type-specific method dispatchers  --

    def _str_method(self, s: str, m: str) -> Any:
        _meths = {
            "upper": lambda: s.upper(),
            "lower": lambda: s.lower(),
            "trim": lambda: s.strip(),
            "trim_left": lambda: s.lstrip(),
            "trim_right": lambda: s.rstrip(),
            "split": lambda sep=" ": s.split(sep),
            "replace": lambda old, new: s.replace(old, new),
            "contains": lambda sub: sub in s,
            "starts_with": lambda p: s.startswith(p),
            "ends_with": lambda p: s.endswith(p),
            "chars": lambda: list(s),
            "repeat": lambda n: s * int(n),
            "capitalize": lambda: s.capitalize(),
            "title": lambda: s.title(),
            "is_digit": lambda: s.isdigit(),
            "is_alpha": lambda: s.isalpha(),
            "index_of": lambda sub: s.find(sub),
            "count": lambda sub: s.count(sub),
            "at": lambda i: s[int(i)],
            "slice": lambda a, b: s[int(a):int(b)],
            "pad_left": lambda n, ch=" ": s.rjust(int(n), ch),
            "pad_right": lambda n, ch=" ": s.ljust(int(n), ch),
        }
        if m in _meths:
            return _meths[m]
        raise NameErr(f"String has no method '{m}'")

    def _list_method(self, lst: list, m: str) -> Any:
        # Helper: wrap TechFunction so it's callable in Python lambdas
        def _w(fn):
            if isinstance(fn, TechFunction):
                return lambda *a: self._call_fn(fn, list(a), self.global_env)
            return fn

        _meths = {
            "push": lambda v: lst.append(v) or lst,
            "pop": lambda: lst.pop(),
            "shift": lambda: lst.pop(0),
            "unshift": lambda v: lst.insert(0, v) or lst,
            "insert": lambda i, v: lst.insert(int(i), v) or lst,
            "remove": lambda v: lst.remove(v) or lst,
            "remove_at": lambda i: lst.pop(int(i)),
            "index_of": lambda v: lst.index(v) if v in lst else -1,
            "contains": lambda v: v in lst,
            "is_empty": lambda: len(lst) == 0,
            "sort": lambda key=None: sorted(lst, key=_w(key) if key else None),
            "reverse": lambda: list(reversed(lst)),
            "copy": lambda: lst.copy(),
            "clear": lambda: lst.clear(),
            "slice": lambda a, b: lst[int(a):int(b)],
            "map": lambda fn: [_w(fn)(x) for x in lst],
            "filter": lambda fn: [x for x in lst if _w(fn)(x)],
            "reduce": lambda fn, init: self._reduce(lst, _w(fn), init),
            "find": lambda fn: next((x for x in lst if _w(fn)(x)), None),
            "some": lambda fn: any(_w(fn)(x) for x in lst),
            "every": lambda fn: all(_w(fn)(x) for x in lst),
            "each": lambda fn: [_w(fn)(x) for x in lst] and None,
            "flat": lambda: [x for sub in lst for x in (sub if isinstance(sub, list) else [sub])],
            "unique": lambda: list(dict.fromkeys(lst)),
            "take": lambda n: lst[:int(n)],
            "drop": lambda n: lst[int(n):],
            "count": lambda v: lst.count(v),
            "chunk": lambda n: [lst[i:i + int(n)] for i in range(0, len(lst), int(n))],
            "join": lambda sep="": sep.join(_to_str(x) for x in lst),
        }
        if m in _meths:
            return _meths[m]
        raise NameErr(f"List has no method '{m}'")

    def _dict_method(self, d: dict, m: str) -> Any:
        _meths = {
            "keys": lambda: list(d.keys()),
            "values": lambda: list(d.values()),
            "entries": lambda: [[k, v] for k, v in d.items()],
            "has_key": lambda k: k in d,
            "get": lambda k, default=None: d.get(k, default),
            "set_key": lambda k, v: d.update({k: v}) or d,
            "delete_key": lambda k: d.pop(k, None) or d,
            "merge": lambda other: {**d, **other},
            "is_empty": lambda: len(d) == 0,
        }
        if m in _meths:
            return _meths[m]
        raise NameErr(f"Map has no method '{m}'")

    @staticmethod
    def _reduce(lst: list, fn: Any, init: Any) -> Any:
        acc = init
        for item in lst:
            acc = fn(acc, item)
        return acc

    # ------------------------------------------------------------------
    # function call
    # ------------------------------------------------------------------

    def _eval_call(self, node: CallExpr, env: Environment) -> Any:
        callee = self._eval(node.callee, env)
        args = [self._eval(a, env) for a in node.args]

        # Python callable (builtin)
        if callable(callee) and not isinstance(callee, (TechFunction, TechClass)):
            try:
                return callee(*args)
            except TechScriptError:
                raise
            except Exception as e:
                raise RuntimeErr(str(e))

        # TechScript function
        if isinstance(callee, TechFunction):
            return self._call_fn(callee, args, env)

        # TechScript class (constructor)
        if isinstance(callee, TechClass):
            instance = TechInstance(callee)
            init = callee.find_method("init")
            if init:
                bound = instance._bind(init)
                self._call_fn(bound, args, env)
            return instance

        # Native Python Classes (like WebPageNative constructor)
        if isinstance(callee, type):
            return callee(*args)

        raise TypeErr(f"'{_to_str(callee)}' is not callable")

    def _call_fn(self, fn: TechFunction, args: list, env: Environment) -> Any:
        fn_env = Environment(parent=fn.closure)
        for i, param in enumerate(fn.params):
            if i < len(args):
                fn_env.set(param.name, args[i])
            elif param.default is not None:
                fn_env.set(param.name, self._eval(param.default, env))
            else:
                raise ValueErr(f"Missing argument '{param.name}' in call to {fn.name}()")
        try:
            self._exec_block(fn.body, fn_env)
        except _ReturnSignal as ret:
            return ret.value
        return None

    # ------------------------------------------------------------------
    # f-string interpolation
    # ------------------------------------------------------------------

    def _eval_fstring(self, raw: str, env: Environment) -> str:
        result: list[str] = []
        i = 0
        while i < len(raw):
            if raw[i] == "{":
                j = raw.index("}", i)
                expr_src = raw[i + 1 : j]
                from techscript.lexer import Lexer
                from techscript.parser import Parser
                tokens = Lexer(expr_src).tokenize()
                expr = Parser(tokens).parse().body
                if expr:
                    val = self._eval(expr[0].expression if isinstance(expr[0], ExpressionStmt) else expr[0], env)
                    result.append(_to_str(val))
                i = j + 1
            else:
                result.append(raw[i])
                i += 1
        return "".join(result)

    # ------------------------------------------------------------------
    # truthiness
    # ------------------------------------------------------------------

    @staticmethod
    def _truthy(val: Any) -> bool:
        if val is None:
            return False
        if isinstance(val, bool):
            return val
        if isinstance(val, (int, float)):
            return val != 0
        if isinstance(val, str):
            return len(val) > 0
        if isinstance(val, (list, dict)):
            return len(val) > 0
        return True
