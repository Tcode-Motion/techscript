# Graph Report - .  (2026-07-15)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 249 nodes · 481 edges · 25 communities (24 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9773cb0b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 24|Community 24]]

## God Nodes (most connected - your core abstractions)
1. `Span` - 36 edges
2. `NodeId` - 35 edges
3. `Vec` - 13 edges
4. `Expression` - 13 edges
5. `Ident` - 12 edges
6. `Expression` - 11 edges
7. `Diagnostic` - 11 edges
8. `techscript_common` - 11 edges
9. `techscript_interpreter` - 10 edges
10. `techscript_syntax` - 10 edges

## Surprising Connections (you probably didn't know these)
- `techscript_cli` --crate_depends_on--> `techscript_stdlib`  [EXTRACTED]
  cli/Cargo.toml → stdlib/Cargo.toml
- `techscript_stdlib` --crate_depends_on--> `techscript_interpreter`  [EXTRACTED]
  stdlib/Cargo.toml → runtime/interpreter/Cargo.toml
- `techscript_cli` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  cli/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_formatter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/formatter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_interpreter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  runtime/interpreter/Cargo.toml → compiler/ast/Cargo.toml

## Import Cycles
- 1-file cycle: `compiler/errors/src/lib.rs -> compiler/errors/src/lib.rs`
- 1-file cycle: `compiler/parser/src/lib.rs -> compiler/parser/src/lib.rs`
- 1-file cycle: `compiler/semantic/src/lib.rs -> compiler/semantic/src/lib.rs`
- 1-file cycle: `compiler/syntax/src/lib.rs -> compiler/syntax/src/lib.rs`
- 1-file cycle: `runtime/builtins/src/lib.rs -> runtime/builtins/src/lib.rs`
- 1-file cycle: `runtime/interpreter/src/lib.rs -> runtime/interpreter/src/lib.rs`
- 1-file cycle: `stdlib/src/lib.rs -> stdlib/src/lib.rs`
- 1-file cycle: `tools/linter/tests/linter_tests.rs -> tools/linter/tests/linter_tests.rs`
- 1-file cycle: `tools/lsp/src/lib.rs -> tools/lsp/src/lib.rs`

## Communities (25 total, 1 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.12
Nodes (56): Block, Box, Option, Span, String, Vec, Expression, FieldDecl (+48 more)

### Community 1 - "Community 1"
Cohesion: 0.11
Nodes (8): NativeCallback, HashMap, Result, Self, String, RuntimeError, BuiltinRegistry, Value

### Community 2 - "Community 2"
Cohesion: 0.20
Nodes (9): Option, Self, Span, String, Vec, Diagnostic, DiagnosticLevel, DiagnosticReporter (+1 more)

### Community 3 - "Community 3"
Cohesion: 0.24
Nodes (14): Diagnostic, DiagnosticReporter, HashMap, Program, Result, Self, String, Vec (+6 more)

### Community 4 - "Community 4"
Cohesion: 0.40
Nodes (16): techscript_ast, techscript_builtins, techscript_cli, techscript_common, techscript_errors, techscript_formatter, techscript_gc, techscript_interpreter (+8 more)

### Community 5 - "Community 5"
Cohesion: 0.27
Nodes (10): Diagnostic, DiagnosticReporter, Program, Result, Self, Token, Vec, parse() (+2 more)

### Community 6 - "Community 6"
Cohesion: 0.26
Nodes (7): LibraryFunction, StdlibRegistry, StdModule, HashMap, Option, Self, String

### Community 7 - "Community 7"
Cohesion: 0.29
Nodes (10): CheckedProgram, HashMap, Result, Self, String, Environment, interpret(), Interpreter (+2 more)

### Community 8 - "Community 8"
Cohesion: 0.24
Nodes (9): Diagnostic, DiagnosticReporter, Result, Self, Token, Vec, lex(), Lexer (+1 more)

### Community 9 - "Community 9"
Cohesion: 0.24
Nodes (7): Client, InitializeParams, InitializeResult, LanguageServer, Backend, Result, Self

### Community 10 - "Community 10"
Cohesion: 0.25
Nodes (5): LintRule, DummyRule, CheckedProgram, Diagnostic, Vec

### Community 11 - "Community 11"
Cohesion: 0.36
Nodes (6): DependencyResolver, Package, Result, Self, String, Vec

### Community 12 - "Community 12"
Cohesion: 0.39
Nodes (5): Self, String, Ident, NodeId, Span

### Community 13 - "Community 13"
Cohesion: 0.43
Nodes (6): Self, Span, String, Precedence, Token, TokenKind

### Community 14 - "Community 14"
Cohesion: 0.50
Nodes (3): Commands, Cli, Commands

## Knowledge Gaps
- **40 isolated node(s):** `Commands`, `Commands`, `Statement`, `Parameter`, `FieldDecl` (+35 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What connects `Commands`, `Commands`, `Statement` to the rest of the system?**
  _40 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.12218045112781954 - nodes in this community are weakly interconnected._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.10822510822510822 - nodes in this community are weakly interconnected._