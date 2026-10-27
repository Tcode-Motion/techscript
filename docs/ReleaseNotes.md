# TechScript Release Notes

Release history for the TechScript compiler, runtime, standard library, and developer tools.

---

## TechScript 2.0.1 (Stable Release & Language Spec Lock)

**Release date:** 2026-10-27
**Release tag:** `v2.0.1`
**Compatibility line:** `v2.0.x`

This milestone release locks the canonical TechScript language syntax specification across all compiler crates, implements full conformance verification across all 100 official examples, delivers the pure-TechScript AppForge mobile engine with liquid glass UI on Android, and packages the verified production release APK.

### Key Highlights

- **Authoritative Canonical Syntax Locking**: Fully unified grammar across lexer, parser, semantic analyzer, interpreter, and VM:
  - Variable declaration with `is` (`count is 0`) and reassignment with `becomes` (`count becomes count + 1`).
  - Constant declarations using `constant name is value`.
  - Functions declared using `to name(params):` with colon and 4-space indentation.
  - Return statements using `return` and output using `say`.
  - Native double-quoted string interpolation (`"Value: {val}"`) with escaped braces (`\{`, `\}`) and automatic literal fallback for raw JSON.
  - Loops with `repeat N times:`, `for item in collection:`, and `while condition:`.
- **Compiler & Lexer Fixes**:
  - Implemented top-level and DSL block dedent token consumption in parser, resolving expression parsing conflicts.
  - Enhanced lexer string scanner to distinguish raw JSON curly braces from expression interpolation blocks.
- **100% Example Verification**:
  - All 100 official TechScript examples (`examples/01_display/` through `examples/10_cli_tool/` + WebForge + AppForge) verified and passing (`100/100 PASSED`) via `tsc run`.
- **AppForge Native Android Release APK**:
  - Compiled and verified `dist/TechScript-AppForge-v2.0.0-release.apk` (30,529,303 bytes).
  - Production v2 signature verified using `apksigner` with SHA-256 certificate thumbprint: `02:27:B2:0B:40:23:AC:AF:32:66:DB:D9:89:CE:D1:31:B8:9B:AA:E4:06:4F:A7:5E:43:19:9F:61:90:EA:E0:57`.
  - Fully tested on physical Android device (`10ME8DF1CT0001Z`, Android 14, arm64-v8a).
- **CI/CD & Repository Security**:
  - Multi-platform GitHub Actions workflows updated for Ubuntu, Windows, and macOS.
  - Protected private keystores (`*.jks`, `*.keystore`) and Android `local.properties` in `.gitignore`.
  - Canonical website and repository URLs verified (`https://techscript.is-a.dev/techscript/`).

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
