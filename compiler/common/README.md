# techscript_common


Foundational types and utilities shared across all TechScript 2.0 compiler, runtime, and tooling crates.

## Architecture

```
techscript_common   ← Foundation layer (this crate)
    ↓
techscript_syntax   ← Token kinds, keywords, precedence
techscript_errors   ← Diagnostics, error codes
    ↓
techscript_ast      ← AST node definitions, visitor trait
    ↓
techscript_lexer    ← Tokenizer
techscript_parser   ← Recursive descent + Pratt parser
    ↓
techscript_semantic ← Scope/name resolution
    ↓
techscript_interpreter ← Tree-walking execution
```

## Public API

### Core Types
- **`Span`** — Byte-offset range `[start, end)` for source location tracking.
- **`NodeId`** — Unique `u32` identifier for AST nodes.
- **`NodeIdGenerator`** — Thread-safe sequential generator for `NodeId` values.
- **`Ident`** — Named identifier paired with its source `Span`.

### Source Management
- **`FileId`** — Handle identifying a loaded source file within a compilation session.
- **`SourceFile`** — Immutable record of a file's path (`PathBuf`), contents, and precomputed line-start offsets.
- **`SourceManager`** — Registry of all loaded source files, providing `Arc<SourceFile>` shared access.
- **`Position`** — Resolved human-readable position: file, 1-indexed line, 1-indexed column, and byte offset.

### File Validation
- **`is_techscript_file(path)`** — Returns `true` if the path has the `.txs` extension.
- **`validate_extension(path)`** — Returns `Ok(())` or `CommonError::InvalidExtension`.
- **`TECHSCRIPT_EXTENSION`** / **`TECHSCRIPT_DOT_EXTENSION`** — `"txs"` / `".txs"` constants.

### Constants
- **`TECHSCRIPT_VERSION`** — Version string from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`.
- **`MAX_RECURSION_DEPTH`** — Stack overflow guard (1024).
- **`MAX_SOURCE_FILE_SIZE`** — Maximum file size guard (10 MiB).

## Dependencies

- `serde` — Serialization/deserialization derives for all core types.

## Usage

```rust
use techscript_common::{Span, NodeId, NodeIdGenerator, Ident};
use techscript_common::{FileId, SourceFile, SourceManager, Position};
use techscript_common::{is_techscript_file, validate_extension};
use techscript_common::{TECHSCRIPT_VERSION, MAX_RECURSION_DEPTH};
```
