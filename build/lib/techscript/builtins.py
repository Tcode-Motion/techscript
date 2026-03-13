"""TechScript built-in functions — the first 80 registered into the global env.

Each function is a plain Python callable.  The ``register_builtins(env)``
function populates a given ``Environment`` with all of them.

Categories (numbered to match TECHSCRIPT_REFERENCE.md):
  1–10   I/O & output
 11–25   Math & numeric
 26–45   String
 46–65   List & map
 66–75   Type conversion & checking
 76–80   System / misc
"""

from __future__ import annotations

import json
import math
import os
import random
import time as _time
from typing import Any

from techscript.environment import Environment
from techscript.errors import TechScriptError, ValueErr, FileErr


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _ts_print(*args: Any) -> None:
    """``say`` implementation — called for SayStmt in the interpreter."""
    print(*args)


def _ts_write(*args: Any) -> None:
    print(*args, end="")


def _ts_debug(val: Any) -> None:
    print(f"[debug] {type(val).__name__}: {val!r}")


def _ts_log(msg: str) -> None:
    t = _time.strftime("%H:%M:%S")
    print(f"[{t}] {msg}")


def _ts_warn(msg: str) -> None:
    import sys
    print(f"[warn] {msg}", file=sys.stderr)


def _ts_error_print(msg: str) -> None:
    import sys
    print(f"[error] {msg}", file=sys.stderr)


def _ts_clear() -> None:
    os.system("cls" if os.name == "nt" else "clear")


def _ts_format(template: str, *args: Any) -> str:
    return template.format(*args)


# ---------------------------------------------------------------------------
# Math helpers
# ---------------------------------------------------------------------------

def _clamp(x: float, lo: float, hi: float) -> float:
    return max(lo, min(x, hi))

def _sign(x: float) -> int:
    return (x > 0) - (x < 0)

def _is_even(x: int) -> bool:
    return x % 2 == 0

def _is_odd(x: int) -> bool:
    return x % 2 != 0


# ---------------------------------------------------------------------------
# File helpers
# ---------------------------------------------------------------------------

def _read_file(path: str) -> str:
    try:
        with open(path, encoding="utf-8") as f:
            return f.read()
    except FileNotFoundError:
        raise FileErr(f"File not found: '{path}'")
    except PermissionError:
        raise FileErr(f"Permission denied: '{path}'")


