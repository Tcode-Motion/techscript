# Changelog

All notable changes to the **TechScript 2.0** VS Code extension will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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
