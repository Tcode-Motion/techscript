# TechScript Migration Guide: 1.0.8 → 2.0

> Migrating from TechScript 1.0.8 syntax to the canonical TechScript 2.0 dialect.
> **Version:** 2.0.0 | **Last Updated:** 2026-07-26

---

## Overview

TechScript 2.0 introduces a cleaner, more consistent syntax. All deprecated 1.0.8 constructs
have been replaced with canonical forms. This guide walks you through every change, provides
automated tooling to handle most conversions, and flags the edge cases that require manual review.

> [!IMPORTANT]
> TechScript 2.0 is **not backward-compatible** with 1.0.8 syntax in production builds.
> The compiler emits `TSWxxxx` warnings for deprecated forms in strict mode and errors in release mode.

---

## Automated Migration

Run the official migration tool against your project root **before** making any manual edits:

```
tsc migrate .
```

The tool performs a best-effort pass and handles all patterns listed in the table below that carry
a `TSWxxxx` code. After it finishes, open the migration report (`tsc-migrate-report.txt`) and
review every file flagged with `[MANUAL]`.

> [!NOTE]
> `tsc migrate` will **not** touch files inside `examples/compat/`. See the
> [Compat Layer](#compat-layer) section at the end of this guide.

---

## Complete Syntax Mapping Table

| Old (1.0.8) | New (2.0) | TSW Code | Notes |
|---|---|---|---|
| `make x = 5` | `x = 5` | TSW1001 | First assignment declares the variable |
| `let x = 5` | `x = 5` | TSW1001 | Same rule — no declaration keyword needed |
| `var x = 5` | `x = 5` | TSW1001 | Same rule |
| `build fn() {` | `do fn()` | TSW1002 | All function-declaration keywords unified |
| `fun fn() {` | `do fn()` | TSW1002 | |
| `function fn() {` | `do fn()` | TSW1002 | |
| `return x;` | `send x` | TSW1003 | No semicolon; `send` is the sole return keyword |
| `give x;` | `send x` | TSW1005 | `give` was an alias — now removed |
| `if cond {` | `when cond` | TSW1007 | No parentheses around condition required |
| `elif cond {` | `else when cond` | TSW1007 | Chained with `else when` |
| `} else {` | `else` | TSW1007 | |
| `while cond {` | `repeat cond` | TSW1008 | Condition-controlled loop |
| `each x in y {` | `for x in y` | TSW1010 | |
| `attempt {` | `try` | TSW1004 | |
| `} catch e {` | `catch e` | TSW1004 | |
| `import mod` | `use mod` | TSW1009 | Single import keyword for all module forms |
| `from mod import x` | `use mod` | TSW1009 | Access member as `mod.x` after `use mod` |
| `model Name {` | `class Name` | TSW1013 | |
| `none` | `null` | TSW1011 | |
| `f"Hello {x}"` | `$"Hello {x}"` | TSW1012 | Only the prefix changes; braces stay the same |
| `std.io.println(x)` | `say x` | TSW1014 | Built-in implicit call — no parentheses |
| `keep X = 5` | `const X = 5` | — | Handled by `tsc migrate`; no lint code |
| `stop` | `break` | — | |
| `skip` | `continue` | — | |
| `{ }` blocks | `end` terminator | TSW1006 | Every block closes with `end`, not `}` |
| `;` terminators | *(remove)* | TSW1006 | Semicolons are a hard syntax error in 2.0 |
| `// comment` | `# comment` | — | |
| `/* comment */` | `# comment` (per line) | — | Block comments entirely removed |

---

## Step-by-Step Examples

### 1 — Hello World Function

**Before (1.0.8)**

```
# 1.0.8 — hello.ts
import std.io

build greet(name) {
    let msg = f"Hello, {name}!"
    std.io.println(msg)
    return msg;
}
```

**After (2.0)**

```
# hello.ts
use std.io

do greet(name)
    msg = $"Hello, {name}!"
    say msg
    send msg
end
```

Key changes:
- `import` → `use`
- `build` → `do`
- `let` declaration keyword removed
- `f"..."` → `$"..."`
- `std.io.println(x)` → `say x`
- `return` → `send`
- Curly braces replaced by `end`
- Semicolons removed

---

### 2 — Class with Methods

**Before (1.0.8)**

```
# 1.0.8 — counter.ts
model Counter {
    make count = 0

    fun increment() {
        this.count = this.count + 1
    }

    fun reset() {
        this.count = 0
    }

    fun value() {
        return this.count;
    }
}
```

**After (2.0)**

```
# counter.ts
class Counter
    count = 0

    do increment()
        this.count = this.count + 1
    end

    do reset()
        this.count = 0
    end

    do value()
        send this.count
    end
end
```

Key changes:
- `model` → `class`
- `make` declaration keyword removed
- `fun` → `do`
- `return` → `send`
- All `{ }` blocks replaced by `end`
- Semicolons removed

---

### 3 — Error Handling

**Before (1.0.8)**

```
# 1.0.8 — fetch.ts
import http
import json

fun fetchUser(url) {
    attempt {
        let resp = http.get(url)
        let data = json.parse(resp)
        if data == none {
            return none;
        }
        return data;
    } catch e {
        std.io.println(f"Error: {e}")
        return none;
    }
}
```

**After (2.0)**

```
# fetch.ts
use http
use json

do fetchUser(url)
    try
        resp = http.get(url)
        data = json.parse(resp)
        when data == null
            send null
        end
        send data
    catch e
        say $"Error: {e}"
        send null
    end
end
```

Key changes:
- `import` → `use`
- `fun` → `do`
- `attempt` → `try`
- `} catch e {` → `catch e`
- `none` → `null`
- `if` → `when`
- `return` → `send`
- `f"..."` → `$"..."`
- `std.io.println(x)` → `say x`
- All blocks closed with `end`
- All semicolons removed

---

### 4 — Loop with Conditionals

**Before (1.0.8)**

```
# 1.0.8 — process.ts
fun processItems(items) {
    make results = []
    var i = 0
    while i < items.len {
        let item = items[i]
        if item.active == none {
            skip
        } elif item.score > 90 {
            results.push(f"Top: {item.name}")
        } else {
            results.push(item.name)
        }
        i = i + 1
    }
    return results;
}
```

**After (2.0)**

```
# process.ts
do processItems(items)
    results = []
    i = 0
    repeat i < items.len
        item = items[i]
        when item.active == null
            continue
        else when item.score > 90
            results.push($"Top: {item.name}")
        else
            results.push(item.name)
        end
        i = i + 1
    end
    send results
end
```

Key changes:
- `fun` → `do`
- `make` / `var` / `let` declaration keywords removed
- `while` → `repeat`
- `if` → `when`
- `elif` → `else when`
- `} else {` → `else`
- `skip` → `continue`
- `none` → `null`
- `f"..."` → `$"..."`
- `return` → `send`
- All blocks closed with `end`
- All semicolons removed

> [!TIP]
> When a loop runs a fixed number of times, prefer `loop N` over `repeat` with a counter.
> For example, `loop 10` replaces a `while i < 10 { i = i + 1 }` pattern entirely.

---

## Common Pitfalls

### `repeat` vs `loop`

`repeat cond` is the **condition-controlled** (while) loop. Do **not** use it when you only
need to run a block a fixed number of times — use `loop N` instead.

```
# WRONG — using repeat for a counted loop
i = 0
repeat i < 5
    say "tick"
    i = i + 1
end

# CORRECT — counted loop
loop 5
    say "tick"
end
```

### `null`, not `none`

`none` is a hard syntax error in 2.0. Every occurrence must become `null`.
`tsc migrate` handles this automatically, but double-check hand-written strings that
contain the word `none` — the tool does not modify string literals.

### `class`, not `model`

The `model` keyword no longer parses. The migration tool converts it, but any dynamically
generated source that emits `model` will fail at compile time.

### Comment Style

Only `#` line comments are valid in 2.0. Both `//` and `/* */` are syntax errors.

```
# WRONG
// this is a comment
/* block comment */

# CORRECT
# this is a comment
# block comment line 1
# block comment line 2
```

### No Semicolons — Ever

Semicolons are not optional delimiters in 2.0; they are a **hard parse error**. The compiler
will not emit a warning — it will refuse to compile the file.

### No Curly Braces for Blocks

Every block — functions, conditionals, loops, classes, try/catch — is terminated with the
`end` keyword. Curly braces are a parse error.

### Built-in Implicit Call Style

`say`, `ask`, `env`, and `file` are built-in statements, **not** functions. Do not add
parentheses:

```
# WRONG
say("hello")
name = ask("Enter name: ")

# CORRECT
say "hello"
name = ask "Enter name: "
```

### Standard Library Qualified Calls

Standard library functions use qualified dot notation and **do** require parentheses:

```
# CORRECT
result = math.abs(-5)
data   = json.parse(raw)
resp   = http.get(url)
```

### `from mod import x` Pattern

In 1.0.8, `from mod import x` let you use `x` directly. In 2.0, `use mod` imports the
whole module and you access members as `mod.x`. Update all unqualified references after
migrating import statements.

```
# 1.0.8
from json import parse
data = parse(raw)

# 2.0
use json
data = json.parse(raw)
```

---

## Checking for Remaining Issues

After running `tsc migrate`, run the linter to catch any patterns the automated tool missed:

```
tsc lint .
```

Any remaining `TSW100x` warnings identify lines that still use deprecated 1.0.8 syntax.
Resolve each warning manually using this guide, then re-run until the output is clean.

> [!WARNING]
> Do **not** suppress `TSW100x` warnings with inline directives as a workaround.
> Deprecated syntax is scheduled for removal in 2.1. Fix the root cause instead.

---

## Compat Layer

`examples/compat/` contains test files that **intentionally** use deprecated 1.0.8 syntax
to verify the compiler's backward-compatibility and warning infrastructure. Do **not** run
`tsc migrate` on these files — they must remain in their original form.

The `tsc migrate` tool skips this directory automatically. If you invoke the tool on a
specific file path inside `examples/compat/`, it will abort with exit code `2` and print:

```
[SKIP] examples/compat/<file> is a protected compat fixture. Aborting.
```

---

## Quick Reference Card

| Task | 1.0.8 | 2.0 |
|---|---|---|
| Declare variable | `make x = 1` | `x = 1` |
| Declare constant | `keep X = 1` | `const X = 1` |
| Define function | `build f() {` | `do f()` |
| Return value | `return x;` | `send x` |
| Conditional | `if c {` | `when c` |
| Else-if | `elif c {` | `else when c` |
| While loop | `while c {` | `repeat c` |
| Counted loop | *(manual counter)* | `loop N` |
| For-each | `each x in y {` | `for x in y` |
| Try/catch | `attempt {` / `} catch e {` | `try` / `catch e` |
| Import | `import mod` | `use mod` |
| Class | `model Name {` | `class Name` |
| Null literal | `none` | `null` |
| String interp | `f"Hi {x}"` | `$"Hi {x}"` |
| Print | `std.io.println(x)` | `say x` |
| Block end | `}` | `end` |
| Line comment | `//` | `#` |
| No semicolons | `;` | *(omit entirely)* |

---

*For questions or to report migration issues, open a ticket tagged `migration-2.0` in the project tracker.*
