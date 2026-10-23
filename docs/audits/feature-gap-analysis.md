# TechScript 2.0 Feature Gap Analysis & Prioritized Defect Inventory

**Document Path**: `docs/audits/feature-gap-analysis.md`  
**Classification**: Audit & Engineering Specification  
**Status**: Active  

---

## 1. Prioritization Framework

Issues and features are grouped by severity and operational impact:
- **P0**: Data loss, security vulnerabilities, corrupted builds, or fundamental compiler failures breaking standard language constructs.
- **P1**: Core compiler bugs, CLI routing gaps, build pipeline blockers, and missing project generation commands.
- **P2**: Framework features, cross-platform adapters, native host integrations, and packaging pipelines.
- **P3**: Advanced optimizations, profiler refinements, documentation polish, and optional toolchain integrations.

---

## 2. Inventory of Issues and Gaps

### Category A: Compiler Defects & Gaps

#### Issue COMP-01: Premature Block Termination on Inner `when` Statements
- **Severity**: **P0** (Fundamental compiler parse failure)
- **Evidence**:
  ```text
  error[E0100]: Expected expression, found: 'end'
  --> examples\calculator\calculator.txs:20:1
  ```
- **Impact**: Any function (`do`), loop (`for`), or branch (`else`) containing a standard `when condition:` statement fails compilation, stranding the outer block's `end` token. 5 out of 20 official examples fail.
- **Affected Files**:
  - `compiler/parser/src/statements.rs` (lines 300–305)
- **Proposed Fix**:
  Only treat `when` as a top-level unclosed block terminator if:
  1. The parser is at top-level (not inside a nested function, loop, or block), AND
  2. The `when` is followed by an event declaration pattern (e.g., `when <id> is clicked:`), rather than a condition expression.
- **Tests**:
  - `examples/calculator/calculator.txs`
  - `examples/enums/enums.txs`
  - `examples/error_handling/errors.txs`
  - `examples/guess_number/guess.txs`
  - `examples/todo_cli/todo.txs`
- **Acceptance Criteria**: All 5 example files parse and execute with exit code 0.

---

#### Issue COMP-02: Variable Identifier `data` Hijacked as DSL Block Keyword
- **Severity**: **P0** (Compiler parsing defect)
- **Evidence**:
  ```text
  error[E0100]: Expected expression, found: '='
  --> examples\json_parser\parser.txs:7:6
  data = json.decode(raw_json)
       ^
  ```
- **Impact**: Developers cannot name variables `data`, access properties on `data`, or assign index elements (`data["key"] = val`). 2 official examples fail.
- **Affected Files**:
  - `compiler/parser/src/parser.rs` (lines 35–100)
  - `compiler/parser/src/statements.rs` (lines 240–245)
- **Proposed Fix**:
  1. In `parse_statement`, when checking if an identifier starts a DSL block, inspect lookahead tokens: if followed by `=`, `+=`, `-=`, `*=`, `/=`, `[`, or `.`, it is an expression/assignment, NOT a DSL block.
  2. Restrict `data` as a DSL keyword so it only triggers when followed by a database declaration name and colon (`data Notes:`).
- **Tests**:
  - `examples/json_parser/parser.txs`
  - `examples/web_api/web_api.txs`
- **Acceptance Criteria**: Both examples compile and produce expected JSON decode/encode output.

---

#### Issue COMP-03: Lack of Application Entry Point & Appforge DSL Support
- **Severity**: **P1** (Core architecture blocker)
- **Evidence**: The compiler currently parses `website`, `page`, `database`, `scene3d`, but has no AST or lowered representation for native mobile/desktop application components (`app`, `screen`, `widget`, `layout`, `navigation`, `state`).
- **Impact**: Cannot write mobile or desktop applications in pure TechScript.
- **Affected Files**:
  - `compiler/ast/src/dsl.rs`
  - `compiler/parser/src/statements.rs`
  - `compiler/parser/src/parser.rs`
- **Proposed Fix**: Extend DSL keyword registration to include Appforge primitives (`app`, `screen`, `widget`, `stack`, `row`, `column`, `card`, `input`, `button`, `text`, `image`, `navbar`, `tabbar`).
- **Tests**: Appforge component parsing unit tests.
- **Acceptance Criteria**: Appforge UI trees parse into typed AST nodes and serialize cleanly.

---

### Category B: CLI Architecture & Commands

#### Issue CLI-01: Missing `tsc app` Subcommand Hierarchy
- **Severity**: **P1** (Advertised interface missing)
- **Evidence**: `tsc app create`, `tsc app build`, `tsc app run`, `tsc app test`, `tsc app doctor`, `tsc app wrap` do not exist in `cli/src/lib.rs`.
- **Impact**: Developers cannot scaffold, build, or deploy native applications using `tsc`.
- **Affected Files**:
  - `cli/src/lib.rs`
  - `cli/src/entrypoint.rs`
  - `cli/src/commands/`
- **Proposed Fix**:
  Implement the `tsc app` subcommand tree with typed clap structs:
  - `tsc app create <name> [--template <tpl>]`
  - `tsc app build <platform> [--debug|--release|--aab]`
  - `tsc app run <platform> [--device <id>]`
  - `tsc app test`
  - `tsc app doctor`
  - `tsc app clean`
  - `tsc app wrap <web>`
