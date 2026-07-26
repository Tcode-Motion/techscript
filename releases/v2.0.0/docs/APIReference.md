# TechScript 2.0 Compiler API Reference

> **Status**: Frozen Specification — 2.0.0 Stable
> **Last Updated**: 2026-07-26

This reference details the public Rust APIs exposed by the compiler workspace
crates for tools (LSP, CLI, Linter, Formatter) and direct integration.

---

## 1. `techscript_syntax`

Contains lexical and syntactic definitions, token kinds, and operators.

### Key Types

#### `TokenKind` (enum)
The list of all canonical 2.0 keywords, deprecated aliases, literals, and delimiters.

Functions:
- `is_canonical_keyword() -> bool`: Returns `true` if it is a canonical keyword (no warning).
- `is_alias_keyword() -> bool`: Returns `true` if it is a deprecated compatibility alias.
- `is_future_reserved_keyword() -> bool`: Returns `true` if it is a future reserved word.
- `to_canonical() -> Option<TokenKind>`: Returns the canonical 2.0 equivalent for deprecated aliases.
- `static_lexeme() -> Option<&'static str>`: Returns static string spelling if any.

#### `lookup_keyword(lexeme: &str) -> Option<TokenKind>`
Searches the keyword map. Handles all canonical, alias, and reserved words.

---

## 2. `techscript_lexer`

Lexical scanner that scans source string slices into a stream of tokens.

### Key Types

#### `Lexer<'a>` (struct)
```rust
pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
    // ...
}
```

Functions:
- `Lexer::new(source: &'a str) -> Self`: Creates a scanner instance.
- `next_token(&mut self) -> Token`: Scans and returns the next token.
- `lex_recovered(&mut self) -> (Vec<Token>, Vec<Diagnostic>)`: Scans entire source file, collecting recovery tokens and any lexical warnings/errors.

---

## 3. `techscript_parser`

Constructs the AST from a stream of scanned tokens using recursive descent.

### Key Types

#### `Parser<'a>` (struct)
```rust
pub struct Parser<'a> {
    tokens: &'a [Token],
    cursor: usize,
    // ...
}
```

Functions:
- `Parser::new(tokens: &'a [Token]) -> Self`: Creates parser instance.
- `parse_recovered(&mut self) -> (ProgramNode, Vec<Diagnostic>)`: Parses entire stream, returning the AST and collecting recovery error diagnostics.

---

## 4. `techscript_semantic`

Validates scoping, performs name resolution, and checks semantic constraints.

### Key Types

#### `Analyzer` (struct)
Performs scope checks, duplicate definitions check, and shadows checks.
- `Analyzer::new() -> Self`
- `analyze(&mut self, program: &ProgramNode) -> Vec<Diagnostic>`: Runs semantic passes. Returns a vector of diagnostics containing warnings and compile errors.
