# TechScript: Compact Codebase Audit & Evolution Review

## 1. Architecture Overview
- **Workspace**: 25 Rust crates (`compiler/*`, `runtime/*`, `stdlib`, `cli`, `tools/*`).
- **Frontend**: Logos tokenizer (`compiler/lexer`) $\rightarrow$ Recursive-descent & Pratt parser (`compiler/parser`) $\rightarrow$ AST (`compiler/ast`) with declarative DSL block support $\rightarrow$ Semantic checker (`compiler/semantic`).
- **Execution Pipeline**:
  - **Interpreter (`runtime/interpreter`)**: Tree-walking AST evaluator for local script execution.
  - **IR & Optimizer (`compiler/ir`, `compiler/optimizer`)**: SSA-like CFG blocks + optimization passes.
  - **Bytecode & VM (`compiler/bytecode`, `runtime/vm`)**: Stack-based VM (currently blocked by broken dependency).
  - **LLVM Backend (`compiler/llvm_backend`)**: Guarded by `#[cfg(feature = "llvm")]`; emits object files for linker (`link.exe` / `gcc`).
- **Web & Stdlib**: `stdlib/src/web.rs` renders DSL blocks (`website`, `page`, `hero`, `section`, `card`, `button`) to HTML and serves via `tiny_http`.

---

## 2. Verified Feature Inventory

| Area | Status | Evidence / Notes |
|:---|:---:|:---|
| **Lexer & Parser** | **VERIFIED** | Parses canonical 2.0 (`do`, `send`, `when`, `loop`, `repeat`, `for`, `say`, `ask`, `end`), 1.0 aliases, and DSL blocks. |
| **AST Interpreter** | **VERIFIED** | Executes core logic, arithmetic, variables, loops, closures, and stdlib calls directly. |
| **Embedded Web Server** | **VERIFIED** | `stdlib/src/web.rs` translates DSL blocks into styled HTML5 and serves on localhost via `tiny_http`. |
| **CLI (`tsc` / `tech`)** | **VERIFIED** | 28 subcommands (`run`, `build`, `check`, `fmt`, `lint`, `repl`, `test`, `doctor`, `migrate`). |
| **Bytecode / VM** | **BROKEN** | Uses `bincode = "3.0.0"` in `vm`, `bytecode`, and `cli`, triggering `compile_error!("https://xkcd.com/2347/")`. |
| **LLVM / Native Build** | **PARTIAL** | Requires host LLVM toolchain + `feature = "llvm"` + MSVC `link.exe`; otherwise falls back to compile error. |
| **Standard Library** | **PARTIAL** | Core modules work (`web`, `sqlite`, `math`, `strings`), but 25+ modules (`gui`, `video`, `excel`, `word`) are hollow stubs (<1 KB). |
| **Documentation Sync** | **PARTIAL** | Docs reference a legacy `WebPage` builder API that does not exist in 2.0 Rust code (which uses DSL blocks). |
| **Android Builds** | **NOT IMPLEMENTED** | **0% implemented.** No APK/AAB builds, Gradle generator, SDK integration, or Android UI transpilation. |
| **Client-Side Web Interactivity** | **NOT IMPLEMENTED** | Web output is static HTML without DOM event bindings (`onclick`) or reactive state. |
| **3D / Three.js** | **NOT IMPLEMENTED** | No Three.js or WebGL scene graph integration exists in compiler or runtime. |
| **Beginner English Syntax** | **NOT IMPLEMENTED** | Natural sentences like `make a website called MySite` currently fail parsing. |

---

## 3. Top 10 Critical Gaps
1. **No Android Tooling**: No Gradle scaffolding, APK packaging, or Android UI translation.
2. **Broken Dependency**: `bincode = "3.0.0"` halts workspace compilation on `cargo check`.
3. **No Web Interactivity**: Output HTML lacks client JS for clicks, forms, and dynamic updates.
4. **Rigid Syntax for Beginners**: Requires verbose `do...end` / `when...end` instead of natural English instructions.
5. **Missing 3D Engine**: No WebGL / Three.js pipeline despite roadmap mentions.
6. **Outdated Documentation**: Docs describe non-existent `WebPage` class rather than real DSL blocks.
7. **Hollow Stdlib**: Numerous modules are dummy placeholders.
8. **Missing Target CLI Commands**: Lacks `build web`, `build android`, `dev`, `new website`, `new android`.
9. **Heavy LLVM Prerequisite**: Native binaries require LLVM installation rather than portable transpilation.
10. **Technical Error Messages**: Parser diagnostics use compiler jargon unsuitable for beginners.