- **Tests**: CLI parsing tests and snapshot tests.
- **Acceptance Criteria**: All subcommands are recognized and dispatch to appropriate handlers with proper help strings and exit codes.

---

#### Issue CLI-02: Missing Diagnostics, Logging, and Reporting Commands
- **Severity**: **P2** (Operational tooling missing)
- **Evidence**: `tsc logs`, `tsc report`, `tsc native add` are missing.
- **Impact**: Difficult to troubleshoot failed builds, inspect Gradle/NDK outputs, or report issues.
- **Affected Files**:
  - `cli/src/lib.rs`
  - `cli/src/commands/`
  - `cli/src/logging.rs`
- **Proposed Fix**: Implement `tsc logs [--follow|--level]`, `tsc report`, `tsc native add <lang>`, and platform toolchain health checks.
- **Acceptance Criteria**: Real diagnostic logs are recorded and accessible via CLI.

---

### Category C: Appforge & Android Pipeline

#### Issue APP-01: Missing Appforge Project Scaffolding
- **Severity**: **P1** (Core feature missing)
- **Evidence**: Running `tsc app create my-app` fails with unknown subcommand.
- **Impact**: No standard project structure for cross-platform TechScript applications.
- **Affected Files**:
  - `cli/src/templates.rs`
  - New module `cli/src/commands/app/create.rs`
- **Proposed Fix**: Generate source-centered project with `appforge.toml`, `techscript.toml`, `src/main.txs`, `src/app.txs`, `assets/`, `android/` host scaffolding, Gradle Wrapper, and tests.
- **Acceptance Criteria**: `tsc app create test-app` generates a clean, valid project.

---

#### Issue APP-02: Missing Android Build Pipeline (Gradle + SDK + AAPT2 + D8/R8)
- **Severity**: **P1** (Android milestone blocker)
- **Evidence**: No mechanism exists to compile TechScript to an Android executable host, invoke Gradle, package resources, and generate APK/AAB.
- **Impact**: Cannot produce Android binaries.
- **Affected Files**:
  - New module `cli/src/commands/app/build.rs`
  - New module `cli/src/commands/app/android.rs`
- **Proposed Fix**:
  Implement an automated Android build engine that:
  1. Validates `ANDROID_HOME`, JDK, and Gradle.
  2. Compiles TechScript code and assets into the Android project's asset/bytecode bundle.
  3. Invokes Gradle (`assembleDebug`, `assembleRelease`, `bundleRelease`).
  4. Locates, inspects, and verifies output APK and AAB files.
- **Tests**: Build debug APK, release APK, and AAB.
- **Acceptance Criteria**: Output `.apk` and `.aab` artifacts exist, are non-empty, pass ZIP/manifest verification, and can be installed via ADB.

---

#### Issue APP-03: Missing ADB Device Deployment & Runtime Verification
- **Severity**: **P2** (Device execution missing)
- **Evidence**: No `tsc app run android` or `tsc app install android` implementation.
- **Impact**: Users cannot test apps on physical devices or emulators.
- **Affected Files**:
  - New module `cli/src/commands/app/run.rs`
- **Proposed Fix**: Use `adb` to detect connected devices, install APK, launch main activity via `am start`, and tail logs via `logcat`.
- **Acceptance Criteria**: Successfully installs and starts the app on the connected device (`10ME8DF1CT0001Z`).

---

### Category D: Webforge Integration & Web Wrapping

#### Issue WEB-01: Missing `tsc app wrap web` Command
- **Severity**: **P2** (Framework interop missing)
- **Evidence**: No command exists to wrap a Webforge website as a native application.
- **Impact**: Webforge applications cannot easily be packaged for app stores.
- **Affected Files**:
  - New module `cli/src/commands/app/wrap.rs`
- **Proposed Fix**:
  Implement `tsc app wrap web [--platform android|windows|ios]`:
  1. Runs `tsc build --target web` to generate `dist/`.
  2. Embeds `dist/` into native host WebView wrapper (Android WebView with local asset loader and secure JS bridge).
  3. Builds native APK/AAB or desktop executable.
- **Acceptance Criteria**: Generates an installable native Android wrapper APK around Webforge web assets.

---

## 3. Summary Priority Matrix

| Issue ID | Area | Summary | Priority | Status |
| :--- | :--- | :--- | :--- | :--- |
| **COMP-01** | Compiler | Fix premature block exit on `when` in `parse_block_or_then_end` | **P0** | Planned |
| **COMP-02** | Compiler | Fix `data` keyword clash with variable assignment | **P0** | Planned |
| **CLI-01** | CLI | Implement `tsc app` and `tsc web` command groups | **P1** | Planned |
| **APP-01** | Appforge | Implement `tsc app create` project scaffolder | **P1** | Planned |
| **APP-02** | Appforge | Implement Android build pipeline (Debug APK, Release APK, AAB) | **P1** | Planned |
| **APP-03** | Appforge | Implement `tsc app run android` with ADB deployment | **P2** | Planned |
| **WEB-01** | Appforge | Implement `tsc app wrap web` hybrid native wrapper | **P2** | Planned |
| **CLI-02** | CLI | Implement `tsc logs`, `tsc report`, `tsc native add` | **P2** | Planned |
| **COMP-03** | Compiler | Add Appforge declarative UI AST & parsing support | **P2** | Planned |
