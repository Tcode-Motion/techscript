# 03 — RULES

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Invariant rules for code, compilation, testing, and contribution
> **Parent Link**: [MEMORY](./02_memory.md)
> **Child Links**: [ARCHITECTURE](./04_architecture.md) · [LANGUAGE](./05_language.md)

---

## 1. Compiler Invariants

1. **Decoupled stages**: The parser must not perform name resolution or semantic checks. The lexer must not parse. Keep interfaces clean:
   ```
   techscript_lexer  → outputs Vec<Token>
   techscript_parser → outputs Program (AST)
   techscript_sema   → outputs CheckedProgram
   ```
2. **Visitor Pattern**: All AST traversals (pretty-printing, linting, semantic analysis, interpreting) must implement the shared `Visitor` trait defined in the `techscript_ast` crate.
3. **No Unsafe Code**: The use of `unsafe` blocks in Rust code is prohibited. Unsafe code is only permitted within `techscript_gc` (v2.1) and `techscript_llvm` (v3.0) after safety invariants are documented and reviewed.

---

## 2. Formatting & Coding Standards

1. **Rust fmt**: Every Rust source file must pass `cargo fmt` checks.
2. **Clippy warnings**: All targets must compile with zero clippy warnings.
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```
3. **Rust Doc comments**: All public traits, functions, structs, and enum variants must be documented using `///` comments.

---

## 3. Git & Pull Requests

1. **Conventional commits**: Every commit message must match the conventional format:
   - Structure: `<type>(<scope>): <description>`
   - Scopes: `lexer`, `parser`, `ast`, `sema`, `interp`, `runtime`, `stdlib`, `cli`, `lsp`.
   - Examples: `feat(parser): parse unified build methods`, `fix(lexer): correct f-string brace escapes`.
2. **Branch naming**: Feature branches must use: `<type>/<issue-number>-<short-description>`.
3. **Squash merges**: All PRs are squashed when merging to `main`. `main` remains compile-stable.

---

## 4. Compatibility & Migration Rules

1. **Renaming**: Do not parse `.tech` files directly. Prompt the user to rename them to `.txs`.
2. **Warning Emitter**: Maintain the deprecated `fun` keyword mapping. Emit warning `W0015` in the semantic pass instead of erroring out in the parser.
3. **Parity**: The Rust execution engine must pass all legacy Version 1 test assertions.

---

## 5. Non-Functional Budgets

### 5.1 Performance Goals
- **Parser Speed**: Lex and parse 10,000 lines in < 100ms.
- **Binary Size**: Compiled `tech` CLI executable must remain under 30 MB.
- **Memory Footprint**: Executing simple scripts should use < 20 MB of RAM.

### 5.2 Security Goals
- **Memory safety**: Avoid out-of-bounds array reads and stack overflows.
- **Recursion protection**: Call stack depth is capped at 1000 frames by default.
