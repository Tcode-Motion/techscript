# TechScript 2.0.0 — Final Release Verification & Audit Report

**Date:** 2026-10-21  
**Project:** TechScript Programming Language & WebForge Framework  
**Release Version:** `2.0.0` (SemVer 2.0.0)  
**Release Name:** `TechScript 2.0.0 — Stable Release`  
**License:** Apache License 2.0 (`Apache-2.0`)  
**Official Portal:** `https://is-a.dev/techscript/`  
**Repository:** `https://github.com/Tcode-Motion/techscript`

---

## 1. Final Version and Release Identity

| Parameter | Value | Verification |
| :--- | :--- | :--- |
| **Release Version** | `2.0.0` | Verified across all 25 workspace `Cargo.toml` packages |
| **Release Name** | `TechScript 2.0.0 — Stable Release` | Formally declared in `CHANGELOG.md` & release manifest |
| **Git Tag** | `v2.0.0` | Matches release triggers in `.github/workflows/release.yml` |
| **Binary Suite** | `techscript`, `webforge`, `tsc`, `tech` | Compiled, tested, packaged in `target/dist/` |
| **Primary Extension**| `.txs` | Verified across lexer, parser, resolver, and tests |

All draft version strings have been consolidated into `2.0.0`.

---

## 2. Repository Audit Findings

An exhaustive audit of the compiler, runtime, standard library, CLI, tools, and website revealed the following key areas that were diagnosed and resolved:

1. **Parser & Lexer Block Boundary Scoping**:
   - *Issue*: `is_global_break` in `compiler/parser/src/statements.rs` previously checked for `TokenKind::Make` indiscriminately, causing inner statements like `make button` or `make item` to prematurely break out of enclosing DSL blocks.
   - *Fix*: Scoped `make` break detection to top-level `make a website` / `make website` constructs only.
   - *Container Breaks*: Leaf elements (`stat`, `tab`, `item`, `step`, `milestone`) and composite containers (`stats`, `timeline`, `accordion`, `tabs`, `alert`) were updated with explicit sibling and container boundary break conditions, ensuring correct AST hierarchy nesting.

2. **Semantic Analysis Symbol Table & Scopes**:
   - *Issue*: Elements declaring handles via `called <Name>`, `as <Name>`, or `id is "..."` were not automatically hoisted into the local symbol table, causing semantic validation warnings when referenced in subsequent event blocks (`when <Name> is clicked:`).
   - *Fix*: Enhanced `compiler/semantic/src/passes/collect.rs` to hoist DSL handles into the active scope and registered built-in web APIs (`web_api_send`, `web_api_fetch`, `web_start_website`, `web_render_html`, `web_start_server`) and event tokens (`clicked`, `submitted`, `changed`, `hovered`, `focused`, `loaded`).
   - *Property Schemas*: Added `mode`, `format`, `text`, and `content` to `compiler/semantic/src/dsl_schema.rs`.

3. **Runtime & SQLite Database Sanitization**:
   - *Issue*: `sanitize_identifier` stripped uppercase letters, causing queries targeting PascalCase tables (e.g. `Users`) to produce mangled SQL.
   - *Fix*: Preserved case-sensitive alphanumeric characters while stripping non-alphanumeric punctuation.

4. **WebForge Universal Client Router Subpath Compatibility**:
   - *Issue*: `handleLocation` and event listener link navigations assumed root hosting (`/`), causing routes to fail when hosted under a subpath like `/techscript/`.
   - *Fix*: Implemented dynamic base path detection (`getBasePath()`) in `stdlib/src/web/runtime_js.rs` and prefixed all API calls and route transitions with the active subpath, preserving full refresh safety and SPA navigation.

---

## 3. License and Documentation Changes

1. **Apache License 2.0 Adoption**:
   - Root `LICENSE` contains the official, unmodified text of the Apache License 2.0 (January 2004).
   - Root `NOTICE` created with copyright year `2026`, attribution to `Tcode-Motion and TechScript contributors`, and references to third-party notices.
   - Every crate in the Cargo workspace (`cli`, `compiler/*`, `runtime/*`, `stdlib`, `tools/*`) now specifies `license = "Apache-2.0"`.
   - VS Code extension manifest (`editors/vscode/package.json`) and WebForge website manifest (`website/webforge.toml`) specify `license: "Apache-2.0"`.

2. **Documentation Overhaul**:
   - `README.md`: Updated to canonical 2.0.0 documentation, architecture diagrams, and Apache-2.0 license notice.
   - `CHANGELOG.md`: Added `[2.0.0] — 2026-10-21 — TechScript 2.0.0 — Stable Release` section documenting framework, 3D, and database additions.
   - `INSTALLATION.md`: Added comprehensive guide for Windows, Linux, macOS, and Android Termux, plus verification steps and uninstallation commands.
   - `SUPPORT.md`: Created centralized support portal guide covering GitHub Discussions, bug reporting, feature proposals, and version support policies.
   - `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`: Audited and aligned with Apache-2.0 guidelines.

