# 09 — AI BOOTSTRAP

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Immediate quick-start guide for AI agents
> **Parent Link**: [INDEX](./12_index.md)
> **Child Links**: [PROGRESS](./06_progress.md) · [HANDOFF](./10_handoff.md)

---

## 1. Quick-Start Context

TechScript 2.0 is a complete compiler and runtime rewrite in **Rust**, replacing the legacy Version 1 Python prototype.
- **Source extension**: Strictly `.txs`. Reject all other file extension endings.
- **Unified Keyword**: Functions and class methods are declared using `build`. `fun` is supported inside models as a deprecated alias (emits `W0015` warnings).
- **Core Executable**: A single binary `tech` compiled via Cargo workspace.

---

## 2. Coding & Style Rules

### 2.1 Rust Style
- Must pass `cargo fmt` and `cargo clippy --all-targets -- -D warnings`.
- Zero `unsafe` blocks allowed in the codebase (unless documented in `techscript_gc` or `techscript_llvm`).
- Structured error propagation using `Result<T, Diagnostic>` instead of panic.

### 2.2 Commit Messages
Must match Conventional Commit formats:
- Examples: `feat(parser): parse when statements`, `fix(lexer): correct block comment checks`.
- Scopes: `lexer`, `parser`, `ast`, `sema`, `interp`, `runtime`, `stdlib`, `cli`, `lsp`.

---

## 3. Important "Do Not Change" List

- **File extension**: Never modify `.txs` target checks or allow compilation of `.tech` files.
- **Unified keyword behavior**: Never remove the linter/warning check for `fun` methods. Legacy code must parse but emit `W0015`.
- **CLI Subcommands**: Do not alter, rename, or drop subcommands `run`, `repl`, `check`, `fmt`, `lint`, `test`, `version`, `help`, or `new`.
- **Diagnostic output format**: Error messages must maintain the standard format, displaying error code, file name, line, column, and help suggestions.

---

## 4. Current Target (Milestone 1)

1. **Task**: Initialize Cargo workspace workspace crates (`compiler/errors`, `compiler/ast`, `compiler/lexer`).
2. **Crate**: `techscript_errors` (implements diagnostics rendering) and `techscript_lexer` (scans `.txs` source text into tokens).
3. **Preferred Dependency**: Use `logos` crate for compiling lexer token rules.
4. **Current Branch**: `feat/lexer-bootstrap` (proposed).
5. **Expected Output**: Running `cargo test -p techscript_lexer` executes lexer test assertions.
