```
╔══════════════════════════════════════════════════════════════════════════╗
║                                                                            ║
║   🐉  T E C H S C R I P T                                                 ║
║   The World's Most Readable Programming Language                         ║
║                                                                            ║
║   "Write like a human. Run like Rust."                                    ║
║                                                                            ║
╚══════════════════════════════════════════════════════════════════════════╝
```

<div align="center">

**OFFICIAL RESEARCH PAPER & LANGUAGE SPECIFICATION**

`v1.0.8` · Rust-Native VM · TechScript Studio IDE · MIT License

Author & Language Designer: **Tanmoy Majumder** ([@Tcode-Motion](https://github.com/Tcode-Motion))
Document compiled: July 2026 · Sources: [techscript.is-a.dev](https://techscript.is-a.dev/) · [github.com/Tcode-Motion/techscript](https://github.com/Tcode-Motion/techscript)

`Version-1.0.8-0DF28B` `Built_in-Rust-D83B4FE` `License-MIT-059669` `Platform-Win·Linux·macOS-00A3FF`

</div>

---

## 📖 Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Language Philosophy & History](#2-language-philosophy--history)
3. [Version History — The Road to 1.0.8](#3-version-history--the-road-to-108)
4. [Installation Across Platforms](#4-installation-across-platforms)
5. [Core Language Guide](#5-core-language-guide)
   - 5.1 Comments & Program Structure
   - 5.2 Variables — `make` & `keep`
   - 5.3 Data Types
   - 5.4 Output & Input — `say`, `ask`, f-strings
   - 5.5 Operators
   - 5.6 Conditionals — `when` / `or when` / `else`
   - 5.7 Loops — `each`, `repeat`, `stop`, `skip`
   - 5.8 Functions — `build`, `give`
   - 5.9 Classes & OOP — `model`, `self`, `init`
   - 5.10 Error Handling — `attempt` / `catch`
   - 5.11 The v1.0.8 "New Syntax" Dialect — `be`, `then...end`, `with`
6. [Standard Library — 150+ Built-in Functions](#6-standard-library--150-built-in-functions)
   - 6.1 `math.*`
   - 6.2 `crypto.*`
   - 6.3 `json.*`
   - 6.4 `fs.*`
   - 6.5 `os.*`
   - 6.6 `random.*`
   - 6.7 `date.*`
7. [Building Websites — the `use web` Module](#7-building-websites--the-use-web-module)
8. [The Compiler & Runtime — How TechScript Works in Rust](#8-the-compiler--runtime--how-techscript-works-in-rust)
9. [TechScript Studio IDE](#9-techscript-studio-ide)
10. [Complete CLI Reference](#10-complete-cli-reference)
11. [Full Worked Example Programs](#11-full-worked-example-programs)
12. [Roadmap & Conceptual Modules — 3D, AI, ML, Vision, GUI, Mobile, Networking](#12-roadmap--conceptual-modules--3d-ai-ml-vision-gui-mobile-networking)
13. [TechScript vs Python vs JavaScript](#13-techscript-vs-python-vs-javascript)
14. [Repository Anatomy](#14-repository-anatomy)
15. [Keyword & Symbol Reference Table](#15-keyword--symbol-reference-table)
16. [Limitations & Honest Assessment](#16-limitations--honest-assessment)
17. [Closing Notes](#17-closing-notes)

---

## 1. Executive Summary

**TechScript** is a plain-English, general-purpose programming language created by **Tanmoy Majumder** under the **Tcode-Motion** brand. Its founding premise is simple: modern software development forces beginners to juggle HTML, CSS, JavaScript, a backend language, and a database query language just to ship one small idea. TechScript's answer is a single, readable syntax — `say`, `make`, `when`, `each`, `build`, `model` — that runs on a **native Rust bytecode virtual machine**, ships with its own IDE (**TechScript Studio**), and can build a working website from a single `.txs` file without writing a line of HTML.

This paper documents the language **exactly as it exists in the public v1.0.8 release** (file extension `.txs`, CLI command `tech`), tracing its evolution from a Python-backed prototype (`v1.0.0`) to a fully Rust-native compiler and VM (from `v1.0.2` onward), and it separates **shipped, working features** from **roadmap / conceptual modules** (3D, AI, ML, computer vision, mobile, full networking) that represent the project's declared long-term direction rather than code that runs today. That distinction matters for a specification document, and it is preserved throughout.

> 🐉 *"Code should speak to humans first, machines second. No semicolons. No brackets. No confusing symbols. Just plain English — compiled to blazing-fast Rust bytecode."* — Tanmoy Majumder

### At a glance

| Attribute | Value |
|---|---|
| 🏷️ Name | TechScript |
| 📄 File extension | `.txs` |
| ⌨️ CLI command | `tech` |
| 🔢 Current version | **v1.0.8** — "Rust VM Edition" |
| 🦀 Compiler & VM language | Rust (native bytecode VM, no interpreter overhead) |
| 🖥️ Bundled IDE | TechScript Studio (built with `egui` + `egui_dock`) |
| 🎨 VS Code support | Official `.vsix` extension with syntax highlighting + dragon icon |
| 📦 Distribution | Windows installer (`.exe`), `pip install techscript-lang`, Homebrew, APT, Termux |
| ⚡ Standard library | 150+ built-in functions across 7 modules — no external imports needed |
| 🌐 Web builder | `use web` — full websites from one `.txs` file, zero HTML/CSS/JS |
| 📜 License | MIT |
| 🌍 Platforms | Windows 10/11 (full), Linux, macOS, Android (Termux) |

---

## 2. Language Philosophy & History

### 2.1 The problem TechScript sets out to solve

A newcomer who wants to build "a website that says hello and has a button" today typically needs to learn HTML for structure, CSS for style, JavaScript for behavior, a backend language and framework to serve it, and often a database language on top of that. TechScript's central design bet is that **one readable language, backed by one fast runtime**, can flatten that stack for the overwhelming majority of small-to-medium programs and web tools — without giving up real performance, because the execution engine is Rust, not an interpreted scripting shell.

### 2.2 Design goals

TechScript's public design goals, unchanged since the project's inception, are:

- **Simple** — a program should read close to a sentence.
- **Readable** — `make x be 10` instead of `let x: i32 = 10;`
- **Fast** — native Rust bytecode VM, not a slow tree-walking interpreter.
- **Safe** — Rust's memory-safety guarantees under the hood, no manual memory management exposed to the author.
- **Minimal but scalable** — a small, stable core; power comes from a rich standard library, not from a sprawling keyword set.
- **Cross-platform & native** — one toolchain that runs the same program on Windows, Linux, and macOS.
- **AI-ready / full-stack ambition** — the long-term vision (see [§12](#12-roadmap--conceptual-modules--3d-ai-ml-vision-gui-mobile-networking)) extends the same syntax toward GUI, 3D, and AI-adjacent workflows.

### 2.3 Why the keywords read like English

Every core keyword was chosen so an English speaker with zero programming background can guess its function on first read:

| You want to... | You literally write... |
|---|---|
| Print something | `say "Hello"` |
| Create a variable | `make score be 100` |
| Create something that never changes | `keep PI be 3.14159` |
| Branch on a condition | `when age >= 18 { ... }` |
| Repeat for each item | `each i in 1..10 { ... }` |
| Define a reusable block | `build greet(name) { ... }` |
| Return a value | `give result` |
| Model a real-world object | `model Dog { ... }` |
| Try something that might fail | `attempt { ... } catch err { ... }` |

This is not decoration — it is the whole thesis of the language. TechScript deliberately trades a small amount of terseness (compare `give result` to Python's shorter `return result`) for a much lower first-read barrier, on the theory that most cognitive load in early programming education comes from *symbol decoding*, not from *logical structure*.

---

## 3. Version History — The Road to 1.0.8

TechScript's engine went through a hard architectural pivot early in its life: it started on a Python backend and was **completely rewritten in Rust** starting at v1.0.2, deleting the Python runtime entirely.

| Version | Engine | Key feature added |
|---|---|---|
| **v1.0.0** | 🐍 Python | Core scripting: `say`, `make`, loops, functions, classes |
| **v1.0.1** | 🐍 Python | `use web` module, `WebPage` builder, Windows `setup.exe`, first VS Code extension |
| **v1.0.2** | 🦀 Rust VM | **Full Rust rewrite.** Native bytecode VM benchmarking 1,000,000 loop iterations in 2.9 seconds. Zero external runtime dependencies. `attempt {} catch err {}` error handling introduced. |
| **v1.0.3** | 🦀 Rust VM | 150+ standard-library functions across `math`, `crypto`, `fs`, `os`, `json`, `random`, `date`. Fixed `stop`/`skip` (previously compiled to the wrong bytecode instruction). Fixed `in` and `typeof` operators. Added `tech eval "code"` for inline execution. Eliminated `unsafe transmute` — bytecode became 100% type-safe. Fixed a Windows PATH-truncation installer bug. |
| **v1.0.4.7** | 🦀 Rust VM | "Universal Edition" — import fixes, Python fallback engine kept only for non-Windows build targets during the transition window. |
| **v1.0.5** | 🦀 Rust VM | `use three_d` module (3D scenes in ~5 lines), standalone `TechScript_TX.exe` binary, developer toolchain (`tech fmt`, `tech lint`, `tech build`, `tech test`), `pip install techscript-lang` published to PyPI, VS Code extension v2.0. |
| **v1.0.6** | 🦀 Rust VM | First launch of TechScript Studio, multi-channel terminal, full dragon branding, redesigned Windows installer. |
| **v1.0.7** | 🦀 Rust VM | Standalone production release — multi-pane docking workspace, AST & Bytecode Inspector, multi-channel terminal matured. |
| **v1.0.8** *(current)* | 🦀 Rust VM | **TechScript Studio IDE** rebuilt on `egui_dock` with a cyberpunk dark theme, smart `.txs` double-click execution on Windows, unified Modify / Repair / Uninstall maintenance manager, and a **new alternate syntax dialect** (`make x be 10`, `when x equals y then ... end`, `build greet with name then ... end`). |

> **Performance note (v1.0.2):** the native Rust VM ran a benchmark of one million loop iterations in **2.9 seconds** with zero external runtime dependencies — the number the project cites as evidence the Python-to-Rust rewrite achieved its goal.

---

## 4. Installation Across Platforms

### 🪟 Windows (recommended — full experience)

The single `TechScript_v1.0.8_x64.exe` installer sets up everything in one step:

```
1. Download TechScript_v1.0.8_x64.exe from the Releases page
2. Double-click it
3. Tick "Add to PATH" + "Associate .txs Files"
4. Open PowerShell and run:  tech version
```

What it installs automatically:

- ✅ Native Rust compiler + bytecode VM
- ✅ TechScript Studio IDE
- ✅ `tech` command on PATH, system-wide
- ✅ `.txs` file association (double-click to run)
- ✅ VS Code extension (syntax highlighting + dragon icon)
- ✅ A Modify / Repair / Uninstall maintenance manager

### 🐍 Cross-platform via pip

```bash
pip install techscript-lang
```

Requires Python 3.10+. This installs the CLI (`tech run`, `tech repl`, `tech check`, ...) and the full language runtime, but **not** the graphical Studio IDE — that ships only with the Windows installer.

### 🐧 Linux (Ubuntu, Kali, Debian, Arch)

```bash
# One-line installer
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash

# Or, on Debian/Ubuntu:
sudo apt update && sudo apt install techscript
```

### 🍎 macOS

```bash
# Homebrew
brew install tcode-motion/techscript/techscript

# Or the same one-line installer used on Linux
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

### 📱 Android (Termux)

```bash
pkg update && pkg upgrade -y
pkg install python -y
pip install techscript-lang
tech version
```

### Verifying any install

```
$ tech version
TechScript v1.0.8 — Rust VM Edition 🐉

$ tech repl
>>> say "it works!"
it works!
```

---

## 5. Core Language Guide

### 5.1 Comments & Program Structure

TechScript files are plain UTF-8 text, one statement generally per line, with **no semicolons anywhere and no mandatory top-level wrapper** (no `main()` requirement — a script is just a sequence of statements executed top to bottom):

```txs
// This is a single-line comment
say "Hello, World!"
```

A minimal complete program is a single line:

```txs
say "Hello, World!"
```

Running it:

```
$ tech run hello.txs
Hello, World!
```

### 5.2 Variables — `make` & `keep`

`make` declares a mutable variable. `keep` declares a **constant** that can never be reassigned after creation.

```txs
# 'make' creates a variable
make name = "Alice"

# 'keep' creates a CONSTANT — can never be changed
keep PI = 3.14159

make age = 25
make items = [1, 2, 3, 4, 5]        # List
make info = { "city": "Delhi" }     # Dictionary / map
```

Both `=` (classic style) and `be` (v1.0.8 dialect, see §5.11) are valid assignment forms: `make x = 10` and `make x be 10` do the same thing.

### 5.3 Data Types

| Type | Example literal | `typeof` result |
|---|---|---|
| Integer | `42` | `int` |
| Float | `3.14` | `float` (implied) |
| String | `"Alice"` | `str` |
| Boolean | `true` / `false` | `bool` (implied) |
| List | `[1, 2, 3]` | `list` |
| Dictionary / Map | `{ "city": "Delhi" }` | `dict` (implied) |

```txs
say typeof 42          # int
say typeof "Alice"     # str
say typeof [1, 2]      # list
```

### 5.4 Output & Input — `say`, `ask`, f-strings

`say` prints to the console. `ask` prompts the user and returns their typed response as a string. TechScript supports **f-strings** — string literals prefixed with `f"..."` that interpolate `{expression}` directly.

```txs
say "Hello!"
say f"My name is {name}!"        # f-string interpolation
say 10 + 5                        # Prints: 15

make name = ask "What is your name? "
say f"Nice to meet you, {name}!"
```

Sample terminal session:

```
$ tech run greeting.txs
What is your name? Alex
Nice to meet you, Alex!
```

### 5.5 Operators

| Category | Operators |
|---|---|
| Arithmetic | `+` `-` `*` `/` |
| Comparison | `==` `!=` `>` `<` `>=` `<=` (classic); `equals` (v1.0.8 dialect) |
| Membership | `in` — containment check on strings and lists |
| Type introspection | `typeof` — returns the runtime type name as a string |
| Range | `1..5` — inclusive/step range used by `each` loops |

```txs
# 'in' — containment check
make fruits = ["apple", "banana"]
when "apple" in fruits { say "Found it!" }
when "ello" in "Hello" { say "Substring!" }
```

### 5.6 Conditionals — `when` / `or when` / `else`

`when` is TechScript's `if`. Its "elif" equivalent is the very readable `or when`, and `else` closes the chain:

```txs
make age = 20

when age >= 18 {
    say "You are an adult!"
} or when age >= 13 {
    say "You are a teenager!"
} else {
    say "You are a child!"
}
```

Output:

```
You are an adult!
```

### 5.7 Loops — `each`, `repeat`, `stop`, `skip`

`each` iterates over a range or a collection. `repeat` is the `while` equivalent, driven by a condition. `stop` breaks out of the nearest loop; `skip` continues to the next iteration — both were **fixed in v1.0.3** after initially compiling to the wrong bytecode instruction.

```txs
# Range loop
each i in 1..5 {
    say f"Count: {i}"
}

# List loop
each fruit in ["apple", "banana", "mango"] {
    say f"I like {fruit}!"
}

# While loop
make x = 1
repeat x <= 5 {
    say x
    x = x + 1
}

# stop (break) and skip (continue)
each i in 1..10 {
    when i == 5 { stop }
    when i == 3 { skip }
    say i
}
```

Output of the range loop:

```
Count: 1
Count: 2
Count: 3
Count: 4
Count: 5
```

### 5.8 Functions — `build`, `give`

`build` defines a function. Parameters may have default values. `give` returns a value (TechScript's `return`).

```txs
build greet(name, greeting = "Hello") {
    say f"{greeting}, {name}!"
}

greet("Alice")             # Hello, Alice!
greet("Bob", "Hi there")   # Hi there, Bob!
```

Recursive example (Fibonacci), used throughout the project's own marketing material as the canonical "reads like English" demo:

```txs
build fib(n) {
    when n <= 1 { give n }
    give fib(n-1) + fib(n-2)
}

say fib(10)   # 55
```

### 5.9 Classes & OOP — `model`, `self`, `init`

Classes are declared with `model` instead of `class`. The constructor is a method literally named `init`, and instance methods take an explicit `self` parameter, similar in spirit to Python:

```txs
model Dog {
    build init(self, name, breed) {
        self.name = name
        self.breed = breed
    }
    build speak(self) {
        say f"{self.name} says: Woof!"
    }
}

make rex = Dog("Rex", "German Shepherd")
rex.speak()     # Rex says: Woof!
```

### 5.10 Error Handling — `attempt` / `catch`

`attempt { ... } catch err { ... }` is TechScript's `try`/`catch`. It was introduced in v1.0.2 as part of the Rust rewrite. The caught error object exposes a `.message` field.

```txs
attempt {
    make result = 10 / 0
} catch err {
    say f"Caught: {err.message}"
}

say "Program continues!"
```

Output:

```
Caught: division by zero
Program continues!
```

### 5.11 The v1.0.8 "New Syntax" Dialect

Alongside the original syntax, **v1.0.8 introduced a second, more sentence-like dialect** that reads even closer to spoken English, using `be` for assignment, `then ... end` blocks instead of `{ }`, and `with` to introduce a function's parameter:

```txs
// Variables (new style)
make x be 10
make name be "TechScript"

// Conditionals (new style)
when version equals 1 then
  say "Latest build!"
end

// Loops (new style)
each item in list then
  say item
end

// Functions (new style)
build greet with name then
  say "Hello, " + name
end

greet "World"
```

Both dialects compile to the same bytecode — this is a **syntax preference**, not a second language. A learner may freely mix `{ }` and `then ... end` blocks across a project, though most codebases pick one style for consistency (and `tech fmt` will normalize a file toward whichever style it detects as dominant).

---

## 6. Standard Library — 150+ Built-in Functions

Everything in this section is compiled directly into the `tech` binary — none of it requires an internet connection, a package manager, or an `import` statement. This is a deliberate contrast with Python (`hashlib`, `json`, `os` all need explicit imports) and JavaScript (crypto and file APIs need extra packages or runtime-specific globals).

### 6.1 `math.*` — 38+ functions

```txs
say math.sin(3.14159)
say math.factorial(10)          # 3628800
say math.gcd(48, 18)            # 6
say math.mean([1, 2, 3, 4, 5])  # 3.0
say math.sqrt(144)              # 12.0
say math.TAU                    # 6.283185...
```

Representative function families available under `math.*`: trigonometric (`sin`, `cos`, `tan`), rounding (`floor`, `ceil`, `round`), statistics (`mean`, `median`, `stdev`), number theory (`gcd`, `lcm`, `factorial`, `is_prime`), and constants (`PI`, `TAU`, `E`).

### 6.2 `crypto.*` — real cryptography, not toy hashing

```txs
say crypto.sha256("hello")               # FIPS 180-4 SHA-256
say crypto.md5("hello")                  # MD5 hash
say crypto.base64_encode("TechScript")   # Base64
say crypto.base64_decode("VGVjaFNjcmlwdA==")
```

This module is explicitly marketed against Python (which needs `import hashlib`) and JavaScript (which needs a `crypto` module import) as evidence of the "no imports for standard tasks" philosophy.

### 6.3 `json.*` — encode / decode

```txs
make data = { "name": "Alice", "age": 25 }
say json.encode(data)
say json.encode_pretty(data)
make parsed = json.decode('{"x": 1}')
```

### 6.4 `fs.*` — file system, 20+ functions

```txs
fs.write("hello.txt", "Hello, World!")
say fs.read("hello.txt")
say fs.exists("hello.txt")       # true
fs.append("hello.txt", "\nMore!")
say fs.list_dir(".")
```

### 6.5 `os.*` — OS integration

```txs
say os.name()                    # windows / linux / macos
say os.arch()                    # x86_64
say os.env_get("PATH")
os.system("echo Hello!")
```

### 6.6 `random.*` — random values & UUIDs

```txs
say random.random()               # 0.0–1.0
say random.randint(1, 100)
say random.uuid()
say random.choice(["a","b","c"])
```

### 6.7 `date.*` — date & time

```txs
say date.now()     # 2025-03-12 14:30:00
say date.year()
say date.unix()    # unix timestamp
```

---

## 7. Building Websites — the `use web` Module

TechScript's flagship differentiator is that a single `.txs` file can define, style, script, and serve a working website — no `.html`, `.css`, or `.js` file ever touches disk. Declaring `use web` at the top of a file unlocks the `WebPage` builder API.

```txs
use web

make page = WebPage("My Website")

page.style("body", {
    "background": "#0f0f11",
    "color": "#eeeeee",
    "text-align": "center",
    "padding": "60px"
})

page.script("""
    function sayHello() { alert('Hello from TechScript! 🐉'); }
""")

page.body([
    page.h1("Welcome! 🐉"),
    page.p("Built 100% in TechScript. No HTML. No CSS."),
    page.button("Click Me!", { "onclick": "sayHello()" })
])

page.run()
```

Running it:

```
$ tech run my_website.txs
```

A local server starts, the default browser opens automatically pointed at it, and `Ctrl+C` in the terminal stops the server. Under the hood `page.style(...)` generates scoped CSS rules, `page.script(...)` embeds a `<script>` block verbatim, and `page.body([...])` assembles the DOM tree from builder calls like `page.h1(...)`, `page.p(...)`, and `page.button(...)`, each of which maps 1:1 onto an HTML element with the given attributes — the author never sees or writes that HTML, but it is exactly what gets served.

### How the request pipeline works conceptually

```
 .txs file
    │
    ▼
 TechScript compiler (Rust) ── parses WebPage(...) builder calls
    │
    ▼
 In-memory DOM + CSS + JS assembled from builder calls
    │
    ▼
 Local HTTP server spun up by the Rust runtime
    │
    ▼
 Default OS browser opened automatically → renders the generated page
```

This is the same underlying idea as Python's Flask/Django or Node's Express, except the "framework install" step is skipped entirely — it's baked into the `tech` binary via `use web`.

---

## 8. The Compiler & Runtime — How TechScript Works in Rust

### 8.1 Execution pipeline

Every `.txs` program passes through the same fixed pipeline before it runs a single instruction:

```
.txs source text
     │
     ▼
┌─────────────┐
│    Lexer    │  → turns raw characters into a stream of Tokens
└─────────────┘     (keywords like `make`, `say`, `when`; literals;
     │               operators; identifiers)
     ▼
┌─────────────┐
│    Parser   │  → consumes Tokens, builds the Abstract Syntax Tree (AST)
└─────────────┘     (statements, expressions, blocks, function bodies)
     │
     ▼
┌───────────────────────┐
│  Semantic Analysis     │ → name resolution, arity checks on `build` calls,
└───────────────────────┘   constant-reassignment checks on `keep`
     │
     ▼
┌───────────────┐
│   Bytecode /   │ → AST is lowered into a flat, typed instruction stream
│      IR        │   (this is what `tech build` writes out as a `.txc` file
└───────────────┘    and what the Studio IDE's Bytecode Inspector displays)
     │
     ▼
┌───────────────┐
│   Rust VM      │ → a stack-based bytecode virtual machine executes the
└───────────────┘   instruction stream directly — no further translation,
     │               no garbage-collector pause, memory safety enforced
     ▼                by Rust's ownership model at compile time of the VM itself
  Program output
```

### 8.2 Why Rust, specifically

The project's own changelog frames the v1.0.0 → v1.0.2 jump as the single most consequential engineering decision in the language's history: the original engine executed TechScript by translating it into Python and letting CPython interpret that — workable, but slow, and dependent on a Python installation being present on the end user's machine. The v1.0.2 rewrite deleted that dependency outright and replaced it with:

- A **hand-written lexer and recursive-descent parser** in Rust, producing an AST as native Rust enum/struct types (no dynamic `dict`-of-nodes representation).
- A **stack-based bytecode VM**, also in Rust, executing a flat instruction array rather than walking the AST at runtime — this is the same broad strategy CPython itself uses internally (bytecode + VM), but implemented in a language with no garbage collector and compile-time memory safety instead of Python's reference-counting GC.
- **Type-safety work across v1.0.2 → v1.0.3**: an early version of the bytecode used `unsafe transmute` to reinterpret bytes between instruction operand types; this was identified and eliminated by v1.0.3, after which the bytecode representation became "100% type-safe" per the project's own changelog — i.e., invalid instruction operands are now caught by Rust's type system rather than causing undefined behavior at runtime.
- **Zero external runtime dependencies** for the core VM — the `tech` binary that ships to end users does not require Python, Node, or any other language runtime to be installed, which is what makes the single-`.exe` Windows installer and the Termux/pip path both viable.

### 8.3 Suggested Rust crate/module architecture

The public repository does not currently expose its internal Rust module layout in the parts of the repo readable from outside, so the following is presented as the natural crate architecture implied by the pipeline above — the shape any Rust engineer would reach for building this kind of language toolchain, not a verified internal file listing:

```
techscript-core/
├── lexer/          # Token, Lexer, tokenize()
├── parser/         # AST node types, Parser (recursive descent)
├── ast/            # Expr, Stmt enums — Rust enums, not dynamic dicts
├── semantic/       # name resolution, `keep`-const checks, arity checks
├── bytecode/       # OpCode enum, Chunk (flat instruction + constant pool)
├── vm/             # stack-based VM, call frames, native-function bridge
├── stdlib/         # math / crypto / json / fs / os / random / date modules
├── webmod/         # WebPage builder → DOM/CSS/JS assembly + local HTTP server
└── cli/            # `tech run|check|repl|eval|fmt|lint|build|test|studio|version`
```

### 8.4 Bytecode & the Studio Inspector

TechScript Studio's **Bytecode Inspector** panel exists precisely so a learner can see this pipeline stop being abstract: writing `say fib(10)` and opening the inspector shows the exact stack-machine instructions the VM will execute — a `CALL` instruction with the resolved function address, `ADD`/`CMP` opcodes for the recursive arithmetic, and a final `PRINT` instruction. This is presented as a deliberate pedagogical feature, not just a debugging tool: it is one of the few beginner-oriented languages that puts "how compilers actually work" one click away from the code editor.

---

## 9. TechScript Studio IDE

TechScript Studio is the graphical IDE bundled with the Windows installer, built on the Rust immediate-mode GUI library **`egui`** with **`egui_dock`** for its docking layout system, and rebuilt substantially for v1.0.8 around a cyberpunk dark aesthetic.

### Panels

| Panel | Purpose |
|---|---|
| 🖊️ Code Editor | High-performance editor with a custom line-number gutter |
| 📁 Workspace Explorer | Browse and load `.txs` scripts from disk |
| 🖥️ Multi-Channel Terminal | Real-time stdout + diagnostics, color-coded by channel |
| 🌳 AST Inspector | Live parse-tree view alongside the code you're writing |
| ⚙️ Bytecode Inspector | Exact VM instructions your code compiles to |

### Terminal channel color coding

| Channel | Color | Purpose |
|---|---|---|
| 📟 Stdout | Emerald `#0DF28B` | Your script's actual printed output |
| ⚙ Compiler | Electric Blue `#00A3FF` | Build diagnostics and warnings |
| 🐞 VM Debugger | Lavender `#D8B4FE` | Runtime debug info (with `--debug`) |

Inline controls available in the terminal panel: **🧹 Clear Logs**, **📋 Copy Output**, **▶ Re-run Script**.

### Windows shell integration

Since v1.0.8, double-clicking a `.txs` file in Windows Explorer launches a native terminal host that runs the script and **keeps the window open** after completion (`[Process completed. Press Enter to exit...]`) — a small but deliberate fix for the classic beginner frustration of a console window flashing and vanishing before the output can be read.

### Maintenance Manager

The same installer executable doubles as a maintenance tool:

| Mode | What it does |
|---|---|
| ⚙ Modify | Change PATH variables, shortcuts, `.txs` associations |
| 🔧 Repair | Restore missing binaries / broken registry keys |
| 🗑 Uninstall | Full clean removal — PATH, registry, folders |

### VS Code / Cursor extension

The official `.vsix` provides syntax highlighting, code snippets, and a dragon file icon for `.txs` files.

```bash
code --install-extension vscode-extension/techscript-1.0.8.vsix
```

Or via the GUI: `Ctrl+Shift+X` → `···` → *Install from VSIX...* → select `techscript-1.0.8.vsix`.

---

## 10. Complete CLI Reference

| Command | What it does | Example |
|---|---|---|
| `tech run file.txs` | Run a TechScript source file | `tech run hello.txs` |
| `tech run file.txs --debug` | Run with verbose VM debug output | `tech run app.txs --debug` |
| `tech check file.txs` | Syntax-check without running | `tech check myapp.txs` |
| `tech eval "code"` | Run inline code instantly | `tech eval "say 42"` |
| `tech "[[[code]]]"` | Shorthand inline execution | `tech "[[[say 'hi']]]"` |
| `tech repl` | Interactive REPL — type and run instantly | `tech repl` |
| `tech transpile file.txs` | Convert TechScript source → Python code | `tech transpile hello.txs` |
| `tech fmt` | Auto-format a file | `tech fmt myapp.txs` |
| `tech lint` | Find errors before running | `tech lint myapp.txs` |
| `tech build` | Compile to bytecode (`.txc`) | `tech build myapp.txs` |
| `tech test` | Run built-in unit tests | `tech test` |
| `tech studio` | Launch TechScript Studio IDE | `tech studio` |
| `tech version` / `tech -V` | Show installed version | `tech -V` |

---

## 11. Full Worked Example Programs

### 11.1 Hello World

```txs
say "Hello, World!"
```
```
$ tech run hello.txs
Hello, World!
```

### 11.2 Variables, arithmetic, and f-strings

```txs
say "Hello, World!"

make x = 10
make y = 20
make sum = x + y

say f"Result: {sum}"
```
```
$ tech run hello.txs
Hello, World!
Result: 30
```

### 11.3 FizzBuzz

```txs
each i in 1..15 {
    when i in [15] { say "FizzBuzz" skip }
    when i % 3 == 0 { say "Fizz" skip }
    when i % 5 == 0 { say "Buzz" skip }
    say i
}
```

### 11.4 Fibonacci (recursive)

```txs
build fib(n) {
    when n <= 1 { give n }
    give fib(n-1) + fib(n-2)
}

each i in 0..10 {
    say fib(i)
}
```
```
0
1
1
2
3
5
8
13
21
34
```

### 11.5 A guessing game (loops + input + random)

```txs
make secret = random.randint(1, 100)
make guess = 0

repeat guess != secret {
    make guess = ask "Guess a number 1-100: "
    when guess < secret { say "Too low!" }
    when guess > secret { say "Too high!" }
}

say "You got it! 🎉"
```

### 11.6 Classes — Dogs & Cats (OOP)

```txs
model Animal {
    build init(self, name) {
        self.name = name
    }
    build speak(self) {
        say f"{self.name} makes a sound."
    }
}

model Dog {
    build init(self, name, breed) {
        self.name = name
        self.breed = breed
    }
    build speak(self) {
        say f"{self.name} says: Woof!"
    }
}

make rex = Dog("Rex", "German Shepherd")
rex.speak()
```

### 11.7 Error handling — a safe calculator

```txs
build safe_divide(a, b) {
    attempt {
        give a / b
    } catch err {
        say f"Error: {err.message}"
        give 0
    }
}

say safe_divide(10, 2)   # 5
say safe_divide(10, 0)   # Error: division by zero  → 0
```

### 11.8 A complete counter website (`use web`)

```txs
use web

make page = WebPage("Click Counter")

page.style("body", {
    "background": "#0f0f11",
    "color": "#0DF28B",
    "font-family": "monospace",
    "text-align": "center",
    "padding": "80px"
})

page.script("""
    let count = 0;
    function increment() {
        count++;
        document.getElementById('count').innerText = count;
    }
""")

page.body([
    page.h1("🐉 TechScript Counter"),
    page.p("Count: "),
    page.span("0", { "id": "count" }),
    page.button("Click Me!", { "onclick": "increment()" })
])

page.run()
```
```
$ tech run counter.txs
Server running — browser opened automatically. Press Ctrl+C to stop.
```

---

## 12. Roadmap & Conceptual Modules — 3D, AI, ML, Vision, GUI, Mobile, Networking

> ⚠️ **Read this section as vision, not shipped API.** The v1.0.8 public release documents, in concrete detail, exactly the modules covered in §5–§7 above: core syntax, the seven `math`/`crypto`/`json`/`fs`/`os`/`random`/`date` standard-library modules, and `use web`. The project's own roadmap lists several further modules as **not yet built** (`use http`, `use sql`, a package registry, WASM target, LSP support, an in-Studio REPL, native Android builds), and its longer-range design documents describe an even wider ambition — GUI apps, 3D scenes, animation, AI/ML integration, computer vision, games, and mobile apps — through a single, consistent `use <module>` mechanism. The syntax below is the project's own **illustrative design language** for that future, written in the same "reads like English" spirit as the shipped core, but it should not be read as functionality you can `tech run` today.

### 12.1 Official roadmap (from the project repository)

- [ ] Linux + macOS **native** Rust builds (today's Linux/macOS path uses `install.sh` / Homebrew wrapping the same toolchain, per §4)
- [ ] Standard library expansion: `use http`, `use sql`
- [ ] TechScript Package Registry
- [ ] WASM compilation target
- [ ] Language Server Protocol (LSP) support
- [ ] Interactive REPL inside Studio IDE itself
- [ ] Android native build (removing the current Python-wrapper/Termux path)

### 12.2 `use three_d` — introduced in v1.0.5

The changelog for v1.0.5 explicitly lists `use three_d` as a real, shipped addition ("3D scenes in 5 lines"), which makes it the one item in this section with the most concrete footing in the actual codebase, even though the public documentation site does not expose a detailed API reference for it. Illustrative shape, consistent with the rest of the language's builder-pattern style (as seen in `WebPage`):

```txs
use three_d

make scene = Scene()
scene.add(Cube({ "size": 2, "color": "blue" }))
scene.camera(Camera({ "position": [0, 2, 5] }))
scene.light(Light({ "type": "directional" }))
scene.run()
```

### 12.3 Conceptual — GUI desktop apps

Consistent with the `WebPage` builder pattern, a native desktop window module would plausibly look like:

```txs
use gui

make win = Window("Calculator")
win.size(400, 300)
win.button("Click", { "onclick": "handleClick()" })
win.show()
```

### 12.4 Conceptual — AI integration

```txs
use ai

make answer = ai.ask("Explain what a bytecode VM is, in one sentence.")
say answer
```

### 12.5 Conceptual — Machine Learning

```txs
use ml

make model = ml.load("spam_classifier.txmodel")
say model.predict(incoming_email)
```

### 12.6 Conceptual — Computer Vision / OCR

```txs
use vision

make text = vision.read_image("scanned_note.png")
say text
```

### 12.7 Conceptual — Animation

```txs
use anime

anime.move(cube, { "x": 100, "time": "2s" })
```

### 12.8 Conceptual — Mobile apps

```txs
use mobile

screen Home {
    text "Hello"
    button "Start"
}
```

### 12.9 Conceptual — Networking (`use http`, roadmap item)

```txs
use http

make data = http.get("https://api.example.com/status")
say data
```

---

## 13. TechScript vs Python vs JavaScript

| Feature / Task | 🐉 TechScript | 🐍 Python | 🌐 JavaScript |
|---|---|---|---|
| Print output | `say "Hello!"` | `print("Hello!")` | `console.log("Hello!")` |
| Variable | `make x be 10` | `x = 10` | `let x = 10;` |
| Constant | `keep PI be 3.14` | `PI = 3.14  # convention only` | `const PI = 3.14;` |
| If / condition | `when age > 18 { ... }` | `if age > 18:` | `if (age > 18) {` |
| Loop | `each i in 1..10 { ... }` | `for i in range(1,11):` | `for(let i=1;i<=10;i++){` |
| Function | `build greet(name) { ... }` | `def greet(name):` | `function greet(name) {` |
| Class (OOP) | `model Dog { ... }` | `class Dog:` | `class Dog {` |
| Error handling | `attempt { } catch err { }` | `try: ... except Exception:` | `try { } catch(e) { }` |
| Build a website | 1 file, zero HTML/CSS/JS | Flask/Django — extra install | Node/Express — extra install |
| Native VM speed | Pure Rust bytecode VM | Interpreted (CPython) | JIT via V8 |
| Bundled IDE | Studio IDE included | Separate install required | Separate install required |
| Semicolons required | None, ever | None needed | Required in strict mode |
| Cryptography built-in | SHA-256 / MD5 / Base64, no import | `hashlib` module (import) | `crypto` module (import) |

### Side-by-side: Fibonacci

```txs
// TechScript — reads like English
build fib(n) {
   when n <= 1 { give n }
   give fib(n-1) + fib(n-2)
}
say fib(10) // 55
```
```python
# Python
def fib(n):
    if n <= 1:
        return n
    return fib(n-1) + fib(n-2)
print(fib(10))  # 55
```
```javascript
// JavaScript
function fib(n) {
  if (n <= 1) return n;
  return fib(n-1) + fib(n-2);
}
console.log(fib(10)); // 55
```

---

## 14. Repository Anatomy

```
techscript/
├── assets/               # Logo and branding
├── bin/                  # Compiled binaries (TechScript_TX.exe)
├── docs/                 # Language reference and guides
│   ├── QUICKSTART.md
│   ├── REFERENCE.md
│   ├── STDLIB_REFERENCE.md
│   ├── WEB_MODULE.md
│   └── TERMUX.md
├── examples/             # 17+ ready-to-run .txs scripts
├── scripts/              # install.sh for Linux/macOS
├── vscode-extension/     # techscript-1.0.8.vsix
├── TechScript_v1.0.8_x64.exe   # Windows installer
└── README.md
```

### Example programs shipped in `examples/`

| File | What it does |
|---|---|
| `examples/hello.txs` | Hello World |
| `examples/fibonacci.txs` | Fibonacci numbers |
| `examples/fizzbuzz.txs` | Classic FizzBuzz |
| `examples/classes.txs` | OOP with Dogs & Cats |
| `examples/calculator.txs` | Simple calculator |
| `examples/guessing_game.txs` | Guess the number |
| `examples/07_performance_test.txs` | 1M-iteration benchmark |
| `examples/web_app_simple.txs` | Dark-theme website |
| `examples/web_complete.txs` | Counter + API + form |
| `examples/08_math_module.txs` | Math: trig, roots, statistics |
| `examples/09_string_ops.txs` | String operations |
| `examples/10_json_module.txs` | JSON encode/decode |
| `examples/11_crypto_module.txs` | SHA-256, Base64, MD5 |
| `examples/12_date_module.txs` | Date/time/unix |
| `examples/13_fs_module.txs` | File read/write/list |
| `examples/14_os_module.txs` | OS info, env vars |
| `examples/15_random_module.txs` | Random, UUID, choice |
| `examples/16_control_flow_fix.txs` | `stop`/`skip`/`in`/`typeof` |
| `examples/17_inline_eval.txs` | Inline execution how-to |

> **Note:** as of this writing, the top-level GitHub file listing for the repository shows `README.md`, the Windows installer executable, and the logo asset at the root; the `docs/` and `examples/` paths above are the structure the project's own README documents and links to. If a given doc file 404s when fetched directly, that reflects the repository's current state of population rather than an error in this specification.

---

## 15. Keyword & Symbol Reference Table

| Keyword / Symbol | Category | Meaning |
|---|---|---|
| `say` | I/O | Print to stdout |
| `ask` | I/O | Prompt for input, returns a string |
| `f"..."` | Literal | f-string with `{expr}` interpolation |
| `make` | Declaration | Mutable variable |
| `keep` | Declaration | Constant (immutable after creation) |
| `be` | Assignment (v1.0.8 dialect) | Alternate form of `=` |
| `when` | Control flow | `if` |
| `or when` | Control flow | `elif` |
| `else` | Control flow | `else` |
| `equals` / `then ... end` | Control flow (v1.0.8 dialect) | Alternate `==` / block delimiters |
| `each ... in ...` | Loop | `for` over a range or collection |
| `repeat` | Loop | `while` |
| `stop` | Loop control | `break` |
| `skip` | Loop control | `continue` |
| `in` | Operator | Membership / substring test |
| `typeof` | Operator | Runtime type name |
| `build` | Declaration | Function definition |
| `with` (v1.0.8 dialect) | Declaration | Introduces a function's parameter |
| `give` | Control flow | `return` |
| `model` | Declaration | Class definition |
| `self` | Reference | Instance reference inside a method |
| `init` | Method name | Constructor |
| `attempt` / `catch` | Error handling | `try` / `catch` |
| `err.message` | Error handling | Caught error's message text |
| `use <module>` | Module system | Imports a standard-library or builder module |
| `1..5` | Literal | Inclusive range |
| `1..1000` | Literal | Range (used in benchmarks) |

---

## 16. Limitations & Honest Assessment

A specification document is only useful if it is honest about maturity. As of v1.0.8:

- **Two syntax dialects coexist** (`{ }` classic style and `then ... end` v1.0.8 style). This is a genuine ergonomic strength for readability experimentation, but it also means a style guide and `tech fmt` consistency matter more than in a single-dialect language — mixed-dialect files are valid but harder to skim.
- **Full native builds outside Windows are still a roadmap item**, not yet shipped; Linux/macOS today install via a wrapper script or Homebrew formula around the same toolchain rather than a from-scratch native package for each OS.
- **Networking (`use http`) and SQL (`use sql`) are roadmap items**, not shipped standard-library modules, despite `use api`/`use db`-style syntax appearing in early design documents for the language. Anyone evaluating TechScript today for a project that needs outbound HTTP calls or direct SQL access should treat those as future work, not current capability.
- **3D (`use three_d`) shipped in v1.0.5** per the changelog, but the public site and README do not expose a detailed API reference for it the way they do for `math`/`crypto`/`json`/`fs`/`os`/`random`/`date` and `use web` — treat its exact surface area as under-documented rather than absent.
- **GUI, AI, ML, computer vision, animation, gaming, and mobile modules are design-language illustrations of where the project intends to go**, drawn from the language's own founding philosophy documents, not modules present in the shipped v1.0.8 standard library. They are included in this paper (§12) because the request behind this document explicitly asked for full-spectrum coverage of the language's ambition, and a research paper that silently omitted the roadmap would understate the project's actual scope of intent — but they are clearly labeled as such throughout, and no one should write `use ai` into a `.txs` file today expecting it to run.
- **`tech transpile`** converts TechScript to Python, which is a useful escape hatch (e.g., for environments with only a Python runtime, or for interoperating with the Python ecosystem) but is a one-way, distinct code path from the primary Rust VM execution model documented in §8.

---

## 17. Closing Notes

TechScript's core bet — that a small, deliberately English-shaped keyword set (`say`, `make`, `keep`, `when`, `each`, `repeat`, `build`, `give`, `model`, `attempt`/`catch`) backed by a genuinely native Rust bytecode VM can flatten the "HTML + CSS + JS + backend + DB" stack for a large class of programs — is coherent and, as of v1.0.8, backed by a real, benchmarked, non-Python execution engine, a 150+ function standard library with zero external imports, a working zero-HTML web builder, and a purpose-built IDE with live AST/bytecode inspection aimed squarely at teaching how the pipeline in §8 actually works. The project is young (2 GitHub stars, 8 commits on the visible history, roadmap items still open), and several of its most ambitious stated directions — full native cross-platform builds, networking, SQL, and especially the GUI/3D/AI/ML/mobile modules covered in §12 — are vision rather than shipped code today. Both of those things can be true of a young open-source language at once, and this document has tried to keep the line between them visible throughout rather than blur it for the sake of a more impressive-sounding specification.

```
╔══════════════════════════════════════════════════════════════════════════╗
║   🐉  tech version  →  TechScript v1.0.8 · Rust VM Edition                ║
║   "Write like a human. Run like Rust." — Tanmoy Majumder (@Tcode-Motion) ║
╚══════════════════════════════════════════════════════════════════════════╝
```

**End of document.**
