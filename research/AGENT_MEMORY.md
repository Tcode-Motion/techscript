# Agent Memory: TechScript 2.0 Onboarding & Memory Pack

This document is the master onboarding guide and source of truth for AI agents (Gemini CLI, Codex, Claude Code, Cursor, Copilot, etc.) working on the TechScript 2.0 compiler monorepo. **Read this file first before performing any research or modifications.**

---

## 1. Project Overview

**TechScript 2.0** is an English-like scripting language designed for absolute beginners, rapid prototypers, and web builders. It is a complete compiler and runtime rewrite in **Rust**, replacing the legacy Version 1 Python prototype.
- **Why it exists**: The legacy interpreter was slow and required a Python runtime. TechScript 2.0 compiles to a self-contained, high-performance single-binary executable (`tech`) with no external runtime dependencies.
- **Long-term vision**: A fast, memory-safe, and independent scripting ecosystem with tree-walking execution, stack-based bytecode VM, native LLVM compilation, and an integrated package manager.
- **Current development stage**: Phase 1 (Compiler Frontend). The monorepo workspace has been scaffolded; all 17 crates are defined and verified compile-clean.

---

## 2. Current Status

| Metric | Status |
| --- | --- |
| **Current Phase** | Phase 1 (Compiler Frontend) |
| **Current Milestone** | Milestone 1 (Lexer & Diagnostics Implementation) |
| **Architecture Completion %** | 100% (All engineering specs written) |
| **Implementation Completion %** | ~5% (Skeletal crates scaffolded with baseline APIs and tests) |
| **Graphify Status** | Up-to-date (249 nodes, 481 links) |
| **Workspace Status** | Functional Cargo workspace monorepo (17 crates compiling warning-free) |
| **CI Status** | Automated workflows (`ci.yml`, `release.yml`) configured; all local validation checks passing |
| **Current Branch** | `feat/workspace-scaffolding` |
| **Repository Health** | Excellent (Verified compile-clean, format-clean, warning-free) |

---

## 3. Project Goals

- **Short-term (v2.0)**: Complete compiler frontend (lexer, parser, semantic analyzer), tree-walking interpreter, CLI binary with subcommands, and baseline standard library modules (`io`, `math`).
- **Medium-term (v2.1)**: Build bytecode VM compiler, stack-based virtual execution machine, tracing generational garbage collector, and developer tooling (LSP server, formatter, linter).
- **Long-term (v3.0)**: Native execution compiler target utilizing an LLVM (`inkwell`) backend.
- **Ultimate Goal (v4.0)**: Self-hosting compiler (rewriting the compiler frontend in TechScript itself).

---

## 4. Repository Map

