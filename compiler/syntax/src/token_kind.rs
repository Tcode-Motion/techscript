//! Token kind definitions and classification helpers for TechScript 2.0.

use crate::precedence::{Associativity, Precedence};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Classification of literal values in TechScript 2.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiteralKind {
    /// Integer literal (e.g., `42`).
    Int,
    /// Floating-point literal (e.g., `3.14`).
    Float,
    /// String literal (e.g., `"hello"`).
    Str,
    /// Boolean literal (`true` or `false`).
    Bool,
    /// Null literal (`null` or `none`).
    Null,
}

impl fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            LiteralKind::Int => "Int",
            LiteralKind::Float => "Float",
            LiteralKind::Str => "Str",
            LiteralKind::Bool => "Bool",
            LiteralKind::Null => "Null",
        };
        write!(f, "{}", label)
    }
}

/// Representation formats for numeric literals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NumericLiteralKind {
    /// Base-10 standard float or integer notation (e.g., `42`, `3.14`).
    Decimal,
    /// Hexadecimal base-16 integer notation (e.g., `0xFF`).
    Hex,
    /// Binary base-2 integer notation (e.g., `0b1010`).
    Binary,
    /// Octal base-8 integer notation (e.g., `0o755`).
    Octal,
}

impl fmt::Display for NumericLiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            NumericLiteralKind::Decimal => "Decimal",
            NumericLiteralKind::Hex => "Hex",
            NumericLiteralKind::Binary => "Binary",
            NumericLiteralKind::Octal => "Octal",
        };
        write!(f, "{}", label)
    }
}

