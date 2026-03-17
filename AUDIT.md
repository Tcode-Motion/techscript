# TechScript Repository Audit (2026-03-17)

This document is a **ground-truth audit** of the current TechScript repository implementation vs the promised behavior in docs/examples. It is intended to drive the stabilization/refactor work.

## Architecture snapshot

- **Workspace**: Rust workspace with `techscript-core`, `techscript-cli`, `techscript-lsp`.
  - `Cargo.toml`
  - `crates/techscript-core/Cargo.toml`
  - `crates/techscript-cli/Cargo.toml`
  - `crates/techscript-lsp/Cargo.toml`
- **Pipeline**: `Lexer` → `Parser` → `Compiler` (bytecode) → `VM`.
  - `crates/techscript-core/src/lexer.rs`
  - `crates/techscript-core/src/parser.rs`
  - `crates/techscript-core/src/compiler.rs`
  - `crates/techscript-core/src/vm.rs`

## Feature × status matrix

Legend: **OK** (implemented and used), **PARTIAL** (implemented but incomplete/buggy), **DRIFT** (docs/examples disagree), **MISSING** (promised but absent).

### Language core (syntax / semantics)

- **Lexing**: **OK**
  - Supports numbers (incl `0x`/`0b`/`0o`), strings (single/double/triple), raw strings, f-strings token (`TokenType::FString`).
  - `crates/techscript-core/src/lexer.rs`
- **Parsing**: **OK** (broad coverage), **DRIFT** (some keywords)
  - Statements: `say/make/keep/use/take/share/when/or when/else/each/repeat/until/build/model/attempt/rescue|catch/always/match/case/guard/with/defer/drop/stop/skip/pass/send/fail`
  - `crates/techscript-core/src/parser.rs`
- **Formatting**: **PARTIAL / BUG**
  - Formatter prints `when ... } alt ... {` but parser/docs use `} or when ... {`.
  - `crates/techscript-core/src/formatter.rs` vs `crates/techscript-core/src/parser.rs`
- **Compilation**: **OK** for implemented AST surface, **NOTE**
  - Compiler does **not** emit `OpCode::Invoke`; it emits `GetProperty` + `Call` for method-style calls.
  - `crates/techscript-core/src/compiler.rs`, `crates/techscript-core/src/opcode.rs`
- **VM execution**: **PARTIAL**
  - `OpCode::Invoke` exists in opcode set but is unimplemented in VM (`/* TODO */`); currently **likely unused** by the compiler.
  - String method binding appears incorrect (`make_native_str` ignores receiver).
  - `crates/techscript-core/src/vm.rs`
- **Error reporting**: **OK baseline**, can improve
  - Central error types + formatting used by CLI.
  - `crates/techscript-core/src/error.rs`
- **Async/task**: **PARTIAL**
  - `spawn` queues tasks, `await` is a no-op; event loop runs tasks by calling them.
  - `crates/techscript-core/src/vm.rs`

### Builtins (globals)

- **I/O**: **OK** (`say/print/write`, plus `log/warn/error/debug/clear`)
  - `crates/techscript-core/src/builtins.rs`
- **Core**: **OK** (`assert/sleep/time/time_ms/exit/callable`)
  - `crates/techscript-core/src/builtins.rs`
- **String & list helpers**: **OK as globals**, **DRIFT as methods**
  - `split/join/replace/replace_all/contains/...` exist as **global functions**.
  - Docs claim these as **string methods** (e.g. `"hello".split()`), but VM only exposes a small set of string properties/methods and method-binding is buggy.
  - `crates/techscript-core/src/builtins.rs`, `crates/techscript-core/src/vm.rs`, `docs/REFERENCE.md`
- **List methods**: **PARTIAL**
  - VM list property supports `append/remove/reverse` (and mentions `sort` but does not implement it).
  - Docs claim `map/filter/sort/reverse`.
  - `crates/techscript-core/src/vm.rs`, `docs/REFERENCE.md`

### Standard library modules (`use ...`)

Builtin module resolution list includes: `math/fs/os/random/json/crypto/date/api/web/gui/three_d/anime/debug` (plus `net` in stdlib).
  - `crates/techscript-core/src/module_resolver.rs`

- **math**: **OK**
  - `crates/techscript-core/src/stdlib/math.rs`
- **fs**: **OK**
  - `crates/techscript-core/src/stdlib/fs.rs`
- **os**: **OK** (but includes shell execution)
  - `crates/techscript-core/src/stdlib/os.rs`
- **random**: **OK** (LCG-ish, not cryptographic)
  - `crates/techscript-core/src/stdlib/random.rs`
- **json**: **OK** (custom JSON codec; limited types)
  - `crates/techscript-core/src/stdlib/json.rs`
- **crypto**: **PARTIAL / SECURITY NOTE**
  - `sha256` and base64 are implemented; `md5` is a non-MD5 FNV-like hash despite the name.
  - `crates/techscript-core/src/stdlib/crypto.rs`
- **date**: **OK**
  - `crates/techscript-core/src/stdlib/date.rs`
- **debug**: **OK**
  - `crates/techscript-core/src/stdlib/debug.rs`
- **net**: **PARTIAL / NON-PORTABLE**
  - Uses `curl` via shell-out for GET/POST.
  - `crates/techscript-core/src/stdlib/net.rs`
- **web/gui/three_d/anime**: **OK baseline (browser-based hybrid)**, needs hardening
  - Generates HTML/JS and serves via a blocking localhost TCP server, auto-opens browser/app mode.
  - Implemented under `crates/techscript-core/src/stdlib/web.rs` (`web`, `gui`, `scene`, `anime` namespaces).
- **api**: **MISSING**
  - Docs + examples reference `use api` + `api.listen(3000)` but there is no stdlib `api` module registration and VM import handler does not include `api`.
  - `docs/REFERENCE.md`, `examples/api_server.txs`, `crates/techscript-core/src/vm.rs`, `crates/techscript-core/src/stdlib/mod.rs`

### CLI tooling

- **Commands**: **OK baseline**
  - `run/build/check/fmt/lint/repl/test/init/doc/install/doctor`
  - `crates/techscript-cli/src/main.rs`
- **Native build**: **PARTIAL**
  - `tech build --native` generates a temporary Cargo project and compiles a runner embedding bytecode.
  - `crates/techscript-cli/src/main.rs`
- **Test runner**: **PARTIAL**
  - Executes `.txs` files but does not verify outputs beyond “no runtime error”.
  - `crates/techscript-cli/src/main.rs`
- **Package install**: **OK baseline**
  - Clones git repos into `.techscript-modules/` and updates `tech.toml`.
  - `crates/techscript-cli/src/pkg.rs`

### LSP

- **Diagnostics**: **OK baseline**
  - Lex/parse/compile to produce diagnostics.
  - `crates/techscript-lsp/src/main.rs`
- **Completions**: **DRIFT**
  - Includes keywords like `alt` even though parser uses `or when` form; also includes builtins that don’t exist as list methods.
  - `crates/techscript-lsp/src/main.rs`

## High-impact mismatches (action list)

1. **Implement `api` module** to make `examples/api_server.txs` and `docs/REFERENCE.md` real.
2. **Fix formatter** to emit syntax the parser accepts (`or when`, and `take ... in` vs `take ... from` drift if needed).
3. **Fix receiver-based methods** (strings/lists) or adjust docs to match “globals-only” API. For compatibility, methods are preferred.
4. **Remove `curl` dependency** in `net` (replace with Rust HTTP client implementation) to support Windows and “zero external deps”.
5. **Add headless / no-auto-open modes** for `web/gui/three_d/anime` to enable automated testing.