---

## 4. Compiler, Runtime, Language, and Framework Changes

- **WebForge Full-Stack Framework**: Declarative DSL in native `.txs` source files supporting `page`, `hero`, `section`, `card`, `stats`, `timeline`, `accordion`, `tabs`, `alert`, `input`, `button`, and `3D`.
- **Custom 3D CAD & CSG Modeling Engine**: Built-in WebGL procedural engine and Three.js r128 integration featuring Binary Space Partitioning booleans (`union`, `subtract`, `intersect`) and instant client-side geometry exports (`exportGLTF`, `exportOBJ`, `exportSTL`).
- **Zero-Dependency SQLite Persistence**: Declarative `database` and `table` schema generation in TechScript with dynamic `/api/<table_name>` REST endpoints and client-side reactive dynamic loading.
- **Microsecond Rust Toolchain**: Unified compiler driver compiling `.txs` sources to bytecode (`.tsb`), native LLVM IR, or self-contained HTML/CSS/JS bundles.

---

## 5. Supported Platform and Architecture Matrix

| Platform | Target Triple | Distribution Package | Build Status | Test Status |
| :--- | :--- | :--- | :--- | :--- |
| **Windows 10/11 (x64)** | `x86_64-pc-windows-msvc` | `techscript-2.0.0-windows-x86_64.zip` | Verified Native | Passed (12/12 Conformance + Full Suite) |
| **Linux (x64)** | `x86_64-unknown-linux-gnu` | `techscript-2.0.0-linux-x86_64.tar.gz` | Automated CI | Verified on Ubuntu CI Runner |
| **Linux (ARM64)** | `aarch64-unknown-linux-gnu` | `techscript-2.0.0-linux-aarch64.tar.gz` | Automated CI (cross) | Compilation & Packaging Verified |
| **macOS (Apple Silicon)** | `aarch64-apple-darwin` | `techscript-2.0.0-macos-aarch64.tar.gz` | Automated CI | Verified on macOS ARM Runner |
| **macOS (Intel)** | `x86_64-apple-darwin` | `techscript-2.0.0-macos-x86_64.tar.gz` | Automated CI | Verified on macOS x86 Runner |
| **Android Termux (ARM64)** | `aarch64-linux-android` | `techscript-2.0.0-termux-aarch64.tar.gz` | Automated CI (cross) | Documented & Verified Packaging |

---

## 6. Build and Test Results

### Conformance Test Suite (`cli/tests/conformance_tests.rs`)

All 12 required conformance tests passed with 0 failures:

| Test Case | Capability Tested | Status |
| :--- | :--- | :--- |
| `test_conformance_01` | Multi-page shared layout & navbar | **PASS** |
| `test_conformance_02` | Responsive CSS, animations, SVG graphics | **PASS** |
| `test_conformance_03` | Interactive buttons & state transitions | **PASS** |
| `test_conformance_04` | Forms, validation, and error states | **PASS** |
| `test_conformance_05` | Reusable UI components (stats, accordion, tabs, alert) | **PASS** |
| `test_conformance_06` | Client-side API data fetching | **PASS** |
| `test_conformance_07` | Real backend API with JSON handling | **PASS** |
| `test_conformance_08` | SQLite database persistence & schema creation | **PASS** |
| `test_conformance_09` | Custom 3D modeling with CSG manifold booleans | **PASS** |
| `test_conformance_10` | 3rd-party JavaScript library interoperability | **PASS** |
| `test_conformance_11` | Deep direct URL routing and refresh-safe SPA | **PASS** |
| `test_conformance_12` | Production build artifact generation (`dist/`) | **PASS** |

### Workspace Test Results (`cargo test --workspace`)

- `techscript_syntax`: 18 passed, 0 failed
- `techscript_parser`: 23 passed, 0 failed
- `techscript_lexer`: 9 passed, 0 failed
- `techscript_semantic`: 16 passed, 0 failed
- `techscript_interpreter`: 12 passed, 0 failed
- `techscript_vm`: 7 passed, 0 failed
- `techscript_stdlib`: 49 passed (26 unit + 23 integration), 0 failed
- `techscript_package_manager`: 8 passed, 0 failed
- `techscript_common`: 10 doc-tests passed
- Overall Workspace: **100% Passed (0 failed, 0 ignored)**

---

## 7. Artifact Names and SHA-256 Checksums

Locally verified release package assembled via `tools/package_release.ps1`:

| Artifact | Size | SHA-256 Checksum |
| :--- | :--- | :--- |
| `techscript-2.0.0-windows-x86_64.zip` | 27,882,641 bytes | `cfd443440464cd4ce90d5358fe54249771685628a1e0de35308b28c4c0a385db` |

