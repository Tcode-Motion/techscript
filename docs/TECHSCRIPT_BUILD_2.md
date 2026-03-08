# TechScript — Build Guide Part 2: Interpreter, CLI & Packaging

> Continued from [TECHSCRIPT_BUILD.md](./TECHSCRIPT_BUILD.md) (Lexer & Parser)

---

## Phase 3: AST Node Definitions (`ast_nodes.py`)

```python
"""TechScript AST Nodes — Data structures representing parsed code."""
from dataclasses import dataclass, field
from typing import Any


# === Base Types ===

@dataclass
class Program:
    body: list

@dataclass
class Param:
    name: str
    default: Any = None

# === Statements ===

@dataclass
class SayStmt:
    values: list

@dataclass
class SetStmt:
    name: str
    value: Any

@dataclass
class AssignStmt:
    target: Any
    op: str        # '=', '+=', '-=', '*=', '/='
    value: Any

@dataclass
class ExpressionStmt:
    expression: Any

@dataclass
class IfStmt:
    condition: Any
    body: list
    elif_clauses: list = field(default_factory=list)
    else_body: list = None

@dataclass
class ForStmt:
    var_name: str
    iterable: Any
    body: list

@dataclass
class WhileStmt:
    condition: Any
    body: list

@dataclass
class FnStmt:
    name: str
    params: list
    body: list

@dataclass
class ClassStmt:
    name: str
    parent: str = None
    body: list = field(default_factory=list)

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
    body: list
    catch_var: str = None
    catch_body: list = field(default_factory=list)
    finally_body: list = None

@dataclass
class ThrowStmt:
    value: Any

@dataclass
class MatchStmt:
    subject: Any
    cases: list = field(default_factory=list)

@dataclass
class ImportStmt:
    module: str
    names: list = None
    alias: str = None

@dataclass
class DelStmt:
    name: str

@dataclass
class DeferStmt:
    expression: Any

@dataclass
class GuardStmt:
    condition: Any
    else_body: list

@dataclass
class WithStmt:
    expression: Any
    var_name: str
    body: list

@dataclass
class ConstStmt:
    name: str
    value: Any

@dataclass
class ExportStmt:
    declaration: Any

# === Expressions ===

@dataclass
class NumberLit:
    value: int | float

@dataclass
class StringLit:
    value: str

@dataclass
class FStringLit:
    raw: str  # Raw template, parser/interpreter handles interpolation

@dataclass
class BoolLit:
    value: bool

@dataclass
class NoneLit:
    pass

@dataclass
class ListLit:
    elements: list

@dataclass
class MapLit:
    entries: list  # list of (key_expr, value_expr) tuples

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
    args: list

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
    params: list
    body: Any

@dataclass
class AskExpr:
    prompt: Any

@dataclass
class TernaryExpr:
    true_val: Any
    condition: Any
    false_val: Any
```

---

## Phase 4: Interpreter / Evaluator (`interpreter.py`)

The interpreter walks the AST and executes each node.