/// Complete enumeration of all TechScript 2.0 token kinds.
///
/// Contains canonical keywords, alias keywords, reserved future keywords,
/// literals, identifiers, operators, delimiters, and special tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenKind {
    // ── Canonical Keywords ──────────────────────────────────────────────────
    /// `make` variable declaration
    Make,
    /// `const` constant declaration
    Const,
    /// `say` statement
    Say,
    /// `ask` expression
    Ask,
    /// `build` function/method declaration
    Build,
    /// `return` statement
    Return,
    /// `model` class/type definition
    Model,
    /// `self` instance self-reference
    SelfKw,
    /// `new` instantiation operator
    New,
    /// `if` conditional keyword
    If,
    /// `elif` conditional keyword
    Elif,
    /// `else` conditional keyword
    Else,
    /// `for` loop keyword
    For,
    /// `in` membership and iteration keyword
    In,
    /// `while` loop keyword
    While,
    /// `repeat` loop keyword
    Repeat,
    /// `break` loop control statement
    Break,
    /// `continue` loop control statement
    Continue,
    /// `try` block keyword
    Try,
    /// `catch` handler keyword
    Catch,
    /// `throw` exception statement
    Throw,
    /// `import` module keyword
    Import,
    /// `from` module selection keyword
    From,
    /// `export` declaration keyword
    Export,
    /// `true` boolean literal
    True,
    /// `false` boolean literal
    False,
    /// `null` canonical null value literal
    Null,

    // ── Alias Keywords (Backward Compatibility) ─────────────────────────────
    /// `let` alias for `make`
    Let,
    /// `var` alias for `make`
    Var,
    /// `fun` alias for `build`
    Fun,
    /// `function` alias for `build`
    Function,
    /// `when` alias for `if`
    When,
    /// `attempt` alias for `try`
    Attempt,
    /// `none` alias for `null`
    None,
    /// `class` alias for `model`
    Class,

    // ── Future Reserved Keywords ────────────────────────────────────────────
    /// `async` keyword
    Async,
    /// `await` keyword
    Await,
    /// `type` keyword
    Type,
    /// `interface` keyword
    Interface,
    /// `match` keyword
    Match,
    /// `switch` keyword
    Switch,
    /// `case` keyword
    Case,
    /// `enum` keyword
    Enum,
    /// `struct` keyword
    Struct,
    /// `trait` keyword
    Trait,
    /// `yield` keyword
    Yield,
    /// `spawn` keyword
    Spawn,
    /// `pub` keyword
    Pub,
    /// `mut` keyword
    Mut,

    // ── Literals and Identifiers ────────────────────────────────────────────
    /// User defined name (e.g. variable name)
    Identifier,
    /// Integer number literal (e.g. `123`)
    IntLiteral,
    /// Floating-point number literal (e.g. `3.14`)
    FloatLiteral,
    /// Double-quoted string literal (e.g. `"hello"`)
    StringLiteral,
    /// Start of interpolated string (`f"`)
    FStringStart,
    /// Literal segment inside interpolated string
    FStringText,
    /// Opening bracket for interpolation (`{`)
    FStringExprStart,
    /// Closing bracket for interpolation (`}`)
    FStringExprEnd,
    /// End of interpolated string (`"`)
    FStringEnd,

    // ── Operators ───────────────────────────────────────────────────────────
    /// Plus operator (`+`)
    Plus,
    /// Minus operator (`-`)
    Minus,
    /// Star operator (`*`)
    Star,
    /// Slash operator (`/`)
    Slash,
    /// Double slash operator (`//`)
    DoubleSlash,
    /// Percent operator (`%`)
    Percent,
    /// Exponentiation operator (`**`)
    DoubleStar,
    /// Equal-equal operator (`==`)
    EqualEqual,
    /// Bang-equal operator (`!=`)
    BangEqual,
    /// Triple-equal operator (`===`)
    TripleEqual,
    /// Bang-equal-equal operator (`!==`)
    BangEqualEqual,
    /// Less than operator (`<`)
    Less,
    /// Greater than operator (`>`)
    Greater,
    /// Less-equal operator (`<=`)
    LessEqual,
    /// Greater-equal operator (`>=`)
    GreaterEqual,
    /// Logical AND operator (`&&` or `and`)
    And,
    /// Logical OR operator (`||` or `or`)
    Or,
    /// Logical NOT operator (`!` or `not`)
    Not,
    /// Identity check operator (`is`)
    Is,

    // ── Assignment Operators ────────────────────────────────────────────────
    /// Assignment operator (`=`)
    Equal,
    /// Add-assign operator (`+=`)
    PlusEqual,
    /// Subtract-assign operator (`-=`)
    MinusEqual,
    /// Multiply-assign operator (`*=`)
    StarEqual,
    /// Divide-assign operator (`/=`)
    SlashEqual,
    /// Modulo-assign operator (`%=`)
    PercentEqual,

    // ── Ranges and Navigation ───────────────────────────────────────────────
    /// Range exclusive (`..`)
    DotDot,
    /// Range inclusive (`..=`)
    DotDotEqual,
    /// Optional chaining (`?.`)
    QuestionDot,
    /// Null coalescing (`??`)
    QuestionQuestion,
    /// Function return type arrow (`->`)
    Arrow,

    // ── Delimiters and Separators ───────────────────────────────────────────
    /// Left parenthesis (`(`, `U+0028`)
    LeftParen,
    /// Right parenthesis (`)`, `U+0029`)
    RightParen,
    /// Left curly brace (`{`, `U+007B`)
    LeftBrace,
    /// Right curly brace (`}`, `U+007D`)
    RightBrace,
    /// Left square bracket (`[`, `U+005B`)
    LeftBracket,
    /// Right square bracket (`]`, `U+005D`)
    RightBracket,
    /// Comma separator (`,`)
    Comma,
    /// Dot separator (`.`)
    Dot,
    /// Colon separator (`:`)
    Colon,
    /// Semicolon separator (`;`)
    Semicolon,

    // ── Special and Control ─────────────────────────────────────────────────
    /// Significant statement boundary newline (`\n` or `\r\n`)
    Newline,
    /// End-of-file sentinel
    Eof,
    /// Malformed/unrecognized token
    Error,
}

impl TokenKind {
    /// Returns `true` if this token kind is any keyword.
    ///
    /// Matches canonical, alias, and future reserved keywords.
    pub fn is_keyword(&self) -> bool {
        self.is_canonical_keyword() || self.is_alias_keyword() || self.is_future_reserved_keyword()
    }

