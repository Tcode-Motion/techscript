# Graph Report - .  (2026-07-27)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 35 nodes · 113 edges · 6 communities (5 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f867f3fc`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]

## God Nodes (most connected - your core abstractions)
1. `techscript_cli` - 19 edges
2. `techscript_common` - 18 edges
3. `techscript_syntax` - 16 edges
4. `techscript_ast` - 15 edges
5. `techscript_errors` - 12 edges
6. `techscript_interpreter` - 10 edges
7. `techscript_ir` - 10 edges
8. `techscript_semantic` - 10 edges
9. `techscript_vm` - 9 edges
10. `techscript_runtime` - 8 edges

## Surprising Connections (you probably didn't know these)
- `techscript_cli` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  cli/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_formatter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/formatter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_interpreter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  runtime/interpreter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_linter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/linter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_lsp` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/lsp/Cargo.toml → compiler/ast/Cargo.toml

## Import Cycles
- None detected.

## Communities (6 total, 1 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.40
Nodes (7): check_api_keys(), check_graphify_installed(), check_python_version(), check_rust_toolchain(), log_message(), main(), run_command()

### Community 1 - "Community 1"
Cohesion: 0.61
Nodes (9): techscript_common, techscript_errors, techscript_formatter, techscript_lexer, techscript_linter, techscript_lsp, techscript_module_resolver, techscript_parser (+1 more)

### Community 2 - "Community 2"
Cohesion: 0.80
Nodes (6): techscript_ast, techscript_bytecode, techscript_ir, techscript_llvm_backend, techscript_optimizer, techscript_syntax

### Community 3 - "Community 3"
Cohesion: 0.53
Nodes (6): techscript_builtins, techscript_gc, techscript_interpreter, techscript_runtime, techscript_stdlib, techscript_vm

### Community 4 - "Community 4"
Cohesion: 0.67
Nodes (3): techscript_cli, techscript_native_runtime, techscript_package_manager

## Knowledge Gaps
- **4 isolated node(s):** `techscript_gc`, `techscript_native_runtime`, `techscript_package_manager`, `techscript_packager`
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `techscript_cli` connect `Community 4` to `Community 1`, `Community 2`, `Community 3`?**
  _High betweenness centrality (0.117) - this node is a cross-community bridge._
- **Why does `techscript_interpreter` connect `Community 3` to `Community 1`, `Community 2`, `Community 4`?**
  _High betweenness centrality (0.060) - this node is a cross-community bridge._
- **Why does `techscript_common` connect `Community 1` to `Community 2`, `Community 3`, `Community 4`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **What connects `techscript_gc`, `techscript_native_runtime`, `techscript_package_manager` to the rest of the system?**
  _4 weakly-connected nodes found - possible documentation gaps or missing edges._