```python
"""TechScript Interpreter — Walks the AST and executes code."""
import re
from .ast_nodes import *
from .environment import Environment
from .errors import TechScriptError, NameErr, TypeErr, ValueErr


# === Signal Exceptions (control flow) ===

class ReturnSignal(Exception):
    def __init__(self, value=None):
        self.value = value

class BreakSignal(Exception):
    pass

class SkipSignal(Exception):
    pass


# === TechScript Runtime Types ===

class TechFunction:
    """A user-defined function."""
    def __init__(self, name, params, body, closure):
        self.name = name
        self.params = params
        self.body = body
        self.closure = closure  # The environment at definition time

    def __repr__(self):
        return f"<fn {self.name}>"


class TechClass:
    """A user-defined class."""
    def __init__(self, name, methods, parent=None):
        self.name = name
        self.methods = methods
        self.parent = parent

    def find_method(self, name):
        if name in self.methods:
            return self.methods[name]
        if self.parent:
            return self.parent.find_method(name)
        return None

    def __repr__(self):
        return f"<class {self.name}>"


class TechInstance:
    """An instance of a TechClass."""
    def __init__(self, klass):
        self.klass = klass
        self.fields = {}

    def get(self, name):
        if name in self.fields:
            return self.fields[name]
        method = self.klass.find_method(name)
        if method:
            return self._bind_method(method)
        raise NameErr(f"'{self.klass.name}' has no property '{name}'")

    def set(self, name, value):
        self.fields[name] = value

    def _bind_method(self, method):
        env = Environment(parent=method.closure)
        env.set("self", self)
        return TechFunction(method.name, method.params[1:], method.body, env)

    def __repr__(self):
        return f"<{self.klass.name} instance>"


# === Main Interpreter ===

class Interpreter:
    def __init__(self):
        self.global_env = Environment()
        self._register_builtins()

    def run(self, program: Program):
        """Execute a parsed program."""
        self._exec_block(program.body, self.global_env)

    def _exec_block(self, statements, env):
        for stmt in statements:
            self._exec(stmt, env)

    def _exec(self, node, env):
        """Execute a statement node."""
        match node:
            case SayStmt(values):
                parts = [str(self._eval(v, env)) for v in values]
                print(" ".join(parts))

            case SetStmt(name, value):
                env.set(name, self._eval(value, env))

            case AssignStmt(target, op, value):
                val = self._eval(value, env)
                if isinstance(target, Identifier):
                    if op == "=":
                        env.set(target.name, val)
                    else:
                        current = env.get(target.name)
                        ops = {"+=": lambda a,b: a+b, "-=": lambda a,b: a-b,
                               "*=": lambda a,b: a*b, "/=": lambda a,b: a/b}
                        env.update(target.name, ops[op](current, val))
                elif isinstance(target, IndexExpr):
                    obj = self._eval(target.obj, env)
                    idx = self._eval(target.index, env)
                    obj[idx] = val
                elif isinstance(target, MemberExpr):
                    obj = self._eval(target.obj, env)
                    if isinstance(obj, TechInstance):
                        obj.set(target.member, val)
                    elif isinstance(obj, dict):
                        obj[target.member] = val

            case ExpressionStmt(expression):
                self._eval(expression, env)

            case IfStmt(condition, body, elif_clauses, else_body):
                if self._is_truthy(self._eval(condition, env)):
                    self._exec_block(body, Environment(parent=env))
                else:
                    matched = False
                    for elif_cond, elif_body in elif_clauses:
                        if self._is_truthy(self._eval(elif_cond, env)):
                            self._exec_block(elif_body, Environment(parent=env))
                            matched = True
                            break
                    if not matched and else_body:
                        self._exec_block(else_body, Environment(parent=env))

            case ForStmt(var_name, iterable, body):
                items = self._eval(iterable, env)
                for item in items:
                    loop_env = Environment(parent=env)
                    loop_env.set(var_name, item)
                    try:
                        self._exec_block(body, loop_env)
                    except BreakSignal:
                        break
                    except SkipSignal:
                        continue

            case WhileStmt(condition, body):
                while self._is_truthy(self._eval(condition, env)):
                    try:
                        self._exec_block(body, Environment(parent=env))
                    except BreakSignal:
                        break
                    except SkipSignal:
                        continue

            case FnStmt(name, params, body):
                fn = TechFunction(name, params, body, env)
                env.set(name, fn)

            case ClassStmt(name, parent_name, body):
                parent = env.get(parent_name) if parent_name else None
                methods = {}
                for member in body:
                    if isinstance(member, FnStmt):
                        methods[member.name] = TechFunction(
                            member.name, member.params, member.body, env
                        )
                klass = TechClass(name, methods, parent)
                env.set(name, klass)

            case ReturnStmt(value):
                val = self._eval(value, env) if value else None
                raise ReturnSignal(val)

            case BreakStmt():
                raise BreakSignal()

            case SkipStmt():
                raise SkipSignal()

            case PassStmt():
                pass

            case TryStmt(body, catch_var, catch_body, finally_body):
                try:
                    self._exec_block(body, Environment(parent=env))
                except TechScriptError as e:
                    catch_env = Environment(parent=env)
                    if catch_var:
                        catch_env.set(catch_var, {"message": str(e), "type": type(e).__name__})
                    self._exec_block(catch_body, catch_env)
                finally:
                    if finally_body:
                        self._exec_block(finally_body, Environment(parent=env))

            case ThrowStmt(value):
                val = self._eval(value, env)
                if isinstance(val, str):
                    raise TechScriptError(val)
                raise TechScriptError(str(val))

            case MatchStmt(subject, cases):
                val = self._eval(subject, env)
                for pattern, case_body in cases:
                    if self._match_pattern(val, pattern, env):
                        self._exec_block(case_body, Environment(parent=env))
                        break

            case DelStmt(name):
                env.delete(name)

            case ConstStmt(name, value):
                env.set_const(name, self._eval(value, env))

    def _eval(self, node, env):
        """Evaluate an expression node and return a value."""
        match node:
            case NumberLit(value):
                return value

            case StringLit(value):
                return value

            case FStringLit(raw):
                return self._eval_fstring(raw, env)

            case BoolLit(value):
                return value

            case NoneLit():
                return None

            case ListLit(elements):
                return [self._eval(e, env) for e in elements]

            case MapLit(entries):
                result = {}
                for key_expr, val_expr in entries:
                    key = self._eval(key_expr, env)
                    if isinstance(key, Identifier):
                        key = key.name
                    result[key] = self._eval(val_expr, env)
                return result

            case Identifier(name):
                return env.get(name)

            case BinaryOp(left, op, right):
                return self._eval_binary(left, op, right, env)

            case UnaryOp(op, operand):
                val = self._eval(operand, env)
                if op == "-": return -val
                if op == "+": return +val
                if op == "not": return not self._is_truthy(val)

            case CallExpr(callee, args):
                return self._eval_call(callee, args, env)

            case IndexExpr(obj, index):
                o = self._eval(obj, env)
                i = self._eval(index, env)
                return o[i]

            case MemberExpr(obj, member):
                o = self._eval(obj, env)
                if isinstance(o, TechInstance):
                    return o.get(member)
                if isinstance(o, dict):
                    if member in o:
                        return o[member]
                    return self._get_map_method(o, member)
                if isinstance(o, list):
                    return self._get_list_method(o, member)
                if isinstance(o, str):
                    return self._get_str_method(o, member)

            case LambdaExpr(params, body):
                return TechFunction("<lambda>", params, [ReturnStmt(body)], env)

            case AskExpr(prompt):
                prompt_str = self._eval(prompt, env)
                return input(str(prompt_str))

            case TernaryExpr(true_val, condition, false_val):
                if self._is_truthy(self._eval(condition, env)):
                    return self._eval(true_val, env)
                return self._eval(false_val, env)

        raise TypeErr(f"Unknown AST node: {type(node).__name__}")

    def _eval_binary(self, left_node, op, right_node, env):
        # Short-circuit for and/or
        if op == "and":
            left = self._eval(left_node, env)
            return left if not self._is_truthy(left) else self._eval(right_node, env)
        if op == "or":
            left = self._eval(left_node, env)
            return left if self._is_truthy(left) else self._eval(right_node, env)

        left = self._eval(left_node, env)
        right = self._eval(right_node, env)

        ops = {
            "+": lambda a,b: a+b, "-": lambda a,b: a-b,
            "*": lambda a,b: a*b, "/": lambda a,b: a/b,
            "//": lambda a,b: a//b, "%": lambda a,b: a%b,
            "**": lambda a,b: a**b,
            "==": lambda a,b: a==b, "!=": lambda a,b: a!=b,
            "<": lambda a,b: a<b, ">": lambda a,b: a>b,
            "<=": lambda a,b: a<=b, ">=": lambda a,b: a>=b,
            "in": lambda a,b: a in b, "is": lambda a,b: isinstance(a, b),
        }
        if op in ops:
            return ops[op](left, right)
        raise TypeErr(f"Unknown operator: {op}")

    def _eval_call(self, callee_node, arg_nodes, env):
        callee = self._eval(callee_node, env)
        args = [self._eval(a, env) for a in arg_nodes]

        # Built-in (Python callable)
        if callable(callee) and not isinstance(callee, TechFunction):
            return callee(*args)

        # TechScript function
        if isinstance(callee, TechFunction):
            fn_env = Environment(parent=callee.closure)
            for i, param in enumerate(callee.params):
                if i < len(args):
                    fn_env.set(param.name, args[i])
                elif param.default is not None:
                    fn_env.set(param.name, self._eval(param.default, env))
                else:
                    raise ValueErr(f"Missing argument: {param.name}")
            try:
                self._exec_block(callee.body, fn_env)
            except ReturnSignal as ret:
                return ret.value
            return None

        # TechScript class (constructor)
        if isinstance(callee, TechClass):
            instance = TechInstance(callee)
            init = callee.find_method("init")
            if init:
                bound = instance._bind_method(init)
                self._eval_call_direct(bound, args)
            return instance

        raise TypeErr(f"'{callee}' is not callable")

    def _eval_call_direct(self, fn, args):
        fn_env = Environment(parent=fn.closure)
        for i, param in enumerate(fn.params):
            if i < len(args):
                fn_env.set(param.name, args[i])
        try:
            self._exec_block(fn.body, fn_env)
        except ReturnSignal:
            pass

    def _eval_fstring(self, raw, env):
        """Evaluate f-string with {expr} interpolation."""
        result = ""
        i = 0
        while i < len(raw):
            if raw[i] == '{':
                j = raw.index('}', i)
                expr_str = raw[i+1:j]
                # Mini re-parse of the expression
                from .lexer import Lexer
                from .parser import Parser
                tokens = Lexer(expr_str).tokenize()
                expr = Parser(tokens).parse_expression()
                val = self._eval(expr, env)
                result += str(val)
                i = j + 1
            else:
                result += raw[i]
                i += 1
        return result

    def _is_truthy(self, value):
        if value is None: return False
        if isinstance(value, bool): return value
        if isinstance(value, (int, float)): return value != 0
        if isinstance(value, str): return len(value) > 0
        if isinstance(value, list): return len(value) > 0
        if isinstance(value, dict): return len(value) > 0
        return True

    def _match_pattern(self, value, pattern, env):
        if isinstance(pattern, Identifier) and pattern.name == "_":
            return True
        pat_val = self._eval(pattern, env)
        return value == pat_val

    # === Built-in type methods ===

    def _get_str_method(self, s, method):
        methods = {
            "length": len(s),
            "upper": lambda: s.upper(),
            "lower": lambda: s.lower(),
            "trim": lambda: s.strip(),
            "split": lambda sep=" ": s.split(sep),
            "replace": lambda old, new: s.replace(old, new),
            "contains": lambda sub: sub in s,
            "starts_with": lambda pre: s.startswith(pre),
            "ends_with": lambda suf: s.endswith(suf),
            "chars": lambda: list(s),
            "repeat": lambda n: s * n,
            "capitalize": lambda: s.capitalize(),
            "title": lambda: s.title(),
            "is_digit": lambda: s.isdigit(),
            "is_alpha": lambda: s.isalpha(),
            "index_of": lambda sub: s.find(sub),
            "count": lambda sub: s.count(sub),
            "at": lambda i: s[i],
            "slice": lambda a, b: s[a:b],
        }
        if method in methods:
            return methods[method]
        raise NameErr(f"String has no method '{method}'")

    def _get_list_method(self, lst, method):
        methods = {
            "length": len(lst),
            "first": lst[0] if lst else None,
            "last": lst[-1] if lst else None,
            "push": lambda v: lst.append(v) or lst,
            "pop": lambda: lst.pop(),
            "shift": lambda: lst.pop(0),
            "insert": lambda i, v: lst.insert(i, v) or lst,
            "remove": lambda v: lst.remove(v) or lst,
            "contains": lambda v: v in lst,
            "index_of": lambda v: lst.index(v) if v in lst else -1,
            "is_empty": lambda: len(lst) == 0,
            "sort": lambda key=None: sorted(lst, key=key),
            "reverse": lambda: list(reversed(lst)),
            "copy": lambda: lst.copy(),
            "clear": lambda: lst.clear(),
            "slice": lambda a, b: lst[a:b],
            "map": lambda fn: [fn(x) for x in lst],
            "filter": lambda fn: [x for x in lst if fn(x)],
            "reduce": lambda fn, init: self._reduce(lst, fn, init),
            "find": lambda fn: next((x for x in lst if fn(x)), None),
            "some": lambda fn: any(fn(x) for x in lst),
            "every": lambda fn: all(fn(x) for x in lst),
            "each": lambda fn: [fn(x) for x in lst] and None,
            "flat": lambda: [item for sub in lst for item in (sub if isinstance(sub, list) else [sub])],
            "unique": lambda: list(dict.fromkeys(lst)),
            "take": lambda n: lst[:n],
            "drop": lambda n: lst[n:],
            "count": lambda v: lst.count(v),
            "chunk": lambda n: [lst[i:i+n] for i in range(0, len(lst), n)],
        }
        if method in methods:
            return methods[method]
        raise NameErr(f"List has no method '{method}'")

    def _get_map_method(self, m, method):
        methods = {
            "keys": lambda: list(m.keys()),
            "values": lambda: list(m.values()),
            "entries": lambda: [[k, v] for k, v in m.items()],
            "has_key": lambda k: k in m,
            "get": lambda k, default=None: m.get(k, default),
            "set_key": lambda k, v: m.update({k: v}) or m,
            "delete_key": lambda k: m.pop(k, None) or m,
            "merge": lambda other: {**m, **other},
            "length": len(m),
            "is_empty": lambda: len(m) == 0,
        }
        if method in methods:
            return methods[method]
        raise NameErr(f"Map has no method '{method}'")

    def _reduce(self, lst, fn, init):
        acc = init
        for item in lst:
            acc = fn(acc, item)
        return acc

    # === Register Built-in Functions ===

    def _register_builtins(self):
        import math, random, time, os, json

        self.global_env.set("abs", abs)
        self.global_env.set("round", round)
        self.global_env.set("min", min)
        self.global_env.set("max", max)
        self.global_env.set("sum", sum)
        self.global_env.set("len", len)
        self.global_env.set("size", len)
        self.global_env.set("range", lambda a, b=None, s=1: list(range(a, b, s) if b else range(a)))
        self.global_env.set("enumerate", lambda lst: [[i, v] for i, v in enumerate(lst)])
        self.global_env.set("zip", lambda a, b: [[x, y] for x, y in zip(a, b)])
        self.global_env.set("sorted", sorted)
        self.global_env.set("reversed", lambda lst: list(reversed(lst)))

        # Type conversion
        self.global_env.set("to_int", lambda x: int(x))
        self.global_env.set("to_float", lambda x: float(x))
        self.global_env.set("to_str", lambda x: str(x))
        self.global_env.set("to_bool", lambda x: bool(x))
        self.global_env.set("to_list", lambda x: list(x))

        # Type checking
        self.global_env.set("typeof", lambda x: type(x).__name__)
        self.global_env.set("is_int", lambda x: isinstance(x, int))
        self.global_env.set("is_float", lambda x: isinstance(x, float))
        self.global_env.set("is_str", lambda x: isinstance(x, str))
        self.global_env.set("is_list", lambda x: isinstance(x, list))
        self.global_env.set("is_map", lambda x: isinstance(x, dict))
        self.global_env.set("is_none", lambda x: x is None)
        self.global_env.set("is_bool", lambda x: isinstance(x, bool))

        # Math
        self.global_env.set("sqrt", math.sqrt)
        self.global_env.set("ceil", math.ceil)
        self.global_env.set("floor", math.floor)
        self.global_env.set("pow", pow)
        self.global_env.set("clamp", lambda x, lo, hi: max(lo, min(x, hi)))
        self.global_env.set("sign", lambda x: (x > 0) - (x < 0))
        self.global_env.set("is_even", lambda x: x % 2 == 0)
        self.global_env.set("is_odd", lambda x: x % 2 != 0)
        self.global_env.set("gcd", math.gcd)

        # Random
        self.global_env.set("random", random.random)
        self.global_env.set("randint", random.randint)
        self.global_env.set("choice", random.choice)
        self.global_env.set("shuffle", lambda lst: random.shuffle(lst) or lst)

        # IO
        self.global_env.set("write", lambda *a: print(*a, end=""))
        self.global_env.set("debug", lambda x: print(f"[debug] {type(x).__name__}: {x!r}"))
        self.global_env.set("clear", lambda: os.system("cls" if os.name == "nt" else "clear"))
        self.global_env.set("sleep", lambda ms: time.sleep(ms / 1000))
        self.global_env.set("exit", lambda code=0: exit(code))
        self.global_env.set("assert", self._builtin_assert)

        # File IO
        self.global_env.set("read_file", lambda p: open(p).read())
        self.global_env.set("write_file", lambda p, d: open(p, "w").write(d))
        self.global_env.set("append_file", lambda p, d: open(p, "a").write(d))
        self.global_env.set("read_lines", lambda p: open(p).read().splitlines())
        self.global_env.set("write_lines", lambda p, l: open(p, "w").write("\n".join(l)))
        self.global_env.set("file_exists", lambda p: os.path.exists(p))
        self.global_env.set("delete_file", lambda p: os.remove(p))
        self.global_env.set("list_dir", lambda p=".": os.listdir(p))
        self.global_env.set("make_dir", lambda p: os.makedirs(p, exist_ok=True))
        self.global_env.set("read_json", lambda p: json.loads(open(p).read()))
        self.global_env.set("write_json", lambda p, d: open(p, "w").write(json.dumps(d, indent=2)))

        # String helpers
        self.global_env.set("format", lambda tmpl, *args: tmpl.format(*args))
        self.global_env.set("hash", hash)
        self.global_env.set("print_env", lambda: print(self.global_env.vars))

        # Error constructors
        self.global_env.set("Error", lambda msg: TechScriptError(msg))
        self.global_env.set("ValueError", lambda msg: ValueErr(msg))
        self.global_env.set("TypeError", lambda msg: TypeErr(msg))

    def _builtin_assert(self, condition, msg="Assertion failed"):
        if not condition:
            raise TechScriptError(msg)
```

