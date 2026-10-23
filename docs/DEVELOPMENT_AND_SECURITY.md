# TechScript 2.0: Development, Architecture & Security Guide

> **Maintainer Handbook for Compiler Engineering, Security Audits & Ecosystem Architecture**  
> Edition: TechScript 2.0 LTS | Primary Implementation: Rust

---

## 1. Compiler Architecture & Pipeline Data Flow

The TechScript compiler transforms source code (`.txs`) into executed programs, static web assets, or native application binaries through a modular multi-stage pipeline:

```
[Source Code .txs]
        │
        ▼
   [Lexer] ──────────► Token Stream (techscript_lexer)
        │
        ▼
   [Parser] ─────────► Abstract Syntax Tree (techscript_ast / parser)
        │
        ▼
 [Semantic Analysis] ─► Symbol Resolution & Type Checking (techscript_semantic)
        │
        ▼
   [IR Lowering] ────► Flat SSA Intermediate Representation (techscript_ir)
        │
        ▼
  [Optimizer] ───────► Dead-code Elimination, Constant Folding (techscript_optimizer)
        │
  ┌─────┴─────────────────────────┬─────────────────────────┐
  │                               │                         │
  ▼                               ▼                         ▼
[Bytecode Lowering]      [LLVM Native Codegen]      [Webforge / Appforge]
(techscript_bytecode)    (techscript_llvm_backend)  (stdlib::web / CLI app)
  │                               │                         │
  ▼                               ▼                         ▼
[Stack-based VM]           [Native Object]          [dist/ & APK/AAB/EXE]
(techscript_vm)            (Linker Executable)      (Gradle & Platform Bundles)
```

---

## 2. Workspace Crate Inventory

The TechScript workspace contains 21 focused crates:

| Crate | Directory | Purpose |
| :--- | :--- | :--- |
| `techscript_common` | `compiler/common` | Source roles, file validation, span descriptors |
| `techscript_syntax` | `compiler/syntax` | Token definitions, keyword sets, precedence |
| `techscript_errors` | `compiler/errors` | Diagnostic codes, error reporting, source snippets |
| `techscript_ast` | `compiler/ast` | Abstract Syntax Tree node definitions |
| `techscript_lexer` | `compiler/lexer` | Lexical scanner and tokenization engine |
| `techscript_parser` | `compiler/parser` | Recursive-descent grammar parser with DSL engine |
| `techscript_semantic` | `compiler/semantic` | Symbol table, scope analysis, type validation |
| `techscript_ir` | `compiler/ir` | Flat Static Single Assignment (SSA) IR |
| `techscript_optimizer`| `compiler/optimizer` | Optimization passes (folding, dead code) |
| `techscript_bytecode` | `compiler/bytecode` | Bytecode instruction set and lowering |
| `techscript_llvm_backend` | `compiler/llvm_backend` | LLVM IR generation and JIT/AOT hooks |
| `techscript_vm` | `runtime/vm` | Stack-based Virtual Machine execution engine |
| `techscript_interpreter` | `runtime/interpreter` | Direct AST evaluation runtime |
| `techscript_builtins` | `runtime/builtins` | Core intrinsic functions |
| `techscript_runtime` | `runtime/runtime` | Memory model, callable traits, values |
| `techscript_stdlib` | `stdlib` | Standard library modules (math, web, json, etc.) |
| `techscript_module_resolver` | `compiler/module_resolver` | Import resolution and path security |
| `techscript_package_manager` | `tools/package-manager` | Dependency resolution, semver, lockfiles |
| `techscript_formatter`| `tools/formatter` | Code formatting engine (`tsc fmt`) |
| `techscript_linter` | `tools/linter` | Static analysis and deprecation warnings |
| `techscript_cli` | `cli` | Unified `tsc` CLI command runner |

---

## 3. Security Architecture & Threat Model

### 3.1 Path Traversal & Archive Extraction Protection
- The module resolver (`techscript_module_resolver`) strictly forbids `..` path components and directory traversal.
- The package manager (`techscript_package_manager` & `tsc install`) validates that dependency names and extract destinations contain no path traversal characters (`..`, `/`, `\`).

### 3.2 Capability-Based Sandboxing
TechScript packages declare required capabilities (`FileSystem`, `Network`, `Process`) in `techscript.toml`. The package manager enforces:
- Dependencies cannot elevate permissions beyond what the parent project is granted.
- Network requests are validated against project-declared host whitelists.

### 3.3 Safe Process Invocation
The unified CLI passes arguments directly as discrete strings (`std::process::Command::new().args([...])`) rather than concatenating shell strings, completely preventing command-injection attacks via filenames or arguments.

### 3.4 Android Application Security
Scaffolded Appforge Android hosts strictly configure:
- `allowFileAccess = false` and `allowContentAccess = false` on WebViews.
- Local streaming via Google's `WebViewAssetLoader` with encrypted `https://appassets.androidplatform.net/assets/` domain mapping.
- Minimal permissions: only `android.permission.INTERNET` by default.
- No hardcoded private keys, keystore passwords, or credentials in templates.

---

## 4. Quality Gates & Test Execution

### 4.1 Running All Tests
```bash
# Verify workspace compilation across all 21 crates
cargo check --workspace

# Run complete automated test suite
cargo test -p techscript_cli
cargo test --workspace

# Run official language test battery
python verify_all_examples.py
```

### 4.2 Code Formatting & Linting
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

---

## 5. Contributing & Adding Features

### Adding a New CLI Command
1. Define the command argument enum in `cli/src/lib.rs`.
2. Implement execution logic in `cli/src/commands/<command>.rs`.
3. Dispatch the command in `cli/src/entrypoint.rs`.
4. Add CLI snapshot and functional tests in `cli/tests/`.

### Adding a New Standard Library Module
1. Create `stdlib/src/<module>.rs`.
2. Implement functions adhering to the `StdFnCallback` signature.
3. Register the module in `stdlib/src/lib.rs` under `StdlibRegistry::register_all()`.
4. Expose documentation in `docs/TECHSCRIPT_LANGUAGE_REFERENCE.md`.
