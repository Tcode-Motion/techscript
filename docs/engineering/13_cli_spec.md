# 13 — TechScript 2.0 CLI Specification

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [04 Compiler Architecture](./04_compiler_architecture.md) · [12 Stdlib](./12_stdlib_design.md) · [14 Error Codes](./14_error_codes.md)

---

## 1. Binary Name

The TechScript 2.0 command-line interface is distributed as a single compiled executable named `tech`.

---

## 2. Command Catalogue

| Command | Action | Compatibility |
|---|---|---|
| `tech run <file.txs>` | Executes a TechScript source file | **Authoritative** (accepts `.txs`) |
| `tech repl` | Starts the interactive REPL | **Authoritative** |
| `tech check <file.txs>` | Runs semantic checks without execution | **Authoritative** |
| `tech fmt <file.txs>` | Formats `.txs` source files | **Authoritative** |
| `tech lint <file.txs>` | Lints source files, reporting warnings | **Authoritative** (supports `--fix`) |
| `tech test [dir]` | Discovers and runs tests matching `*_test.txs` | **Authoritative** |
| `tech version` | Prints binary and compiler version info | **Authoritative** |
| `tech help [cmd]` | Prints CLI help details | **Authoritative** |
| `tech build <file.txs>` | Compiles source file to VM bytecode | **Future (v2.1)** |
| `tech new <name>` | Scaffolds a new project structure | **Authoritative** |

---

## 3. Options & Arguments

### 3.1 `tech run`
```bash
tech run <file.txs> [--verbose] [--debug-ast] [--timeout <secs>] [--max-stack <depth>]
```

### 3.2 `tech lint --fix`
```bash
tech lint <file.txs|dir/> --fix
```
Automatically rewrites deprecated occurrences (like replacing `fun` with `build` inside models).

### 3.3 `tech test`
Discovers and executes files named `*_test.txs` containing test functions starting with `test_`.

---

## 4. Compatibility & Evolution Analysis

### 4.1 Compatibility Notes
- **Frozen extension**: The CLI refuses to execute files with non-`.txs` extensions (e.g., `tech run file.tech` returns exit code 4 - Invalid arguments).
- **Public API Continuity**: Subcommands and flag interfaces are identical to Version 1 to prevent breaking CI/CD runner pipelines.

### 4.2 Migration Notes
- To migrate a project:
  1. Rename all files from `.tech` to `.txs`.
  2. Run:
     ```bash
     tech lint src/ --fix
     ```
     This automatically resolves keyword deprecations.
  3. Run the test suite:
     ```bash
     tech test tests/
     ```

### 4.3 Rationale
- **Single Executable**: Bundling parser, interpreter, formatter, and linter into a single `tech` binary simplifies installation and eliminates Python dependency issues.
- **Auto-Fix capability**: Including a `--fix` option in the linter provides a seamless migration path for legacy code.

### 4.4 Future Roadmap
- **v2.1**: The `tech build` command will compile `.txs` code to `.txb` bytecode files.
- **v3.0**: The compiler will introduce `tech compile` using LLVM to output optimized native binaries directly.
