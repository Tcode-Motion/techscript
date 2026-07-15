# TechScript 2.0

TechScript 2.0 is a modular, general-purpose programming language optimized for productivity, safety, and performance. This repository houses the unified compiler monorepo, runtime, and developers tooling suite.

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
- `stdlib`: Modular library paths (`io`, `math`, `string`, `file`, `web`).
- `cli`: Single binary command line target (`tech`).
- `tools/`
  - `lsp`: IDE completion and code analyzer.
  - `formatter`: Code style formatter.
  - `linter`: Static rule checker.
  - `package-manager`: Registry client and dependency resolver.

## Getting Started

Ensure you have Rust installed. You can compile and test the workspace using:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps
```
