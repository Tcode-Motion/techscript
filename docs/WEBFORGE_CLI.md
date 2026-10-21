# TechScript WebForge: CLI Reference Manual
**Complete Command-Line Interface and Developer Toolchain**

---

## 1. Overview

The `techscript` command-line interface provides everything needed to develop, test, bundle, and deploy full-stack applications.

### Binary Invocation Aliases
All commands can be invoked using any of the three official binary aliases:
```bash
techscript <command>
tsc <command>
tech <command>
```

---

## 2. Command Reference

### Project Scaffolding
- **`techscript new <template> <name>`**
  Scaffolds a new project from a built-in template.
  Templates: `website`, `fullstack`, `api`, `3d`.
  Example: `techscript new fullstack my-app`
- **`techscript init`**
  Initializes a TechScript project in the current working directory.

### Development & Execution
- **`techscript dev [--port <port>]`**
  Starts the local development server with hot reload, embedded SQLite, and live WebGL rendering (default: port 3000).
- **`techscript run <file>`**
  Executes a TechScript source file or entry point directly.
- **`techscript routes`**
  Inspects and lists all registered web routes, REST endpoints, and client action triggers.

### Build & Compilation
- **`techscript build [file] [--release] [--target <web|server>]`**
  Compiles project sources.
  - `--target web`: Generates static web bundles in `dist/` (`index.html`, `styles.css`, `app.js`, `schema.sql`).
  - `--target server`: Generates native server runtime artifacts.
  - Default: Generates compiled TechScript bytecode (`.tsb`).

### Package Management
- **`techscript install [package]`**
  Installs a package or resolves all dependencies declared in `techscript.toml`.
  Alias: `techscript add <package>`
- **`techscript uninstall <package>`**
  Removes a package dependency and updates `techscript.toml` and `techscript.lock`.
  Alias: `techscript remove <package>`
- **`techscript list`**
  Lists all declared dependencies, their resolved versions, and package locations.
  Alias: `techscript ls`
- **`techscript search <query>`**
  Searches the official package catalog for matching libraries.
- **`techscript update`**
  Updates installed packages to the latest matching version constraints.

### Code Quality & Testing
- **`techscript check [file]`**
  Runs semantic and type analysis without generating compilation output.
- **`techscript test [dir]`**
  Discovers and runs test suites matching `*_test.txs`.
- **`techscript fmt [path]`**
  Formats TechScript source files (`.txs`, `.wtxs`, `.stxs`, `.3txs`).
- **`techscript lint [path] [--fix]`**
  Lints codebase for style, deprecations, and potential bugs.

### Utilities & Maintenance
- **`techscript doctor`**
  Evaluates development environment health, PATH settings, and compiler prerequisites.
- **`techscript clean [--all]`**
  Cleans build artifacts (`build/`, `dist/`) and intermediate compilation caches.
- **`techscript deploy [--target <static|docker>]`**
  Prepares release packages for target hosting providers.

---

## 3. Global Flags

| Flag | Description |
| :--- | :--- |
| `--verbose`, `-v` | Enables verbose diagnostic and timing logs. |
| `--quiet`, `-q` | Suppresses non-error terminal output. |
| `--json` | Emits structured JSON diagnostics for IDE and CI/CD tools. |
| `--help`, `-h` | Displays contextual help for any command. |
| `--version` | Displays toolchain version and build metadata. |
