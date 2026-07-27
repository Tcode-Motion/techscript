# Contributing to TechScript

Thank you for deciding to contribute to TechScript! As an open-source project aiming to build a human-first programming language, we value your help in expanding the language features, stabilizing the VM, updating documentation, and building developer tools.

---

## 🛠️ Setting Up Your Development Environment

TechScript is written entirely in **Rust** as a Cargo workspace. To get started, make sure you have the following installed:

1. **Rust Toolchain**: Install via rustup:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Git**: For version control.
3. **Clang/LLVM** (Optional, required for LLVM-backend contributions):
   * **Ubuntu/Debian**: `sudo apt install llvm-dev libclang-dev`
   * **macOS**: Installed via Xcode CLI tools or homebrew (`brew install llvm`).
   * **Windows**: Download the LLVM installer or use `winget install LLVM.LLVM`.

### Fork & Clone
1. Fork the TechScript repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/techscript.git
   cd techscript
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/Tcode-Motion/techscript.git
   ```

---

## 📂 Repository Structure Overview

The codebase is organized like Rust and Zig to keep compilation, execution, and tools clean:

* `compiler/` - Lexer, parser, semantic analyzer, AST, IR, optimizer, and LLVM backend.
* `runtime/` - Virtual Machine (VM), Tree-walking interpreter, garbage collector, and native runtime.
* `stdlib/` - Standard library written in TechScript and Rust.
* `cli/` - The unified `tech` command-line executable.
* `tools/` - Formatter, linter, LSP, and package manager.
* `examples/` - Working demonstration scripts.
* `tests/` - System integration and language regression tests.
* `benchmarks/` - Scripts and suites for testing engine performance.

---

## 🔄 Development & Git Workflow

We use a standard branching model. Always make changes on a descriptive branch, not on the `main` branch.

### 1. Create a Branch
```bash
git checkout -b feature/my-cool-feature
# or
git checkout -b fix/issue-123
```

### 2. Make Edits & Run Linting
We enforce strict styling and quality rules. Run these commands locally before committing:
```bash
# Verify formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets -- -D warnings

# Build all workspace targets
cargo build --workspace
```

### 3. Run Tests
Verify both Rust internal tests and TechScript integration tests pass:
```bash
cargo test --workspace
```

### 4. Committing Changes
We follow [Conventional Commits](https://www.conventionalcommits.org/):
* `feat: ...` for new features or syntax.
* `fix: ...` for compiler/runtime bug fixes.
* `docs: ...` for documentation changes.
* `style: ...` for formatting updates.
* `refactor: ...` for code restructurings.
* `test: ...` for adding/updating tests.

Example:
```bash
git commit -m "feat: add support for pattern matching default keyword"
```

---

## 📝 Pull Request Checklist

Before submitting a Pull Request:
- [ ] Ensure all code compiles without warning.
- [ ] Run `cargo fmt` and `cargo clippy`.
- [ ] Add corresponding documentation in `docs/` if modifying syntax, CLI flags, or standard libraries.
- [ ] Add regression tests inside `tests/` or examples in `examples/`.
- [ ] Link the issue you are fixing in the PR description (e.g., `Fixes #123`).

Once submitted, our CI workflow will run automated checks on Windows, Linux, and macOS. A maintainer will review your code and request adjustments if necessary.