def _write_file(path: str, data: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write(data)


def _append_file(path: str, data: str) -> None:
    with open(path, "a", encoding="utf-8") as f:
        f.write(data)


def _read_lines(path: str) -> list[str]:
    return _read_file(path).splitlines()


def _write_lines(path: str, lines: list[str]) -> None:
    _write_file(path, "\n".join(lines))


def _read_json(path: str) -> Any:
    return json.loads(_read_file(path))


def _write_json(path: str, data: Any) -> None:
    _write_file(path, json.dumps(data, indent=2, ensure_ascii=False))


# ---------------------------------------------------------------------------
# Type conversion & checking
# ---------------------------------------------------------------------------

def _to_int(val: Any) -> int:
    try:
        return int(val)
    except (ValueError, TypeError):
        raise ValueErr(f"Cannot convert {val!r} to int")

def _to_float(val: Any) -> float:
    try:
        return float(val)
    except (ValueError, TypeError):
        raise ValueErr(f"Cannot convert {val!r} to float")

def _to_str(val: Any) -> str:
    if val is None:
        return "none"
    if isinstance(val, bool):
        return "true" if val else "false"
    return str(val)

def _to_bool(val: Any) -> bool:
    return bool(val)

def _to_list(val: Any) -> list:
    return list(val)


def _typeof(val: Any) -> str:
    if val is None:
        return "none"
    if isinstance(val, bool):
        return "bool"
    if isinstance(val, int):
        return "int"
    if isinstance(val, float):
        return "float"
    if isinstance(val, str):
        return "str"
    if isinstance(val, list):
        return "list"
    if isinstance(val, dict):
        return "map"
    return type(val).__name__


# ---------------------------------------------------------------------------
# Assert
# ---------------------------------------------------------------------------

def _ts_assert(condition: Any, msg: str = "Assertion failed") -> None:
    if not condition:
        raise TechScriptError(msg)


# ---------------------------------------------------------------------------
# Registration
# ---------------------------------------------------------------------------

def register_builtins(env: Environment) -> None:
    """Populate *env* with the first 80 built-in functions/values."""

    # ===== I/O (1-10) =====
    env.set("write", _ts_write)                     # 1
    env.set("debug", _ts_debug)                     # 2
    env.set("log", _ts_log)                         # 3
    env.set("warn", _ts_warn)                       # 4
    env.set("error", _ts_error_print)               # 5
    env.set("clear", _ts_clear)                     # 6
    env.set("format", _ts_format)                   # 7
    env.set("read_file", _read_file)                # 8
    env.set("write_file", _write_file)              # 9
    env.set("append_file", _append_file)            # 10

    # ===== Math (11-25) =====
    env.set("abs", abs)                             # 11
    env.set("round", round)                         # 12
    env.set("ceil", math.ceil)                      # 13
    env.set("floor", math.floor)                    # 14
    env.set("min", min)                             # 15
    env.set("max", max)                             # 16
    env.set("sum", sum)                             # 17
    env.set("sqrt", math.sqrt)                      # 18
    env.set("pow", pow)                             # 19
    env.set("clamp", _clamp)                        # 20
    env.set("sign", _sign)                          # 21
    env.set("is_even", _is_even)                    # 22
    env.set("is_odd", _is_odd)                      # 23
    env.set("gcd", math.gcd)                        # 24
    env.set("pi", math.pi)                          # 25

    # ===== Random (26-30) =====
    env.set("random", random.random)                # 26
    env.set("randint", random.randint)              # 27
    env.set("choice", random.choice)                # 28
    env.set("shuffle", lambda lst: random.shuffle(lst) or lst)  # 29
    env.set("e", math.e)                            # 30

    # ===== String helpers (31-45) — most are methods, these are free fns =====
    env.set("upper", lambda s: s.upper())           # 31
    env.set("lower", lambda s: s.lower())           # 32
    env.set("trim", lambda s: s.strip())            # 33
    env.set("split", lambda s, sep=" ": s.split(sep))  # 34
    env.set("join", lambda sep, lst: sep.join(str(x) for x in lst))  # 35
    env.set("replace", lambda s, old, new: s.replace(old, new))  # 36
    env.set("contains", lambda s, sub: sub in s)    # 37
    env.set("starts_with", lambda s, p: s.startswith(p))  # 38
    env.set("ends_with", lambda s, p: s.endswith(p))  # 39
    env.set("chars", lambda s: list(s))             # 40
    env.set("repeat", lambda s, n: s * n)           # 41
    env.set("capitalize", lambda s: s.capitalize()) # 42
    env.set("title", lambda s: s.title())           # 43
    env.set("index_of", lambda s, sub: s.find(sub)) # 44
    env.set("count", lambda s, sub: s.count(sub))   # 45

    # ===== List / collection (46-65) =====
    env.set("len", len)                             # 46
    env.set("size", len)                            # 47
    env.set("range", lambda *a: list(range(*a)))    # 48
    env.set("enumerate", lambda lst: [[i, v] for i, v in enumerate(lst)])  # 49
    env.set("zip", lambda a, b: [[x, y] for x, y in zip(a, b)])  # 50
    env.set("sorted", sorted)                       # 51
    env.set("reversed", lambda lst: list(reversed(lst)))  # 52
    env.set("map", lambda fn, lst: [fn(x) for x in lst])  # 53 (free fn form)
    env.set("filter", lambda fn, lst: [x for x in lst if fn(x)])  # 54
    env.set("flat", lambda lst: [x for sub in lst for x in (sub if isinstance(sub, list) else [sub])])  # 55
    env.set("unique", lambda lst: list(dict.fromkeys(lst)))  # 56
    env.set("take", lambda lst, n: lst[:n])         # 57
    env.set("drop", lambda lst, n: lst[n:])         # 58
    env.set("push", lambda lst, v: lst.append(v) or lst)  # 59
    env.set("pop", lambda lst: lst.pop())           # 60
    env.set("keys", lambda m: list(m.keys()))       # 61
    env.set("values", lambda m: list(m.values()))   # 62
    env.set("entries", lambda m: [[k, v] for k, v in m.items()])  # 63
    env.set("merge", lambda a, b: {**a, **b})       # 64
    env.set("has_key", lambda m, k: k in m)         # 65

    # ===== Type conversion & checking (66-75) =====
    env.set("to_int", _to_int)                      # 66
    env.set("to_float", _to_float)                  # 67
    env.set("to_str", _to_str)                      # 68
    env.set("to_bool", _to_bool)                    # 69
    env.set("to_list", _to_list)                    # 70
    env.set("typeof", _typeof)                      # 71
    env.set("is_int", lambda v: isinstance(v, int) and not isinstance(v, bool))  # 72
    env.set("is_float", lambda v: isinstance(v, float))  # 73
    env.set("is_str", lambda v: isinstance(v, str))  # 74
    env.set("is_list", lambda v: isinstance(v, list))  # 75

    # ===== System (76-80) =====
    env.set("is_map", lambda v: isinstance(v, dict))  # 76
    env.set("is_none", lambda v: v is None)         # 77
    env.set("is_bool", lambda v: isinstance(v, bool))  # 78
    env.set("sleep", lambda ms: _time.sleep(ms / 1000))  # 79
    env.set("exit", lambda code=0: exit(code))      # 80

    # === Bonus extras (commonly used) ===
    env.set("assert", _ts_assert)
    env.set("hash", hash)
    env.set("read_lines", _read_lines)
    env.set("write_lines", _write_lines)
    env.set("read_json", _read_json)
    env.set("write_json", _write_json)
    env.set("file_exists", lambda p: os.path.exists(p))
    env.set("delete_file", lambda p: os.remove(p))
    env.set("list_dir", lambda p=".": os.listdir(p))
    env.set("make_dir", lambda p: os.makedirs(p, exist_ok=True))
    env.set("print_env", lambda: None)   # placeholder

    # Error constructors
    env.set("Error", lambda msg: TechScriptError(msg))
    env.set("ValueError", lambda msg: ValueErr(msg))
