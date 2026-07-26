# TechScript 2.0 — Language Freeze Declaration

> **Status**: FROZEN — PERMANENT SPECIFICATION
> **Date**: 2026-07-26
> **Version**: 2.0.0
> **Authority**: Core Language Team

---

## Summary

The TechScript 2.0 language syntax is **frozen**. This document is the permanent,
authoritative record of the frozen language design. No syntax changes, keyword
additions, or semantic rule modifications will be made in the **2.x series**
without a documented major version bump.

This is a **single source of truth** document. All compiler implementations,
documentation, tools, examples, and templates must align with it.

---

## Frozen Design Decisions

### Q1 — Compatibility Folder Strategy

> **Decision**: Keep `examples/compat/` permanently.

- `examples/compat/` is preserved in the repository as a **permanent compatibility test suite**.
- Every file in `examples/compat/` must have `# LEGACY COMPAT TEST` as the first line.
- These files use intentional deprecated syntax to verify that the compiler:
  - Still parses the old dialect correctly.
  - Emits the correct `TSW100x` deprecation warnings.
  - Does **not** emit errors (deprecated ≠ rejected).
- Do **not** run `tsc migrate` on these files.

---

### Q2 — Stdlib Call Style

> **Decision**: Qualified calls for stdlib. Implicit calls for true built-ins only.

**Canonical style — True Built-ins** (no module prefix, no parentheses):

```txs
say "Hello"
name = ask "What is your name?"
path = env "PATH"
content = file "readme.txt"
```

**Canonical style — Stdlib modules** (qualified, with parentheses):

```txs
use math
use json
use http
use crypto

result = math.abs(-42)
root   = math.sqrt(25)
parsed = json.parse(data)
resp   = http.get(url)
hash   = crypto.sha256(input)
```

**Rationale**: Implicit calls for stdlib would cause ambiguous name collisions
(e.g. a user variable named `abs` vs. `math.abs`). Built-ins (`say`, `ask`, `env`,
`file`) are special-cased in the parser and guaranteed to be unique.

---

### Q3 — Null Literal

> **Decision**: `null` is the only canonical null literal. `none` is a deprecated alias.

**Canonical**:
```txs
value = null
```

**Deprecated** (still compiles, emits TSW1011):
```txs
value = none    # TSW1011: 'none' is deprecated. Use 'null'.
```

**Compiler behaviour**:
- `none` is parsed and immediately lowered to the `Null` AST node.
- No runtime changes. The distinction is purely syntactic.

---

### Q4 — Loop Semantics

> **Decision**: `loop N` = counted loop. `repeat condition` = while loop.

These are permanently distinct constructs with no overlap.

**`loop N`** — runs exactly N times:
```txs
loop 10
    say "Hello"
end
```

**`repeat condition`** — runs while condition is true:
```txs
running = true

repeat running
    update()
    when done
        running = false
    end
end
```

**Deprecated alias**:
- `while cond` → `repeat cond` (TSW1008)
- There is no deprecated equivalent for `loop N` (it is entirely new in 2.0)

---

### Q5 — String Interpolation Prefix

> **Decision**: `$"..."` is canonical. `f"..."` is a deprecated alias.

**Canonical**:
```txs
name = "Boss"
say $"Hello {name}!"
say $"Result is {a + b}"
```

**Deprecated** (still compiles, emits TSW1012):
```txs
say f"Hello {name}"    # TSW1012: 'f"..."' is deprecated. Use '$"..."'.
```

**Compiler behaviour**:
- Both `$"..."` and `f"..."` lex to the same `FStringStart` token internally.
- `FStringStart.static_lexeme()` returns `"$\""` (the canonical form).
- `f"..."` prefix triggers TSW1012 in the lexer before emitting `FStringStart`.

---

## Canonical Keyword Table (Frozen)

