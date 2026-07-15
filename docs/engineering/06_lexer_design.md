# 06 — TechScript 2.0 Lexer Design

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [03 Grammar](./03_grammar_ebnf.md) · [04 Compiler Architecture](./04_compiler_architecture.md) · [14 Error Codes](./14_error_codes.md)

---

## 1. Token Structure

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,    // The raw source text of this token
    pub span: Span,        // Byte offset range in source
}
```

---

## 2. Token Kinds — Complete Enumeration

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // ── Keywords ──────────────────────────────────────────
    Make,           // make
    Const,          // const
    Say,            // say
    Ask,            // ask
    Build,          // build
    Return,         // return
    Fun,            // fun (deprecated v1 method alias)
    Model,          // model
    SelfKw,         // self
    New,            // new
    When,           // when
    Else,           // else
    Each,           // each
    In,             // in
    Repeat,         // repeat
    While,          // while
    Break,          // break
    Continue,       // continue
    Attempt,        // attempt
    Catch,          // catch
    Throw,          // throw
    Import,         // import
    From,           // from
    Export,         // export
    True,           // true
    False,          // false
    None,           // none
    And,            // and
    Or,             // or
    Not,            // not
    Is,             // is

    // ── Future Reserved Keywords ──────────────────────────
    Async,          // async
    Await,          // await
    Type,           // type
    Interface,      // interface
    Match,          // match
    Enum,           // enum
    Yield,          // yield
    Spawn,          // spawn
    Pub,            // pub
    Mut,            // mut

    // ── Literals ──────────────────────────────────────────
    IntLiteral,     // 42, 0xFF, 0b1010, 0o77, 1_000_000
    FloatLiteral,   // 3.14, 1.0e10, 2.5e-3
    StringLiteral,  // "hello"
    FStringStart,   // f" (start of f-string)
    FStringText,    // literal text portion of f-string
    FStringExprStart, // { inside f-string
    FStringExprEnd,   // } inside f-string
    FStringEnd,     // " (end of f-string)

    // ── Identifiers ──────────────────────────────────────
    Identifier,     // variable names, function names, etc.

    // ── Arithmetic Operators ─────────────────────────────
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    DoubleSlash,    // //
    Percent,        // %
    DoubleStar,     // **

    // ── Comparison Operators ─────────────────────────────
    EqualEqual,     // ==
    BangEqual,      // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=

    // ── Assignment Operators ─────────────────────────────
    Equal,          // =
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    PercentEqual,   // %=

    // ── Range Operators ──────────────────────────────────
    DotDot,         // ..
    DotDotEqual,    // ..=

    // ── Delimiters ───────────────────────────────────────
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]

    // ── Separators ───────────────────────────────────────
    Comma,          // ,
    Dot,            // .
    Colon,          // :
    Semicolon,      // ;

    // ── Special ──────────────────────────────────────────
    Newline,        // \n or \r\n (significant for statement termination)
    Eof,            // End of file

    // ── Error ────────────────────────────────────────────
    Error,          // Unrecognized character or malformed token
}
```

---

## 3. Compatibility & Evolution Analysis

### 3.1 Compatibility Notes
- **`fun` Keyword Preservation**: The keyword `fun` is retained as a valid keyword matching `TokenKind::Fun` in 2.0 to prevent parser crashes on legacy Version 1 files.
- **Source Extensions**: The lexer is decoupled from file names, but the `tech` CLI only feeds files ending in `.txs` to the lexer.

### 3.2 Migration Notes
- When the lexer encounters the characters `fun`, it produces `TokenKind::Fun`. The semantic analyzer intercepts `TokenKind::Fun` nodes in the AST and generates warning `W0015`.
- Tooling migrators (`techfmt` / linter) identify `TokenKind::Fun` and replace its raw characters in the source `.txs` file with `build`.

### 3.3 Rationale
- **Logos Integration**: Using the `logos` crate for lexical scanning allows for compilation of DFA tables at build time, increasing scanning performance. Keeping `fun` as a token variant within the `logos` mapping is trivial:
  ```rust
  #[derive(Logos, Debug, Clone, Copy, PartialEq)]
  pub enum TokenKind {
      // ...
      #[token("fun")]
      Fun,
      #[token("build")]
      Build,
      // ...
  }
  ```
- **Implicit Continuation**: The lexer continues to suppress newlines after arithmetic or assignment operators to prevent incorrect statement boundary termination inside long assignments.

### 3.4 Future Roadmap
- **v2.2**: Introduce new token kinds for type signatures:
  - `Colon` (`:`)
  - `Arrow` (`->`)
- **v3.0**: Optimizations in the lexer will target parallel block-skipping for nested comments, utilizing Rust's multi-threading capabilities.
