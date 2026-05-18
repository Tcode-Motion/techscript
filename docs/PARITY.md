# TechScript Python ↔ Rust Parity Matrix

TechScript v1.0.6 uses the **Rust bytecode VM** as the primary runtime. The Python interpreter in `src/techscript/` remains as a reference during migration.

## Status Legend

| Status | Meaning |
|--------|---------|
| ✅ | Feature works identically on both runtimes |
| ⚠️ | Partial parity — behavior differs |
| ❌ | Not implemented on Rust |
| 🦀 | Rust-only (new in v1.0.6) |

## Core Language

| Feature | Python | Rust | Notes |
|---------|--------|------|-------|
| Lexer / Parser | ✅ | ✅ | Rust is a direct port |
| Variables (`make`, bare `=`) | ✅ | ✅ | Aliases: `const` = `keep` |
| Functions (`build`) | ✅ | ✅ | Alias: `do` |
| Classes (`model`) | ✅ | ✅ | Alias: `class` |
| Control flow (`when`, `each`, `repeat`) | ✅ | ✅ | Alias: `loop` |
| Error handling (`attempt`) | ✅ | ✅ | Alias: `try`/`catch`/`throw` |
| F-strings | ✅ | ✅ | |
| Closures / upvalues | ✅ | ✅ | |
| Match statement | ✅ | ✅ | |

## Modules

| Module | Python | Rust | Notes |
|--------|--------|------|-------|
| `use web` / WebPage | ✅ | 🦀 | Ported to Rust with `tiny_http` |
| Web components/routes | ❌ | 🦀 | New framework layer |
| `use gui` | ❌ | 🦀 | eframe/egui MVP |
| `use 3d` | ❌ | 🦀 | 2D preview MVP (egui); full 3D engine planned |
| `use anime` | ❌ | 🦀 | Timeline MVP |
| math/fs/os/json/crypto/date | partial | ✅ | Rust has richer stdlib modules |

## CLI

| Command | Python | Rust |
|---------|--------|------|
| `tech run` | ✅ | ✅ |
| `tech check` | ✅ | ✅ |
| `tech repl` | ✅ | ✅ |
| `tech build` | ❌ | 🦀 |
| `tech new` | ❌ | 🦀 |
| `tech doctor` | ❌ | 🦀 |
| `tech test` | ❌ | 🦀 |
| `tech debug` | ❌ | 🦀 |

## Known Gaps (v1.0.6)

1. **Transpiler** — Python-only; not ported to Rust
2. **Defer/with** — no-op stubs in Python; not in Rust parser
3. **web_complete.txs** — runs on Rust after WebPage port
4. **Performance** — NaN boxing / arena GC planned; not required for parity

## Parity test scope (v1.0.6)

Scripts compare trimmed stdout on:

- `runtime_examples/01_basics.txs` … `06_advanced.txs`
- `examples/hello.txs`, `calc.txs`, `classes.txs`, `syntax_aliases.txs`

Module examples (`web_app`, `gui_app`, etc.) are validated by `cargo test` in `runtime/` with `TECHSCRIPT_*_TEST=1` env skips.

## Version alignment

| Component | Version |
|-----------|---------|
| Rust runtime (`runtime/Cargo.toml`) | 1.0.6 |
| Python reference (`pyproject.toml`) | 1.0.6 |
| VS Code extension | 1.0.6 |

## Performance (deferred to 1.0.7)

- String interner (`runtime/src/interner.rs`) — implemented, not yet wired into compiler/VM
- NaN boxing / arena GC — not required for v1.0.6 parity

## Running Parity Checks

```powershell
# Windows
.\scripts\parity_check.ps1

# Linux/macOS
./scripts/parity_check.sh
```

The parity script runs shared examples through both runtimes (when Python is installed) and reports stdout differences.