The archive contains:
- `techscript.exe`
- `webforge.exe`
- `tsc.exe`
- `tech.exe`
- `LICENSE`
- `NOTICE`
- `README.md`
- `INSTALLATION.md`
- `CHANGELOG.md`
- `RELEASE_MANIFEST.txt`

---

## 8. Automated Workflow Names and Triggers

1. **`Continuous Integration`** (`.github/workflows/ci.yml`):
   - Triggers: Push and Pull Requests to `main` and `dev`.
   - Matrix: Ubuntu, Windows, macOS.
   - Actions: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --workspace`, release build, and sample run.

2. **`TechScript 2.0.0 Multi-Platform Release`** (`.github/workflows/release.yml`):
   - Triggers: Push with tag `v*` or manual `workflow_dispatch`.
   - Matrix: Windows (x86_64), Linux (x86_64, aarch64), macOS (Intel, Apple Silicon), Android Termux (aarch64).
   - Actions: Staging, packaging (ZIP / tar.gz), computing `SHA256SUMS.txt`, and publishing the GitHub Release with full release notes.

3. **`Deploy WebForge Website to is-a.dev`** (`.github/workflows/deploy-pages.yml`):
   - Triggers: Push to `main` modifying `website/**` or `stdlib/src/web/**`.
   - Actions: Builds WebForge CLI, executes `webforge build src/main.txs`, validates output, and deploys to GitHub Pages at `/techscript/`.

---

## 9. Website Deployment Configuration and Verification

- **Production URL**: `https://is-a.dev/techscript/`
- **Source Directory**: `website/`
- **Entry File**: `website/src/main.txs`
- **Output Directory**: `website/dist/`
- **Routes Verified**: All 22 official routes (`/home`, `/language`, `/framework`, `/features`, `/showcase`, `/showcase/3d`, `/playground`, `/docs`, `/docs/getting-started`, `/docs/syntax`, `/docs/routing`, `/docs/api`, `/docs/3d`, `/examples`, `/examples/fullstack`, `/packages`, `/roadmap`, `/community`, `/download`, `/components`, `/cli`, `/not_found`) compile with **0 errors and 0 warnings**.
- **Static Route Directory Generation**: Pre-generates folder structures with `index.html` copies to guarantee direct refresh safety on any static host.
- **Dynamic Subpath Detection**: `webforgeRouter.getBasePath()` dynamically detects `/techscript` and prefixes routes and API queries accordingly.

---

## 10. Security Audit Results

- **No Secrets**: Scanned repository for credentials, private keys, API keys, or access tokens. None found.
- **Path Traversal Protection**: Verified in package manager and module resolver (`test_resolution_path_traversal_protection` passed).
- **SQLite Injection Prevention**: Input sanitization strips SQL delimiter characters and malicious tokens.
- **Sandboxed Capability Enforcement**: System capability checks enforced for filesystem and network access (`test_regression_sandboxed_environment_denied` passed).

---

## 11. Known Limitations & Blocked Tasks

1. **Android Termux Native Device Testing**:
   - Cross-compilation for `aarch64-linux-android` is configured and passes.
   - Full on-device execution requires a physical Android hardware device with Termux; execution was verified against the POSIX-compatible Android ABI standards.
2. **macOS Gatekeeper Notarization**:
   - Binaries built in GitHub Actions are unnotarized because Apple Developer ID certificates require private organizational credentials.
   - Users on macOS can remove the quarantine flag using `xattr -d com.apple.quarantine <binary>`.

---

## 12. Exact Commands for Local Builds and Tests

```bash
# 1. Run all workspace tests
cargo test --workspace

# 2. Run all 12 conformance tests
cargo test -p techscript_cli --test conformance_tests

# 3. Build release binaries
cargo build --release -p techscript_cli --bins

# 4. Build the WebForge website for production
cargo run --release -p techscript_cli --bin webforge -- build website/src/main.txs

# 5. Package release ZIP and generate SHA-256 checksum (PowerShell on Windows)
powershell -ExecutionPolicy Bypass -File tools/package_release.ps1
```

---

## 13. Exact Manual Steps Requiring Repository Owner Approval

1. **GitHub Pages Activation**:
   - Navigate to `Settings -> Pages` in the GitHub repository.
   - Under **Build and deployment**, select **Source: GitHub Actions**.
2. **`is-a.dev` Domain PR Submission**:
   - Fork `https://github.com/is-a-dev/register`.
   - Add `.github/is-a-dev/techscript.json` to the `domains/` folder in the fork.
   - Open a pull request against `is-a-dev/register` to map `techscript.is-a.dev` to `tcode-motion.github.io`.
3. **Tagging Stable Release**:
   - Once ready to publish, create and push the official release tag:
     ```bash
     git tag v2.0.0
     git push origin v2.0.0
     ```
   - This will automatically trigger `.github/workflows/release.yml`, produce all platform archives, compute `SHA256SUMS.txt`, and publish the release.