---

## Phase 5: Environment (`environment.py`)

```python
"""Variable scope management with parent-chain lookup."""

class Environment:
    def __init__(self, parent=None):
        self.vars = {}
        self.consts = set()
        self.parent = parent

    def get(self, name):
        if name in self.vars:
            return self.vars[name]
        if self.parent:
            return self.parent.get(name)
        from .errors import NameErr
        raise NameErr(f"Undefined variable: '{name}'")

    def set(self, name, value):
        if name in self.consts:
            from .errors import TechScriptError
            raise TechScriptError(f"Cannot reassign constant '{name}'")
        self.vars[name] = value

    def set_const(self, name, value):
        self.vars[name] = value
        self.consts.add(name)

    def update(self, name, value):
        if name in self.vars:
            if name in self.consts:
                from .errors import TechScriptError
                raise TechScriptError(f"Cannot reassign constant '{name}'")
            self.vars[name] = value
        elif self.parent:
            self.parent.update(name, value)
        else:
            self.vars[name] = value

    def delete(self, name):
        if name in self.vars:
            del self.vars[name]
            self.consts.discard(name)
        elif self.parent:
            self.parent.delete(name)
```

---

## Phase 6: Error System (`errors.py`)

```python
"""TechScript error types and formatting with 'Did you mean?' suggestions."""
from difflib import get_close_matches


class TechScriptError(Exception):
    """Base error for all TechScript runtime errors."""
    def __init__(self, message, line=None, column=None):
        self.message = message
        self.line = line
        self.column = column
        super().__init__(message)


class NameErr(TechScriptError):
    pass

class TypeErr(TechScriptError):
    pass

class ValueErr(TechScriptError):
    pass

class IndexErr(TechScriptError):
    pass

class FileErr(TechScriptError):
    pass

class ImportErr(TechScriptError):
    pass


def suggest_correction(unknown, known_words, max_results=3, cutoff=0.6):
    """Find similar words using difflib."""
    matches = get_close_matches(unknown, known_words, n=max_results, cutoff=cutoff)
    return matches


def format_error(error, source_lines=None):
    """Format a TechScript error with colors and suggestions."""
    err_type = type(error).__name__.replace("Err", "Error")
    msg = error.message

    lines = [""]
    lines.append(f"╭─ TechScript Error ─────────────────────────────────")
    lines.append(f"│")
    lines.append(f"│  {err_type}: {msg}")
    lines.append(f"│")

    if error.line and source_lines:
        line_num = error.line
        if 0 < line_num <= len(source_lines):
            code_line = source_lines[line_num - 1]
            lines.append(f"│    {line_num} │  {code_line}")
            if error.column:
                pointer = " " * (error.column - 1) + "^^^"
                lines.append(f"│      │  {pointer}")
            lines.append(f"│")

    # Suggestions for NameError
    if isinstance(error, NameErr) and "Undefined" in msg or "Unknown" in msg:
        from .tokens import KEYWORDS
        word = msg.split("'")[1] if "'" in msg else ""
        if word:
            suggestions = suggest_correction(word, list(KEYWORDS))
            if suggestions:
                lines.append(f"│  Did you mean: {suggestions[0]}?")
                lines.append(f"│")

    lines.append(f"╰─────────────────────────────────────────────────────")
    return "\n".join(lines)
```

