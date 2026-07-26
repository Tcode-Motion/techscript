# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.0.0] — 2026-07-26 — **Syntax Freeze**

> _No breaking syntax changes will be made after this release in the 2.x series._

### Language Freeze Decisions

Five language design questions were locked permanently on 2026-07-26:

| Decision | Resolution |
|---|---|
| `compat/` handling | `examples/compat/` kept permanently; each file gets `# LEGACY COMPAT TEST` header |
| Stdlib call style | Qualified (`math.abs()`, `json.parse()`); implicit only for true built-ins (`say`, `ask`, `env`, `file`) |
| Null literal | `null` is canonical; `none` is deprecated alias → TSW1011 |
| Loop semantics | `loop N` = counted (runs exactly N times); `repeat cond` = while (runs while condition is true) |
| String interpolation | `$"..."` is canonical; `f"..."` is deprecated alias → TSW1012 |

### Added

- **`Loop` token variant** — new canonical keyword `loop` for counted loops (`loop N`)
- **`Parallel` token variant** — new canonical keyword `parallel` for parallel blocks
- **`Default` token variant** — new canonical keyword `default` for match default arms
- **Full TSW error code namespace** — TSW1001–TSW1014, TSW2001–TSW2002, TSI3001 all defined
- **`docs/engineering/LANGUAGE_FREEZE_2.0.md`** — permanent freeze declaration document
- **Expanded semantic analysis** — 10 canonical rules now formally documented in `10_semantic_analysis.md`
- **Full grammar expansion** — `03_grammar_ebnf.md` updated with lambda, bitwise ops, DSL, parallel, `$"..."`, `loop`, and complete operator precedence

### Changed (Compiler)

- **`token_kind.rs`** — inverted canonical/deprecated classification:
  - **Canonical 2.0** (no warning): `do`, `send`, `when`, `loop`, `repeat`, `for`, `in`, `match`, `case`, `default`, `try`, `catch`, `throw`, `use`, `class`, `struct`, `enum`, `trait`, `interface`, `const`, `null`, `say`, `ask`, `break`, `continue`, `else`, `async`, `await`, `parallel`, `end`, `export`, `new`, `self`, `true`, `false`, `typeof`, `with`
  - **Deprecated Alias** (emit TSW): `build`, `make`, `return`, `model`, `if`, `elif`, `while`, `import`, `from`, `let`, `var`, `fun`, `function`, `attempt`, `none`, `keep`, `give`, `stop`, `skip`, `each`, `switch`, `be`, `equals`, `then`
- **`errors/src/lib.rs`** — E0104 description updated to "Expected `end` to close block"
- **`to_canonical()`** — inverted: now maps `Build→Do`, `Return→Send`, `If→When`, `While→Repeat`, `Model→Class`, `Import/From→Use`, etc.
- **`is_canonical_keyword()`** — updated to reflect 2.0 canonical set
- **`is_alias_keyword()`** — updated to reflect 2.0 deprecated set
- **`is_future_reserved_keyword()`** — narrowed to only `Type`, `Yield`, `Spawn`, `Pub`, `Mut`
- **`FStringStart.static_lexeme()`** — changed from `f"` to `$"` (canonical)

### Changed (Docs)

- `docs/engineering/01_language_spec_v1.md` — full rewrite; all canonical keywords correct
- `docs/engineering/03_grammar_ebnf.md` — expanded with all missing grammar rules
- `docs/engineering/10_semantic_analysis.md` — expanded with 10 canonical semantic rules
- `docs/engineering/14_error_codes.md` — full TSW namespace documented
- `docs/StyleGuide.md` — full rewrite with `tsc fmt` specification
- `docs/MigrationGuide.md` — rewritten as 1.0.8 → 2.0 guide
- `docs/LanguageGuide.md` — updated with `$"..."`, `loop`, `parallel`, `null` examples
- `README.md` — hero example updated to canonical 2.0 syntax

### Changed (Examples)

- `examples/compat/` files — added `# LEGACY COMPAT TEST` header to each file

### Deprecated (Keyword Reclassification)

The following keywords, previously documented as canonical in some docs,
are now officially deprecated and will emit warnings:

`build`, `make`, `let`, `var`, `return`, `give`, `if`, `elif`, `while`,
`import`, `from`, `each`, `attempt`, `model`, `none`, `f"..."`, `keep`,
`stop`, `skip`, `switch`, `then`, `be`, `equals`

**These keywords continue to compile** in the 2.x series. They are never removed —
only deprecated. Run `tsc migrate .` to auto-convert.

---

## [1.0.8] — 2026-06-01

### Added
- Created 17 Cargo workspace member crates, covering the entire compiler, runtime, and tools stack.
- Unified common types (`Span`, `NodeId`, `Ident`) inside the `techscript_common` crate.
- Unified syntax constants, keyword lists, tokens, and operator precedence levels inside the `techscript_syntax` crate.
- Scaffolding skeleton for all remaining modules: AST, Lexer, Parser, Errors, Semantic Analyzer, Interpreter, VM, GC, Builtins, Stdlib, CLI, LSP, Formatter, Linter, and Package Manager.
- Established default placeholder unit tests and example entry points.
- Configured CI/CD workflows and GitHub templates.
