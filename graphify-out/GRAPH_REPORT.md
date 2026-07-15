# Graph Report - .  (2026-07-15)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 17 nodes · 48 edges · 4 communities (3 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bb5cb889`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]

## God Nodes (most connected - your core abstractions)
1. `techscript_common` - 11 edges
2. `techscript_interpreter` - 10 edges
3. `techscript_syntax` - 10 edges
4. `techscript_ast` - 9 edges
5. `techscript_cli` - 9 edges
6. `techscript_errors` - 8 edges
7. `techscript_semantic` - 8 edges
8. `techscript_lsp` - 7 edges
9. `techscript_parser` - 6 edges
10. `techscript_lexer` - 5 edges

## Surprising Connections (you probably didn't know these)
- `techscript_cli` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  cli/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_interpreter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  runtime/interpreter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_lsp` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/lsp/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_cli` --crate_depends_on--> `techscript_common`  [EXTRACTED]
  cli/Cargo.toml → compiler/common/Cargo.toml
- `techscript_cli` --crate_depends_on--> `techscript_errors`  [EXTRACTED]
  cli/Cargo.toml → compiler/errors/Cargo.toml

## Import Cycles
- None detected.

## Communities (4 total, 1 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.40
Nodes (6): techscript_builtins, techscript_cli, techscript_gc, techscript_interpreter, techscript_stdlib, techscript_vm

### Community 1 - "Community 1"
Cohesion: 0.80
Nodes (5): techscript_ast, techscript_formatter, techscript_linter, techscript_semantic, techscript_syntax

### Community 2 - "Community 2"
Cohesion: 0.90
Nodes (5): techscript_common, techscript_errors, techscript_lexer, techscript_lsp, techscript_parser

## Knowledge Gaps
- **4 isolated node(s):** `techscript_builtins`, `techscript_gc`, `techscript_package_manager`, `techscript_vm`
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `techscript_interpreter` connect `Community 0` to `Community 1`, `Community 2`?**
  _High betweenness centrality (0.360) - this node is a cross-community bridge._
- **Why does `techscript_common` connect `Community 2` to `Community 0`, `Community 1`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **Why does `techscript_syntax` connect `Community 1` to `Community 0`, `Community 2`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._
- **What connects `techscript_builtins`, `techscript_gc`, `techscript_package_manager` to the rest of the system?**
  _4 weakly-connected nodes found - possible documentation gaps or missing edges._