---

## Phase 7: CLI Tool (`cli.py` & `__main__.py`)

```python
# === cli.py ===
"""TechScript CLI — the `tech` command."""
import sys
import os
from .lexer import Lexer, LexerError
from .parser import Parser, ParseError
from .interpreter import Interpreter
from .errors import TechScriptError, format_error
from .repl import start_repl


def main():
    args = sys.argv[1:]

    if not args or args[0] in ("help", "--help", "-h"):
        print_help()
        return

    cmd = args[0]

    if cmd == "version" or cmd == "--version":
        print("TechScript v1.0.0")
        return

    if cmd == "repl":
        start_repl()
        return

    if cmd == "run":
        if len(args) < 2:
            print("Usage: tech run <file.txs>")
            sys.exit(1)
        run_file(args[1], debug="--debug" in args)
        return

    if cmd == "check":
        if len(args) < 2:
            print("Usage: tech check <file.txs>")
            sys.exit(1)
        check_file(args[1])
        return

    # Default: try to run as a file
    if os.path.isfile(cmd) and cmd.endswith((".txs", ".tx")):
        run_file(cmd)
        return

    print(f"Unknown command: {cmd}")
    print("Run 'tech help' for usage information.")
    sys.exit(1)


def run_file(filepath, debug=False):
    if not os.path.exists(filepath):
        print(f"Error: File not found: {filepath}")
        sys.exit(1)

    with open(filepath, "r", encoding="utf-8") as f:
        source = f.read()

    source_lines = source.splitlines()

    try:
        tokens = Lexer(source).tokenize()
        if debug:
            for t in tokens:
                print(f"  {t}")
            print("---")
        program = Parser(tokens).parse()
        interpreter = Interpreter()
        interpreter.run(program)
    except (LexerError, ParseError) as e:
        print(format_error(e, source_lines) if hasattr(e, 'message') else str(e))
        sys.exit(1)
    except TechScriptError as e:
        print(format_error(e, source_lines))
        sys.exit(1)
    except KeyboardInterrupt:
        print("\nInterrupted.")
        sys.exit(130)


def check_file(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        source = f.read()
    try:
        tokens = Lexer(source).tokenize()
        Parser(tokens).parse()
        print(f"✓ {filepath}: No syntax errors found.")
    except (LexerError, ParseError) as e:
        print(f"✗ {filepath}: {e}")
        sys.exit(1)


def print_help():
    print("""
TechScript v1.0.0 — A simple, friendly programming language

Usage:
  tech run <file.txs>       Run a TechScript file
  tech run <file> --debug    Run with debug output
  tech repl                  Start interactive REPL
  tech check <file.txs>     Check syntax without running
  tech version               Show version
  tech help                  Show this help
    """.strip())


# === __main__.py ===
# from .cli import main
# main()
```

