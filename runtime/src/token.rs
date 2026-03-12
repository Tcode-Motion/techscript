// ── TechScript Token Types ───────────────────────────────────────────
// Direct port of tokens.py — all token categories and keywords.

use std::fmt;

/// Every token produced by the lexer is one of these variants.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    NumberInt,
    NumberFloat,
    String,
    FString,
    BoolTrue,
    BoolFalse,
    None,

    // Identifiers & keywords
    Identifier,
    Keyword,

    // Arithmetic
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    DoubleSlash,   // //
    Percent,       // %
    Power,         // **

    // Assignment
    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=

    // Comparison
    Equal,         // ==
    NotEqual,      // !=
    Less,          // <
    Greater,       // >
    LessEqual,     // <=
    GreaterEqual,  // >=

    // Special operators
    Arrow,         // =>
    Pipe,          // |>
    Question,      // ?
    Dot,           // .
    DotDot,        // ..
    DotDotEqual,   // ..=
    Spread,        // ...
    At,            // @
    Hash,          // #
    Nullish,       // ??
    OptionalChain, // ?.

    // Delimiters
    LParen,        // (
    RParen,        // )
    LBracket,      // [
    RBracket,      // ]
    LBrace,        // {
    RBrace,        // }
    Comma,         // ,
    Colon,         // :

    // Structural
    Newline,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A single token produced by the lexer.
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: impl Into<String>, line: usize, column: usize) -> Self {
        Token { token_type, value: value.into(), line, column }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token({}, {:?}, L{}:{})", self.token_type, self.value, self.line, self.column)
    }
}

/// All 52 reserved keywords in TechScript.
pub fn is_keyword(word: &str) -> bool {
    matches!(word,
        // I/O
        "say" | "ask" |
        // Variables
        "make" | "keep" | "mut" | "drop" | "global" |
        // Functions & classes
        "build" | "send" | "model" | "self" | "base" | "new" |
        // Control flow
        "when" | "alt" | "else" | "each" | "repeat" | "in" |
        "unless" | "until" | "match" | "case" |
        "stop" | "skip" | "pass" |
        // Error handling
        "attempt" | "rescue" | "fail" | "always" |
        // Modules
        "use" | "take" | "share" | "as" |
        // Scope / misc
        "do" | "end" | "with" | "defer" | "guard" |
        // Literals
        "true" | "false" | "none" |
        // Logical
        "and" | "or" | "not" |
        // Type / identity
        "is" | "has" | "typeof" |
        // Async
        "async" | "await" | "yield"
    )
}
