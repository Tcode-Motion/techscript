# 10 — TechScript 2.0 Semantic Analysis Specification

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-26
> **Related Documents**: [05 AST Design](./05_ast_design.md) · [09 Runtime Design](./09_runtime_design.md) · [11 Interpreter Design](./11_interpreter_design.md) · [14 Error Codes](./14_error_codes.md)

---

## 1. Overview

The semantic analyzer processes the AST, validating scopes, resolving identifiers,
constructing the symbol table, and issuing deprecation warnings. It runs in two
passes: Pass 1 hoists top-level declarations; Pass 2 validates all statements.

---

## 2. Scope & Name Resolution

A lexical **Scope Frame** contains symbol references. Name resolution maps every
identifier node to its matching declaration. Hoisting in Pass 1 pre-registers:
- Top-level `do` function declarations
- Top-level `class` definitions
- Top-level `const` declarations

This allows forward references to functions declared later in the file.

---

## 3. Canonical Semantic Rules (Frozen 2.0)

The following 10 rules are frozen and authoritative for TechScript 2.0:

### Rule 1 — Auto-Declaration

The first assignment to an identifier in a scope **declares** it. No `make`, `let`,
or `var` keyword is required or allowed in canonical 2.0 code.

```txs
name = "Alice"    # declares 'name' in current scope
age = 30          # declares 'age'
```

> Use of `make`/`let`/`var` emits **TSW1001** and is stripped at compile time.

---

### Rule 2 — Const Immutability

Identifiers declared with `const` cannot be reassigned. Attempting to do so is a
**compile-time hard error** (`TSE0302`).

```txs
const MAX = 100
MAX = 200          # TSE0302: Cannot reassign `const`
```

---

### Rule 3 — Shadowing

A declaration in an inner scope may shadow an outer scope identifier. This is
**allowed** but emits **TSW2002** (Variable shadows outer scope).

```txs
x = 10
do example()
    x = 99     # TSW2002: 'x' shadows outer scope
end
```

---

### Rule 4 — Closures & Scope Capture

Functions capture the **value** of outer scope variables at the time of the
function call (copy semantics for primitives; reference semantics for objects).
Lambda expressions capture the enclosing scope lexically.

---

### Rule 5 — Loop Variables

Variables introduced inside a `for`/`loop`/`repeat` block are **scoped to the
loop body**. They are not accessible after the loop ends.

```txs
for item in list
    total += item
end
say total    # ok — total was declared outside
say item     # TSE0300: Undefined variable 'item'
```

---

### Rule 6 — Catch Variable Scope

The error variable in a `catch` block is scoped exclusively to that block:

```txs
try
    throw "problem"
catch err
    say err      # ok
end
say err          # TSE0300: Undefined variable 'err'
```

---

### Rule 7 — Class Field Defaults

Class fields without an explicit initializer are **automatically set to `null`**.
No separate constructor initialisation is required.

```txs
class Point
    x = 0      # explicit default
    label      # implicitly null (TSW2001 if never assigned before use)
end
```

---

### Rule 8 — `send` Outside Function

Using `send` (or the deprecated `return`/`give`) outside a `do` function body is
a **compile-time hard error** (`TSE0312`).

```txs
send 42    # TSE0312: `send` outside function body
```

---

### Rule 9 — Division by Zero

Integer division by the literal `0` is detected at **compile time** when the
divisor is a constant expression. At runtime it raises **TSE1010**.

---

### Rule 10 — Undefined Variable Access

Reading a variable that has not yet been assigned in the current scope is a
**compile-time error** (`TSE0300`). Variables are not auto-created on read.

```txs
say missing_var    # TSE0300: Undefined variable 'missing_var'
```

---

## 4. Deprecation Warning Issuance

The semantic pass issues `TSW` warnings for all deprecated syntax patterns that
the parser preserves (to support backward compatibility). Key triggers:

| Trigger | Code | Canonical replacement |
|---|---|---|
| `build`/`fun`/`function` keyword | `TSW1002` | `do` |
| `make`/`let`/`var` keyword | `TSW1001` | plain assignment |
| `return` | `TSW1003` | `send` |
| `model` keyword | `TSW1013` | `class` |
| `if`/`elif` keyword | `TSW1007` | `when`/`else when` |
| `while` keyword | `TSW1008` | `repeat` |
| `import`/`from` keyword | `TSW1009` | `use` |
| `each` keyword | `TSW1010` | `for` |
| `attempt` keyword | `TSW1004` | `try` |
| `give` keyword | `TSW1005` | `send` |
| `none` literal | `TSW1011` | `null` |
| `f"..."` string | `TSW1012` | `$"..."` |
| `std.io.println(x)` | `TSW1014` | `say x` |
| Unused variable | `TSW2001` | (remove or use) |
| Variable shadows outer scope | `TSW2002` | (rename or intentional) |

---

## 5. Diagnostics & Suggestions

When errors or warnings are found, they are gathered in the diagnostics vector.
- For name typos, Levenshtein distance generates suggestions ("Did you mean X?").
- Deprecation warnings never set `has_errors = true` — they are non-blocking.
- TSE errors set `has_errors = true` — compilation halts before code generation.

---

## 6. Compatibility & Evolution

### 6.1 Compatibility Notes
- **Deprecated Keyword Parsing**: The semantic analyzer processes deprecated keywords
  (e.g. `build`, `if`, `model`) via their lowered canonical equivalents. No behavioural
  difference exists between canonical and deprecated forms at runtime.
- **TSW Warnings Are Non-Blocking**: All `TSW` codes produce warnings but do not
  prevent program execution. This preserves backward compatibility for the 2.x series.

### 6.2 Migration
Run `tsc migrate .` to auto-convert TSW1001–TSW1013 patterns. Remaining warnings
can be found with `tsc lint .`. `examples/compat/` files are intentionally exempt.

### 6.3 Future Roadmap
- **v2.2**: Optional type annotations — the semantic analyzer will validate annotated
  parameter types against call-site argument types.
- **v3.0**: Optimizations will leverage type analysis records to emit optimized
  native instruction sequences.
