# TechScript Release Notes

Detailed release logs and features added across major TechScript versions.

---

## Version 2.0.0.1 (2026-09-15)

**Release tag:** `v2.0.0.1`  
**Base compatibility line:** `v2.0.x`

### What's New
* Added `tsc lint --fix` for applying safe, automated lint corrections.
* Expanded tests across AST construction, JSON parsing, runtime behavior, web error paths, and stdlib module registration.
* Added coverage for database, CSV, JWT, PDF, FTP, math, Redis, Excel, Word, barcode, graphics, and notification modules.

### What’s Fixed
* Removed avoidable temporary allocations from formatter, VM debugger, bytecode disassembler, LSP, IR, SQLite, package-manager, and stdlib hot paths.
* Added missing capability enforcement for web servers, notifications, canvas saves, and process-backed operations.
* Fixed command-injection risks in the doctor command and Windows notification handling.
* Corrected loopback IP validation and broken documentation links.

### Full Report
* **Area:** Compiler and tooling performance, security hardening, test coverage, and maintainability.
* **Impact:** Faster repeated tooling operations, safer privileged stdlib behavior, and better regression detection.
* **Compatibility:** Existing TechScript 2.0 source and bytecode contracts remain unchanged.
* **Package version:** Cargo packages remain `2.0.0`; `2.0.0.1` is the release revision tag used for packaging and publication.

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
