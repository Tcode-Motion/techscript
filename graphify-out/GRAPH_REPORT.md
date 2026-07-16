# Graph Report - .  (2026-07-16)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 46 nodes · 152 edges · 7 communities (6 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bfe51385`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 6|Community 6]]

## God Nodes (most connected - your core abstractions)
1. `techscript_common` - 18 edges
2. `main()` - 17 edges
3. `techscript_cli` - 17 edges
4. `Result` - 15 edges
5. `techscript_syntax` - 15 edges
6. `techscript_ast` - 14 edges
7. `techscript_errors` - 11 edges
8. `techscript_interpreter` - 10 edges
9. `techscript_ir` - 10 edges
10. `techscript_semantic` - 10 edges

## Surprising Connections (you probably didn't know these)
- `techscript_cli` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  cli/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_interpreter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  runtime/interpreter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_linter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/linter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_lsp` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/lsp/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_runtime` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  runtime/runtime/Cargo.toml → compiler/ast/Cargo.toml

## Import Cycles
- 1-file cycle: `tools/packager/src/main.rs -> tools/packager/src/main.rs`

## Communities (7 total, 1 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.40
Nodes (16): Path, Result, compile_binary(), compile_inno_installer(), generate_checksums(), generate_documentation(), generate_inno_script(), generate_release_manifest() (+8 more)

### Community 1 - "Community 1"
Cohesion: 0.46
Nodes (8): techscript_builtins, techscript_cli, techscript_gc, techscript_interpreter, techscript_package_manager, techscript_runtime, techscript_stdlib, techscript_vm

### Community 2 - "Community 2"
Cohesion: 0.64
Nodes (8): techscript_common, techscript_errors, techscript_lexer, techscript_linter, techscript_lsp, techscript_module_resolver, techscript_parser, techscript_semantic

### Community 3 - "Community 3"
Cohesion: 0.57
Nodes (7): techscript_ast, techscript_bytecode, techscript_formatter, techscript_ir, techscript_llvm_backend, techscript_optimizer, techscript_syntax

### Community 4 - "Community 4"
Cohesion: 0.67
Nodes (3): extract_workspace_version(), get_git_commit(), String

## Knowledge Gaps
- **3 isolated node(s):** `techscript_gc`, `techscript_package_manager`, `techscript_packager`
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `techscript_common` connect `Community 2` to `Community 1`, `Community 3`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `techscript_cli` connect `Community 1` to `Community 2`, `Community 3`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Why does `main()` connect `Community 0` to `Community 4`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `techscript_gc`, `techscript_package_manager`, `techscript_packager` to the rest of the system?**
  _3 weakly-connected nodes found - possible documentation gaps or missing edges._