---

## Phase 8: REPL (`repl.py`)

```python
"""TechScript REPL — Interactive shell."""
from .lexer import Lexer, LexerError
from .parser import Parser, ParseError
from .interpreter import Interpreter
from .errors import TechScriptError, format_error


def start_repl():
    print("╭──────────────────────────────────────╮")
    print("│  TechScript v1.0.0 Interactive REPL  │")
    print("│  Type 'help' for commands, 'exit'    │")
    print("│  to quit.                            │")
    print("╰──────────────────────────────────────╯")
    print()

    interpreter = Interpreter()
    buffer = ""

    while True:
        try:
            prompt = "... " if buffer else ">>> "
            line = input(prompt)

            if not buffer and line.strip() == "exit":
                print("Goodbye! 👋")
                break

            if not buffer and line.strip() == "help":
                print_repl_help()
                continue

            buffer += line + "\n"

            # If line ends with ':', expect more input
            if line.strip().endswith(":"):
                continue

            # If in multi-line mode and line is not empty, continue
            if buffer.count("\n") > 1 and line.strip():
                continue

            # Try to execute
            source = buffer.strip()
            buffer = ""

            if not source:
                continue

            tokens = Lexer(source).tokenize()
            program = Parser(tokens).parse()
            interpreter.run(program)

        except (LexerError, ParseError, TechScriptError) as e:
            buffer = ""
            msg = format_error(e) if isinstance(e, TechScriptError) else str(e)
            print(msg)
        except KeyboardInterrupt:
            buffer = ""
            print("\nKeyboardInterrupt")
        except EOFError:
            print("\nGoodbye! 👋")
            break


def print_repl_help():
    print("""
REPL Commands:
  help          Show this help
  exit          Exit the REPL

Quick Reference:
  say "Hello"              Print output
  name = ask "Name? "      Read input
  x = 42                   Set variable
  if x > 0: ...            Conditional
  for i in 1..5: ...       Loop
  fn add(a, b): return a+b Define function
    """.strip())
```