| Keyword | Category | Replaces |
|---|---|---|
| `do` | Function declaration | `build`, `fun`, `function` |
| `send` | Return value | `return`, `give` |
| `when` | Conditional | `if`, `elif` → `else when` |
| `loop` | Counted loop (NEW) | — |
| `repeat` | While-style loop | `while` |
| `for` | For-each iteration | `each` |
| `in` | Iteration boundary | — |
| `match` | Pattern match | `switch` |
| `case` | Match arm | — |
| `default` | Default match arm (NEW) | — |
| `try` | Error block | `attempt` |
| `catch` | Error handler | — |
| `throw` | Raise error | — |
| `use` | Module import | `import`, `from` |
| `class` | Class definition | `model` |
| `struct` | Struct definition | — |
| `enum` | Enum definition | — |
| `trait` | Trait definition | — |
| `interface` | Interface definition | — |
| `const` | Constant declaration | `keep` |
| `null` | Null literal | `none` |
| `say` | Print (implicit) | `std.io.println(x)` |
| `ask` | Read (implicit) | — |
| `break` | Exit loop | `stop` |
| `continue` | Next iteration | `skip` |
| `else` | Else branch | — |
| `async` | Async block | — |
| `await` | Await expression | — |
| `parallel` | Parallel block (NEW) | — |
| `end` | Block terminator | `{` / `}` |
| `export` | Export declaration | — |
| `new` | Instantiation | — |
| `self` | Self-reference | — |
| `true` | Boolean literal | — |
| `false` | Boolean literal | — |
| `typeof` | Type evaluation | — |
| `with` | Supplemental block | — |

---

## Deprecated Keyword Table (Frozen)

All deprecated keywords **compile without errors** in the 2.x series.
They emit `TSW1xxx` warnings. Use `tsc migrate .` to auto-convert.

| Deprecated | TSW Code | Canonical Replacement |
|---|---|---|
| `make x = 5` | TSW1001 | `x = 5` (plain assignment) |
| `let x = 5` | TSW1001 | `x = 5` |
| `var x = 5` | TSW1001 | `x = 5` |
| `build fn()` | TSW1002 | `do fn()` |
| `fun fn()` | TSW1002 | `do fn()` |
| `function fn()` | TSW1002 | `do fn()` |
| `return x` | TSW1003 | `send x` |
| `attempt` | TSW1004 | `try` |
| `give x` | TSW1005 | `send x` |
| `{ }` blocks | TSW1006 | `end` |
| `;` terminators | TSW1006 | (remove) |
| `if cond` | TSW1007 | `when cond` |
| `elif cond` | TSW1007 | `else when cond` |
| `while cond` | TSW1008 | `repeat cond` |
| `import mod` | TSW1009 | `use mod` |
| `from mod import x` | TSW1009 | `use mod` |
| `each x in y` | TSW1010 | `for x in y` |
| `none` | TSW1011 | `null` |
| `f"..."` | TSW1012 | `$"..."` |
| `model Name` | TSW1013 | `class Name` |
| `std.io.println(x)` | TSW1014 | `say x` |

---

## New Token Variants Added in 2.0.0

Three new `TokenKind` variants were added to `compiler/syntax/src/token_kind.rs`:

| Variant | Keyword | Semantics |
|---|---|---|
| `Loop` | `loop` | Counted loop — `loop N` runs exactly N times |
| `Parallel` | `parallel` | Parallel execution block |
| `Default` | `default` | Default arm in `match` statements |

---

## Compatibility Guarantee

> All TechScript 1.0.x source files (`.txs`) that compiled without errors will
> continue to compile in 2.x with deprecation warnings but **without errors**.

This guarantee holds for the entire 2.x series. Removal of deprecated syntax
is not scheduled before 3.0.

---

## Enforcement

The following mechanisms enforce this freeze:

1. **`is_canonical_keyword()`** — returns `true` only for the 2.0 canonical set.
2. **`is_alias_keyword()`** — returns `true` for all deprecated keywords.
3. **`to_canonical()`** — maps every deprecated keyword to its canonical equivalent.
4. **`lookup_keyword()`** — recognises all keywords (canonical + deprecated + reserved).
5. **TSW1001–TSW1014** — diagnostic codes emitted for each deprecated pattern.
6. **`tsc migrate`** — auto-migrates deprecated patterns to canonical forms.
7. **`tsc lint`** — flags remaining deprecated patterns without modifying files.
8. **`examples/compat/`** — permanent test suite verifying backward compatibility.

---

## Frozen By

This document was generated from confirmed language freeze decisions on 2026-07-26.
The decisions are final and permanent for the TechScript 2.x series.
