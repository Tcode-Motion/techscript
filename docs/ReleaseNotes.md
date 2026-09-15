# TechScript Release Notes

Release history for the TechScript compiler, runtime, standard library, and developer tools.

---

## TechScript 2.0.0.1

**Release date:** 2026-09-15
**Release tag:** `v2.0.0.1`
**Compatibility line:** `v2.0.x`

This is a maintenance revision for TechScript 2.0. It keeps the 2.0 language and bytecode contracts stable while improving tooling, security checks, performance, and regression coverage.

### New in This Revision

- Added `tsc lint --fix` for safe, automated lint corrections.
- Expanded tests for AST construction, JSON parsing, runtime behavior, web error paths, and standard-library registration.
- Added registration and behavior coverage for database, CSV, JWT, PDF, FTP, math, Redis, Excel, Word, barcode, graphics, and notification modules.

### Fixes and Security

- Added missing capability enforcement for web servers, notifications, canvas saves, and process-backed operations.
- Fixed command-injection risks in the doctor command and Windows notification handling.
- Corrected loopback IP validation and broken documentation links.

### Performance and Maintenance

- Reduced avoidable allocations in formatter, VM debugger, bytecode disassembler, LSP, IR, SQLite, package-manager, and standard-library hot paths.
- Optimized IR predecessor mapping, diagnostic quick-fix lookup, LSP character lookup, SQLite query preparation, and capability validation.
- Refactored large compiler, standard-library, DSL, resolver, packager, and CLI functions without changing public behavior.

### Compatibility

- Existing TechScript 2.0 source and bytecode contracts remain unchanged.
- Cargo packages remain at SemVer `2.0.0`; `2.0.0.1` is the release revision tag used for packaging and publication.

### Downloads

Download the platform packages from the [v2.0.0.1 asset release](https://github.com/Tcode-Motion/techscript/releases/tag/v2.0.0.1-assets).

---

## TechScript 2.0.0

- Syntax freeze for the 2.x language line.
- Added canonical `loop`, `parallel`, and `default` keywords.
- Added formatter and linter tooling.
- Standardized `$"..."` string interpolation.

---

## TechScript 1.0.8

- Reorganized the codebase into a modular Cargo workspace.
- Introduced TechScript Studio and shell integration for `.txs` scripts.

---

## TechScript 1.0.2

- Rewrote the compiler and VM in Rust, removing the Python wrapper dependency.
- Improved loop execution performance.
