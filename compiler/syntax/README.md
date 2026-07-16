# techscript_syntax

Unified token definitions, keyword tables, operator precedence, and associativity mappings for TechScript 2.0.

This crate serves as the single source of truth for the language grammar and token categories, completely decoupled from compiler implementation details.

## Modules

- **`TokenKind`** — Enumeration of all canonical keywords, backward compatibility aliases, future reserved keywords, operator symbols, delimiters, and special markers.
- **`LiteralKind`** — Logical categorization for literals: `Int`, `Float`, `Str`, `Bool`, `Null`.
- **`NumericLiteralKind`** — Direct representation formats for numbers: `Decimal`, `Hex`, `Binary`, `Octal`.
- **`Token`** — Combines a `TokenKind` category, its raw matched string slice (`lexeme`), and its source location (`Span`).
- **`Precedence`** — Stable Pratt parsing precedence levels (from `None` up to `Call`).
- **`Associativity`** — Operator grouping rules (`Left`, `Right`, `None`).

## Key API Features

### 1. Keyword Management & Translation
Checks if a token is a keyword or resolves alias keywords to their canonical equivalents:
```rust
use techscript_syntax::TokenKind;

assert!(TokenKind::Make.is_keyword());
assert!(TokenKind::Let.is_alias_keyword());
assert_eq!(TokenKind::Let.to_canonical(), Some(TokenKind::Make));
```

### 2. Static Keyword Lookup
Efficient, compile-time matched keyword lookup from lexeme slices:
```rust
use techscript_syntax::{lookup_keyword, TokenKind};

assert_eq!(lookup_keyword("make"), Some(TokenKind::Make));
assert_eq!(lookup_keyword("let"), Some(TokenKind::Let));
```

### 3. Operator Metadata
Query precedence and associativity properties directly from tokens:
```rust
use techscript_syntax::{TokenKind, Precedence, Associativity};

let op = TokenKind::DoubleStar;
assert_eq!(op.precedence(), Precedence::Exponent);
assert_eq!(op.associativity(), Associativity::Right);
```

### 4. Numeric Base Analyzer
Resolves numeric literal format base representation:
```rust
use techscript_syntax::{numeric_literal_kind, NumericLiteralKind};

assert_eq!(numeric_literal_kind("0xFF"), Some(NumericLiteralKind::Hex));
assert_eq!(numeric_literal_kind("123"), Some(NumericLiteralKind::Decimal));
```

## Dependencies
- `techscript_common` — Sourced location spans.
- `serde` — Serialization and deserialization support.