    /// Returns `true` if this token kind is a canonical keyword.
    pub fn is_canonical_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Make
                | TokenKind::Const
                | TokenKind::Say
                | TokenKind::Ask
                | TokenKind::Build
                | TokenKind::Return
                | TokenKind::Model
                | TokenKind::SelfKw
                | TokenKind::New
                | TokenKind::If
                | TokenKind::Elif
                | TokenKind::Else
                | TokenKind::For
                | TokenKind::In
                | TokenKind::While
                | TokenKind::Repeat
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Try
                | TokenKind::Catch
                | TokenKind::Throw
                | TokenKind::Import
                | TokenKind::From
                | TokenKind::Export
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Null
        )
    }

    /// Returns `true` if this token kind is a backward-compatibility alias keyword.
    pub fn is_alias_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Let
                | TokenKind::Var
                | TokenKind::Fun
                | TokenKind::Function
                | TokenKind::When
                | TokenKind::Attempt
                | TokenKind::None
                | TokenKind::Class
        )
    }

    /// Returns `true` if this token kind is a reserved future keyword.
    pub fn is_future_reserved_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Async
                | TokenKind::Await
                | TokenKind::Type
                | TokenKind::Interface
                | TokenKind::Match
                | TokenKind::Switch
                | TokenKind::Case
                | TokenKind::Enum
                | TokenKind::Struct
                | TokenKind::Trait
                | TokenKind::Yield
                | TokenKind::Spawn
                | TokenKind::Pub
                | TokenKind::Mut
        )
    }

    /// Returns the canonical equivalent of an alias keyword.
    ///
    /// Returns `None` if this is not an alias keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_syntax::TokenKind;
    ///
    /// assert_eq!(TokenKind::Let.to_canonical(), Some(TokenKind::Make));
    /// assert_eq!(TokenKind::Make.to_canonical(), None);
    /// ```
    pub fn to_canonical(&self) -> Option<TokenKind> {
        match self {
            TokenKind::Let | TokenKind::Var => Some(TokenKind::Make),
            TokenKind::Fun | TokenKind::Function => Some(TokenKind::Build),
            TokenKind::When => Some(TokenKind::If),
            TokenKind::Attempt => Some(TokenKind::Try),
            TokenKind::None => Some(TokenKind::Null),
            TokenKind::Class => Some(TokenKind::Model),
            _ => None,
        }
    }

    /// Returns the primitive `LiteralKind` if this token kind represents a literal.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_syntax::{TokenKind, LiteralKind};
    ///
    /// assert_eq!(TokenKind::IntLiteral.literal_kind(), Some(LiteralKind::Int));
    /// assert_eq!(TokenKind::True.literal_kind(), Some(LiteralKind::Bool));
    /// ```
    pub fn literal_kind(&self) -> Option<LiteralKind> {
        match self {
            TokenKind::IntLiteral => Some(LiteralKind::Int),
            TokenKind::FloatLiteral => Some(LiteralKind::Float),
            TokenKind::StringLiteral => Some(LiteralKind::Str),
            TokenKind::True | TokenKind::False => Some(LiteralKind::Bool),
            TokenKind::Null | TokenKind::None => Some(LiteralKind::Null),
            _ => None,
        }
    }

    /// Returns `true` if this token is any operator.
    pub fn is_operator(&self) -> bool {
        self.is_assignment_operator()
            || self.is_comparison_operator()
            || self.is_logical_operator()
            || matches!(
                self,
                TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Star
                    | TokenKind::Slash
                    | TokenKind::DoubleSlash
                    | TokenKind::Percent
                    | TokenKind::DoubleStar
                    | TokenKind::DotDot
                    | TokenKind::DotDotEqual
                    | TokenKind::QuestionDot
                    | TokenKind::QuestionQuestion
            )
    }

    /// Returns `true` if this is a variable assignment operator.
    pub fn is_assignment_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Equal
                | TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
        )
    }

    /// Returns `true` if this is a relational or equality comparison operator.
    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::EqualEqual
                | TokenKind::BangEqual
                | TokenKind::TripleEqual
                | TokenKind::BangEqualEqual
                | TokenKind::Less
                | TokenKind::Greater
                | TokenKind::LessEqual
                | TokenKind::GreaterEqual
                | TokenKind::Is
                | TokenKind::In
        )
    }

    /// Returns `true` if this is a logical operator.
    pub fn is_logical_operator(&self) -> bool {
        matches!(self, TokenKind::And | TokenKind::Or | TokenKind::Not)
    }

    /// Returns `true` if this operator can be used as a prefix unary operator.
    pub fn is_unary_operator(&self) -> bool {
        matches!(self, TokenKind::Minus | TokenKind::Plus | TokenKind::Not)
    }

    /// Returns `true` if this is a binary operator.
    pub fn is_binary_operator(&self) -> bool {
        self.is_operator() && !matches!(self, TokenKind::Not)
    }

    /// Returns the static string slice representation (lexeme) of this token kind.
    ///
    /// Returns `None` for dynamic kinds (e.g. `Identifier`, literals).
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_syntax::TokenKind;
    ///
    /// assert_eq!(TokenKind::Make.static_lexeme(), Some("make"));
    /// assert_eq!(TokenKind::Identifier.static_lexeme(), None);
    /// ```
    pub fn static_lexeme(&self) -> Option<&'static str> {
        match self {
            TokenKind::Make => Some("make"),
            TokenKind::Const => Some("const"),
            TokenKind::Say => Some("say"),
            TokenKind::Ask => Some("ask"),
            TokenKind::Build => Some("build"),
            TokenKind::Return => Some("return"),
            TokenKind::Model => Some("model"),
            TokenKind::SelfKw => Some("self"),
            TokenKind::New => Some("new"),
            TokenKind::If => Some("if"),
            TokenKind::Elif => Some("elif"),
            TokenKind::Else => Some("else"),
            TokenKind::For => Some("for"),
            TokenKind::In => Some("in"),
            TokenKind::While => Some("while"),
            TokenKind::Repeat => Some("repeat"),
            TokenKind::Break => Some("break"),
            TokenKind::Continue => Some("continue"),
            TokenKind::Try => Some("try"),
            TokenKind::Catch => Some("catch"),
            TokenKind::Throw => Some("throw"),
            TokenKind::Import => Some("import"),
            TokenKind::From => Some("from"),
            TokenKind::Export => Some("export"),
            TokenKind::True => Some("true"),
            TokenKind::False => Some("false"),
            TokenKind::Null => Some("null"),

            TokenKind::Let => Some("let"),
            TokenKind::Var => Some("var"),
            TokenKind::Fun => Some("fun"),
            TokenKind::Function => Some("function"),
            TokenKind::When => Some("when"),
            TokenKind::Attempt => Some("attempt"),
            TokenKind::None => Some("none"),
            TokenKind::Class => Some("class"),

            TokenKind::Async => Some("async"),
            TokenKind::Await => Some("await"),
            TokenKind::Type => Some("type"),
            TokenKind::Interface => Some("interface"),
            TokenKind::Match => Some("match"),
            TokenKind::Switch => Some("switch"),
            TokenKind::Case => Some("case"),
            TokenKind::Enum => Some("enum"),
            TokenKind::Struct => Some("struct"),
            TokenKind::Trait => Some("trait"),
            TokenKind::Yield => Some("yield"),
            TokenKind::Spawn => Some("spawn"),
            TokenKind::Pub => Some("pub"),
            TokenKind::Mut => Some("mut"),

            TokenKind::FStringStart => Some("f\""),
            TokenKind::FStringExprStart => Some("{"),
            TokenKind::FStringExprEnd => Some("}"),
            TokenKind::FStringEnd => Some("\""),

            TokenKind::Plus => Some("+"),
            TokenKind::Minus => Some("-"),
            TokenKind::Star => Some("*"),
            TokenKind::Slash => Some("/"),
            TokenKind::DoubleSlash => Some("//"),
            TokenKind::Percent => Some("%"),
            TokenKind::DoubleStar => Some("**"),
            TokenKind::EqualEqual => Some("=="),
            TokenKind::BangEqual => Some("!="),
            TokenKind::TripleEqual => Some("==="),
            TokenKind::BangEqualEqual => Some("!=="),
            TokenKind::Less => Some("<"),
            TokenKind::Greater => Some(">"),
            TokenKind::LessEqual => Some("<="),
            TokenKind::GreaterEqual => Some(">="),
            TokenKind::Is => Some("is"),

            TokenKind::Equal => Some("="),
            TokenKind::PlusEqual => Some("+="),
            TokenKind::MinusEqual => Some("-="),
            TokenKind::StarEqual => Some("*="),
            TokenKind::SlashEqual => Some("/="),
            TokenKind::PercentEqual => Some("%="),

            TokenKind::DotDot => Some(".."),
            TokenKind::DotDotEqual => Some("..="),
            TokenKind::QuestionDot => Some("?."),
            TokenKind::QuestionQuestion => Some("??"),
            TokenKind::Arrow => Some("->"),

            TokenKind::LeftParen => Some("("),
            TokenKind::RightParen => Some(")"),
            TokenKind::LeftBrace => Some("{"),
            TokenKind::RightBrace => Some("}"),
            TokenKind::LeftBracket => Some("["),
            TokenKind::RightBracket => Some("]"),
            TokenKind::Comma => Some(","),
            TokenKind::Dot => Some("."),
            TokenKind::Colon => Some(":"),
            TokenKind::Semicolon => Some(";"),
            _ => None,
        }
    }

    /// Returns the precedence level of this operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_syntax::{TokenKind, Precedence};
    ///
    /// assert_eq!(TokenKind::Plus.precedence(), Precedence::Term);
    /// assert_eq!(TokenKind::Identifier.precedence(), Precedence::None);
    /// ```
    pub fn precedence(&self) -> Precedence {
        match self {
            TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual => Precedence::Assignment,
            TokenKind::QuestionQuestion => Precedence::NullCoalescing,
            TokenKind::Or => Precedence::Or,
            TokenKind::And => Precedence::And,
            TokenKind::EqualEqual
            | TokenKind::BangEqual
            | TokenKind::TripleEqual
            | TokenKind::BangEqualEqual => Precedence::Equality,
            TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            | TokenKind::Is
            | TokenKind::In => Precedence::Comparison,
            TokenKind::DotDot | TokenKind::DotDotEqual => Precedence::Range,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash | TokenKind::DoubleSlash | TokenKind::Percent => {
                Precedence::Factor
            }
            TokenKind::DoubleStar => Precedence::Exponent,
            TokenKind::QuestionDot
            | TokenKind::Dot
            | TokenKind::LeftBracket
            | TokenKind::LeftParen => Precedence::Call,
            _ => Precedence::None,
        }
    }

    /// Returns the associativity rule of this operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_syntax::{TokenKind, Associativity};
    ///
    /// assert_eq!(TokenKind::Plus.associativity(), Associativity::Left);
    /// assert_eq!(TokenKind::DoubleStar.associativity(), Associativity::Right);
    /// ```
    pub fn associativity(&self) -> Associativity {
        match self {
            TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual => Associativity::Right,
            TokenKind::QuestionQuestion => Associativity::Right,
            TokenKind::DoubleStar => Associativity::Right,
            TokenKind::DotDot | TokenKind::DotDotEqual => Associativity::None,
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::DoubleSlash
            | TokenKind::Percent
            | TokenKind::Or
            | TokenKind::And
            | TokenKind::EqualEqual
            | TokenKind::BangEqual
            | TokenKind::TripleEqual
            | TokenKind::BangEqualEqual
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            | TokenKind::Is
            | TokenKind::In
            | TokenKind::QuestionDot
            | TokenKind::Dot
            | TokenKind::LeftBracket
            | TokenKind::LeftParen => Associativity::Left,
            _ => Associativity::None,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(lexeme) = self.static_lexeme() {
            write!(f, "'{}'", lexeme)
        } else {
            let label = match self {
                TokenKind::Identifier => "Identifier",
                TokenKind::IntLiteral => "Integer Literal",
                TokenKind::FloatLiteral => "Float Literal",
                TokenKind::StringLiteral => "String Literal",
                TokenKind::FStringText => "f-string segment",
                TokenKind::Newline => "Newline",
                TokenKind::Eof => "EOF",
                TokenKind::Error => "Error",
                _ => "Unknown Token",
            };
            write!(f, "{}", label)
        }
    }
}

