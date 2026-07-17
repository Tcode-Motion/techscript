# TechScript 2.0 API Reference

This reference details the public APIs exposed by crates inside the workspace.

## 1. techscript_lexer
- `Lexer::new(source)`: returns new Lexer instance.
- `Lexer::lex_recovered()`: returns tokens list along with recovery diagnostics.

## 2. techscript_parser
- `Parser::new(tokens)`: returns Parser.
- `Parser::parse_recovered()`: returns partial AST in presence of parser recovery sync loops.

## 3. techscript_package_manager
- `DependencySolver`: matches constraints and resolves dependency trees.
- `Manifest`: TOML manifest struct.
- `Lockfile`: lockfile serialization layout.
