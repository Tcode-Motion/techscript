# Contributing to TechScript

Thank you for your interest in TechScript!

TechScript is natively written in Rust. The project is structured as a standard Cargo Workspace.

## Project Structure

* `crates/techscript-core/`: The core library. Contains Lexer, Parser, AST, Compiler, VM, Builtins, and standard library (`stdlib/`).
* `crates/techscript-cli/`: The `tech` binary. Includes runner, REPL, formatter, linter, package manager (`tech install`), and testing framework (`tech test`).
* `crates/techscript-lsp/`: Language Server Protocol implementation for IDEs (like VSCode).
* `vscode-extension/`: The official Visual Studio Code syntax highlighting and snippet extension.

## Building from Source

Prerequisites:
- [Rust toolchain](https://rustup.rs/) (stable)

```bash
git clone https://github.com/techscript/techscript.git
cd techscript

# Build the workspace
cargo build --release

# Run the test suite
cargo test --workspace
```

## Creating Pull Requests

1. Fork the repository
2. Create a new feature branch (`git checkout -b feature/cool-idea`)
3. Make your changes in the respective `.rs` files.
4. Add tests to `crates/techscript-core/tests/` (we use snapshot testing).
5. Run `cargo fmt` and `cargo clippy`.
6. Submit your PR!

All PRs must pass the GitHub Actions CI pipeline which runs formatting, linting, and full tests across Linux, Windows, and macOS.