---

## 4. Beginner-First English Syntax Design

A dual-tier approach: simple English statements lower directly to unambiguous AST nodes.

### Core Grammar Mapping:
- `make [type] called [name]` $\rightarrow$ Target project/component declaration
- `page [Name]:` / `screen [Name]:` $\rightarrow$ Declarative container block
- `show [element] [properties]` $\rightarrow$ UI element emission
- `when [target] is [event]:` $\rightarrow$ Interactive event listener
- `change [target] to [value]` $\rightarrow$ Reactive state mutation

### Realistic Examples:

#### 1. Interactive Website
```techscript
make a website called FunMath

page Home:
    show heading "Welcome to Fun Math!"
    show text "Points: 0" called PointsText
    show button "Add Point" called AddBtn

when AddBtn is clicked:
    change PointsText to "Points: 1"
    show message "Awesome!"

start website
```

#### 2. Android Mobile App
```techscript
make an app called QuickNotes

screen Home:
    show heading "Quick Notes"
    show input "Type note here" called NoteInput
    show button "Save Note" called SaveBtn

when SaveBtn is clicked:
    store NoteInput in local storage
    show message "Saved!"

build android app
```

#### 3. 3D Web Scene
```techscript
make a 3D website called Orbit

page Home:
    show heading "My 3D World"
    show 3D scene called World:
        add rotating cube:
            color: "blue"
            speed: 2
        enable mouse controls

start website
```

---

## 5. Recommended Transpilation Pipeline

- **Web Target (`techscript build web` / `dev`)**:
  - Transpiles AST to standard `index.html`, responsive `styles.css`, and a lightweight `app.js` (DOM event bindings, reactive state updates, and Three.js CDN integration for 3D).
  - Serves via `tiny_http` with live reload.
- **Android Target (`techscript build android`)**:
  - Scaffolds a minimal Gradle wrapper project (`gradlew`, `build.gradle.kts`, `AndroidManifest.xml`).
  - Transpiles `screen` blocks to Kotlin Jetpack Compose (`@Composable`).
  - Invokes `./gradlew assembleDebug` or `assembleRelease` to produce `.apk` and `.aab`.
- **Desktop/CLI**:
  - Retains the fast AST Interpreter and Bytecode VM for instant local execution.

---

## 6. Implementation Plan & Files to Modify First

| Phase | Objective | Key Files to Modify |
|:---:|:---|:---|
| **Phase 1** | **Fix Build & Foundation** | [compiler/bytecode/Cargo.toml](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/compiler/bytecode/Cargo.toml), [runtime/vm/Cargo.toml](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/runtime/vm/Cargo.toml), [cli/Cargo.toml](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/cli/Cargo.toml) (revert `bincode` to `"1.3.3"`). Update [docs/WebGuide.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/docs/WebGuide.md). |
| **Phase 2** | **Beginner Syntax & Parser** | [compiler/syntax/src/token_kind.rs](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/compiler/syntax/src/token_kind.rs), [compiler/ast/src/lib.rs](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/compiler/ast/src/lib.rs), [compiler/parser/src/statements.rs](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/compiler/parser/src/statements.rs) (support `show`, `screen`, `called`, `when ... is clicked`, colon blocks). |
| **Phase 3** | **Interactive Web Engine** | [stdlib/src/web.rs](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/stdlib/src/web.rs) (generate client JS event listeners, state updates, and Three.js template). |
| **Phase 4** | **CLI Web & Android Commands** | [cli/src/lib.rs](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/cli/src/lib.rs), [cli/src/commands/build.rs](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/cli/src/commands/build.rs) (add `build web`, `build android`, `dev`). |
| **Phase 5** | **Android Compose Scaffolding** | Create Android Gradle + Jetpack Compose template generator in `cli/src/commands/android.rs`. |
| **Phase 6** | **Release Signing & Diagnostics** | [cli/src/commands/doctor.rs](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/cli/src/commands/doctor.rs) (JDK/Android SDK checks), release APK/AAB signing. |
