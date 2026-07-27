# TechScript Versioning Policy

TechScript follows the **Semantic Versioning 2.0.0 (SemVer)** specification. Version numbers are structured as:

$$\text{MAJOR}.\text{MINOR}.\text{PATCH}$$

* **MAJOR** version: Incremented when making incompatible API changes, language-breaking syntax changes, or VM instruction layout modifications.
* **MINOR** version: Incremented when adding functionality in a backwards-compatible manner (e.g., adding a new standard library module, new keyword extensions, optimization passes).
* **PATCH** version: Incremented when introducing backwards-compatible bug fixes or minor compiler performance tweaks.

---

## Pre-Release Phase Tagging

During early development stages (pre-1.0.0), TechScript uses the following lifecycle indicators:

| Suffix | Phase | Example | Goal / Status |
|:---|:---|:---|:---|
| `-alpha.x` | Alpha | `v0.1.0-alpha.1` | Core language mechanics and compiler/interpreter MVP. Highly unstable. |
| `-beta.x` | Beta | `v0.5.0-beta.2` | Feature-complete compiler/VM. Stabilization, test coverage, and documentation focus. |
| `-rc.x` | Release Candidate | `v1.0.0-rc.1` | Production-ready candidate. No new features, only blocker bug fixes. |

---

## Backwards Compatibility Definition

For the purposes of SemVer, a change is considered **backwards-compatible** if:
* Code written for a previous minor/patch version of the same major release compiles and executes with the same behavior.
* Standard library module signatures remain identical or backwards-compatible (e.g., parameters added only with defaults).
* Bytecode formats match current VM decode instructions.

A change is considered **breaking** (requiring a Major bump) if:
* Existing valid TechScript syntax is rejected by the parser.
* Built-in functions are renamed or removed.
* VM bytecode layout changes in a way that breaks previously compiled binary `.txc` files.
* Minimum compiler dependencies or toolchains are updated with incompatible versions.
