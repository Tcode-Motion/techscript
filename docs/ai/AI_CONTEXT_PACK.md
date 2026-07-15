# TechScript 2.0 Master AI Context Pack

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Self-contained, token-efficient, single-file context bootstrap for immediate project onboarding.
> **Last Updated**: 2026-07-15
> **Version**: 2.0.0
> **Parent Link**: [INDEX](./12_index.md)

---

## 1. Project Context & Vision

TechScript 2.0 is an English-like scripting language designed for absolute beginners, rapid prototypers, and web builders. It is a complete compiler and runtime rewrite in **Rust**, replacing the legacy Version 1 Python prototype.
- **Goal**: Build a self-contained, single-binary scripting ecosystem (`tech`) that is fast, memory-safe, and independent of external runtimes.
- **Language Philosophy**: Readability over brevity, progressive disclosure, safety by default.

---

## 2. Invariant Rules (Do Not Change)

1. **Frozen Extension**: All source files, examples, and tests must use the `.txs` extension. Reject `.tech` files.
2. **Unified Syntax**: Standalone functions and class methods are canonically declared using `build`.
3. **`fun` Deprecation**: `fun` is supported only inside models as a deprecated alias. Using it produces warning `W0015` at compile-time but runs successfully.
4. **Official CLI Commands**:
   - `tech run <file.txs>` (Executes script)
   - `tech repl` (Starts interactive shell)
   - `tech check <file.txs>` (Lints and checks syntax)
   - `tech fmt <file.txs>` (Formats code)
   - `tech lint <file.txs>` (Static code validation, support `--fix` to rewrite `fun` to `build`)
   - `tech test [dir]` (Discovers and runs `*_test.txs` files)
   - `tech version` (Prints version info)
5. **No Unsafe Code**: Prohibit `unsafe` Rust blocks unless reviewed and required for FFI (LLVM backend).

---

## 3. Language Cheat Sheet

```
// Dynamic mutable variable
make age = 30

// Constant
const PI = 3.14159

// Print to stdout
say f"Age is {age}"

// Conditional
when age >= 18 {
    say "Adult"
} else {
    say "Minor"
}

// Loop
each i in 0..5 {
    say i
}

// Unified Function
build add(a, b) {
    return a + b
}

// Model (Class)
model Person {
    make name = ""
    build init(name) {
        self.name = name
    }
}
```

---

## 4. Compiler Pipeline & Crate Map

```
Source (.txs) → Lexer (logos) → Parser (Pratt) → AST → Semantic Analyzer (Sema) → Interpreter (Tree-walker)
```

### Crate Structure
- `techscript_errors`: Unified error reporting and span mapping (`E0001`–`E1999`, `W0001`–`W0099`).
- `techscript_ast`: Holds AST node definitions and the shared AST `Visitor` trait.
- `techscript_lexer`: Scans UTF-8 source code using `logos` DFA matching.
- `techscript_parser`: Recursive descent for statements, Pratt parser for expressions.
- `techscript_sema`: Scopes, name resolution, duplicate checks, and deprecation triggers.
- `techscript_interpreter`: AST walker, manages local environments and call frames.
- `techscript_stdlib`: Extended libraries (`io`, `math`, `string`, `file`, `web`, `time`, `random`, `json`, `collections`).
- `techscript_cli`: CLI main entry point.

---

## 5. Development Targets

- **Current Milestone**: Milestone 1 (Lexer & Diagnostics implementation).
- **Next Task**: Workspace setup. Implement diagnostics printing in `techscript_errors` and DFA token matching inside `techscript_lexer`.
- **Coding Conventions**: All Rust code must pass `cargo fmt` and `cargo clippy --all-targets -- -D warnings`. Commits use the conventional format (e.g. `feat(lexer): scan integers`).