- `compiler/` - Compiler frontend crates:
  - [`common/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/compiler/common): Common data types (`Span`, `NodeId`, `Ident`).
  - [`syntax/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/compiler/syntax): Unified token registry, keywords, and operator precedence levels.
  - [`ast/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/compiler/ast): AST node configurations and Visitor traits.
  - [`errors/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/compiler/errors): ErrorCode register mapping and DiagnosticReporter.
  - [`lexer/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/compiler/lexer): DFA lexical tokenizer using Logos.
  - [`parser/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/compiler/parser): EBNF recursive descent statement builder and Pratt expression parser.
  - [`semantic/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/compiler/semantic): Scope resolution, shadowing rules, and semantic warnings.
- `runtime/` - Compiler backend execution and memory crates:
  - [`interpreter/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/runtime/interpreter): Tree-walking evaluator walker.
  - [`builtins/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/runtime/builtins): Registered native standard functions (`say`, `ask`, `len`).
  - [`gc/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/runtime/gc): Dynamic garbage collector allocator interface.
  - [`vm/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/runtime/vm): Bytecode virtual machine stack instruction builder.
- [`stdlib/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/stdlib) - Extended modular libraries (`io`, `math`, `string`, `file`, `web`).
- [`cli/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/cli) - Command-line target binary (`tech`) runner.
- `tools/` - Auxiliary developer tool packages:
  - [`lsp/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/tools/lsp): LSP Language Server tower-lsp bindings.
  - [`formatter/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/tools/formatter): Formatting print engine (`tech fmt`).
  - [`linter/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/tools/linter): Static analysis rules (`tech lint`).
  - [`package-manager/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/tools/package-manager): Remote registry indexing downloader (`tech install`).
- [`docs/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs) - Architectural, language, and AI onboarding documents.
- `examples/` - Integration execution examples.
- `tests/` - Integration test suite files.
- `.github/` - GitHub workflow actions (`ci.yml`, `release.yml`) and templates.
- [`graphify-out/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/graphify-out) - Knowledge graph maps.

---

## 5. Crate Map

| Crate Name | Path | Purpose / Responsibilities | Dependencies | Status / Future Work |
| --- | --- | --- | --- | --- |
| **`techscript_common`** | `compiler/common` | Primitives for AST nodes and source mappings. | `serde` | Scaffolded. Add source mapper helpers. |
| **`techscript_syntax`** | `compiler/syntax` | Keywords (31 active, 10 reserved), token kind enums, and precedence. | `serde`, `techscript_common` | Scaffolded. Add serialization. |
| **`techscript_ast`** | `compiler/ast` | Node tree layout configurations and AST Visitor trait. | `serde`, `techscript_common`, `techscript_syntax` | Scaffolded. Add JSON representations. |
| **`techscript_errors`** | `compiler/errors` | Diagnostic levels, ErrorCode register maps, and diagnostic reporter. | `serde`, `techscript_common` | Scaffolded. Add coloured reports. |
| **`techscript_lexer`** | `compiler/lexer` | DFA tokenizer rules and literal scanners. | `logos`, `serde`, `techscript_common`, `techscript_syntax`, `techscript_errors` | Skeletal. Implement Logos rule sets. |
| **`techscript_parser`** | `compiler/parser` | Recursive descent statements builder and Pratt expressions parser. | `serde`, `techscript_common`, `techscript_syntax`, `techscript_ast`, `techscript_errors` | Skeletal. Implement EBNF parse rules. |
| **`techscript_semantic`** | `compiler/semantic` | Lexical scoping hoisting, shadowing lints, and keyword checks. | `serde`, `techscript_common`, `techscript_syntax`, `techscript_ast`, `techscript_errors` | Scaffolded. Add reference validation. |
| **`techscript_interpreter`** | `runtime/interpreter` | Tree-walker execution visitor and dynamic scopes environment. | `serde`, `techscript_common`, `techscript_syntax`, `techscript_ast`, `techscript_errors`, `techscript_semantic` | Skeletal. Write statement visitors. |
| **`techscript_builtins`** | `runtime/builtins` | Registers native standard callbacks (`say`, `ask`, `len`). | `techscript_interpreter` | Scaffolded. Integrate interpreter call frames. |
| **`techscript_gc`** | `runtime/gc` | Allocator trait and GC specifications. | `techscript_interpreter` | Scaffolded (dummy). Build mark-and-sweep GC. |
| **`techscript_vm`** | `runtime/vm` | Bytecode OpCodes and VM stack frames execution loop. | `techscript_interpreter` | Scaffolded. Write AST-to-bytecode VM compiler. |
| **`techscript_stdlib`** | `stdlib` | Registers standard library package imports. | `techscript_interpreter` | Scaffolded. Add file-system and network helpers. |
| **`techscript_cli`** | `cli` | Main Clap-based executable command line client. | `clap`, compiler/runtime workspace crates | Scaffolded. Hook to compiler pipeline. |
| **`techscript_lsp`** | `tools/lsp` | tower-lsp JSON-RPC capabilities for IDEs. | `tower-lsp`, `serde`, compiler/runtime workspace crates | Scaffolded. Hook LSP to parser outputs. |
| **`techscript_formatter`** | `tools/formatter` | Reformats source files inline to standard indentation. | `techscript_common`, `techscript_syntax`, `techscript_ast` | Skeletal. Write layout formatter printer. |
| **`techscript_linter`** | `tools/linter` | Static linter rule sets and compatibility checks. | `techscript_common`, `techscript_syntax`, `techscript_ast`, `techscript_errors`, `techscript_semantic` | Scaffolded. Add unused import sweeps. |
| **`techscript_package_manager`**| `tools/package-manager`| Online package index registry registry client. | `serde` | Scaffolded. Add metadata downloaders. |

---

## 6. Compiler Pipeline

1. **Source (`.txs`)**: Raw UTF-8 source script.
2. **Lexer (`techscript_lexer`)**: Tokenizer scanning source code to emit a stream of `Token` structures containing spans and lexemes.
3. **Parser (`techscript_parser`)**: Builds Abstract Syntax Trees using EBNF recursive descent (statements) and Pratt parsing (expressions).
4. **AST (`techscript_ast`)**: Root `Program` node containing lists of `Statement` and `Expression` variants.
5. **Semantic Analyzer (`techscript_semantic`)**: Resolves symbol mappings across nested scopes, flags duplicate declarations, and issues deprecation warnings.
6. **Interpreter (`techscript_interpreter`)**: Dynamic tree-walking evaluator execution, managing runtime scopes environments.
7. **Virtual Machine (`techscript_vm` - Future)**: Stack VM compiler compiling AST nodes to instruction OpCodes.
8. **LLVM Backend (Future)**: inkwell native compiler IR code emission.

---

## 7. Language Summary

- **File Extension**: `.txs` (strictly lowercase). Reject `.tech` across the pipeline.
- **Syntax**: English-like keywords, block structures grouped using braces `{ }`. Semicolons `;` are optional statement terminators (primarily used for multi-statement lines).
- **Keywords**: `make`, `const`, `say`, `ask`, `build`, `return`, `fun` (deprecated), `model`, `self`, `new`, `when`, `else`, `each`, `in`, `repeat`, `while`, `break`, `continue`, `attempt`, `catch`, `throw`, `import`, `from`, `export`, `true`, `false`, `none`, `and`, `or`, `not`, `is`.
- **Variables**: Dynamic typing. Decared with `make` (mutable) or `const` (immutable constant).
- **Functions**: Declared using `build` (e.g. `build add(a, b) { return a + b }`). Supports lambdas.
- **Models**: Defines class blueprints containing fields and method builds:
  ```
  model Person {
      make name = ""
      build init(name) { self.name = name }
  }
  ```
  Instantiated using `new Person("Alice")`. Methods can use deprecated `fun` alias, which compiles successfully but emits a deprecation warning (`W0015`).
- **Modules**: Selectively import using `from module import symbol` or full import `import module`. Export using `export build symbol`.
- **Error Handling**: Braced exception capture block: `attempt { ... } catch err { ... }`, thrown via `throw expression`.
- **Collections**: Lists `[1, 2, 3]` and Maps `{"key": "value"}`.
- **Strings**: Unicode-safe string literals `"hello"` and string interpolation f-strings `f"Hello {name}"`.
- **Loops**: `each item in list { ... }`, `repeat count { ... }`, and `while condition { ... }`.
- **Built-in Functions**: `say()` (prints with newline), `ask()` (reads stdin), and `len()` (collection/string length).
- **CLI Commands**: Single binary client (`tech`) with subcommands: `run`, `repl`, `check`, `fmt`, `lint`, `test`, `new`, `version`.

---

## 8. Runtime Summary

- **Execution Model**: AST Tree-Walking visitor evaluator (v2.0) transitioning to bytecode stack VM (v2.1).
- **Scopes**: Lexical variables environments mapping identifiers to values.
- **Values**: Runtime representation variant enums: `Int`, `Float`, `Str`, `Bool`, `None`, `List`, `Map`.
- **Memory**: Tracing generational garbage collector managing allocations (dummy tracking allocator in v2.0).
- **Objects**: Map layouts binding properties to instances.
- **Modules**: Module dictionary namespace lookups.
- **Built-ins**: Native function callbacks registered with the interpreter environment.
- **Errors**: `RuntimeError` conversions (type mismatch, divide by zero, stack overflow, out-of-bounds index).

---

## 9. Development Rules

- **Coding Standards**: All Rust code must compile warning-free, follow idiomatic structures, use no `unsafe` code blocks, and pass linter checks:
  - Formatting: `cargo fmt --all --check`
  - Clippy: `cargo clippy --workspace --all-targets -- -D warnings`
  - Compilation: `cargo check --workspace`
  - Workspace tests: `cargo test --workspace`
  - Documentation: `cargo doc --workspace --no-deps`
- **Architecture Rules**: Modular crate division. Circular dependencies are strictly forbidden. Common types reside in `techscript_common` or `techscript_syntax`.
- **Naming Conventions**:
  - Crates: `snake_case` prefixed with `techscript_` (e.g. `techscript_lexer`).
  - Source files/Modules: `snake_case` (e.g. `symbol_table.rs` or `.txs` scripts).
  - Structs/Enums: `PascalCase` (e.g. `CheckedProgram`).
  - Variables/Functions: `snake_case` (e.g. `resolve_names()`).
  - Constants: `SCREAMING_SNAKE_CASE` (e.g. `MAX_STACK_DEPTH`).
- **Documentation**: All public crates, interfaces, modules, and structs must be documented (`///`) with module-level instructions (`//!`) at the top of the entry files.
- **Git Workflow**: Branch from `dev` using descriptive prefixes `feature/name` or `fix/name`. Commits follow conventional commits rules (e.g. `feat: ...`, `fix: ...`).
- **CI/CD Expectations**: Code pushes trigger automated test runs, format checks, clippy lints, and Graphify updates. No PR can be merged unless all checks are green.

---

## 10. AI Rules

Every AI assistant working on this codebase must adhere to the following rules:
1. **Read this file first** to load context before proposing changes.
2. **Never redesign completed architecture** unless explicitly instructed by the user.
3. **Never break backward compatibility** with existing TechScript 2.0 specs.
4. **Never rename crates** without a documented architectural reason.
5. **Never invent syntax** or keywords outside of the language specifications.
6. **Always follow the engineering documents** located in [`docs/engineering/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/engineering) as the source of truth.
7. **Always update Graphify** after repository structure modifications by running `python tools/update_graphify.py`.
8. **Always update AI context documents** (such as `SESSION_CONTEXT.md` and `AI_CONTEXT_PACK.md`) when completing milestone phases.
9. **Always preserve `.txs`** as the only valid script file extension.
10. **Always preserve the TechScript 2.0 design philosophy**: readability over brevity, progressive disclosure, safety by default.

---

## 11. Current Progress

- [x] **Research**: Complete, authoritative language requirements written.
- [x] **Architecture**: Master system architecture finalized.
- [x] **Repository**: Monorepo structures established.
- [x] **Workspace**: Cargo workspace workspace configuration set up.
- [x] **CI/CD**: Auto workflows (tests, clippy, doc, graph check) configured.
- [x] **Graphify**: Configured and verified integration tools.
- [x] **Documentation**: 19 detailed engineering specification documents written.
- [x] **AI Context**: Context and handoff index files initialized.
- [x] **Scaffolding**: All 17 crates scaffolded, compiling, and tested green.
- [x] **Language Design**: Specification, keywords, and operators finalized.
- [x] **Compiler Design**: Pratt precedence and parsing plans mapped.
- [x] **Runtime Design**: Execution scope models documented.

---

## 12. Remaining Work

All implementation tasks are scheduled sequentially:
1. **Lexer (`techscript_lexer`)**: Write Logos lexer tokenizer DFA scan rules.
2. **Errors (`techscript_errors`)**: Write diagnostics warning/error print formatting.
3. **Parser (`techscript_parser`)**: Write recursive descent parser and Pratt expression compiler.
4. **AST (`techscript_ast`)**: Write AST nodes parser construction.
5. **Semantic Analyzer (`techscript_semantic`)**: Implement scopes declaration hoisting and check passes.
6. **Interpreter (`techscript_interpreter`)**: Implement tree-walking statement and expression visitor execution.
7. **Builtins (`techscript_builtins`)**: Implement standard native callbacks mapping.
8. **GC (`techscript_gc`)**: Implement generational mark-and-sweep collector.
9. **VM (`techscript_vm`)**: Implement stack virtual machine execution.
10. **Stdlib (`stdlib`)**: Implement modular io and math interfaces.
11. **CLI (`cli`)**: Complete CLI subcommands.
12. **Formatter (`tools/formatter`)**: Implement formatting layout prints.
13. **Linter (`tools/linter`)**: Implement linting rules.
14. **LSP (`tools/lsp`)**: Implement tower-lsp JSON-RPC capability handlers.
15. **Package Manager (`tools/package-manager`)**: Implement remote registry downloader.
16. **LLVM Backend**: Implement LLVM IR code emission using inkwell.
17. **Self-Hosting**: Rewrite compiler frontend in TechScript.

---

## 13. Important Files

- [`README.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/README.md): Workspace instructions and startup guide.
- [`Cargo.toml`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/Cargo.toml): Cargo workspace manifest.
- [`IMPLEMENTATION_ORDER.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/IMPLEMENTATION_ORDER.md): 16-step execution sequence.
- [`docs/ai/AI_CONTEXT_PACK.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/ai/AI_CONTEXT_PACK.md): Self-contained AI context pack.
- [`docs/GRAPHIFY_AI.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/GRAPHIFY_AI.md): Graphify usage and query instructions.
- [`docs/engineering/01_language_spec_v1.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/engineering/01_language_spec_v1.md): Language specification, keywords, and operators.
- [`docs/engineering/00_master_architecture.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/engineering/00_master_architecture.md): Master system pipeline architectures.
- [`docs/engineering/03_grammar_ebnf.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/engineering/03_grammar_ebnf.md): Language EBNF grammar specifications.
- [`docs/engineering/05_ast_design.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/engineering/05_ast_design.md): AST node structures andVisitor traits.

---

## 14. AI Workflow

Whenever a new AI agent starts a task:
1. **Read this file (`AGENT_MEMORY.md`)** first.
2. Read the Graphify navigation report [`graphify-out/GRAPH_REPORT.md`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/graphify-out/GRAPH_REPORT.md).
3. Read the relevant specification files under [`docs/engineering/`](file:///c:/Users/Tanmoy/OneDrive/Documents/TechScript%202.0/docs/engineering) that match your task.
4. **Never scan unrelated folders**; focus your context window.
5. Work only inside the assigned crate/module scope.
6. If modifying code dependencies, update `Cargo.toml`.
7. **Regenerate Graphify** index by running `python tools/update_graphify.py` after completing file structures changes.

---

## 15. Session Memory

- **Current Milestone**: Milestone 1 (Lexer & Diagnostics implementation).
- **Last Completed Feature**: Completed Cargo monorepo workspace scaffolding. All 17 crates are defined and verified compiling green.
- **Current Blocker**: None.
- **Next Recommended Task**: Implement DFA Logos token matching rules in `techscript_lexer` and error print formatting in `techscript_errors`.
- **Recent Architectural Decisions**:
  - Structured CLI target as a library+binary crate to enable integration testing.
  - Re-exported shared common types (`Span`, `NodeId`, `Ident`) publicly from `techscript_ast` to keep imports clean.
  - Set cargo target directory outside OneDrive (`C:\Users\Tanmoy\.gemini\cargo_target`) during development to avoid real-time sync file locking errors.

---

## 16. Quick Context (One Minute Read)

- **What TechScript is**: An English-like dynamique language designed for beginners, built as a single Rust binary toolchain (`tech`).
- **Repository status**: 17 crates scaffolded and compiling green. Checks pass clippy, fmt, check, and test suite.
- **Current milestone**: M1 (Lexer & Diagnostics).
- **Next task**: Implement Logos scanner rules inside `techscript_lexer`.
- **Important rules**: No unsafe, no circular dependencies, keep `.txs` extension lowercase, follow engineering specifications, regenerate Graphify after file modifications.
- **Development philosophy**: Readability over brevity, progressive disclosure, safety by default.
