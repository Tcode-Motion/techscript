# 08 — REPOSITORY

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Repository folder structure and build tasks guide
> **Parent Link**: [ARCHITECTURE](./04_architecture.md)
> **Child Links**: [AI_BOOTSTRAP](./09_ai_bootstrap.md) · [INDEX](./12_index.md)

---

## 1. Directory Structure

```
techscript/
├── Cargo.toml                    # Cargo workspace root config
├── rustfmt.toml                  # Code formatting guidelines
├── compiler/                     # Compiler frontend modules
│   ├── errors/                   # Diagnostics, spans, and source map
│   ├── ast/                      # AST nodes and Visitor trait
│   ├── lexer/                    # Tokenization using logos crate
│   ├── parser/                   # Recursive descent + Pratt parser
│   └── semantic/                 # Scopes, hoisting, symbol table
├── runtime/                      # Compiler backend execution
│   ├── interpreter/              # AST tree-walking interpreter
│   ├── builtins/                 # Core global built-in functions
│   └── vm/                       # Stack-based VM (v2.1)
├── stdlib/                       # Standard library modules (math, web, etc.)
├── cli/                          # Cargo binary entry point (tech executable)
├── tools/                        # Formatter, linter, and LSP server
├── tests/                        # Snapshot and integration tests (*.txs)
└── docs/                         # Documentation
    ├── engineering/              # Human-facing specs (00–18)
    └── ai/                       # AI-facing specs (00–12) (THIS directory)
```

---

## 2. Key Entry Points

- **Workspace CLI Entry**: `cli/src/main.rs` (clap parses subcommands).
- **Lexer Entry**: `compiler/lexer/src/lib.rs` (public `lex(source) -> Vec<Token>`).
- **Parser Entry**: `compiler/parser/src/lib.rs` (public `parse(tokens) -> Program`).
- **Sema Entry**: `compiler/semantic/src/lib.rs` (public `analyze(ast) -> CheckedProgram`).
- **Interpreter Entry**: `runtime/interpreter/src/lib.rs` (public `interpret(checked_ast) -> Result`).

---

## 3. Build & Test Commands

Every contributor and AI must run these commands to verify edits:

### 3.1 Build all targets
```bash
cargo build --workspace
```

### 3.2 Execute test suite
```bash
cargo test --workspace
```

### 3.3 Verify formatting
```bash
cargo fmt --all -- --check
```

### 3.4 Execute linter checks
```bash
cargo clippy --all-targets -- -D warnings
```
All checks must pass with zero issues before merging.
