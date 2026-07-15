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
- **`techscript_common`**: Shared primitives: `Span`, `NodeId`, `Ident`, and source locations.
- **`techscript_syntax`**: Unified keyword lists (31 active, 10 reserved), token enums, and precedence tables.
- **`techscript_ast`**: AST node structure layouts and visitor traits.
- **`techscript_errors`**: Diagnostic registers (`E0001`–`E1999`, `W0001`–`W0099`) and rendering formats.
- **`techscript_lexer`**: Logos-based UTF-8 DFA token scanner.
- **`techscript_parser`**: Pratt expression and recursive descent parser.
- **`techscript_semantic`**: Scopes analysis, duplicate checkers, and deprecation lints.
- **`techscript_interpreter`**: Tree-walking execution engine.
- **`techscript_builtins`**: Standard pre-registered native helper methods.
- **`techscript_gc`**: generational mark-and-sweep tracking allocator.
- **`techscript_vm`**: stack-based bytecode compiler and virtual machine.
- **`techscript_stdlib`**: Extended modules (`io`, `math`, `string`, `file`, `web`).
- **`techscript_cli`**: Clap CLI binary launcher.
- **`techscript_lsp`**: tower-lsp service for IDE support.
- **`techscript_formatter`**: `tech fmt` AST standard layout manager.
- **`techscript_linter`**: `tech lint` static code rule checkers.
- **`techscript_package_manager`**: Registry client and version dependency solver.

---

## 5. Development Targets

- **Current Milestone**: Phase 1 Workspace Scaffolding completed. All 17 crates are successfully configured, compiling, clippy warning-free, and tested.
- **Next Task**: Implement full Logos parser rules and EBNF syntax bindings under M1 guidelines.
- **Coding Conventions**: All Rust code must pass `cargo fmt` and `cargo clippy --workspace --all-targets -- -D warnings`. Commits use the conventional format (e.g. `feat(lexer): scan integers`).