---

## Phase 9: Packaging (`setup.py` / `pyproject.toml`)

### `pyproject.toml`

```toml
[build-system]
requires = ["setuptools>=67.0", "wheel"]
build-backend = "setuptools.backends._legacy:_Backend"

[project]
name = "techscript"
version = "1.0.0"
description = "TechScript — A simple, friendly programming language"
readme = "README.md"
requires-python = ">=3.10"
license = {text = "MIT"}
authors = [{name = "TechScript Team"}]
classifiers = [
    "Programming Language :: Python :: 3",
    "License :: OSI Approved :: MIT License",
    "Topic :: Software Development :: Interpreters",
]

[project.scripts]
tech = "techscript.cli:main"

[tool.setuptools.packages.find]
where = ["src"]
```

### Installation & Distribution

```bash
# Install locally for development
pip install -e .

# Now 'tech' command is available globally
tech version         # TechScript v1.0.0
tech run hello.txs   # Run a program
tech repl            # Start REPL

# Build distributable package
python -m build
# Creates dist/techscript-1.0.0-py3-none-any.whl

# Publish to PyPI
python -m twine upload dist/*
```

---

## Developer Guidelines

### Adding a New Keyword

1. Add the keyword string to `KEYWORDS` set in `tokens.py`
2. Add parsing logic in `Parser.parse_statement()` (match case)
3. Create a new AST node in `ast_nodes.py`
4. Add execution logic in `Interpreter._exec()` (match case)
5. Add tests in `tests/`

