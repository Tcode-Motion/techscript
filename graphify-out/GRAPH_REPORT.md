# Graph Report - TechScript 2.0  (2026-07-15)

## Corpus Check
- 41 files · ~27,371 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 490 nodes · 552 edges · 36 communities (34 shown, 2 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 1 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `09547067`
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
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 33|Community 33]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]

## God Nodes (most connected - your core abstractions)
1. `Executive Summary` - 21 edges
2. `01 — TechScript v2.0 Language Specification` - 18 edges
3. `08 — TechScript 2.0 Repository Milestones` - 14 edges
4. `00 — TechScript 2.0 Master Architecture` - 12 edges
5. `03 — TechScript 2.0 EBNF Grammar` - 10 edges
6. `1. Module Specification` - 10 edges
7. `18 — TechScript 2.0 AI Context Document` - 10 edges
8. `Graphify AI Usage Guide` - 9 edges
9. `05 — LANGUAGE` - 9 edges
10. `17 — TechScript 2.0 Development Roadmap` - 9 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `validate_graphify_outputs()`  [INFERRED]
  tools/update_graphify.py → tools/check_graphify.py

## Import Cycles
- None detected.

## Communities (36 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.23
Nodes (12): check_python_dependencies(), find_repo_root(), main(), Dynamically scans output_dir and validates generated files.     Excludes files s, validate_graphify_outputs(), check_api_keys(), check_graphify_installed(), check_python_version() (+4 more)

### Community 1 - "Community 1"
Cohesion: 0.08
Nodes (23): 00 — PROJECT, 1. Project Identity, 2. Core Vision & Philosophy, 3. High-Level Evolution, 4. Current Repository Progress, 10 — HANDOFF, 1. What has been Completed, 2. Invariants (Never Repeat) (+15 more)

### Community 2 - "Community 2"
Cohesion: 0.06
Nodes (32): 01 — TechScript v2.0 Language Specification, 10. Error Handling, 11. Objects (Models), 12. Comments, 13. Strings, 14. Collections, 15. Built-in Functions, 16.1 Compatibility Notes (+24 more)

### Community 3 - "Community 3"
Cohesion: 0.08
Nodes (25): 8-Week Starter Plan, Architecture Diagrams, Concurrency and Other Goals, Deliverables Checklist, Dependency & Interoperability Strategy, Developer Roles & Resources, Editor Integration (LSP) & VSCode Extension, Estimated Effort & Requirements (+17 more)

### Community 4 - "Community 4"
Cohesion: 0.10
Nodes (21): 00 — TechScript 2.0 Master Architecture, 10. Document Index, 1. Design Philosophy, 2. Engineering Principles, 3. System Overview, 4.1 Compiler Frontend, 4.2 Compiler Backend, 4.3 Runtime (+13 more)

### Community 5 - "Community 5"
Cohesion: 0.10
Nodes (21): 03 — TechScript 2.0 EBNF Grammar, 1. Program Structure, 2. Declarations, 3. Statements, 4. Control Flow, 5. Imports, 6.1 Expression Hierarchy, 6.2 Primary Expressions (+13 more)

### Community 6 - "Community 6"
Cohesion: 0.10
Nodes (21): 17 — TechScript 2.0 Development Roadmap, Compatibility & Evolution Analysis, Compatibility Notes, Future Roadmap, Migration Notes, Phase 1 — Compiler Frontend (Weeks 1–8), Phase 2 — Standard Library + Tools (Weeks 9–14), Phase 3 — Bytecode VM (Weeks 15–22) (+13 more)

### Community 8 - "Community 8"
Cohesion: 0.11
Nodes (18): 08 — TechScript 2.0 Repository Milestones, 8.3 Rationale, 8.4 Future Roadmap, Compatibility & Evolution Analysis, Compatibility Notes, Migration Notes, Milestone 10 — Package Manager (Future), Milestone 11 — Self-Hosting (Future) (+10 more)

### Community 9 - "Community 9"
Cohesion: 0.11
Nodes (18): 12 — TechScript 2.0 Standard Library Specification, 1.1 `io`, 1.2 `math`, 1.3 `string`, 1.4 `file`, 1.5 `web` (Optional v2.0), 1.6 `time`, 1.7 `random` (+10 more)

### Community 10 - "Community 10"
Cohesion: 0.12
Nodes (17): 1.1 Python & Rust Prerequisites, 1.2 Installation, 1. Requirements & Installation, 2. Configuration, 3.1 Update Command (`update_graphify.py`), 3.2 Verification Command (`check_graphify.py`), 3. CLI Commands, 4. Exit Codes (+9 more)

