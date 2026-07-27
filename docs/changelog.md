# Changelog

All notable changes to the TechScript language compiler, runtime, and tools are tracked in this document.

For the detailed keep-a-changelog layout, see the root [CHANGELOG.md](../CHANGELOG.md).

---

## [2.0.0] - 2026-07-26 (Syntax Freeze)
* **Language Freeze**: Inverted deprecated and canonical keyword lists. Verified syntax is locked for major version 2.x.
* **New Keywords**: Added `loop`, `parallel`, `default` as canonical keywords.
* **Deprecations**: Formally deprecated legacy keywords (`make`, `keep`, `build`, `model`, etc.) and set up warnings.
* **Error Handling**: Documented the full TSW error code namespace.

---

## [1.0.8] - 2026-06-01
* **IDE**: Launched TechScript Studio built on `egui` and `egui_dock`.
* **Workspace**: Reorganized compiler, runtime, and tools into 17 Cargo subcrates.
* **Shell Integration**: Double-clicking `.txs` files executes program, keeping shell open.

---

## [1.0.5] - 2026-03-15
* **Features**: Added `use three_d` for 3D graphic rendering.
* **CLI**: Added support for formatting, linting, testing, and building via the CLI.
