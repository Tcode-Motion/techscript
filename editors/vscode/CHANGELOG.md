# Changelog

All notable changes to the **TechScript 2.0** VS Code extension will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [2.0.2] — Virtian — 2026-09-15

### Added
- Canonical TechScript 2.0 snippets for functions, control flow, classes, traits, interfaces, modules, and error handling.

### Fixed
- Rewrote the extension README for the Virtian release and removed obsolete brace-style examples.
- Ensured attributes and hash comments receive the correct syntax highlighting.

---

## [2.0.1] — Virtian — 2026-09-15

### Added
- Configurable format-on-save and lint-on-save actions for TechScript files.
- A configurable compiler path for installations where `tsc` is not on the default PATH.
- Canonical 2.0 syntax highlighting for `do`/`end` blocks, `when`, `loop`, `repeat`, `try`, `send`, `class`, `interface`, and interpolated strings.

### Fixed
- Bundled the Language Server Protocol client required to start `techscript-lsp`.
- Updated indentation rules to use canonical `end`-delimited blocks.

---

## [2.0.0] — 2026-07-27

### 🎉 Initial Marketplace Release

#### Added
- **Syntax Highlighting** — Full TextMate grammar for `.txs`, `.tsx`, `.tech`, and `.tspkg` files
- **IntelliSense & Autocomplete** — LSP-powered context-aware completions (variables, functions, models, stdlib)
- **Real-time Diagnostics** — Inline errors and lint warnings powered by `techscript-lsp`
- **File Icon Theme** — Custom TechScript icons for `.txs` and `.tsx` files in the Explorer
- **Debugger Integration** — DAP-compatible debug adapter for breakpoints and stepping
- **Code Snippets** — 14 productivity snippets: `build`, `model`, `when`, `each`, `repeat`, `attempt`, `say`, `ask`, `enum`, `trait`, `test`, `main`, `package`, and more
- **Toolchain Commands** (via Command Palette):
  - `TechScript: Run File`
  - `TechScript: Build Project`
  - `TechScript: Check Code`
  - `TechScript: Test Project`
  - `TechScript: Format File`
  - `TechScript: Lint File`
  - `TechScript: Open REPL`
  - `TechScript: Generate Docs`
  - `TechScript: Package Project`
  - `TechScript: Show Compiler Version`
  - `TechScript: Show AST / IR / Bytecode`
  - `TechScript: Restart Language Server`
- **Sidebar Panel** — Activity Bar panel with Project Explorer, Package Manager, Examples, Templates, and Documentation views
- **Run Button** — Editor title bar ▶ button for `.txs` files
- **Task Provider** — VS Code task integration for `build`, `check`, `run`, `test`
- **Language Configuration** — Smart bracket matching, comment toggling, and indentation rules
