# TechScript Language Specification v1.0

> **A Research-Level Technical Specification & Implementation Blueprint**
> File Extension: `.txs` (primary), `.tx` (fallback)
> Implementation Language: Python 3.10+
> Author: TechScript Language Project
> Date: March 2026

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Design Philosophy](#2-design-philosophy)
3. [File Extensions & Project Structure](#3-file-extensions--project-structure)
4. [Lexical Structure & Tokens](#4-lexical-structure--tokens)
5. [Syntax Rules & Grammar](#5-syntax-rules--grammar)
6. [Type System](#6-type-system)
7. [Operators & Expressions](#7-operators--expressions)
8. [Control Flow](#8-control-flow)
9. [Functions & Modules](#9-functions--modules)
10. [Error Handling System](#10-error-handling-system)
11. [Interpreter Architecture](#11-interpreter-architecture)
12. [CLI Tool & REPL Design](#12-cli-tool--repl-design)
13. [Runtime & Execution Model](#13-runtime--execution-model)

> **Companion Documents:**
> - [TECHSCRIPT_REFERENCE.md](./TECHSCRIPT_REFERENCE.md) — Complete 200 keyword/function reference
> - [TECHSCRIPT_GUIDE.md](./TECHSCRIPT_GUIDE.md) — User documentation & getting started
> - [TECHSCRIPT_EXAMPLES.md](./TECHSCRIPT_EXAMPLES.md) — Example `.txs` programs
> - [TECHSCRIPT_BUILD.md](./TECHSCRIPT_BUILD.md) — Step-by-step interpreter build guide

---

## 1. Executive Summary

**TechScript** is a new, general-purpose programming language designed from the ground up to be:

- **Simple like Python** — minimal boilerplate, clean syntax, no semicolons
- **Readable like plain English** — keywords chosen to match how beginners think
- **Easy to type** — short symbols, minimal shift-key usage, abbreviated operators
- **Beginner-friendly** — clear error messages with suggestions, gentle learning curve
- **Practically useful** — capable of scripting, automation, file I/O, web basics, and CLI tools

TechScript programs are stored in `.txs` files (fallback `.tx`) and executed via the `tech` command-line tool:

```bash
tech run hello.txs        # Run a script
tech repl                 # Start interactive REPL
tech check program.txs    # Syntax check without running
tech build project/       # Package a project
```

The initial reference implementation is an **AST-walking interpreter** written in Python, with a clear path to a bytecode compiler in later versions.

### Why Another Language?

Most beginner languages either sacrifice power for simplicity (Scratch) or require understanding complex paradigms early (JavaScript). TechScript occupies a unique niche:

| Feature | Python | JavaScript | TechScript |
|---------|--------|------------|------------|
| Output keyword | `print()` | `console.log()` | `say` |
| Input keyword | `input()` | `prompt()` | `?` or `ask` |
| Variable declaration | `x = 5` | `let x = 5` | `x = 5` or `set x = 5` |
| Block structure | Indentation | `{ }` | Indentation (like Python) |
| Error messages | Technical | Cryptic | Plain English + suggestions |
| Typing | `len(x)` | `x.length` | `x.length` or `size(x)` |

---

## 2. Design Philosophy

### 2.1 Core Principles

1. **English-First Readability**
   Code should read almost like English sentences. A non-programmer should be able to glance at TechScript code and understand its intent.

   ```
   set name = ask "What is your name?"
   say "Hello, " + name + "!"
   ```

2. **Minimal Ceremony**
   No imports required for basic operations. No `main()` function needed. Scripts execute top-to-bottom. No semicolons, no type annotations required, no curly braces.

3. **Short Symbols, Big Meaning**
   Common operations use the shortest possible syntax:
   - `?` — input from user
   - `say` — print output
   - `fn` — define a function
   - `@` — decorator / annotation
   - `=>` — lambda / arrow function
   - `..` — range operator

4. **Forgiving & Helpful**
   The language should never just say "SyntaxError." It should tell you *what went wrong*, *where*, and *suggest a fix*:
   ```
   Error on line 3: Unknown keyword 'sya'
   Did you mean: say?
   ```

5. **Progressive Complexity**
   A beginner writes `say "hello"`. An intermediate user writes modules and classes. An advanced user writes decorators and metaprogramming. The language grows with the user.

### 2.2 Design Anti-Patterns (What TechScript Avoids)

- ❌ No `public static void main(String[] args)`
- ❌ No mandatory type annotations
- ❌ No `===` vs `==` confusion (just `==`)
- ❌ No `null` / `undefined` split (just `none`)
- ❌ No callback hell — use `await` for async
- ❌ No `this` keyword confusion — use `self` in classes
- ❌ No implicit type coercion surprises

### 2.3 Language Influences

| Influence | What TechScript Borrows |
|-----------|------------------------|
| Python | Indentation-based blocks, dynamic typing, list comprehensions |
| Ruby | `unless`, readable method chaining, `do..end` alt-blocks |
| Lua | Lightweight embedding, simple table/map structure |
| Swift | `guard`, optional chaining (`?.`), string interpolation |
| Go | `defer`, simple error returns, fast compilation philosophy |
| Rust | `match` expression, `Result` type pattern |

---

## 3. File Extensions & Project Structure

### 3.1 File Extensions

| Extension | Purpose |
|-----------|---------|
| `.txs` | **Primary** — TechScript source file |
| `.tx` | **Fallback** — Short alternative for TechScript source |
| `.txmod` | TechScript module (importable library code) |
| `.txcfg` | TechScript configuration file |
| `.txtest` | TechScript test file |
| `.txpkg` | TechScript package manifest |

### 3.2 Standard Project Layout

```
my-project/
├── tech.txpkg              # Package manifest (name, version, deps)
├── src/
│   ├── main.txs            # Entry point
│   ├── utils.txs            # Utility functions
│   └── models/
│       ├── user.txs
│       └── item.txs
├── tests/
│   ├── test_utils.txtest
│   └── test_models.txtest
├── libs/                    # Third-party libraries
├── docs/                    # Documentation
├── assets/                  # Static files
└── .techrc                  # Local runtime configuration
```

### 3.3 Package Manifest (`tech.txpkg`)

```yaml
name: my-project
version: 1.0.0
author: "Your Name"
description: "A TechScript project"
entry: src/main.txs
tech_version: ">=1.0"

dependencies:
  http: "1.2.0"
  json: "builtin"

scripts:
  start: "tech run src/main.txs"
  test: "tech test tests/"
  build: "tech build src/ --output dist/"
```

### 3.4 Runtime Configuration (`.techrc`)

```yaml
# .techrc - local runtime settings
strict_mode: false          # Enable strict type checking
max_recursion: 1000         # Max recursion depth
encoding: utf-8             # Source file encoding
warnings: true              # Show warnings
color_output: true          # Colored terminal output
tab_size: 4                 # Indentation size
```

---

## 4. Lexical Structure & Tokens

### 4.1 Character Set

TechScript source files are UTF-8 encoded. Identifiers support ASCII alphanumeric characters and underscores. String literals support full Unicode.

### 4.2 Token Categories

The lexer produces the following token types:

```
TOKEN_TYPES = {
    # Literals
    "NUMBER_INT",       # 42, 0, -7
    "NUMBER_FLOAT",     # 3.14, 0.5, -2.7
    "STRING",           # "hello", 'world'
    "FSTRING",          # f"Hello {name}"
    "BOOL",             # true, false
    "NONE",             # none

    # Identifiers & Keywords
    "IDENTIFIER",       # variable/function names
    "KEYWORD",          # say, set, fn, if, etc.
    "BUILTIN",          # built-in function names

    # Operators
    "PLUS",             # +
    "MINUS",            # -
    "STAR",             # *
    "SLASH",            # /
    "DOUBLE_SLASH",     # // (integer division)
    "PERCENT",          # %
    "POWER",            # **
    "ASSIGN",           # =
    "PLUS_ASSIGN",      # +=
    "MINUS_ASSIGN",     # -=
    "STAR_ASSIGN",      # *=
    "SLASH_ASSIGN",     # /=
    "EQUAL",            # ==
    "NOT_EQUAL",        # !=
    "LESS",             # <
    "GREATER",          # >
    "LESS_EQUAL",       # <=
    "GREATER_EQUAL",    # >=
    "AND",              # and
    "OR",               # or
    "NOT",              # not
    "ARROW",            # =>
    "PIPE",             # |>
    "QUESTION",         # ?
    "DOT",              # .
    "DOTDOT",           # ..
    "DOTDOTDOT",        # ...
    "AT",               # @
    "HASH",             # #

    # Delimiters
    "LPAREN",           # (
    "RPAREN",           # )
    "LBRACKET",         # [
    "RBRACKET",         # ]
    "LBRACE",           # {
    "RBRACE",           # }
    "COMMA",            # ,
    "COLON",            # :
    "NEWLINE",          # \n (significant)
    "INDENT",           # Increase in indentation
    "DEDENT",           # Decrease in indentation
    "EOF",              # End of file
}
```

### 4.3 Reserved Keywords (Complete List)

```
say       ask       set       fn        return
if        elif      else      for       while
in        do        end       break     skip
match     case      try       catch     throw
class     self      new       import    from
export    as        with      defer     guard
true      false     none      and       or
not       is        has       typeof    await
async     yield     unless    until     each
del       mut       const     global    pass
```

**Total: 50 reserved keywords**

### 4.4 Comments

```python
# This is a single-line comment

## This is a doc-comment (attached to the next declaration)

#[
   This is a
   multi-line block comment
]#
```

### 4.5 String Literals

```python
# Simple strings
name = "hello"
name = 'hello'

# Multi-line strings
text = """
This is a
multi-line string
"""

# F-strings (interpolation)
greeting = f"Hello, {name}! You are {age} years old."

# Raw strings (no escape processing)
path = r"C:\Users\tanmoy\file.txt"

# Escape sequences
tab = "\t"       # Tab
newline = "\n"   # Newline
quote = "\""     # Double quote
backslash = "\\" # Backslash
unicode = "\u0041"  # Unicode character (A)
```

### 4.6 Numeric Literals

```python
# Integers
x = 42
y = 1_000_000       # Underscore separators for readability
z = 0xFF             # Hexadecimal
b = 0b1010           # Binary
o = 0o777            # Octal

# Floats
pi = 3.14159
sci = 2.5e10         # Scientific notation
small = 1.2e-5
```

### 4.7 Indentation Rules

TechScript uses **indentation-based blocks** (like Python):

- Standard indent is **4 spaces** (configurable in `.techrc`)
- **Tabs are rejected** by default with a clear error message:
  ```
  Error on line 5: Tab character detected.
  TechScript uses spaces for indentation (default: 4 spaces).
  Tip: Configure your editor to insert spaces when you press Tab.
  ```
- Mixing tabs and spaces in the same file is a hard error
- Blank lines within indented blocks are allowed and ignored
- A colon (`:`) at the end of a line signals the start of an indented block

---

## 5. Syntax Rules & Grammar

### 5.1 Formal Grammar (EBNF Notation)

```ebnf
program         = { statement NEWLINE } ;

statement       = simple_stmt | compound_stmt ;

simple_stmt     = say_stmt | set_stmt | assign_stmt | return_stmt
                | break_stmt | skip_stmt | del_stmt | import_stmt
                | export_stmt | throw_stmt | defer_stmt | pass_stmt
                | expression_stmt ;

compound_stmt   = if_stmt | for_stmt | while_stmt | fn_stmt
                | class_stmt | try_stmt | match_stmt | with_stmt
                | unless_stmt | until_stmt ;

(* === Simple Statements === *)

say_stmt        = "say" expression { "," expression } ;
set_stmt        = "set" IDENTIFIER "=" expression ;
assign_stmt     = IDENTIFIER ( "=" | "+=" | "-=" | "*=" | "/=" ) expression ;
return_stmt     = "return" [ expression ] ;
break_stmt      = "break" ;
skip_stmt       = "skip" ;        (* equivalent to 'continue' *)
del_stmt        = "del" IDENTIFIER ;
pass_stmt       = "pass" ;
throw_stmt      = "throw" expression ;
defer_stmt      = "defer" expression ;

import_stmt     = "import" module_path [ "as" IDENTIFIER ]
                | "from" module_path "import" name_list ;
export_stmt     = "export" ( fn_stmt | class_stmt | assign_stmt ) ;

module_path     = IDENTIFIER { "." IDENTIFIER } ;
name_list       = IDENTIFIER { "," IDENTIFIER } ;

(* === Compound Statements === *)

if_stmt         = "if" expression ":" block
                  { "elif" expression ":" block }
                  [ "else" ":" block ] ;

for_stmt        = "for" IDENTIFIER "in" expression ":" block ;

while_stmt      = "while" expression ":" block ;

until_stmt      = "until" expression ":" block ;

unless_stmt     = "unless" expression ":" block ;

fn_stmt         = "fn" IDENTIFIER "(" [ param_list ] ")" ":" block ;
param_list      = param { "," param } ;
param           = IDENTIFIER [ "=" expression ] ;

class_stmt      = "class" IDENTIFIER [ "(" IDENTIFIER ")" ] ":" class_block ;
class_block     = NEWLINE INDENT { class_member } DEDENT ;
class_member    = fn_stmt | assign_stmt | pass_stmt ;

try_stmt        = "try" ":" block
                  "catch" [ IDENTIFIER ] ":" block
                  [ "finally" ":" block ] ;

match_stmt      = "match" expression ":" NEWLINE INDENT
                  { "case" pattern ":" block }
                  [ "case" "_" ":" block ]
                  DEDENT ;

with_stmt       = "with" expression "as" IDENTIFIER ":" block ;

block           = NEWLINE INDENT { statement NEWLINE } DEDENT ;

(* === Expressions === *)

expression      = ternary_expr ;
ternary_expr    = or_expr [ "if" or_expr "else" or_expr ] ;
or_expr         = and_expr { "or" and_expr } ;
and_expr        = not_expr { "and" not_expr } ;
not_expr        = "not" not_expr | comparison ;
comparison      = addition { ( "==" | "!=" | "<" | ">" | "<=" | ">=" | "is" | "in" ) addition } ;
addition        = multiply { ( "+" | "-" ) multiply } ;
multiply        = unary { ( "*" | "/" | "//" | "%" ) unary } ;
unary           = ( "-" | "+" ) unary | power ;
power           = call [ "**" unary ] ;
call            = primary { "(" [ arg_list ] ")" | "[" expression "]" | "." IDENTIFIER } ;
arg_list        = expression { "," expression } ;

primary         = NUMBER_INT | NUMBER_FLOAT | STRING | FSTRING
                | "true" | "false" | "none"
                | IDENTIFIER
                | "(" expression ")"
                | list_literal | map_literal
                | lambda_expr
                | ask_expr ;

list_literal    = "[" [ expression { "," expression } ] "]" ;
map_literal     = "{" [ map_entry { "," map_entry } ] "}" ;
map_entry       = ( STRING | IDENTIFIER ) ":" expression ;
lambda_expr     = "(" [ param_list ] ")" "=>" expression ;
ask_expr        = "ask" expression | "?" expression ;

pattern         = literal | IDENTIFIER | "_" ;
```

### 5.2 Statement Examples

```python
# Variable assignment (set is optional, for beginners)
set name = "TechScript"
age = 1

# Output
say "Hello, World!"
say "Name:", name, "Age:", age

# Input
user_name = ask "Enter your name: "
age = ask "Enter age: " |> to_int

# Conditional
if age >= 18:
    say "You are an adult"
elif age >= 13:
    say "You are a teenager"
else:
    say "You are a child"

# Unless (inverse if)
unless logged_in:
    say "Please log in first"

# For loop
for i in 1..10:
    say i

for item in my_list:
    say item

# While loop
while count > 0:
    say count
    count -= 1

# Until loop (inverse while — runs until condition is true)
until found:
    item = search_next()
    if item == target:
        found = true

# Functions
fn greet(name, greeting = "Hello"):
    say f"{greeting}, {name}!"

fn add(a, b):
    return a + b

# Lambda
double = (x) => x * 2
```

---

## 6. Type System

### 6.1 Built-in Types

TechScript is **dynamically typed** with the following built-in types:

| Type | Literal Examples | Description |
|------|-----------------|-------------|
| `int` | `42`, `-7`, `0xFF` | Arbitrary-precision integer |
| `float` | `3.14`, `2.5e10` | IEEE 754 double-precision float |
| `str` | `"hello"`, `'hi'` | Immutable Unicode string |
| `bool` | `true`, `false` | Boolean |
| `list` | `[1, 2, 3]` | Ordered, mutable sequence |
| `map` | `{name: "Jo", age: 5}` | Key-value mapping (ordered) |
| `none` | `none` | Absence of value |
| `fn` | `(x) => x + 1` | First-class function |
| `range` | `1..10`, `0..5` | Lazy integer range |
| `error` | `Error("msg")` | Error value |
| `bytes` | `b"data"` | Byte sequence |

### 6.2 Type Checking & Conversion

```python
# Type checking
typeof(42)          # => "int"
typeof("hi")        # => "str"
x is int            # => true / false

# Type conversion
to_int("42")        # => 42
to_float("3.14")    # => 3.14
to_str(42)          # => "42"
to_bool(0)          # => false
to_list("abc")      # => ["a", "b", "c"]
```

### 6.3 Truthiness Rules

| Value | Truthy? |
|-------|---------|
| `true` | ✅ |
| Non-zero numbers | ✅ |
| Non-empty strings | ✅ |
| Non-empty lists/maps | ✅ |
| `false` | ❌ |
| `0`, `0.0` | ❌ |
| `""` | ❌ |
| `[]`, `{}` | ❌ |
| `none` | ❌ |

---

## 7. Operators & Expressions

### 7.1 Operator Precedence (Highest to Lowest)

| Precedence | Operator | Description | Associativity |
|-----------|----------|-------------|---------------|
| 1 | `()` `[]` `.` | Grouping, index, member access | Left |
| 2 | `**` | Exponentiation | Right |
| 3 | `+x` `-x` `not` | Unary plus, unary minus, not | Right |
| 4 | `*` `/` `//` `%` | Multiplication, division, modulo | Left |
| 5 | `+` `-` | Addition, subtraction | Left |
| 6 | `..` | Range | Left |
| 7 | `==` `!=` `<` `>` `<=` `>=` `is` `in` | Comparison | Left |
| 8 | `and` | Logical AND | Left |
| 9 | `or` | Logical OR | Left |
| 10 | `\|>` | Pipe (chaining) | Left |
| 11 | `if...else` | Ternary conditional | Right |
| 12 | `=>` | Lambda arrow | Right |
| 13 | `=` `+=` `-=` `*=` `/=` | Assignment | Right |

### 7.2 Special Operators

```python
# Pipe operator — chains function calls left to right
"hello" |> upper |> reverse    # => "OLLEH"
# Equivalent to: reverse(upper("hello"))

# Range operator
1..5          # => [1, 2, 3, 4]  (exclusive end)
1..=5         # => [1, 2, 3, 4, 5]  (inclusive end)
1..10..2      # => [1, 3, 5, 7, 9]  (with step)

# Spread operator
combined = [...list_a, ...list_b]

# Optional chaining
user?.address?.city    # Returns none if any part is none

# Null coalescing
result = value ?? "default"
```

---

## 8. Control Flow

### 8.1 Conditionals

```python
# Standard if-elif-else
if condition:
    do_something()
elif other_condition:
    do_other()
else:
    do_default()

# Inline ternary
status = "adult" if age >= 18 else "minor"

# Unless (negated if)
unless user_logged_in:
    redirect("/login")

# Guard clause (early return)
guard age >= 0 else:
    throw Error("Age cannot be negative")
```

### 8.2 Loops

```python
# For-in loop
for item in collection:
    say item

# For with index
for i, item in enumerate(collection):
    say f"{i}: {item}"

# For with range
for i in 0..10:
    say i

# While loop
while condition:
    process()

# Until loop (runs while condition is false)
until queue.is_empty():
    process(queue.pop())

# Loop with else (runs if loop completes without break)
for item in items:
    if item == target:
        say "Found!"
        break
else:
    say "Not found"

# Each (method-style iteration)
items.each(item => say item)

# Loop control
break           # Exit loop
skip            # Skip to next iteration (like 'continue')
```

### 8.3 Pattern Matching

```python
match command:
    case "start":
        start_engine()
    case "stop":
        stop_engine()
    case "status":
        show_status()
    case _:
        say "Unknown command"

# Match with destructuring
match point:
    case {x: 0, y: 0}:
        say "Origin"
    case {x: 0, y}:
        say f"On Y-axis at {y}"
    case {x, y: 0}:
        say f"On X-axis at {x}"
    case {x, y}:
        say f"Point at ({x}, {y})"
```

---

## 9. Functions & Modules

### 9.1 Function Definitions

```python
# Basic function
fn greet(name):
    say f"Hello, {name}!"

# With default parameters
fn greet(name, greeting = "Hello"):
    say f"{greeting}, {name}!"

# Variadic arguments
fn sum_all(...nums):
    total = 0
    for n in nums:
        total += n
    return total

# Lambda / arrow function
double = (x) => x * 2
add = (a, b) => a + b

# Functions as first-class values
fn apply(func, value):
    return func(value)

result = apply(double, 5)    # => 10

# Doc-comments
## Calculates factorial of n.
## Returns: int
fn factorial(n):
    guard n >= 0 else:
        throw Error("n must be non-negative")
    if n <= 1:
        return 1
    return n * factorial(n - 1)
```

### 9.2 Closures & Higher-Order Functions

```python
fn make_counter(start = 0):
    count = start
    fn increment():
        count += 1
        return count
    return increment

counter = make_counter()
say counter()    # 1
say counter()    # 2

# Built-in higher-order functions
nums = [1, 2, 3, 4, 5]
evens = nums.filter(n => n % 2 == 0)        # [2, 4]
doubled = nums.map(n => n * 2)              # [2, 4, 6, 8, 10]
total = nums.reduce((acc, n) => acc + n, 0) # 15
```

### 9.3 Classes & OOP

```python
class Animal:
    fn init(self, name, sound):
        self.name = name
        self.sound = sound

    fn speak(self):
        say f"{self.name} says {self.sound}!"

    fn to_str(self):
        return f"Animal({self.name})"

# Inheritance
class Dog(Animal):
    fn init(self, name):
        super.init(name, "Woof")

    fn fetch(self, item):
        say f"{self.name} fetches the {item}!"

dog = new Dog("Buddy")
dog.speak()             # Buddy says Woof!
dog.fetch("ball")       # Buddy fetches the ball!
```

### 9.4 Modules & Imports

```python
# Import entire module
import math
say math.sqrt(16)

# Import specific items
from math import sqrt, pi
say sqrt(16)

# Import with alias
import http_client as http

# Relative imports (within a project)
from .utils import helper_fn
from ..models import User

# Export (in a module file)
export fn public_function():
    return "I am public"

fn _private_function():
    return "I am private"
```

---

## 10. Error Handling System

### 10.1 Design Goals

TechScript's error system is designed around **three pillars**:
1. **Clear** — Tell the user exactly what went wrong, in plain English
2. **Located** — Point to the exact line, column, and token causing the error
3. **Helpful** — Suggest a likely fix using fuzzy matching and context analysis

### 10.2 Error Categories

| Category | Code | Example |
|----------|------|---------|
| `SyntaxError` | `E001`–`E099` | Missing colon, unexpected token |
| `NameError` | `E100`–`E199` | Undefined variable, typo in keyword |
| `TypeError` | `E200`–`E299` | Wrong type in operation |
| `ValueError` | `E300`–`E399` | Invalid argument value |
| `IndexError` | `E400`–`E449` | Index out of bounds |
| `KeyError` | `E450`–`E499` | Key not found in map |
| `FileError` | `E500`–`E599` | File not found, permission denied |
| `ImportError` | `E600`–`E699` | Module not found |
| `RuntimeError` | `E700`–`E799` | Stack overflow, division by zero |
| `CustomError` | `E800`–`E999` | User-defined errors |

### 10.3 Error Message Format

```
╭─ TechScript Error ─────────────────────────────────
│
│  NameError [E102]: Unknown name 'sya'
│
│    3 │  sya "Hello, World!"
│      │  ^^^
│
│  Did you mean: say?
│
│  Tip: 'say' is used to print output to the screen.
│       Example: say "Hello!"
│
╰─────────────────────────────────────────────────────
```

### 10.4 "Did You Mean?" System

The suggestion engine uses **Levenshtein distance** with a threshold of 2 edits:

```python
# Implementation concept (Python)
def suggest_correction(unknown_word, known_words, max_distance=2):
    candidates = []
    for word in known_words:
        distance = levenshtein(unknown_word, word)
        if distance <= max_distance:
            candidates.append((word, distance))
    candidates.sort(key=lambda c: c[1])
    return candidates[0][0] if candidates else None
```

It matches against: keywords, built-in functions, variables in scope, imported names.

### 10.5 Try-Catch-Finally

```python
try:
    result = risky_operation()
    say result
catch err:
    say f"Something went wrong: {err.message}"
finally:
    cleanup()

# Catching specific error types
try:
    data = read_file("config.txcfg")
catch FileError as err:
    say "Config file not found, using defaults"
catch err:
    say f"Unexpected error: {err}"

# Throw custom errors
throw Error("Something went wrong")
throw ValueError("Age must be positive")
```

---

## 11. Interpreter Architecture

### 11.1 Pipeline Overview

```
┌──────────────┐     ┌────────────┐     ┌───────────┐     ┌──────────────┐
│ Source Code   │────>│   Lexer    │────>│  Parser   │────>│     AST      │
│  (.txs file) │     │ (Tokenizer)│     │ (Grammar) │     │  (Tree)      │
└──────────────┘     └────────────┘     └───────────┘     └──────┬───────┘
                                                                 │
                                                                 v
┌──────────────┐     ┌────────────┐     ┌───────────────────────────────┐
│    Output    │<────│Interpreter │<────│   Semantic Analyzer (optional)│
│   (Result)   │     │(Evaluator) │     │   (Type check, resolve names)│
└──────────────┘     └────────────┘     └───────────────────────────────┘
```

### 11.2 Component Responsibilities

| Component | Input | Output | Role |
|-----------|-------|--------|------|
| **Lexer** | Raw source string | Token stream | Breaks source into tokens, tracks line/col numbers, handles indentation |
| **Parser** | Token stream | AST (Abstract Syntax Tree) | Builds tree structure from grammar rules, produces error messages |
| **Semantic Analyzer** | AST | Annotated AST | Name resolution, scope checking, basic type inference (optional) |
| **Interpreter** | AST | Program output | Walks the tree and executes each node recursively |

### 11.3 AST Node Types

```python
# Core AST nodes (Python dataclass representation)
@dataclass
class Program:
    body: list[Statement]

# Statements
@dataclass
class SayStmt:
    values: list[Expression]

@dataclass
class SetStmt:
    name: str
    value: Expression

@dataclass
class AssignStmt:
    target: Expression
    op: str              # '=', '+=', '-=', '*=', '/='
    value: Expression

@dataclass
class IfStmt:
    condition: Expression
    body: list[Statement]
    elif_clauses: list[tuple[Expression, list[Statement]]]
    else_body: list[Statement] | None

@dataclass
class ForStmt:
    var_name: str
    iterable: Expression
    body: list[Statement]

@dataclass
class WhileStmt:
    condition: Expression
    body: list[Statement]

@dataclass
class FnStmt:
    name: str
    params: list[Param]
    body: list[Statement]

@dataclass
class ClassStmt:
    name: str
    parent: str | None
    body: list[Statement]

@dataclass
class ReturnStmt:
    value: Expression | None

@dataclass
class TryStmt:
    body: list[Statement]
    catch_var: str | None
    catch_body: list[Statement]
    finally_body: list[Statement] | None

@dataclass
class MatchStmt:
    subject: Expression
    cases: list[tuple[Pattern, list[Statement]]]

@dataclass
class ImportStmt:
    module: str
    names: list[str] | None
    alias: str | None

# Expressions
@dataclass
class NumberLit:
    value: int | float

@dataclass
class StringLit:
    value: str

@dataclass
class FStringLit:
    parts: list[str | Expression]

@dataclass
class BoolLit:
    value: bool

@dataclass
class NoneLit:
    pass

@dataclass
class ListLit:
    elements: list[Expression]

@dataclass
class MapLit:
    entries: list[tuple[Expression, Expression]]

@dataclass
class Identifier:
    name: str

@dataclass
class BinaryOp:
    left: Expression
    op: str
    right: Expression

@dataclass
class UnaryOp:
    op: str
    operand: Expression

@dataclass
class CallExpr:
    callee: Expression
    args: list[Expression]

@dataclass
class IndexExpr:
    obj: Expression
    index: Expression

@dataclass
class MemberExpr:
    obj: Expression
    member: str

@dataclass
class LambdaExpr:
    params: list[Param]
    body: Expression

@dataclass
class AskExpr:
    prompt: Expression

@dataclass
class TernaryExpr:
    true_val: Expression
    condition: Expression
    false_val: Expression

@dataclass
class PipeExpr:
    left: Expression
    right: Expression
```

### 11.4 Environment & Scope

```python
class Environment:
    """Manages variable scoping with parent chain."""

    def __init__(self, parent=None):
        self.vars = {}
        self.parent = parent

    def get(self, name):
        if name in self.vars:
            return self.vars[name]
        if self.parent:
            return self.parent.get(name)
        raise NameError(f"Undefined variable: '{name}'")

    def set(self, name, value):
        self.vars[name] = value

    def update(self, name, value):
        """Update existing variable (searches up scope chain)."""
        if name in self.vars:
            self.vars[name] = value
        elif self.parent:
            self.parent.update(name, value)
        else:
            raise NameError(f"Cannot update undefined variable: '{name}'")
```

---

## 12. CLI Tool & REPL Design

### 12.1 CLI Commands

The `tech` command is the unified entry point:

```bash
# Running programs
tech run script.txs              # Execute a .txs file
tech run script.txs --debug      # Run with debug output
tech run script.txs --strict     # Run in strict mode

# Interactive REPL
tech repl                        # Start interactive shell
tech repl --load prelude.txs     # Start REPL with preloaded file

# Code quality
tech check script.txs            # Syntax check (no execution)
tech fmt script.txs              # Auto-format code
tech lint script.txs             # Lint for common issues

# Project management
tech init [project-name]         # Create new project scaffolding
tech build src/ --output dist/   # Package project
tech test tests/                 # Run test files

# Utilities
tech version                     # Show TechScript version
tech help                        # Show help
tech help say                    # Show help for 'say' keyword
tech docs                        # Open documentation
```

### 12.2 REPL Design

```
$ tech repl
  ╭──────────────────────────────────────╮
  │  TechScript v1.0.0 Interactive REPL  │
  │  Type 'help' for commands, 'exit'    │
  │  to quit.                            │
  ╰──────────────────────────────────────╯

>>> say "Hello!"
Hello!

>>> x = 42
>>> x * 2
84

>>> fn square(n):
...     return n ** 2
...
>>> square(7)
49

>>> help say
  say <value> [, <value>, ...]
  Prints values to the screen, separated by spaces.
  Example: say "Hello", name

>>> exit
Goodbye! 👋
```

REPL Features:
- Multi-line input with `...` continuation prompt
- Auto-indent after `:` lines
- History (up/down arrows)
- Tab completion for keywords, built-ins, and variables in scope
- Colored syntax output
- `help <keyword>` for inline documentation
- `.save <file>` to save REPL session to a file
- `.load <file>` to load and execute a file in current session

---

## 13. Runtime & Execution Model

### 13.1 Execution Flow

```
tech run program.txs
        │
        v
  ┌─ Read source file (.txs) ──────────────────┐
  │                                              │
  │  1. File is read as UTF-8 string             │
  │  2. Lexer tokenizes into token stream        │
  │  3. Parser produces AST from tokens          │
  │  4. (Optional) Semantic analysis pass        │
  │  5. Interpreter walks AST and executes       │
  │  6. Errors formatted with location + tips    │
  │  7. Exit code 0 (success) or 1 (error)       │
  │                                              │
  └──────────────────────────────────────────────┘
```

### 13.2 Memory Model

- **Garbage Collection**: Reference counting with cycle detection (using Python's GC underneath in v1)
- **Immutability**: Strings and tuples are immutable; lists and maps are mutable
- **Pass-by-Reference**: Lists, maps, and objects are passed by reference; scalars are passed by value
- **Max Recursion**: Configurable, default 1000. Clear error: `"Maximum recursion depth exceeded (limit: 1000)"`

### 13.3 Standard Library Modules

| Module | Purpose | Key Functions |
|--------|---------|---------------|
| `math` | Mathematics | `sqrt`, `abs`, `ceil`, `floor`, `sin`, `cos`, `pi`, `e` |
| `text` | String processing | `upper`, `lower`, `trim`, `split`, `join`, `replace` |
| `io` | File operations | `read_file`, `write_file`, `exists`, `list_dir` |
| `http` | HTTP requests | `get`, `post`, `put`, `delete` |
| `json` | JSON handling | `parse`, `stringify` |
| `time` | Date/time | `now`, `sleep`, `timestamp`, `format` |
| `random` | Random numbers | `random`, `randint`, `choice`, `shuffle` |
| `os` | System operations | `args`, `env`, `exit`, `exec` |
| `path` | File paths | `join`, `dirname`, `basename`, `extension` |
| `test` | Testing framework | `assert`, `assert_equal`, `describe`, `it` |

---

*End of core specification. Continue to companion documents for the complete reference, user guide, examples, and build instructions.*