/// Look up a lexeme string to check if it matches a keyword.
///
/// Returns the matching `TokenKind` if it is a canonical, alias, or
/// reserved future keyword. Returns `None` otherwise.
///
/// Implemented entirely using compile-time static branch tables.
///
/// # Examples
///
/// ```
/// use techscript_syntax::{lookup_keyword, TokenKind};
///
/// assert_eq!(lookup_keyword("make"), Some(TokenKind::Make));
/// assert_eq!(lookup_keyword("let"), Some(TokenKind::Let));
/// assert_eq!(lookup_keyword("not_a_keyword"), None);
/// ```
pub fn lookup_keyword(lexeme: &str) -> Option<TokenKind> {
    match lexeme {
        // Canonical Active Keywords
        "make" => Some(TokenKind::Make),
        "const" => Some(TokenKind::Const),
        "say" => Some(TokenKind::Say),
        "ask" => Some(TokenKind::Ask),
        "build" => Some(TokenKind::Build),
        "return" => Some(TokenKind::Return),
        "model" => Some(TokenKind::Model),
        "self" => Some(TokenKind::SelfKw),
        "new" => Some(TokenKind::New),
        "if" => Some(TokenKind::If),
        "elif" => Some(TokenKind::Elif),
        "else" => Some(TokenKind::Else),
        "for" => Some(TokenKind::For),
        "in" => Some(TokenKind::In),
        "while" => Some(TokenKind::While),
        "repeat" => Some(TokenKind::Repeat),
        "break" => Some(TokenKind::Break),
        "continue" => Some(TokenKind::Continue),
        "try" => Some(TokenKind::Try),
        "catch" => Some(TokenKind::Catch),
        "throw" => Some(TokenKind::Throw),
        "import" => Some(TokenKind::Import),
        "from" => Some(TokenKind::From),
        "export" => Some(TokenKind::Export),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        "null" => Some(TokenKind::Null),

        // Alias Keywords
        "let" => Some(TokenKind::Let),
        "var" => Some(TokenKind::Var),
        "fun" => Some(TokenKind::Fun),
        "function" => Some(TokenKind::Function),
        "when" => Some(TokenKind::When),
        "attempt" => Some(TokenKind::Attempt),
        "none" => Some(TokenKind::None),
        "class" => Some(TokenKind::Class),

        // Future Reserved Keywords
        "async" => Some(TokenKind::Async),
        "await" => Some(TokenKind::Await),
        "type" => Some(TokenKind::Type),
        "interface" => Some(TokenKind::Interface),
        "match" => Some(TokenKind::Match),
        "switch" => Some(TokenKind::Switch),
        "case" => Some(TokenKind::Case),
        "enum" => Some(TokenKind::Enum),
        "struct" => Some(TokenKind::Struct),
        "trait" => Some(TokenKind::Trait),
        "yield" => Some(TokenKind::Yield),
        "spawn" => Some(TokenKind::Spawn),
        "pub" => Some(TokenKind::Pub),
        "mut" => Some(TokenKind::Mut),

        // Word operator aliases (logical)
        "and" => Some(TokenKind::And),
        "or" => Some(TokenKind::Or),
        "not" => Some(TokenKind::Not),

        _ => None,
    }
}

