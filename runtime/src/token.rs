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

/// All reserved keywords in TechScript (canonical + legacy aliases).
pub fn is_keyword(word: &str) -> bool {
    matches!(word,
        // I/O
        "say" | "ask" |
        // Variables
        "make" | "keep" | "const" | "mut" | "drop" | "global" | "state" |
        // Functions & classes
        "build" | "do" | "send" | "return" | "model" | "class" | "self" | "base" | "new" |
        // Control flow
        "when" | "alt" | "else" | "each" | "repeat" | "loop" | "while" | "in" |
        "unless" | "until" | "match" | "case" |
        "stop" | "break" | "skip" | "continue" | "pass" |
        // Error handling
        "attempt" | "try" | "rescue" | "catch" | "fail" | "throw" | "always" | "finally" |
        // Modules
        "use" | "take" | "share" | "as" |
        // Web framework
        "component" | "page" | "api" | "route" | "render" |
        // GUI / 3D / anime
        "window" | "button" | "input" | "label" | "placeholder" | "scene" | "camera" | "light" | "mesh" | "timeline" |
        "move" | "fade" | "over" | "ease" | "to" | "color" | "pos" |
        // Scope / misc
        "end" | "with" | "defer" | "guard" | "run" |
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
