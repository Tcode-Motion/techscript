# 16 — TechScript 2.0 Coding Standards

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [02 Folder Structure](./02_folder_structure.md) · [15 Testing](./15_testing.md)

---

## 1. Rust Style Guide

All Rust code must be formatted with `rustfmt` using project-wide defaults. Clean clippy execution (`cargo clippy -- -D warnings`) is required. `unsafe` is prohibited except where explicitly required (e.g. LLVM bindings).

---

## 2. Naming & Case Conventions

### 2.1 Rust code
Crates: `snake_case` (e.g. `techscript_lexer`).
Structs/Enums: `PascalCase`.
Functions/Methods: `snake_case`.

### 2.2 TechScript language code
Variables/Functions: `snake_case`.
Constants: `SCREAMING_SNAKE_CASE`.
Models: `PascalCase`.
File extensions: **Strictly `.txs`**.

---

## 3. Commit Message & PR Guidelines

Uses Conventional Commit messages (`feat(scope): desc`, `fix(scope): desc`).
PRs require unit or snapshot test coverage and must pass formatting, linting, and Fuzz checks.

---

## 4. Compatibility & Evolution Analysis

### 4.1 Compatibility Notes
- **Linter Enforcements**: The linter code (`techscript_lint`) enforces `.txs` casing rules.
- **Unified Keyword Formatting**: The formatter (`techscript_fmt`) enforces canonical method formatting by rewriting method declarations from `fun` to `build`.

### 4.2 Migration Notes
- Any commit hook or lint rule that scanned for `.tech` extensions must be updated to scan for `.txs` files.
- Command-line formatting validation:
  ```bash
  tech fmt src/ --check
  ```
  Reports formatting discrepancies in `.txs` files.

### 4.3 Rationale
- **Uniform Coding standards**: Enforcing strict `rustfmt` and `clippy` checks across the monorepo ensures that code remains clean and maintainable.
- **Enforced File renaming**: Rejecting other extensions prevents configuration drift and simplifies IDE integration.

### 4.4 Future Roadmap
- **v2.2**: Integrate static type checking lint rules in `techscript_lint` to flag missing annotations.
- **v3.0**: Add LLVM optimization checks to pull request review guidelines.
