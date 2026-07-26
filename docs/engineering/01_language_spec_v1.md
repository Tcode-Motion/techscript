# 01 — TechScript v2.0 Language Specification

| Field        | Value               |
|--------------|---------------------|
| Status       | Authoritative       |
| Version      | 2.0.0               |
| Last Updated | 2026-07-26          |

---

## Table of Contents

1. [Official Decisions](#1-official-decisions)
2. [Keywords](#2-keywords)
3. [Operators](#3-operators)
4. [Primitive Types](#4-primitive-types)
5. [Variables](#5-variables)
6. [Functions & Methods](#6-functions--methods)
7. [Modules](#7-modules)
8. [Loops](#8-loops)
9. [Conditionals](#9-conditionals)
10. [Error Handling](#10-error-handling)
11. [Classes](#11-classes)
12. [Comments](#12-comments)
13. [Strings](#13-strings)
14. [Collections](#14-collections)
15. [Built-in Functions](#15-built-in-functions)
16. [Stdlib Call Style](#16-stdlib-call-style)
17. [Compatibility & Evolution](#17-compatibility--evolution)

---

## §1 Official Decisions

The following table records every finalized language-level decision for TechScript 2.0. All tooling, documentation, and generated code **must** conform to the Spec column exclusively.

| Decision              | Spec                                      | Rationale                                                                 |
|-----------------------|-------------------------------------------|---------------------------------------------------------------------------|
| File Extension        | `.txs`                                    | Unambiguous, short, avoids conflicts with TypeScript (`.ts`)              |
| Function keyword      | `do`                                      | Replaces `build`, `fun`, `function` — imperative, minimal                 |
| Return keyword        | `send`                                    | Replaces `return`, `give` — consistent with data-flow semantics           |
| Conditional           | `when` / `else when` / `else`             | Replaces `if`, `elif`, `else if` — reads as natural language              |
| Counted loop          | `loop N`                                  | New construct; runs exactly N times, no index variable required           |
| While loop            | `repeat cond`                             | Replaces `while` — reads imperatively, avoids keyword collision            |
| For-each              | `for x in y`                              | Replaces `each` — aligns with mainstream language familiarity             |
| Class keyword         | `class`                                   | Replaces `model` — standard OOP terminology                               |
| Module import         | `use mod`                                 | Replaces `import`, `from … import` — single keyword, no path fragmentation|
| Block style           | Indentation + `end`                       | Replaces `{}` — enforces consistent formatting; no brace mismatch bugs   |
| Null literal          | `null`                                    | Replaces `none` — consistent with JSON and most modern languages          |
| String interpolation  | `$"Hello {name}!"`                        | Replaces `f"..."` — dollar-prefix signals interpolated strings            |
| Typing                | Dynamic (optional annotations in v2.2)    | Keeps entry barrier low; static types are opt-in via future annotation    |
| Semicolons            | Never                                     | Newlines are statement terminators; semicolons are a parse error          |

---

## §2 Keywords

### 2.1 Canonical Keywords

The following are the only recognized keywords in TechScript 2.0. No other identifiers are reserved at the language level.

| Keyword      | Category          | Purpose                                           |
|--------------|-------------------|---------------------------------------------------|
| `do`         | Functions         | Declares a named or anonymous function            |
| `send`       | Functions         | Returns a value from a function                   |
| `when`       | Conditionals      | Opens a conditional branch (`if` equivalent)      |
| `else when`  | Conditionals      | Opens a chained conditional branch (`elif` equiv.) |
| `else`       | Conditionals      | Opens the fallback branch                         |
| `loop`       | Loops             | Counted loop — runs exactly N times               |
| `repeat`     | Loops             | Condition-checked loop (`while` equivalent)       |
| `for`        | Loops             | Iterates over a collection                        |
| `in`         | Loops             | Separates iterator variable from iterable         |
| `break`      | Loops             | Exits the enclosing loop immediately              |
| `continue`   | Loops             | Skips to the next iteration of the enclosing loop |
| `match`      | Pattern Matching  | Opens a pattern-match block                       |
| `case`       | Pattern Matching  | Defines a match arm                               |
| `default`    | Pattern Matching  | Defines the fallback match arm                    |
| `try`        | Error Handling    | Opens a guarded execution block                   |
| `catch`      | Error Handling    | Handles an error thrown inside `try`              |
| `throw`      | Error Handling    | Raises an error value                             |
| `use`        | Modules           | Imports a module into the current scope           |
| `class`      | OOP               | Declares a class                                  |
| `struct`     | OOP               | Declares a value-type record                      |
| `enum`       | OOP               | Declares an enumeration                           |
| `trait`      | OOP               | Declares a trait (behaviour contract)             |
| `interface`  | OOP               | Declares an interface                             |
| `new`        | OOP               | Instantiates a class                              |
| `self`       | OOP               | Refers to the current instance                    |
| `const`      | Variables         | Declares an immutable binding                     |
| `null`       | Literals          | Represents the absence of a value                 |
| `true`       | Literals          | Boolean true                                      |
| `false`      | Literals          | Boolean false                                     |
| `say`        | Built-ins         | Prints a value to stdout (no parentheses)         |
| `ask`        | Built-ins         | Reads a line of stdin with an optional prompt     |
| `typeof`     | Built-ins         | Returns the runtime type name as a string         |
| `export`     | Modules           | Makes a declaration visible to importing modules  |
| `async`      | Concurrency       | Marks a function as asynchronous                  |
| `await`      | Concurrency       | Waits for an async value                          |
| `parallel`   | Concurrency       | Runs a block concurrently                         |
| `end`        | Block terminators | Closes any open block (function, loop, class, …)  |

### 2.2 Deprecated Aliases

The following identifiers were canonical in TechScript 1.x. They are recognised by the 2.x compiler but emit deprecation warnings. **Do not use them in new code or documentation.**

| Deprecated Keyword   | Canonical Replacement | TSW Warning Code |
|----------------------|-----------------------|------------------|
| `make`               | direct assignment     | TSW1001          |
| `let`                | direct assignment     | TSW1001          |
| `var`                | direct assignment     | TSW1001          |
| `build`              | `do`                  | TSW1002          |
| `fun`                | `do`                  | TSW1002          |
| `function`           | `do`                  | TSW1002          |
| `return`             | `send`                | TSW1003          |
| `give`               | `send`                | TSW1005          |
| `attempt`            | `try`                 | TSW1004          |
| `if`                 | `when`                | TSW1007          |
| `elif`               | `else when`           | TSW1007          |
| `while`              | `repeat`              | TSW1008          |
| `import`             | `use`                 | TSW1009          |
| `from … import`      | `use`                 | TSW1009          |
| `each`               | `for`                 | TSW1010          |
| `none`               | `null`                | TSW1011          |
| `f"..."`             | `$"..."`              | TSW1012          |
| `model`              | `class`               | TSW1013          |

---

## §3 Operators

### Arithmetic

| Operator | Operation        | Example       |
|----------|------------------|---------------|
| `+`      | Addition         | `a + b`       |
| `-`      | Subtraction      | `a - b`       |
| `*`      | Multiplication   | `a * b`       |
| `/`      | Division         | `a / b`       |
| `%`      | Modulo           | `a % b`       |
| `**`     | Exponentiation   | `a ** b`      |

### Comparison

| Operator | Operation              | Example       |
|----------|------------------------|---------------|
| `==`     | Equal                  | `a == b`      |
| `!=`     | Not equal              | `a != b`      |
| `<`      | Less than              | `a < b`       |
| `>`      | Greater than           | `a > b`       |
| `<=`     | Less than or equal     | `a <= b`      |
| `>=`     | Greater than or equal  | `a >= b`      |

### Logical

| Operator | Operation   | Example         |
|----------|-------------|-----------------|
| `and`    | Logical AND | `a and b`       |
| `or`     | Logical OR  | `a or b`        |
| `not`    | Logical NOT | `not a`         |

### Assignment

| Operator | Operation      | Example       |
|----------|----------------|---------------|
| `=`      | Assignment     | `x = 5`       |
| `+=`     | Add-assign     | `x += 1`      |
| `-=`     | Subtract-assign| `x -= 1`      |
| `*=`     | Multiply-assign| `x *= 2`      |
| `/=`     | Divide-assign  | `x /= 2`      |
| `%=`     | Modulo-assign  | `x %= 3`      |

### Bitwise

| Operator | Operation   | Example       |
|----------|-------------|---------------|
| `\|`     | Bitwise OR  | `a \| b`      |
| `&`      | Bitwise AND | `a & b`       |
| `^`      | Bitwise XOR | `a ^ b`       |
| `~`      | Bitwise NOT | `~a`          |
| `<<`     | Left shift  | `a << 2`      |
| `>>`     | Right shift | `a >> 2`      |

### Range & Special

| Operator | Operation               | Example             |
|----------|-------------------------|---------------------|
| `..`     | Exclusive range         | `1..10`             |
| `..=`    | Inclusive range         | `1..=10`            |
| `?.`     | Optional chaining       | `user?.address`     |
| `??`     | Null coalescing         | `name ?? "unknown"` |

---

## §4 Primitive Types

| Type    | Description                        | Literal Example            |
|---------|------------------------------------|----------------------------|
| `Int`   | Signed integer                     | `42`, `-7`                 |
| `Float` | IEEE 754 double-precision float    | `3.14`, `-0.5`             |
| `Str`   | UTF-8 string                       | `"hello"`, `$"Hi {name}"` |
| `Bool`  | Boolean                            | `true`, `false`            |
| `null`  | Absence of a value (null literal)  | `null`                     |
| `List`  | Ordered heterogeneous sequence     | `[1, 2, 3]`                |
| `Map`   | Key-value store                    | `{"a": 1, "b": 2}`        |

---

## §5 Variables

Variables are declared by first assignment — no declaration keyword is needed. Use `const` for immutable bindings.

```txs
name = "Alice"
age = 30
is_active = true
score = 98.6
nothing = null

const PI = 3.14159
const MAX_RETRIES = 5
```

> **Rule:** `make`, `let`, and `var` are deprecated and must not appear in 2.0 code. First assignment is the declaration. `const` is the only modifier keyword.

---

## §6 Functions & Methods

### Named Functions

```txs
do add(a, b)
    send a + b
end

do greet(name = "World")
    say $"Hello, {name}!"
end
```

### Anonymous (Lambda) Functions

```txs
double = do(x) -> x * 2

transform = do(items, fn)
    result = []
    for item in items
        result += [fn(item)]
    end
    send result
end
```

### Async Functions

```txs
async do fetch_data(url)
    response = await http.get(url)
    send response
end
```

> **Rule:** `build`, `fun`, and `function` are deprecated. `do` is the only function declaration keyword. `send` is the only return keyword — `return` and `give` are deprecated.

---

## §7 Modules

Modules are imported with `use`. The module name becomes the qualified namespace.

```txs
use math
use http
use json
use fs
```

Selective symbol access uses dot notation after import:

```txs
use math

result = math.abs(-42)
root = math.sqrt(16)
```

Export a symbol from a module:

```txs
export do calculate(x)
    send x * 2
end
```

> **Rule:** `import` and `from … import` are deprecated. `use` is the only import keyword.

---

## §8 Loops

### Counted Loop — `loop N`

Runs the body exactly N times. No index variable is created implicitly.

```txs
loop 5
    say "Hello"
end
```

To access a loop index, use a manual counter:

```txs
i = 0
loop 5
    say $"Iteration {i}"
    i += 1
end
```

### Condition Loop — `repeat cond`

Runs while the condition is truthy. Equivalent to `while` in other languages.

```txs
count = 0
repeat count < 10
    count += 1
end
```

### For-Each Loop — `for x in y`

Iterates over any iterable (List, Map keys, range, string characters).

```txs
items = ["apple", "banana", "cherry"]

for item in items
    say item
end

for i in 1..=5
    say i
end
```

### Loop Control

```txs
for item in items
    when item == "skip"
        continue
    end
    when item == "stop"
        break
    end
    say item
end
```

> **Rule:** `while` and `each` are deprecated. Use `repeat` and `for … in` respectively.

---

## §9 Conditionals

### `when` / `else when` / `else`

```txs
when x > 10
    say "Large"
else when x > 5
    say "Medium"
else
    say "Small"
end
```

### Pattern Matching — `match` / `case` / `default`

```txs
match status
    case "ok"
        say "All good"
    end
    case "error"
        say "Something failed"
    end
    default
        say "Unknown status"
    end
end
```

> **Rule:** `if` and `elif` are deprecated. Use `when` and `else when`. `switch` is deprecated; use `match`.

---

## §10 Error Handling

### `try` / `catch` / `throw`

```txs
try
    throw "Fatal issue"
catch err
    say $"Caught: {err}"
end
```

### Throwing Custom Errors

```txs
do divide(a, b)
    when b == 0
        throw "Division by zero"
    end
    send a / b
end

try
    result = divide(10, 0)
catch err
    say err
end
```

> **Rule:** `attempt` is deprecated. Use `try` / `catch` / `throw`.

---

## §11 Classes

### Class Declaration

```txs
class Person
    name = ""
    age = 0

    do init(n, a)
        self.name = n
        self.age = a
    end

    do greet()
        say $"Hi, I am {self.name}, age {self.age}."
    end
end
```

### Instantiation

```txs
p = new Person()
p.init("Alice", 30)
p.greet()
```

### Inheritance

```txs
class Employee extends Person
    role = ""

    do init(n, a, r)
        self.name = n
        self.age = a
        self.role = r
    end

    do introduce()
        say $"{self.name} works as {self.role}."
    end
end

e = new Employee()
e.init("Bob", 25, "Engineer")
e.introduce()
```

> **Rule:** `model` is deprecated. Use `class` exclusively.

---

## §12 Comments

TechScript 2.0 uses `#` for all inline and block comments. No other comment syntax is recognised.

```txs
# This is a line comment

# ------------------------------------------------------------------
# Section Divider — use two dashes minimum
# ------------------------------------------------------------------

name = ask "Enter your name"  # inline comment after code
```

> **Rule:** `//` and `/* … */` are not valid TechScript syntax. Any occurrence is a parse error.

---

## §13 Strings

All string literals are double-quoted. Single quotes are not valid string delimiters.

### Plain Strings

```txs
greeting = "Hello, World!"
path = "C:/Users/Alice/docs"
empty = ""
```

### Interpolated Strings

Use the `$"…"` prefix. Expressions inside `{…}` are evaluated at runtime.

```txs
name = "Alice"
age = 30

say $"Hello, {name}!"
say $"You are {age} years old."
say $"Next year you will be {age + 1}."
```

### Escape Sequences

| Sequence | Meaning              |
|----------|----------------------|
| `\n`     | Newline              |
| `\t`     | Tab                  |
| `\\`     | Literal backslash    |
| `\"`     | Literal double quote |

> **Rule:** `f"…"` (Python-style f-strings) are deprecated. Use `$"…"` exclusively.

---

## §14 Collections

### Lists

Ordered, zero-indexed, heterogeneous sequences.

```txs
numbers = [1, 2, 3, 4, 5]
mixed = [1, "hello", true, null]
empty_list = []

# Access
first = numbers[0]

# Append (returns new list)
numbers += [6]
```

### Maps

Key-value stores. Keys must be strings.

```txs
person = {"name": "Alice", "age": 30, "active": true}
empty_map = {}

# Access
name = person["name"]

# Assign
person["email"] = "alice@example.com"
```

### Nested Collections

```txs
data = {
    "users": [
        {"name": "Alice", "role": "admin"},
        {"name": "Bob",   "role": "viewer"}
    ],
    "count": 2
}

first_user = data["users"][0]["name"]
```

---

## §15 Built-in Functions

True built-ins are called without parentheses (implicit call style). They are globally available without any `use` statement.

| Built-in | Signature       | Returns | Example                       |
|----------|-----------------|---------|-------------------------------|
| `say`    | `say expr`      | `()`    | `say "Hello, World!"`         |
| `ask`    | `ask expr`      | `Str`   | `name = ask "Enter name: "`   |
| `env`    | `env "VAR"`     | `Str?`  | `path = env "PATH"`           |
| `file`   | `file "path"`   | `Str`   | `txt = file "readme.txt"`     |
| `len`    | `len(expr)`     | `Int`   | `n = len(items)`              |
| `typeof` | `typeof(expr)`  | `Str`   | `t = typeof 42`               |
| `assert` | `assert(expr)`  | `()`    | `assert x > 0`                |
| `panic`  | `panic "msg"`   | `!`     | `panic "unreachable"`         |
| `exit`   | `exit(code)`    | `!`     | `exit 0`                      |
| `sleep`  | `sleep(ms)`     | `()`    | `sleep 1000`                  |
| `json`   | `json "..."`    | `Map`   | `obj = json str`              |
| `time`   | `time()`        | `Int`   | `t = time()`                  |

> `!` in the Returns column denotes a diverging call — the function never returns normally (it terminates or aborts execution).

---

## §16 Stdlib Call Style

Standard library modules must be imported with `use` before use. All stdlib calls are **qualified** (module dot function). True built-ins listed in §15 are the **only** functions that use implicit (parenthesis-free) call style.

```txs
use math
use json
use http
use fs
use str

# Qualified stdlib calls
absolute = math.abs(-5)
root = math.sqrt(25)
pi = math.pi

parsed = json.parse(raw_str)
encoded = json.stringify(data_map)

response = http.get("https://api.example.com/data")

content = fs.read("config.txs")
fs.write("output.txt", result)

upper = str.upper("hello")
trimmed = str.trim("  hello  ")
```

**Never** use unqualified stdlib names:

```txs
# WRONG — do not write these
x = abs(-5)       # use math.abs(-5)
y = sqrt(16)      # use math.sqrt(16)
```

---

## §17 Compatibility & Evolution

### Legacy Syntax in the 2.x Compiler

TechScript 2.x maintains **parse compatibility** with all 1.x syntax listed in §2.2. Legacy constructs are accepted but emit `TSW100x` warnings at compile time. They will **not** be silently accepted in 3.0.

| Phase           | Behaviour                                         |
|-----------------|---------------------------------------------------|
| TechScript 2.x  | Legacy syntax compiles with TSW100x warnings      |
| TechScript 3.0  | Legacy syntax is a hard parse error               |

### Migrating with `tsc migrate`

The official migration tool rewrites all deprecated syntax to 2.0 canonical equivalents in place:

```bash
tsc migrate ./src
tsc migrate ./src --dry-run   # Preview changes without writing
tsc migrate file.txs          # Migrate a single file
```

The tool applies the following transformations automatically:

| Input (1.x)                        | Output (2.0)                 |
|------------------------------------|------------------------------|
| `build fn()`                       | `do fn()`                    |
| `fun fn()` / `function fn()`       | `do fn()`                    |
| `return x` / `give x`             | `send x`                     |
| `model Foo`                        | `class Foo`                  |
| `if cond`                          | `when cond`                  |
| `elif cond`                        | `else when cond`             |
| `while cond`                       | `repeat cond`                |
| `each x in y`                      | `for x in y`                 |
| `import mod` / `from mod import x` | `use mod`                    |
| `attempt`                          | `try`                        |
| `none`                             | `null`                       |
| `f"...{x}"`                        | `$"...{x}"`                  |
| `make x = val` / `let x`          | `x = val`                    |
| `{ … }` blocks                     | indented block + `end`       |

### Version Roadmap

| Version   | Key Changes                                                          |
|-----------|----------------------------------------------------------------------|
| 2.0.0     | Canonical syntax finalised; all deprecated aliases emit TSW warnings |
| 2.1.0     | Expanded stdlib (fs, str, net, crypto modules)                       |
| 2.2.0     | Optional type annotations; gradual typing support                    |
| 2.x LTS   | Legacy syntax support maintained                                     |
| 3.0.0     | Legacy syntax removed; breaking change release                       |

---

*This document is the single source of truth for TechScript 2.0 language design decisions. All compiler implementations, linters, IDE plugins, and documentation generators must conform to the specifications in this file.*