/// Identifies the numeric base format of a string slice representing a number.
///
/// Supported formats include:
/// - Hexadecimal: starts with `0x` or `0X`
/// - Binary: starts with `0b` or `0B`
/// - Octal: starts with `0o` or `0O`
/// - Decimal: default base-10 format
///
/// # Examples
///
/// ```
/// use techscript_syntax::{numeric_literal_kind, NumericLiteralKind};
///
/// assert_eq!(numeric_literal_kind("0xFF"), Some(NumericLiteralKind::Hex));
/// assert_eq!(numeric_literal_kind("123"), Some(NumericLiteralKind::Decimal));
/// assert_eq!(numeric_literal_kind("not_a_number"), None);
/// ```
pub fn numeric_literal_kind(lexeme: &str) -> Option<NumericLiteralKind> {
    if lexeme.is_empty() {
        return None;
    }

    let first = lexeme.chars().next()?;
    if !first.is_ascii_digit() {
        return None;
    }

    if lexeme.starts_with("0x") || lexeme.starts_with("0X") {
        return Some(NumericLiteralKind::Hex);
    }
    if lexeme.starts_with("0b") || lexeme.starts_with("0B") {
        return Some(NumericLiteralKind::Binary);
    }
    if lexeme.starts_with("0o") || lexeme.starts_with("0O") {
        return Some(NumericLiteralKind::Octal);
    }

    // Decimal verification: must contain only digits, dots, exponents, or underscores
    let mut has_exponent = false;
    let mut has_dot = false;
    for (i, c) in lexeme.chars().enumerate() {
        if c == '_' {
            continue;
        }
        if c == '.' {
            if has_dot || has_exponent {
                return None;
            }
            has_dot = true;
            continue;
        }
        if c == 'e' || c == 'E' {
            if has_exponent {
                return None;
            }
            has_exponent = true;
            continue;
        }
        if c == '+' || c == '-' {
            // Unary prefix on exponent: e.g. 1e-5
            if i > 0 {
                let prev = lexeme.chars().nth(i - 1)?;
                if prev == 'e' || prev == 'E' {
                    continue;
                }
            }
            return None;
        }
        if !c.is_ascii_digit() {
            return None;
        }
    }

    Some(NumericLiteralKind::Decimal)
}
