# Third-Party Licenses

TechScript includes or links to third-party software. Each dependency remains under its own license; the TechScript Apache-2.0 license does not relicense third-party code.

The authoritative dependency metadata is recorded in `Cargo.lock`, Python package metadata, and the individual upstream distributions. License texts and notices available in the repository are stored under `licenses/third_party/`.

## Representative direct dependencies

| Component | License information | Used by |
| --- | --- | --- |
| `logos` | MIT | Lexer |
| `clap` | MIT or Apache-2.0 | CLI |
| `serde` and `serde_json` | MIT or Apache-2.0 | Serialization and configuration |
| `tokio` | MIT | Async runtime integrations |
| `tower-lsp` | MIT | Language server |
| `rustyline` | MIT | REPL |
| `time` | MIT or Apache-2.0 | Time handling |
| `zstd` | MIT | Compression support |
| `colored` | MPL-2.0 | CLI presentation |
| `requests` | Apache-2.0 | Python installers |
| `hatchling` and `build` | MIT | Python package builds |

## Distribution requirements

When redistributing TechScript, retain the Apache-2.0 license, this attribution file, and the notices required by each dependency. In particular, files covered by the Mozilla Public License remain subject to its file-level terms. Consult the upstream license files and package metadata when producing a new distribution.

This document is a high-level index, not a replacement for the license text supplied by each dependency. Update it when adding a direct dependency or changing a bundled third-party component.
