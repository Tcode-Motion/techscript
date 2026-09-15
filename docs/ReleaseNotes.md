# TechScript Release Notes

Detailed release logs and features added across major TechScript versions.

---

## 🚀 Version 2.0.0.1 (2026-09-15)

**Release tag:** `v2.0.0.1`  
**Base compatibility line:** `v2.0.x`

### What's New
* Added a complete release report section for this version, including release scope, tags, and rollout-facing notes.
* Improved release documentation consistency across changelog, versioning, and installation references.

### What’s Fixed
* Formatter internals now avoid temporary string allocations in high-frequency formatting paths.
* Expression and numeric output formatting now write directly to buffers, reducing overhead in formatter execution.

### Full Report
* **Area:** Tooling performance and release documentation hardening.
* **Impact:** Faster formatter behavior under repeated formatting operations and clearer release/tag guidance for maintainers.
* **Upgrade note:** Existing 2.0 scripts remain compatible; this release focuses on fixes and release-quality documentation updates.

---

## 🚀 Version 2.0.0 (2026-07-26)
* **Syntax Freeze**: Established canonical keywords (`do`, `send`, `when`, `loop`, `repeat`, `for`, `in`, `match`, `try`, `catch`, `throw`, `use`, `class`, `struct`, `enum`, `trait`, `interface`, `const`, `null`, `say`, `ask`, `break`, `continue`, `else`, `async`, `await`, `parallel`, `end`, `new`, `self`, `true`, `false`, `typeof`, `with`).
* **Formatters**: Added built-in format linter checks and `tech fmt` tools.
* **String Interpolation**: Canonicalized `$"..."` string syntax.

---

## 🚀 Version 1.0.8 (2026-06-01)
* **Ecosystem**: Reorganized the codebase into a Cargo workspace containing 17 modular Rust crates.
* **IDE**: Launched the TechScript Studio IDE.
* **Shell Integration**: Double-clicking `.txs` scripts launches a persistent console runner.

---

## 🚀 Version 1.0.2 (2026-03-10)
* **Rust Rewrite**: Rewrote the entire language compiler and VM in Rust, eliminating the Python wrapper dependency.
* **Performance**: Achieved loop execution speeds under 3 seconds per million operations.
