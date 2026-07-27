# Project Roadmap

This document outlines the milestones for the TechScript language.

---

## 🎯 Phase 1: Core Mechanics (v0.1.0 Alpha)
* **Goal**: Establish stable tokenization, parser algorithms, and stack execution.
* [x] Custom Pratt Parser for expression precedence.
* [x] Basic VM layout with NaN-boxed value representations.
* [x] Standard library support (`math`, `fs`, `os`).

---

## 🚀 Phase 2: Tooling & Concurrency (v0.5.0 Beta)
* **Goal**: Standardize tooling ecosystems and multi-thread performance.
* [x] Cyberpunk-themed TechScript Studio IDE.
* [x] Build tools (`tech fmt`, `tech lint`, `tech test`, `tech package`).
* [x] LSP language server support for auto-completions.
* [x] Concurrent event loop (`async`/`await`) and parallel thread pools.

---

## 🏆 Phase 3: Native Targets & Stability (v1.0.0 Stable)
* **Goal**: Release production-ready compiler with native backend outputs.
* [ ] Integrate LLVM code generation for native standalone builds.
* [ ] Add tracing memory profiler inside the IDE.
* [ ] Formally verify core library code correctness.
* [ ] Standard SQL driver modules (`use sql`).
