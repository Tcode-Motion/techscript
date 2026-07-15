# 01 — TechScript v2.0 Language Specification

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [03 Grammar](./03_grammar_ebnf.md) · [06 Lexer](./06_lexer_design.md) · [12 Stdlib](./12_stdlib_design.md) · [14 Error Codes](./14_error_codes.md)

---

## Table of Contents

1. [Official Decisions](#1-official-decisions)
2. [Keywords](#2-keywords)
3. [Operators](#3-operators)
4. [Primitive Types](#4-primitive-types)
5. [Variables](#5-variables)
6. [Functions & Methods](#6-functions--methods)
7. [Modules and Imports](#7-modules-and-imports)
8. [Loops](#8-loops)
9. [Conditionals](#9-conditionals)
10. [Error Handling](#10-error-handling)
11. [Objects (Models)](#11-objects-models)
12. [Comments](#12-comments)
13. [Strings](#13-strings)
14. [Collections](#14-collections)
15. [Built-in Functions](#15-built-in-functions)
16. [Compatibility & Evolution Analysis](#16-compatibility--evolution-analysis)

---

## 1. Official Decisions

The following decisions are frozen and authoritative for TechScript 2.0:

| Decision | Specification | Rationale |
|---|---|---|
| **File Extension** | `.txs` | Frozen by user constraint. Replaces `.tech` across the entire ecosystem. |
| **Unified Keyword** | `build` | Unified keyword for both standalone functions and class methods. |
| **Deprecated Alias** | `fun` | Inside `model` blocks, the `fun` keyword remains supported but triggers a deprecation warning (`W0015`). |
| **Web Module** | Optional standard library module | Keeps the language core compact while supporting DOM-like page generation. |
| **Typing** | Dynamic typing | Standard dynamic checks at runtime. Optional type annotations are slated for v2.2+. |
| **Semicolons** | Optional | Newlines act as statement terminators unless continuation rules apply. |
| **Block Structure** | Braces `{ }` | Explicit visual grouping. |

---

## 2. Keywords

### 2.1 Reserved Keywords

All reserved keywords are lowercase ASCII.

| Keyword | Category | Purpose |
|---|---|---|
| `make` | Declaration | Mutable variable declaration |
| `const` | Declaration | Immutable variable declaration |
| `say` | I/O | Print to stdout (with newline) |
| `ask` | I/O | Read input from stdin |
| `build` | Function/Method | Define a function or method |
| `return` | Function/Method | Explicit return |
| `fun` | Deprecated Method | Deprecated alias for method definition |
| `model` | Object | Define a class/object type |
| `self` | Object | Reference to current instance |
| `new` | Object | Create an instance of a model |
| `when` | Conditional | If-condition |
| `else` | Conditional | Else-branch |
| `each` | Loop | For-each loop |
| `in` | Loop | Iteration boundary (used with `each`) |
| `repeat` | Loop | Repeat N times |
| `while` | Loop | While-loop |
| `break` | Loop | Exit loop |
| `continue` | Loop | Skip to next iteration |
| `attempt` | Error | Try-block |
| `catch` | Error | Catch-block |
| `throw` | Error | Raise an error |
| `import` | Module | Import a module |
| `from` | Module | Selective import |
| `export` | Module | Export a declaration |
| `true` | Literal | Boolean true |
| `false` | Literal | Boolean false |
| `none` | Literal | Null value |
| `and` | Operator | Logical AND |
| `or` | Operator | Logical OR |
| `not` | Operator | Logical NOT |
| `is` | Operator | Type check / identity comparison |

---

## 3. Operators

### 3.1 Arithmetic Operators
`+` (addition), `-` (subtraction), `*` (multiplication), `/` (division), `//` (integer division), `%` (modulo), `**` (exponentiation).

### 3.2 Comparison Operators
`==` (equal), `!=` (not equal), `<` (less than), `>` (greater than), `<=` (less or equal), `>=` (greater or equal).

### 3.3 Logical Operators
`and` (AND), `or` (OR), `not` (NOT).

### 3.4 Assignment Operators
`=` (assignment), `+=`, `-=`, `*=`, `/=`, `%=` (in-place math updates).

### 3.5 Range Operators
`..` (exclusive range), `..=` (inclusive range).

### 3.6 Member Access and Indexing
`.` (member access), `[]` (index access).

---

## 4. Primitive Types

TechScript 2.0 supports 7 primitive types:
- `Int`: 64-bit signed integer. Supports underscore separators: `1_000_000`.
- `Float`: 64-bit IEEE 754 float.
- `Str`: UTF-8 heap string.
- `Bool`: `true` or `false`.
- `None`: Represents absence of value (literal: `none`).
- `List`: Heterogeneous dynamic array.
- `Map`: Heterogeneous string-keyed associative array (preserves insertion order).

---

## 5. Variables

Declared with `make` (mutable) or `const` (immutable):
```
make count = 0
const MaxItems = 100
```
Lexically scoped to their containing block `{}`. No hoisting.

---

## 6. Functions & Methods

### 6.1 Function Definition
Declared using the `build` keyword:
```
build add(a, b) {
    return a + b
}
```

### 6.2 Method Definition
Declared using the `build` keyword inside a `model`:
```
model Counter {
    make value = 0
    
    build increment() {
        self.value += 1
    }
}
```

### 6.3 Deprecated Syntax
```
model Dog {
    fun bark() { say "Woof!" }  // Deprecated W0015
}
```
`fun` works exactly like `build` but triggers a compile-time warning.

---

## 7. Modules and Imports
Modules correspond to files with `.txs` extension.
```
import math_utils
from math_utils import square
```

---

## 8. Loops
```
each i in 1..10 { say i }
repeat 5 { say "Hello" }
while cond { tick() }
```

---

## 9. Conditionals
```
when x > 10 {
    say "Large"
} else when x > 5 {
    say "Medium"
} else {
    say "Small"
}
```

---

## 10. Error Handling
```
attempt {
    throw "Fatal issue"
} catch err {
    say err
}
```

---

## 11. Objects (Models)
```
model Person {
    make name = ""
    build init(name) {
        self.name = name
    }
}
make p = new Person("Alice")
```

---

## 12. Comments
- `//` single line.
- `/* ... */` multi-line (can nest).

---

## 13. Strings
- Double-quoted strings `"..."`.
- Interpolated f-strings: `f"Hello {name}!"`.

---

## 14. Collections
- Lists: `[1, 2, 3]`.
- Maps: `{"key": "value"}`.

---

## 15. Built-in Functions
Core built-ins: `say(val)`, `ask(prompt)`, `len(collection)`, `type_of(val)`, `to_int(val)`, `to_float(val)`, `to_str(val)`, `to_bool(val)`, `range(start, end)`, `exit(code)`, `assert(cond)`.

---

## 16. Compatibility & Evolution Analysis

### 16.1 Compatibility Notes
- **V1 Code Compatibility**: Programs written for Version 1 will execute under TechScript 2.0 with the exception of file references (which must use `.txs` instead of `.tech`).
- **Method Keyword**: The transition from `fun` to `build` for method declarations inside models is a soft breaking change. The compiler tolerates `fun` but prints warning `W0015`.

### 16.2 Migration Notes
- To migrate code:
  1. Rename all source files to end with `.txs`.
  2. Run `tech lint --fix` to automatically change `fun` keyword occurrences to `build`.
- Programmatic mapping:
  ```
  // Version 1 (Deprecated)
  model Dog {
      fun bark() { say "Woof" }
  }

  // Version 2.0 (Canonical)
  model Dog {
      build bark() { say "Woof" }
  }
  ```

### 16.3 Rationale
- **Keyword Redundancy**: In Version 1, `build` and `fun` served identical purposes but in different scopes. Unifying them under `build` reduces the language's core keyword count, simplifies EBNF grammar production, and ensures that a function is conceptually identical regardless of where it is declared.
- **Strict Extension enforcement**: Transitioning to `.txs` prevents syntax-highlighting overlaps in shared IDE configurations and provides a distinct, clean signature for the TechScript toolchain.

### 16.4 Future Roadmap
- **v2.1**: Introduce VM bytecode optimization for method calls using unified `build` dispatch.
- **v2.2**: Add optional type annotations using a suffix `:` notation (e.g., `build add(a: Int, b: Int) -> Int`).
- **v3.0**: Native code generation optimizations via LLVM for functions defined with `build`.
