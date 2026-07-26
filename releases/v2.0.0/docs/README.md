# TechScript 2.0

> **Syntax Frozen 2026-07-26** — No breaking changes in the 2.x series.

TechScript 2.0 is a modular, general-purpose programming language that reads
like English and compiles like Rust. Optimized for productivity, safety, and
readability — with indentation-based blocks, no semicolons, and a clean
keyword vocabulary.

## Hello, TechScript 2.0

```txs
use math

do greet(name = "World")
    say $"Hello, {name}!"
end

do factorial(n)
    when n <= 1
        send 1
    end
    send n * factorial(n - 1)
end

greet "TechScript"

loop 5
    say "Counting down..."
end

result = factorial(10)
say $"10! = {result}"
say $"sqrt(result) = {math.sqrt(result)}"
```

```
Hello, TechScript!
Counting down...  (×5)
10! = 3628800
sqrt(result) = 1904.94...
```

## Canonical Syntax Quick Reference

| Concept | TechScript 2.0 |
|---|---|
| Function | `do name(params)` … `end` |
| Return | `send value` |
| Conditional | `when cond` … `else when cond` … `else` … `end` |
| Counted loop | `loop N` … `end` |
| While loop | `repeat cond` … `end` |
| For-each | `for x in list` … `end` |
| Match | `match expr` / `case val` / `default` … `end` |
| Class | `class Name` … `end` |
| Import | `use module` |
| Constant | `const MAX = 100` |
| Null | `null` |
| Interpolation | `$"Hello {name}!"` |
| Print | `say "hello"` |
| Input | `name = ask "Your name?"` |
| Comment | `# comment` |

## Repository Layout

The project is structured as a Cargo workspace:

- `compiler/`
  - `common`: Primitives, Spans, Node IDs, and common types.
  - `syntax`: Unified token definition registry, operators, and Pratt parsing precedence.
  - `ast`: AST node representations and visitors.
  - `errors`: Unified diagnostic error and warning codes and rendering.
  - `lexer`: DFA lexical scanner.
  - `parser`: Recursive descent statement and Pratt expression parser.
  - `semantic`: Scopes analysis and name resolution.
- `runtime/`
  - `interpreter`: Tree-walking AST evaluator.
  - `vm`: Bytecode VM compiler and VM registers.
  - `gc`: Generation mark-and-sweep tracking collector.
  - `builtins`: Pre-registered native operations (`say`, `ask`, `len`).
- `stdlib`: Modular library paths (`math`, `json`, `http`, `crypto`, `web`, `file`).
- `cli`: Single binary command line target (`tsc`).
- `tools/`
  - `lsp`: IDE completion and code analyzer.
  - `formatter`: Code style formatter (`tsc fmt`).
  - `linter`: Static rule checker (`tsc lint`).
  - `package-manager`: Registry client and dependency resolver.

## Getting Started

Ensure you have Rust installed. You can compile and test the workspace using:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps
```

## Migrating from 1.x

If you have TechScript 1.x source files, run:

```bash
tsc migrate .
```

This automatically converts deprecated syntax (`build`, `model`, `if`, `while`,
`make`, `return`, etc.) to canonical 2.0 form. Deprecated syntax still compiles
in 2.x — it just emits `TSW100x` warnings.

See [docs/MigrationGuide.md](docs/MigrationGuide.md) for the full mapping table.

## Documentation

| Document | Purpose |
|---|---|
| [LanguageGuide.md](docs/LanguageGuide.md) | Core language tutorial |
| [StyleGuide.md](docs/StyleGuide.md) | `tsc fmt` canonical style |
| [MigrationGuide.md](docs/MigrationGuide.md) | 1.0.8 → 2.0 migration |
| [StdlibReference.md](docs/StdlibReference.md) | Standard library API |
| [engineering/LANGUAGE_FREEZE_2.0.md](docs/engineering/LANGUAGE_FREEZE_2.0.md) | Permanent freeze declaration |
| [engineering/01_language_spec_v1.md](docs/engineering/01_language_spec_v1.md) | Full language specification |
| [engineering/03_grammar_ebnf.md](docs/engineering/03_grammar_ebnf.md) | Complete EBNF grammar |
| [engineering/14_error_codes.md](docs/engineering/14_error_codes.md) | All error/warning codes |
