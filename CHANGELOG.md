# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-07-15

### Added
- Created 17 Cargo workspace member crates, covering the entire compiler, runtime, and tools stack.
- Unified common types (`Span`, `NodeId`, `Ident`) inside the `techscript_common` crate.
- Unified syntax constants, keyword lists, tokens, and operator precedence levels inside the `techscript_syntax` crate.
- Scaffolding skeleton for all remaining modules: AST, Lexer, Parser, Errors, Semantic Analyzer, Interpreter, VM, GC, Builtins, Stdlib, CLI, LSP, Formatter, Linter, and Package Manager.
- Established default placeholder unit tests and example entry points.
- Configured CI/CD workflows and GitHub templates.
