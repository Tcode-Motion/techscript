# 18 — TechScript 2.0 AI Context Document

> **Status**: Living Document
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Purpose**: Provide any AI assistant with complete context about the TechScript 2.0 project in a single document.

---

## Project Overview

**TechScript 2.0** is a complete compiler and runtime rewrite in **Rust**, maintaining the friendly, English-like syntax of the original Python-based Version 1 prototype while laying the foundation for a production-grade compiled systems ecosystem.

**Implementation language**: Rust
**Repository structure**: Cargo workspace monorepo
**Current stage**: Pre-implementation (engineering documentation complete, no code yet)

---

## Language Philosophy

1. **Readability over brevity** — Keywords like `make`, `say`, `when`, `each`, `build` instead of symbols
2. **Progressive disclosure** — Start simple (dynamic typing), add complexity later (static types, async)
3. **Single-binary distribution** — The `tech` CLI is self-contained
4. **Layered compilation** — Same frontend, swappable backends (interpreter → VM → LLVM)
5. **Safety by default** — Written in Rust; no null pointers, no data races

---

## Key Language Features (v2.0)

| Feature | Syntax | Example |
|---|---|---|
| Variables | `make x = value` | `make name = "Alice"` |
| Constants | `const X = value` | `const PI = 3.14159` |
| Print | `say value` | `say f"Hello, {name}!"` |
| Input | `ask "prompt"` | `make name = ask "Name? "` |
| Functions | `build name(params) { }` | `build add(a, b) { return a + b }` |
| Methods | `build name(params) { }` | `build bark() { say "woof!" }` (inside model) |
| Deprecated Method | `fun name(params) { }` | `fun bark() { say "woof!" }` (triggers warning `W0015`) |
| Models (classes) | `model Name { }` | `model Dog { make name = "" }` |
| Conditionals | `when cond { } else { }` | `when age >= 18 { say "Adult" }` |
| For-each | `each x in iterable { }` | `each i in 0..10 { say i }` |
| Repeat | `repeat n { }` | `repeat 5 { say "hi" }` |
| While | `while cond { }` | `while count < 10 { count += 1 }` |
| Error handling | `attempt { } catch err { }` | `attempt { risky() } catch e { say e }` |
| Imports | `import module` | `import math` |

**File extension**: `.txs` (Frozen)
**Typing**: Dynamic (v2.0), optional static types planned for v2.2+
**Concurrency**: None in v2.0 (sequential execution)

---

## Compiler & Repository Architecture

```
Source (.txs) → Lexer → Tokens → Parser → AST → Semantic Analyzer → Interpreter (v2.0)
                                                                    → Bytecode VM (v2.1)
                                                                    → LLVM Native (v3.0)
```

**Crate structure**:
- `techscript_errors` — Error codes, diagnostics, source mapping
- `techscript_ast` — AST node types (shared by all phases)
- `techscript_lexer` — Tokenization (uses `logos`)
- `techscript_parser` — Recursive descent + Pratt parsing
- `techscript_sema` — Scope check, validation, deprecation warnings
- `techscript_interpreter` — Tree-walking AST interpreter
- `techscript_builtins` — Built-in functions (`say`, `ask`, `len`, etc.)
- `techscript_stdlib` — Standard library (math, string, file, web, time, random, json, collections)
- `techscript_cli` — CLI binary (`tech run`, `tech repl`, etc.)
- `techscript_lsp` — Language server for IDE integration
- `techscript_fmt` — Code formatter
- `techscript_lint` — Linter

---

## Folder Structure

```
techscript/
├── compiler/          # Frontend (lexer, parser, ast, semantic, errors)
├── runtime/           # Backend (interpreter, builtins, gc, vm)
├── stdlib/            # Standard library modules
├── cli/               # CLI binary
├── tools/             # LSP, formatter, linter, package manager
├── tests/             # All test files (renamed to *.txs)
├── examples/          # Example programs (renamed to *.txs)
├── docs/engineering/  # This documentation suite (19 documents)
└── .github/           # CI/CD workflows
```

---

## Resolved Design Decisions

| Decision | Choice | Rationale | Compatibility Impact |
|---|---|---|---|
| **File extension** | `.txs` | Frozen by user constraint. Replaces all previous references. | Non-`.txs` files rejected. |
| **Method keyword** | Unified on `build` | Simplifies the grammar; a method is conceptually a function. | `fun` is retained as a deprecated alias inside models (emits `W0015`). |
| **Web module** | Optional v2.0 | Core standard library module (`import web`). | Backward compatible. |
| **Numeric Separators** | Underscores allowed | Improves readability of large numbers (e.g. `1_000_000`). | Backward compatible. |
| **Boolean literals** | `true` / `false` | Standard boolean representation. | Backward compatible. |

---

## Development Roadmap

| Version | Backend | Key Feature | Timeline |
|---|---|---|---|
| v2.0 | Tree-walking interpreter | Rust rewrite, `.txs` file runs, auto-fix `fun` | Q4 2026 |
| v2.1 | Bytecode VM | Tracing GC, Flat virtual machine execution | Q1 2027 |
| v3.0 | LLVM compiler | Compiled native binaries, optimizations | Q2 2027 |
| v3.1 | Package manager | Manifest dependency resolution | Q3 2027 |
| v4.0 | Self-hosting | Compiler written in TechScript | Q4 2027 |

---

## Rules for AI Assistants

1. **Implementation language**: Always use Rust.
2. **File Extension**: Strictly use `.txs` for all source, test, and example files.
3. **Keyword deprecation**: Support both `build` and `fun` inside models; emit `W0015` warnings for `fun`.
4. **Source of truth**: The 19 engineering documents in `docs/engineering/` are the authoritative specification.
5. **No invented features**: Do not add language features not described in the specification.

---

## Document Index

| # | Document | Purpose |
|---|---|---|
| 00 | Master Architecture | System overview, dependency graph, design philosophy, future evolution |
| 01 | Language Spec v2.0 | Complete v2.0 language definition |
| 02 | Folder Structure | Monorepo layout with purpose annotations |
| 03 | EBNF Grammar | Formal grammar for parser implementation |
| 04 | Compiler Architecture | Pipeline stages and design decisions |
| 05 | AST Design | All AST node types as Rust structs/enums |
| 06 | Lexer Design | Token types, scanning rules, disambiguation |
| 07 | Parser Design | Recursive descent + Pratt parsing, error recovery |
| 08 | Milestones | GitHub milestones with acceptance criteria |
| 09 | Runtime Design | Execution model, value representation, GC, call stack |
| 10 | Semantic Analysis | Scope/name resolution, symbol tables, validation rules |
| 11 | Interpreter Design | AST evaluation, operator semantics, signals |
| 12 | Stdlib Design | 9 standard library modules with function signatures |
| 13 | CLI Spec | All commands, flags, exit codes, project manifest |
| 14 | Error Codes | Every error with ID, description, cause, fix, example |
| 15 | Testing Strategy | Test types, coverage targets, CI, fuzzing |
| 16 | Coding Standards | Naming, style, commits, reviews, dependencies |
| 17 | Roadmap | Weekly milestones across 6 development phases |
| 18 | AI Context | THIS DOCUMENT — quick-start for AI assistants |

---

*Provide this document first to any AI assistant working on TechScript 2.0. It contains everything needed to understand the project at a glance.*
