# 02 — MEMORY

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Long-term persistent project decisions and rules
> **Parent Link**: [CONTEXT](./01_context.md)
> **Child Links**: [RULES](./03_rules.md) · [DECISIONS](./07_decisions.md)

---

## 1. Frozen Decisions (Never Change)

1. **Language Name**: `TechScript` (capitalized exactly as shown).
2. **Official File Extension**: `.txs` (strictly lowercase). Never change to `.tech`, `.tsc`, or other extensions. Reject all other extensions.
3. **Official CLI Subcommands**:
   - `tech run <file.txs>`
   - `tech repl`
   - `tech check <file.txs>`
   - `tech fmt <file.txs>`
   - `tech lint <file.txs>`
   - `tech test [dir]`
   - `tech version`
4. **Implementation Language**: Rust for all compiler, interpreter, and toolchain crates.

---

## 2. Frozen Syntax & Keywords

### 2.1 Keyword List
`make`, `const`, `say`, `ask`, `build`, `return`, `fun` (deprecated), `model`, `self`, `new`, `when`, `else`, `each`, `in`, `repeat`, `while`, `break`, `continue`, `attempt`, `catch`, `throw`, `import`, `from`, `export`, `true`, `false`, `none`, `and`, `or`, `not`, `is`.

### 2.2 Unification Rule
- standalone functions use `build`.
- class/model methods use `build`.
- `fun` is supported only inside models as a deprecated alias. Using `fun` compiles successfully but triggers a deprecation warning (`W0015`).

### 2.3 Statement Termination
- Statements are terminated by newlines or explicit semicolons `;`. Semicolons are optional and primarily used for multi-statement lines.

---

## 3. Naming Conventions

### 3.1 Rust Source Code
- **Crates**: `snake_case` prefixed with `techscript_` (e.g. `techscript_lexer`).
- **Files/Modules**: `snake_case` (e.g. `symbol_table.rs`).
- **Structs/Enums**: `PascalCase` (e.g. `CheckedProgram`).
- **Variables/Functions**: `snake_case` (e.g. `resolve_names()`).
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g. `MAX_STACK_DEPTH`).

### 3.2 TechScript Code
- **Source Files**: `snake_case.txs` (e.g. `math_helpers.txs`).
- **Variables/Functions**: `snake_case` (e.g. `make item_count = 10`).
- **Models**: `PascalCase` (e.g. `model HttpClient`).
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g. `const PI = 3.14`).

---

## 4. Engineering Principles

1. **Modularity**: Frontend passes are completely isolated from backends.
2. **Deterministic Outputs**: Compiling or interpreting the same code must always produce identical side effects and structures.
3. **Rust Type Safety**: Minimize runtime panics in Rust code. Prefer compiler errors or structured `Result` error maps. Zero `unsafe` code allowed unless strictly required for FFI.
