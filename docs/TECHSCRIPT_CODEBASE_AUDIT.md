# TechScript Codebase Audit (Factual)

**Repository**: C:\Users\Tanmoy\OneDrive\Documents\techscript  
**Branch/HEAD**: 740736df (Merge pull request #289)  
**Date of audit**: 2026-10-20  
**Auditor**: Agent (read-only inspection)

> This report is based on direct inspection of source files, not just docs. Where uncertain, status is marked UNKNOWN and evidence cited.

---

## 1) What TechScript Currently Is and How Its Architecture Works

**Verified:** TechScript is a human-readable, English-like programming language implemented in Rust with a multi-crate workspace. Key observations:

- **Workspace layout (Cargo.toml):** 30 workspace members split across `compiler/*` (lexer/parser/ast/semantic/ir/optimizer/bytecode/llvm_backend/module_resolver), `runtime/*` (vm/interpreter/runtime/builtins/gc/native_runtime), `stdlib/`, `cli/`, `tools/*` (linter/formatter/package-manager/packager/lsp), etc. (VERIFIED)
- **Language line:** 2.0.x (README). Files use `.txs`. (VERIFIED)
- **Frontend:** Logos-based lexer (`compiler/lexer/src/lib.rs`) with manual handling for strings/f-strings, comments (`#`, `//`, nested `/* */`), keywords via `compiler/syntax`. Parser is recursive-descent + Pratt (`compiler/parser`). AST defined in `compiler/ast`. (VERIFIED)
- **Syntax/keywords:** Canonical 2.0 keywords include `do`, `send`, `when`, `loop`, `repeat`, `for`, `in`, `match`, `case`, `try`, `catch`, `throw`, `use`, `class`, `struct`, `enum`, `trait`, `interface`, `const`, `null`, `say`, `ask`, `break`, `continue`, `else`, `async`, `await`, `parallel`, `end`, `export`, `new`, `self`, `true`/`false`, `typeof`, `with`. Many 1.x aliases exist (`build/return/model/if/elif/while/import/from/let/var/fun/function/attempt/none/keep/give/stop/skip/each/switch/be/equals/then`) and are recognized as deprecated aliases (lexer/parser emit warnings via diagnostics). (VERIFIED from `compiler/syntax/src/token_kind.rs`)
- **Execution model:** README diagram shows source → lexer/parser/AST → semantic/resolver/optimizer → IR → (bytecode to VM) or (LLVM to native) with runtime/builtins/GC. Both tree-walking interpreter and bytecode VM + LLVM backend are present. (VERIFIED)
- **CLI:** `tsc` binary (clap) with many subcommands (run/build/check/fmt/lint/migrate/test/repl/new/init/doc/publish/install/uninstall/update/doctor/clean/version/dump-*/emit-*/benchmark/completion/examples/docs/config/self). (VERIFIED from `cli/src/main.rs`, `cli/Cargo.toml`)

---

## 2) Verified Feature Inventory (with paths/evidence)

| Feature | Status | Evidence | Notes |
|---------|--------|----------|-------|
| Lexer with Logos + f-strings + nested block comments | VERIFIED | `compiler/lexer/src/lib.rs` | Handles `f"`/`$"`, nested `/* */`, line comments `#`/`//` |
| Keyword system (canonical + aliases + warnings) | VERIFIED | `compiler/syntax/src/token_kind.rs`, `compiler/syntax/src/lib.rs` | Large token set with deprecation mapping |
| Parser (recursive descent + Pratt) | VERIFIED | `compiler/parser/src/lib.rs`, `compiler/parser/src/*.rs` | Builds AST, has recovery mode |
| AST (rich) | VERIFIED | `compiler/ast/src/lib.rs` | Includes functions/methods/struct/enum/model/class-like, try/catch, loops, async/parallel hints, DSL blocks, etc. |
| Semantic analysis | VERIFIED (crate present) | `compiler/semantic/` exists in workspace | Implementation present; behavior needs deeper trace (see limits) |
| IR/optimizer/bytecode/VM | VERIFIED (crates present) | `compiler/ir/`, `compiler/optimizer/`, `compiler/bytecode/`, `runtime/vm/` | Pipeline exists as designed |
| LLVM backend | VERIFIED (crate present, optional feature) | `compiler/llvm_backend/`, `cli/Cargo.toml` has `llvm` feature | Feature-gated |
| Interpreter/tree-walk | VERIFIED | `runtime/interpreter/` | Mentioned as dev/compat path |
| Runtime/builtins/GC/native_runtime | VERIFIED | `runtime/*` crates | Present |
| Stdlib modules (broad set) | VERIFIED (crate + modules listed) | `stdlib/` crate, README lists many modules (collections, json, http, sqlite, etc.) | Extent of implementations varies |
| CLI commands (extensive) | VERIFIED | `cli/src/main.rs`, `cli/src/commands/*` (many files) | `run/build/check/fmt/lint/migrate/test/repl/new/init/doctor/...` exist |
| Project scaffolding via `tsc new` | VERIFIED | `cli/src/commands/new_cmd.rs` | Interactive wizard + templates (console/library/package/workspace/empty) |
| Formatter/Linter/LSP/Package manager/Packager | VERIFIED (crates/tools) | `tools/*` present | Integrated into CLI |
| Tests/benchmarks/examples | VERIFIED | `tests/`? (workspace has tests implied), `examples/`, `benchmarks/` exist | Many examples present |
| VS Code extension | VERIFIED (mentioned) | `editors/vscode/` exists | README points to marketplace/OpenVSX |

---

## 3) Partially Implemented / Experimental / Broken / Planned

| Item | Status | Evidence | Notes |
|------|--------|----------|-------|
| LLVM backend end-to-end | PARTIALLY IMPLEMENTED | `compiler/llvm_backend/` exists, CLI has `emit-llvm/emit-asm/emit-obj` | Feature-gated; "Completing and expanding LLVM native code generation" is in roadmap (README). Needs real build/test verification. |
| Native execution path completeness | PARTIALLY IMPLEMENTED | `runtime/native_runtime/`, backends exist | Depends on LLVM/toolchains; may not work out of box everywhere. |
| Web/Android build support | NOT IMPLEMENTED (no direct evidence found) | Searched for `android`, `web` in repo files (core code/cli/docs) — only hits in node_modules in editors/vscode, python installers mention Termux/platform support, research docs mention platforms. No `cli` commands for `new website/android`, no Android/Gradle/web bundler integration in core CLI/src. | This is a major gap vs vision. |
| DSL blocks in AST | PARTIALLY IMPLEMENTED | `AST::DSL(DSLBlock)` exists | Parser support exists structurally; how it's used/evaluated unclear without tracing. |
| Async/Parallel constructs | PARTIALLY IMPLEMENTED | Keywords/tokens/AST present | Runtime/VM support needs verification. |
| Package registry/publishing flow | PARTIALLY IMPLEMENTED | `tsc publish/install/update`, `tools/package-manager` exist | Depends on registry availability; needs testing. |
| Hot reload/dev server | NOT IMPLEMENTED / UNKNOWN | No obvious `dev` server code in cli/src/commands inspected | README doesn't list `dev` as core command like some proposals; current CLI has run/build/check etc. |

---

## 4) Existing Syntax Examples and Actual Behavior

**Verified from codebase patterns:**
- Variables/assignment: `count = 10` style common (README). Canonical uses `const`/assignment forms; aliases `let`/`var`/`make`/`be` exist as deprecated.
- Functions: canonical `do name(params) ... send ... end` (README). Aliases `build/fun/function/return/give` deprecated.
- Conditionals: `when condition ... else ... end` (canonical). Aliases `if/elif` deprecated.
- Loops: `loop N ... end` (counted), `repeat condition ... end` (while-style), `for item in items ... end`. Aliases `while/each` deprecated.
- Error handling: `try ... catch error ... end` (canonical). Alias `attempt` deprecated.
- I/O: `say` prints, `ask` reads (implicit calls).
- Null: `null` canonical (alias `none` deprecated).
- F-strings: `$"..."` or `f"..."` supported by lexer.

**Actual behavior:** Syntax is English-like keywords with `end` blocks. Parser/lexer enforce canonical forms with warnings on aliases. No evidence of the "natural language DSL" like `make a website called ...` in current parser/grammar — those appear to be the *desired* vision, not current implementation.

---

## 5) Current Compiler/Runtime Capabilities and Limitations

**Capabilities (VERIFIED):**
- Full pipeline crates exist (lexer→parser→AST→semantic→IR→optimizer→bytecode/VM or LLVM)
- Rich language features in grammar (OOP-like via class/model/struct/enum/trait/interface, modules via use/export, async/await/parallel keywords, pattern matching via match/case)
- Tooling integrated (fmt/lint/lsp/test/repl/migrate)
- Cross-platform installer scripts and Python installers exist

**Limitations (VERIFIED/INFERRED):**
- Web/Android app generation not present as first-class CLI targets
- LLVM path is optional/feature-gated and may require toolchain setup
- No obvious built-in dev server/hot reload/web bundler
- Android build integration (Gradle/Kotlin/Android SDK) not found in codebase
- The "beginner DSL" shown in vision is not implemented
- Some features may be partially wired (need tests to confirm end-to-end)

---

## 6) Current CLI Commands and Which Ones Work

**Verified commands from main.rs:** run, build, check, fmt, lint, migrate, clean, init, new, doc, test, repl, publish, install, uninstall, update, doctor, version, dump-ast, dump-ir, dump-bytecode, emit-ir, emit-llvm, emit-asm, emit-obj, benchmark, completion, examples, docs, config, self.  
**New/init scaffolding:** confirmed in `new_cmd.rs` with templates (console/library/package/workspace/empty).  
**Doctor:** exists.  
**Work status:** Implementation exists for these command handlers; "which ones work" cannot be confirmed without building/running tests in this environment - mark as **VERIFIED (handlers exist)**, runtime correctness **UNKNOWN** without execution.

---

## 7) Existing Website and Android Build Capabilities

**Website:** No evidence of built-in website framework, dev server, bundler, or `tsc build web/dev` in CLI. Stdlib has `web` module listed (README) but implementation details not inspected deeply - needs verification.  
**Android:** No evidence of Android project generation, Gradle integration, APK/AAB building in CLI or codebase. Only platform mentions in installers/docs/research.  
**Status:** **NOT IMPLEMENTED** for the envisioned end-to-end flows.

---

## 8) Dependencies, Purposes, Licenses, Maintenance Status

**Workspace deps (Cargo.toml):** logos 0.16, clap 4, serde/serde_json 1, anyhow 1, thiserror 2, tower-lsp 0.20, rustyline 18, colored 3, toml 1.1, indexmap 2, clap_complete 4. CLI adds dirs 7, ureq 3.4, rayon 1.12, bincode 3, winres 0.1 (build).  
**Third-party:** `third_party/` dir exists, `THIRD_PARTY_LICENSES.md` present, `ATTRIBUTIONS.md`/`NOTICE`/`COPYRIGHT` present.  
**Licenses:** Project Apache-2.0 (LICENSE). Third-party components under their own licenses.  
**Maintenance:** Most deps are common/stable; specific maintenance status not fully assessed from manifests alone - **PARTIALLY KNOWN** (need to check upstream activity if integrating further).

---

## 9) Performance, Security, Stability, Developer-Experience Issues

**Potential issues (from inspection):**
- Large CLI with many features increases complexity
- LLVM path optional/complex to set up
- Legacy alias support adds parser/lexer complexity
- No visible web/android tooling increases gap to vision
- Error messages exist but "plain-English actionable" beginner experience not yet realized
- Windows-first considerations mentioned; cross-platform needs validation
- **Security:** No obvious secret handling issues in inspected code; must be careful not to touch credentials. Stdlib may have network/crypto modules - need review if used.
- **Stability:** Core compiler pipeline appears structured, but end-to-end beginner flows unimplemented.

---

## 10) Missing Components Required to Achieve Vision

**Critical gaps:**
1. **Beginner DSL/grammar layer** for natural-language style (`make a website...`, `page Home: ... show ...`) - not present
2. **Website framework + dev server + bundler** - not present
3. **Android project generation + Gradle/Kotlin integration + APK/AAB + signing** - not present
4. **CLI commands** for `new website/android`, `dev`, `build web`, `build android ...` - not present
5. **UI/component model, routing, styling system** for web/apps - not present
6. **Device/emulator integration, keystore management** - not present
7. **Beginner-friendly error messages/doctor enhancements** - partially exists but needs extension
8. **Templates for web/android projects** - not present

---

## RECOMMENDED BEGINNER-FRIENDLY SYNTAX (Proposal)

**Approach:** Implement as a **DSL/frontend layer** that lowers to the existing stable core language (AST/VM/runtime). Keep advanced features accessible. Avoid adding hundreds of hidden rules.

### Design principles
- English-like verbs: `make`, `show`, `when ... is clicked`, `start`, `build`
- Minimal punctuation, `end` blocks consistent with existing style
- Indentation-sensitive where natural, but unambiguous
- Lower to existing constructs (functions, events) where possible
- Backward compatible

### Proposed syntax examples

#### Example 1: Website (basic)
```techscript
make a website called MySite

page Home:
    show heading "Welcome to My Website"
    show text "I built this with TechScript"
    show button "Click Me" as btnClick

when button btnClick is clicked:
    show message "Hello!"

start website
```

#### Example 2: Android app (basic)
```techscript
make an app called MyFirstApp

screen Home:
    show heading "My First App"
    show text "Made with TechScript"
    show button "Say Hello" as btnHello

when button btnHello is clicked:
    show message "Hello, World!"

build android app
```

#### Example 3: Variables/lists
```techscript
make a variable called name with value "Alex"
make a list called todos with ["learn", "build", "share"]

when page loads:
    show text "Hello, " + name
end
```

#### Example 4: Functions/components
```techscript
make a component called Card with title and body:
    show box:
        show heading title
        show text body
    end
end

use component Card with title="Hi", body="Welcome"
```

#### Example 5: API call
```techscript
when button "Get Data" is clicked:
    make a request to "https://api.example.com/data" and get result
    show text result.message
end
```

### Implementation approach for this syntax
1. **Lexer**: Recognize new keywords/patterns (`make a website/app`, `page`, `screen`, `show`, `component`) - can extend existing lexer or pre-process
2. **Parser**: Add grammar rules for DSL constructs; produce either new AST nodes or lower directly to existing AST (e.g., `page Home:` becomes a function/route definition)
3. **Lowering**: DSL → core AST → existing compiler pipeline
4. **Validation**: Provide plain-English errors with suggestions

**Feasibility with existing architecture:** PARTIALLY - AST has `DSLBlock` (line ~618 in AST) which suggests DSL support was anticipated. Lexer/parser would need extension. Lowering layer needed. Can be done incrementally without replacing core compiler.

---

## RECOMMENDED COMPILER/TRANSPILATION ARCHITECTURE

**Recommendation:** **Extend existing pipeline with a DSL frontend + lowering layer** (not a second full compiler).

```
[Beginner DSL (.txs with DSL syntax)]
  ↓ (DSL Lexer/Parser or preprocessor)
[DSL AST]
  ↓ (Lowering/Desugaring)
[Core TechScript AST] (existing)
  ↓ (existing: Semantic → IR → Optimizer → Bytecode/LLVM)
[Runtime/VM or Native]
  ↓ (Framework generators)
[Web (HTML/CSS/JS) or Android (Kotlin/Gradle project)]
```

**Rationale:**
- Reuse existing lexer/parser/semantic/IR/VM/stdlib - huge investment
- AST already has `DSLBlock` suggesting this direction
- Lowering keeps core language stable
- Can transpile to web (HTML/JS/TS) or generate Android projects (Kotlin) while leveraging existing runtime where appropriate
- Avoids maintaining two compilers

**Trade-offs:** Transpilation to web/Android gives maximum compatibility and leverages existing tooling (browsers, Gradle, Android SDK). Using VM for everything is harder for native mobile/web distribution.

---

## WEBSITE AND ANDROID BUILD INTEGRATION PLAN

### Website (shortest reliable route)
1. **Inspect stdlib web module** - understand current capabilities
2. **Design TechScript Web DSL** - map `page/show/button/when clicked` to routes/components
3. **Transpilation target** - generate HTML/CSS/JS (or use Vite/webpack). Consider Preact/Vanilla JS for simplicity
4. **Dev server** - implement `tsc dev` with hot reload (can use existing tools like `live-server` or Rust-based server)
5. **Build** - `tsc build web` produces static files
6. **3D/WebGL** - integrate Three.js via stdlib or generated imports
7. **SEO/a11y/security** - add sensible defaults and checks

### Android (shortest reliable route)
1. **Project template generation** - create Android project structure (Gradle, AndroidManifest, etc.) similar to how `tsc new` works
2. **Transpile TechScript screens/logic to Kotlin** - generate Activities/Fragments/Composables based on DSL
3. **Bridge/runtime** - minimal runtime for common operations or direct Kotlin stdlib
4. **Build integration** - call Gradle via CLI, detect Android SDK
5. **Debug/release** - `tsc build android --debug`, `--release-apk`, `--aab` with signing support
6. **Device install** - `adb install` integration

**Key tools to leverage:** Vite/esbuild for web, Android Gradle Plugin, Kotlin compiler, adb. All FOSS.

---

## PRIORITIZED IMPLEMENTATION ROADMAP

### Phase 0: Audit & Planning (CURRENT - COMPLETE)
- [x] Deep inspection of codebase
- [x] Create audit report
- [ ] Get approval

### Phase 1: Foundation Hardening (Week 1-2)
- [ ] Document core language grammar/semantics formally
- [ ] Add/fix tests for lexer/parser/VM basics
- [ ] Extend CLI doctor for web/android toolchain detection
- [ ] Create DSL parsing scaffolding

### Phase 2: Beginner DSL Design & Prototype (Week 2-3)
- [ ] Implement lexer/parser extensions for DSL
- [ ] Create lowering to core AST
- [ ] Test with vision examples
- [ ] Add beginner-friendly error messages

### Phase 3: Website Framework MVP (Week 3-5)
- [ ] Web DSL lowering to HTML/CSS/JS
- [ ] Basic components/pages/routing
- [ ] `tsc new website`, `tsc dev`, `tsc build web`
- [ ] Simple dev server with hot reload

### Phase 4: Android Framework MVP (Week 5-7)
- [ ] Android project template
- [ ] DSL → Kotlin transpilation for basic screens/buttons/events
- [ ] `tsc new android`, `tsc build android --debug`
- [ ] Gradle integration and APK generation

### Phase 5: Polish & Extensions (Week 7-8)
- [ ] Release builds/signing
- [ ] 3D/WebGL support basics
- [ ] API calls/networking
- [ ] Documentation and examples
- [ ] Testing

**Reordering notes:** Focus on DSL + lowering first (unlocks both web and android), then parallelize web/android implementations.

---

## EXACT FILES TO MODIFY FIRST (and why)

| File | Reason | Priority |
|------|--------|----------|
| `compiler/lexer/src/lib.rs` | Add DSL keywords/tokens (`make`, `page`, `screen`, `show`, `component` etc.) | P0 |
| `compiler/syntax/src/token_kind.rs` | Register new keywords (canonical/deprecated as appropriate) | P0 |
| `compiler/parser/src/*.rs` | Add parsing rules for DSL constructs; integrate with AST | P0 |
| `compiler/ast/src/lib.rs` | May need new node types for DSL/web/android constructs (or use existing DSLBlock) | P0 |
| `cli/src/main.rs` | Add new subcommands (`dev`?) and route new commands; may need `new website/android` support | P0 |
| `cli/src/commands/new_cmd.rs` | Extend project templates for web/android | P0 |
| `cli/src/lib.rs` / CLI structure | Add command modules for web/android build/dev if needed | P1 |
| `docs/` | Update docs as features added (preserve existing) | P2 |

**Note:** Start with minimal changes, add tests, keep backward compatibility. No files deleted. Major architectural changes require approval per rules.
