# 07 — DECISIONS

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Architecture Decision Records (ADR) log
> **Parent Link**: [MEMORY](./02_memory.md)
> **Child Links**: [PROGRESS](./06_progress.md) · [AI_BOOTSTRAP](./09_ai_bootstrap.md)

---

## ADR 001 — Rust Implementation Language

- **Status**: Approved
- **Date**: 2026-07-15
- **Decision**: Rewrite the TechScript compiler and execution toolchain in Rust, replacing the Version 1 Python prototype.
- **Reason**: Python suffers from slow start-up overhead and runtime environment inconsistencies. Rust offers zero-cost abstractions, memory safety without GC overhead, static typing, and builds into a single self-contained executable.
- **Alternatives**: C++ (too risky for memory leaks), Go (GC pause overhead, larger binary size).
- **Trade-offs**: Rust has a steeper learning curve and longer compile times, but it guarantees compiler safety and sub-millisecond execution startups.

---

## ADR 002 — Frozen Extension `.txs`

- **Status**: Approved
- **Date**: 2026-07-15
- **Decision**: Freeze the official TechScript file extension as `.txs`. The compiler must reject other extensions during execution calls.
- **Reason**: Avoids collisions with TypeScript (`.ts`) or XML stylesheet paths. Distinct extension name ensures clean IDE highlighting and linter rules configuration.
- **Alternatives**: `.tech`, `.tsc`.
- **Trade-offs**: Existing Version 1 user files must be renamed to `.txs` before execution, causing a one-time migration step.

---

## ADR 003 — Unified `build` Method Syntax

- **Status**: Approved
- **Date**: 2026-07-15
- **Decision**: Unify function and method declarations under the single keyword `build`. Retain `fun` inside models as a deprecated alias.
- **Reason**: Simplifies parsing rules, eliminates duplicate method definition branches in EBNF grammar, and clarifies that a method is structurally identical to a function.
- **Alternatives**: Keep both `build` and `fun` as active separate keywords; remove `fun` completely.
- **Trade-offs**: Retaining `fun` as an alias adds minor parser complexity but preserves backward compatibility with Version 1 scripts.

---

## ADR 004 — Standard Library Separation

- **Status**: Approved
- **Date**: 2026-07-15
- **Decision**: Decouple the standard library into a separate crate (`techscript_stdlib`), loaded optionally using imports (e.g. `import web`).
- **Reason**: Prevents bloat in the core interpreter engine and allows optional compilation targets.
- **Alternatives**: Embedding all functions directly into the global interpreter context.
- **Trade-offs**: Users must explicitly write `import` lines for web or file access, adding a small amount of boilerplate code.