### Community 11 - "Community 11"
Cohesion: 0.13
Nodes (15): 04 — TechScript 2.0 Compiler Architecture, 1. Pipeline Overview, 2. Stage 1: Lexer (Lexical Analysis), 3. Stage 2: Parser (Syntactic Analysis), 4. Stage 3: Semantic Analysis, 5. Stage 4a: Interpreter (v2.0 Backend), 6.1 Compatibility Notes, 6.2 Migration Notes (+7 more)

### Community 12 - "Community 12"
Cohesion: 0.13
Nodes (15): 09 — TechScript 2.0 Runtime Design, 1. Runtime Architecture, 2. Value Representation, 3.1 Call Frame, 3.2 Environment Scopes, 3. Scope and Call Frame Design, 4. Module Loading System, 5.1 Startup Sequence (+7 more)

### Community 13 - "Community 13"
Cohesion: 0.14
Nodes (14): 05 — TechScript 2.0 AST Design, 1. AST Node Hierarchy, 2.1 Span (Source Location), 2.2 Node ID, 2.3 Identifier, 2. Core Types, 3. Program (Root Node), 4.1 Declarations (+6 more)

### Community 14 - "Community 14"
Cohesion: 0.15
Nodes (13): 07 — TechScript 2.0 Parser Design, 1.1 Decision: Recursive Descent + Pratt Parser, 1. Parsing Strategy, 2. Parser State, 3. Statement & Method Parsing, 4.1 Binding Power Table, 4. Expression Parsing — Pratt Parser, 5. Error Recovery (+5 more)

### Community 15 - "Community 15"
Cohesion: 0.15
Nodes (13): 10 — TechScript 2.0 Semantic Analysis Specification, 1. Overview, 2. Scope & Name Resolution, 3.1 Variable & Variable Lifetime Validation, 3.2 Function & Method Validation, 3.3 Control Flow & Module Validation, 3. Semantic Validation Rules, 4. Diagnostics & Suggestions (+5 more)

### Community 16 - "Community 16"
Cohesion: 0.15
Nodes (13): 14 — TechScript 2.0 Error Code Specification, 16.1 Compatibility Notes, 16.2 Migration Notes, 16.3 Rationale, 16.4 Future Roadmap, Compatibility & Evolution Analysis, Error Code Ranges, Lexer Errors (E0001 – E0099) (+5 more)

### Community 17 - "Community 17"
Cohesion: 0.17
Nodes (12): 13 — TechScript 2.0 CLI Specification, 1. Binary Name, 2. Command Catalogue, 3.1 `tech run`, 3.2 `tech lint --fix`, 3.3 `tech test`, 3. Options & Arguments, 4.1 Compatibility Notes (+4 more)

### Community 18 - "Community 18"
Cohesion: 0.17
Nodes (12): 15 — TechScript 2.0 Testing Strategy, 1. Testing Pyramid, 2.1 Unit Tests, 2.2 Snapshot Tests, 2.3 Integration Tests (End-to-End), 2. Testing Levels, 3. Fuzz Testing & Benchmarking, 4.1 Compatibility Notes (+4 more)

### Community 19 - "Community 19"
Cohesion: 0.18
Nodes (11): 05 — LANGUAGE, 1. Syntax Overview, 2. Primitive Types, 3. Variables & Assignment, 4.1 Conditionals, 4.2 Loops, 4. Control Flow, 5. Functions & Lambdas (+3 more)

### Community 20 - "Community 20"
Cohesion: 0.18
Nodes (11): 16 — TechScript 2.0 Coding Standards, 1. Rust Style Guide, 2.1 Rust code, 2.2 TechScript language code, 2. Naming & Case Conventions, 3. Commit Message & PR Guidelines, 4.1 Compatibility Notes, 4.2 Migration Notes (+3 more)

### Community 21 - "Community 21"
Cohesion: 0.18
Nodes (10): 18 — TechScript 2.0 AI Context Document, Compiler & Repository Architecture, Development Roadmap, Document Index, Folder Structure, Key Language Features (v2.0), Language Philosophy, Project Overview (+2 more)

### Community 22 - "Community 22"
Cohesion: 0.20
Nodes (10): 02 — MEMORY, 1. Frozen Decisions (Never Change), 2.1 Keyword List, 2.2 Unification Rule, 2.3 Statement Termination, 2. Frozen Syntax & Keywords, 3.1 Rust Source Code, 3.2 TechScript Code (+2 more)

