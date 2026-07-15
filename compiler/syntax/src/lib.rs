//! # TechScript Syntax Crate
//!
//! Language syntax constants, token kind definitions, and precedence mappings.
//! Keeping grammar metadata separate from the lexer/parser logic.

use serde::{Deserialize, Serialize};
use techscript_common::Span;

/// Complete token enum listing all 83 variants for TechScript 2.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenKind {
    // Keywords (31)
    Make,     // make
    Const,    // const
    Say,      // say
    Ask,      // ask
    Build,    // build
    Return,   // return
    Fun,      // fun (deprecated)
    Model,    // model
    SelfKw,   // self
    New,      // new
    When,     // when
    Else,     // else
    Each,     // each
    In,       // in
    Repeat,   // repeat
    While,    // while
    Break,    // break
    Continue, // continue
    Attempt,  // attempt
    Catch,    // catch
    Throw,    // throw
    Import,   // import
    From,     // from
    Export,   // export
    True,     // true
    False,    // false
    None,     // none
    And,      // and
    Or,       // or
    Not,      // not
    Is,       // is

    // Future Keywords (10)
    Async,     // async
    Await,     // await
    Type,      // type
    Interface, // interface
    Match,     // match
    Enum,      // enum
    Yield,     // yield
    Spawn,     // spawn
    Pub,       // pub
    Mut,       // mut

    // Literals (8)
    IntLiteral,       // e.g. 42, 0xFF, 1_000_000
    FloatLiteral,     // e.g. 3.14, 1.0e10
    StringLiteral,    // e.g. "hello"
    FStringStart,     // f"
    FStringText,      // f-string literal segment
    FStringExprStart, // {
    FStringExprEnd,   // }
    FStringEnd,       // "

    // Identifiers (1)
    Identifier,

    // Operators (19)
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    DoubleSlash,  // //
    Percent,      // %
    DoubleStar,   // **
    EqualEqual,   // ==
    BangEqual,    // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    Equal,        // =
    PlusEqual,    // +=
    MinusEqual,   // -=
    StarEqual,    // *=
    SlashEqual,   // /=
    PercentEqual, // %=

    // Ranges (2)
    DotDot,      // ..
    DotDotEqual, // ..=

    // Delimiters (6)
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Separators (4)
    Comma,     // ,
    Dot,       // .
    Colon,     // :
    Semicolon, // ;

    // Specials (2)
    Newline, // Significant for statement termination
    Eof,     // End of file

    // Error Token (1)
    Error,
}

/// A scanned token output from lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self { kind, lexeme, span }
    }
}

/// Precedence binding power constants for Pratt parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Or,         // or
    And,        // and
    Equality,   // == != is
    Comparison, // < > <= >=
    Range,      // .. ..=
    Term,       // + -
    Factor,     // * / // %
    Exponent,   // **
    Unary,      // - not
    Call,       // . () []
}