### Adding a New Built-in Function

1. In `interpreter.py` → `_register_builtins()`, add:
   ```python
   self.global_env.set("my_func", lambda x: ...)
   ```
2. Document it in `TECHSCRIPT_REFERENCE.md`

### Testing

```bash
# Run all tests
python -m pytest tests/ -v

# Test specific component
python -m pytest tests/test_lexer.py -v
python -m pytest tests/test_parser.py -v
python -m pytest tests/test_interpreter.py -v
```

### Development Roadmap

| Phase | Goal | Priority |
|-------|------|----------|
| v1.0 | AST-walking interpreter, core features, CLI, REPL | ✅ Now |
| v1.1 | Standard library expansion, import system, testing framework | Next |
| v1.2 | Bytecode compiler (compile AST → bytecode → VM) | Medium |
| v1.3 | VS Code extension (syntax highlighting, snippets, diagnostics) | Medium |
| v1.4 | Package manager (`tech install <package>`) | Later |
| v2.0 | Native compilation (via LLVM or Cranelift) | Future |

---

*This concludes the complete TechScript build guide. Together with [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md), [TECHSCRIPT_REFERENCE.md](./TECHSCRIPT_REFERENCE.md), [TECHSCRIPT_GUIDE.md](./TECHSCRIPT_GUIDE.md), and [TECHSCRIPT_EXAMPLES.md](./TECHSCRIPT_EXAMPLES.md), this provides everything needed to build TechScript from scratch.*