### Community 23 - "Community 23"
Cohesion: 0.20
Nodes (10): 04 — ARCHITECTURE, 1. System Pipeline, 2.1 AST (techscript_ast), 2.2 Symbol Table, 2. Shared Data Structures, 3.1 AST Interpreter (v2.0), 3.2 Bytecode VM (v2.1 - Planned), 3.3 LLVM Native Compiler (v3.0 - Planned) (+2 more)

### Community 24 - "Community 24"
Cohesion: 0.20
Nodes (10): 11 — TechScript 2.0 Interpreter Design, 1. Execution Model, 2. Expression Evaluation, 3. Statement Execution, 4. Function & Method Dispatch, 5.1 Compatibility Notes, 5.2 Migration Notes, 5.3 Rationale (+2 more)

### Community 25 - "Community 25"
Cohesion: 0.22
Nodes (9): 02 — TechScript 2.0 Monorepo Folder Structure, Cargo Workspace Configuration, Compatibility & Evolution Analysis, Compatibility Notes, Complete Directory Listing, Future Roadmap, Migration Notes, Rationale (+1 more)

### Community 26 - "Community 26"
Cohesion: 0.25
Nodes (8): 01 — CONTEXT, 1. What is TechScript?, 2. Why does it exist?, 3. Supported Platforms, 4.1 Supported in Version 2.0, 4.2 Unsupported in Version 2.0 (Planned for v2.1+), 4. Feature Matrices, 5. Current Compiler Pipeline

### Community 27 - "Community 27"
Cohesion: 0.25
Nodes (8): 03 — RULES, 1. Compiler Invariants, 2. Formatting & Coding Standards, 3. Git & Pull Requests, 4. Compatibility & Migration Rules, 5.1 Performance Goals, 5.2 Security Goals, 5. Non-Functional Budgets

### Community 28 - "Community 28"
Cohesion: 0.25
Nodes (8): 06 — PROGRESS, 1. Development Stages, 2.1 Completed Tasks, 2.2 In Progress (Active Session), 2.3 Next Priority Tasks (Milestone 1), 2. Status Board, 3. Milestones Overview, 4. Open Questions

### Community 29 - "Community 29"
Cohesion: 0.25
Nodes (8): 08 — REPOSITORY, 1. Directory Structure, 2. Key Entry Points, 3.1 Build all targets, 3.2 Execute test suite, 3.3 Verify formatting, 3.4 Execute linter checks, 3. Build & Test Commands

### Community 30 - "Community 30"
Cohesion: 0.25
Nodes (7): 1. Project Context & Vision, 2. Invariant Rules (Do Not Change), 3. Language Cheat Sheet, 4. Compiler Pipeline & Crate Map, 5. Development Targets, Crate Structure, TechScript 2.0 Master AI Context Pack

### Community 31 - "Community 31"
Cohesion: 0.25
Nodes (8): 06 — TechScript 2.0 Lexer Design, 1. Token Structure, 2. Token Kinds — Complete Enumeration, 3.1 Compatibility Notes, 3.2 Migration Notes, 3.3 Rationale, 3.4 Future Roadmap, 3. Compatibility & Evolution Analysis

### Community 32 - "Community 32"
Cohesion: 0.29
Nodes (7): 09 — AI BOOTSTRAP, 1. Quick-Start Context, 2.1 Rust Style, 2.2 Commit Messages, 2. Coding & Style Rules, 3. Important "Do Not Change" List, 4. Current Target (Milestone 1)

### Community 33 - "Community 33"
Cohesion: 0.40
Nodes (5): 07 — DECISIONS, ADR 001 — Rust Implementation Language, ADR 002 — Frozen Extension `.txs`, ADR 003 — Unified `build` Method Syntax, ADR 004 — Standard Library Separation

## Knowledge Gaps
- **340 isolated node(s):** `graphify`, `Workflow: graphify`, `1. Onboarding Workflow`, `2. Invariant Rules to Remember`, `3. Querying Codebase` (+335 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `01 — TechScript v2.0 Language Specification` connect `Community 2` to `Community 7`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Why does `00 — TechScript 2.0 Master Architecture` connect `Community 4` to `Community 7`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Why does `03 — TechScript 2.0 EBNF Grammar` connect `Community 5` to `Community 7`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **What connects `Dynamically scans output_dir and validates generated files.     Excludes files s`, `graphify`, `Workflow: graphify` to the rest of the system?**
  _341 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.08232118758434548 - nodes in this community are weakly interconnected._
- **Should `Community 2` be split into smaller, more focused modules?**
  _Cohesion score 0.0625 - nodes in this community are weakly interconnected._
- **Should `Community 3` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._