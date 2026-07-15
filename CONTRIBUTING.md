# Contributing to TechScript 2.0

Thank you for your interest in contributing to TechScript 2.0! We welcome all contributions, including bug fixes, feature enhancements, documentation updates, and optimizations.

## Workspace Architecture

TechScript 2.0 is organized as a Cargo monorepo workspace:
- `compiler/`: lexer, parser, semantic analyzer, AST, and error reporter.
- `runtime/`: tree-walking interpreter, vm, garbage collector, and builtin libraries.
- `tools/`: LSP server, formatter, linter, and package manager.
- `cli/`: Command-line executable.

## Development Workflow

1. Fork and clone the repository.
2. Ensure you have the Rust toolchain installed (latest stable version).
3. Create a descriptive feature branch: `git checkout -b feature/your-feature-name`.
4. Make your edits following our coding standards.
5. Run the validation checks:
   - Run compilation check: `cargo check --workspace`
   - Run linter checks: `cargo clippy --workspace --all-targets -- -D warnings`
   - Run workspace tests: `cargo test --workspace`
   - Run documentation checks: `cargo doc --workspace --no-deps`
6. Commit your changes following conventional commits: `feat: add ...` or `fix: resolve ...`.
7. Push to your branch and open a Pull Request.

## Coding Standards

- Write clean, idiomatic Rust.
- Document all public modules, structs, enums, and functions.
- Avoid introducing circular dependencies.
- Ensure that the Pratt parser precedence tables are kept up to date when changing syntax rules.
