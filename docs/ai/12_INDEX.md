# 12 — INDEX

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Master index of the TechScript 2.0 AI Documentation Layer
> **Parent Link**: (None - Master Root)
> **Child Links**: All documents in this directory

---

## 1. Document Directory

| Document | Purpose | Key References |
|---|---|---|
| [00_PROJECT.md](./00_project.md) | High-level overview, vision, and roadmap | CONTEXT, MEMORY |
| [01_CONTEXT.md](./01_context.md) | Project context, feature matrices, and target platforms | MEMORY, RULES |
| [02_MEMORY.md](./02_memory.md) | Invariant decisions, keywords, and conventions | RULES, DECISIONS |
| [03_RULES.md](./03_rules.md) | Coding, parser, git, compatibility, and performance guidelines | ARCHITECTURE, LANGUAGE |
| [04_ARCHITECTURE.md](./04_architecture.md) | Pipeline stages, execution engines, and dependencies | LANGUAGE, REPOSITORY |
| [05_LANGUAGE.md](./05_language.md) | Syntax cheat sheet: types, variables, loops, classes | DECISIONS, REPOSITORY |
| [06_PROGRESS.md](./06_progress.md) | Task status board, milestones, and open questions | AI_BOOTSTRAP, HANDOFF |
| [07_DECISIONS.md](./07_decisions.md) | Architecture Decision Records (ADRs) | PROGRESS, AI_BOOTSTRAP |
| [08_REPOSITORY.md](./08_repository.md) | Folder structure, entry points, and build commands | AI_BOOTSTRAP, INDEX |
| [09_AI_BOOTSTRAP.md](./09_ai_bootstrap.md) | Immediate quick-start guide, styles, and do-not-changes | PROGRESS, HANDOFF |
| [10_HANDOFF.md](./10_handoff.md) | Session-to-session handoff logs | GLOSSARY, SESSION_CONTEXT |
| [11_GLOSSARY.md](./11_glossary.md) | Compiler and language terminology index | INDEX |
| [12_INDEX.md](./12_index.md) | This index file | All files |
| [AI_CONTEXT_PACK.md](./AI_CONTEXT_PACK.md) | Condensed master pack (one-file bootstrap) | All files |
| [SESSION_CONTEXT.md](./SESSION_CONTEXT.md) | Dynamic session status mapping | HANDOFF |
| [AI_BOOTSTRAP.md](../../AI_BOOTSTRAP.md) | Root AI onboarding guide (uses Graphify-Labs) | All files |
| [GRAPHIFY_AI.md](../GRAPHIFY_AI.md) | Official Graphify-Labs AI usage guide | AI_BOOTSTRAP |

---

## 2. Recommended Reading Order

When initializing a new AI session or compiler agent context, load the documents in this order:

1. **[AI_BOOTSTRAP.md](../../AI_BOOTSTRAP.md)** — Root AI onboarding guide using Graphify.
2. **[SESSION_CONTEXT.md](./SESSION_CONTEXT.md)** — Active goals, recent changes, and issues.
3. **[AI_CONTEXT_PACK.md](./AI_CONTEXT_PACK.md)** — Condensed master context pack.
4. **[02_MEMORY.md](./02_memory.md)** — Frozen invariants and keywords.
5. **[00_PROJECT.md](./00_project.md)** — General vision.
6. **[03_RULES.md](./03_rules.md)** — Code lints and performance budgets.
7. **[04_ARCHITECTURE.md](./04_architecture.md)** — Pipeline modules.
8. **[05_LANGUAGE.md](./05_language.md)** — Syntax specs.
9. **Remaining files as needed.**

---

## 3. Synchronization Path

To prevent document isolation, updates follow this chain:
```
PROJECT → CONTEXT → MEMORY → RULES → ARCHITECTURE → LANGUAGE → REPOSITORY → AI_BOOTSTRAP → HANDOFF
```
If a change is made to the language syntax, update `MEMORY.md` first, then propagate to `RULES.md` and `LANGUAGE.md`.

