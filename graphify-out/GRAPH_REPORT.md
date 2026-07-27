# Graph Report - .  (2026-07-27)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 131 nodes · 297 edges · 13 communities (10 shown, 3 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `89f884a7`
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

## God Nodes (most connected - your core abstractions)
1. `techscript_cli` - 19 edges
2. `techscript_common` - 18 edges
3. `techscript_syntax` - 16 edges
4. `techscript_ast` - 15 edges
5. `StdlibRegistry` - 13 edges
6. `Parser<'a>` - 12 edges
7. `DiagnosticReporter` - 12 edges
8. `ParseResult` - 12 edges
9. `techscript_errors` - 12 edges
10. `StdlibModule` - 10 edges

## Surprising Connections (you probably didn't know these)
- `techscript_cli` --crate_depends_on--> `techscript_native_runtime`  [EXTRACTED]
  cli/Cargo.toml → runtime/native_runtime/Cargo.toml
- `techscript_cli` --crate_depends_on--> `techscript_package_manager`  [EXTRACTED]
  cli/Cargo.toml → tools/package-manager/Cargo.toml
- `techscript_cli` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  cli/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_formatter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/formatter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_interpreter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  runtime/interpreter/Cargo.toml → compiler/ast/Cargo.toml

## Import Cycles
- 1-file cycle: `compiler/bytecode/src/lib.rs -> compiler/bytecode/src/lib.rs`
- 1-file cycle: `runtime/interpreter/src/lib.rs -> runtime/interpreter/src/lib.rs`
- 1-file cycle: `stdlib/src/lib.rs -> stdlib/src/lib.rs`

## Communities (13 total, 3 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.23
Nodes (15): DiagnosticReporter, Statement, ConstDecl, EnumDecl, ExportDecl, FuncDecl, Ident, ModelDecl (+7 more)

### Community 1 - "Community 1"
Cohesion: 0.34
Nodes (24): techscript_ast, techscript_builtins, techscript_bytecode, techscript_cli, techscript_common, techscript_errors, techscript_formatter, techscript_gc (+16 more)

### Community 2 - "Community 2"
Cohesion: 0.16
Nodes (11): BytecodeModule, CheckedProgram, Program, LoweringResult, Module, Result, RuntimeError, compile() (+3 more)

### Community 3 - "Community 3"
Cohesion: 0.25
Nodes (10): DiagnosticReporter, Program, Result, Statement, Vec, Diagnostic, parse(), parse_recovered() (+2 more)

### Community 4 - "Community 4"
Cohesion: 0.26
Nodes (4): Default, Self, StdlibRegistry, Option

### Community 5 - "Community 5"
Cohesion: 0.31
Nodes (8): Box, FnOnce, RuntimeContext, RuntimeValue, AsyncTask, Result, RuntimeError, Vec

### Community 6 - "Community 6"
Cohesion: 0.29
Nodes (6): Capability, HashMap, Rc, Scheduler, StdlibModule, VecDeque

### Community 7 - "Community 7"
Cohesion: 0.25
Nodes (5): Callable, MockFunction, StdFunction, StdFnCallback, String

### Community 8 - "Community 8"
Cohesion: 0.40
Nodes (4): Option, Commands, Cli, Commands

## Knowledge Gaps
- **25 isolated node(s):** `Commands`, `Option`, `Commands`, `Program`, `LoweringResult` (+20 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `StdlibRegistry` connect `Community 4` to `Community 6`, `Community 7`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **Why does `StdFunction` connect `Community 7` to `Community 5`, `Community 6`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
- **Why does `StdlibModule` connect `Community 6` to `Community 4`, `Community 5`, `Community 7`?**
  _High betweenness centrality (0.016) - this node is a cross-community bridge._
- **What connects `Commands`, `Option`, `Commands` to the rest of the system?**
  _25 weakly-connected nodes found - possible documentation gaps or missing edges._