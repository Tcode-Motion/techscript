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
        fmt::Debug::fmt(self, f)
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
        fmt::Debug::fmt(self, f)
    }
}

/// Complete enumeration of all TechScript 2.0 token kinds.
///
/// Keywords are grouped into three tiers:
/// - **Canonical 2.0**: The one true spelling — no warnings emitted.
/// - **Deprecated Alias**: Old spelling — still parsed, but emits a `TSW1xxx` warning.
/// - **Reserved**: Recognised by the lexer but not yet active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenKind {
    // ── Canonical 2.0 Keywords ──────────────────────────────────────────────────────
    /// `do` — function/method declaration (replaces `build`, `fun`, `function`)
    Do,
    /// `send` — return value from function (replaces `return`, `give`)
    Send,
    /// `when` — conditional branch (replaces `if`)
    When,
    /// `loop` — counted loop: `loop N` executes exactly N times
    Loop,
    /// `repeat` — while-style loop: `repeat condition` (replaces `while`)
    Repeat,
    /// `for` — for-each iteration: `for x in y` (replaces `each`)
    For,
    /// `in` — membership and iteration boundary
    In,
    /// `match` — pattern match statement (replaces `switch`)
    Match,
    /// `case` — match arm label
    Case,
    /// `default` — default match arm
    Default,
    /// `try` — error-handling block (replaces `attempt`)
    Try,
    /// `catch` — error handler block
    Catch,
    /// `throw` — raise an error
    Throw,
    /// `use` — module import (replaces `import`, `from`)
    Use,
    /// `class` — class/type definition (replaces `model`)
    Class,
    /// `struct` — struct definition
    Struct,
    /// `enum` — enum definition
    Enum,
    /// `trait` — trait definition
    Trait,
    /// `interface` — interface definition
    Interface,
    /// `const` — constant declaration (replaces `keep`)
    Const,
    /// `null` — canonical null literal (replaces `none`)
    Null,
    /// `say` — print to stdout, implicit call: `say "hello"`
    Say,
    /// `ask` — read from stdin, implicit call: `ask "prompt"`
    Ask,
    /// `break` — exit loop early
    Break,
    /// `continue` — skip to next iteration
    Continue,
    /// `else` — conditional else branch
    Else,
    /// `async` — async function or block
    Async,
    /// `await` — await an async expression
    Await,
    /// `parallel` — parallel execution block
    Parallel,
    /// `end` — block terminator (closes `do`/`when`/`loop`/`repeat`/`for`/`try`/`class`/`struct` blocks)
    End,
    /// `export` — declaration export keyword
    Export,
    /// `new` — object instantiation
    New,
    /// `self` — instance self-reference
    SelfKw,
    /// `true` — boolean true literal
    True,
    /// `false` — boolean false literal
    False,
    /// `typeof` — type evaluation operator
    Typeof,
    /// `with` — supplemental block keyword
    With,

    // ── Deprecated / Alias Keywords (all emit TSW1xxx warnings) ────────────────
    /// `build` → `do` (deprecated TSW1002)
    Build,
    /// `make` → plain assignment (deprecated TSW1001)
    Make,
    /// `return` → `send` (deprecated TSW1003)
    Return,
    /// `model` → `class` (deprecated TSW1013)
    Model,
    /// `if` → `when` (deprecated TSW1007)
    If,
    /// `elif` → `else when` (deprecated TSW1007)
    Elif,
    /// `while` → `repeat` (deprecated TSW1008)
    While,
    /// `import` → `use` (deprecated TSW1009)
    Import,
    /// `from` → `use` (deprecated TSW1009)
    From,
    /// `let` → plain assignment (deprecated TSW1001)
    Let,
    /// `var` → plain assignment (deprecated TSW1001)
    Var,
    /// `fun` → `do` (deprecated TSW1002)
    Fun,
    /// `function` → `do` (deprecated TSW1002)
    Function,
    /// `attempt` → `try` (deprecated TSW1004)
    Attempt,
    /// `none` → `null` (deprecated TSW1011)
    None,
    /// `keep` → `const` (deprecated)
    Keep,
    /// `give` → `send` (deprecated TSW1005)
    Give,
    /// `stop` → `break` (deprecated)
    Stop,
    /// `skip` → `continue` (deprecated)
    Skip,
    /// `each` → `for` (deprecated TSW1010)
    Each,
    /// `switch` → `match` (deprecated)
    Switch,
    /// `be` alias for assignment `=` (deprecated)
    Be,
    /// `equals` alias for comparison `==` (deprecated)
    Equals,
    /// `then` block-open delimiter (deprecated TSW1006)
    Then,

    // ── Reserved / Meta Keywords ─────────────────────────────────────────────────
    /// `type` reserved for future type alias syntax
    Type,
    /// `yield` reserved for generator syntax
    Yield,
    /// `spawn` reserved for concurrency primitives
    Spawn,
    /// `pub` reserved for visibility modifiers
    Pub,
    /// `mut` reserved for explicit mutability annotations
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

    /// Returns `true` if this token kind is a canonical 2.0 keyword.
    ///
    /// Canonical keywords are the one-true spelling for TechScript 2.0.
    /// They never emit deprecation warnings.
    pub fn is_canonical_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Do
                | TokenKind::Send
                | TokenKind::When
                | TokenKind::Loop
                | TokenKind::Repeat
                | TokenKind::For
                | TokenKind::In
                | TokenKind::Match
                | TokenKind::Case
                | TokenKind::Default
                | TokenKind::Try
                | TokenKind::Catch
                | TokenKind::Throw
                | TokenKind::Use
                | TokenKind::Class
                | TokenKind::Struct
                | TokenKind::Enum
                | TokenKind::Trait
                | TokenKind::Interface
                | TokenKind::Const
                | TokenKind::Null
                | TokenKind::Say
                | TokenKind::Ask
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Else
                | TokenKind::Async
                | TokenKind::Await
                | TokenKind::Parallel
                | TokenKind::End
                | TokenKind::Export
                | TokenKind::New
                | TokenKind::SelfKw
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Typeof
                | TokenKind::With
        )
    }

    /// Returns `true` if this token kind is a deprecated alias keyword.
    ///
    /// Deprecated keywords are still parsed for backward compatibility but
    /// always emit a `TSW1xxx` deprecation diagnostic.
    pub fn is_alias_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Build
                | TokenKind::Make
                | TokenKind::Return
                | TokenKind::Model
                | TokenKind::If
                | TokenKind::Elif
                | TokenKind::While
                | TokenKind::Import
                | TokenKind::From
                | TokenKind::Let
                | TokenKind::Var
                | TokenKind::Fun
                | TokenKind::Function
                | TokenKind::Attempt
                | TokenKind::None
                | TokenKind::Keep
                | TokenKind::Give
                | TokenKind::Stop
                | TokenKind::Skip
                | TokenKind::Each
                | TokenKind::Switch
                | TokenKind::Be
                | TokenKind::Equals
                | TokenKind::Then
        )
    }

    /// Returns `true` if this token kind is a reserved keyword (not yet active in any production).
    ///
    /// Note: `Async`, `Await`, `Match`, `Case`, `Interface`, `Struct`, `Enum`, `Trait`
    /// are now **canonical 2.0** keywords and are no longer in this reserved set.
    pub fn is_future_reserved_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Type | TokenKind::Yield | TokenKind::Spawn | TokenKind::Pub | TokenKind::Mut
        )
    }

    /// Returns the canonical 2.0 equivalent of a deprecated alias keyword.
    ///
    /// Returns `None` if this token is already canonical, reserved, or has no
    /// single-token canonical replacement (e.g. `make`/`let`/`var` become plain
    /// assignment — no keyword replacement).
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_syntax::TokenKind;
    ///
    /// assert_eq!(TokenKind::Build.to_canonical(), Some(TokenKind::Do));
    /// assert_eq!(TokenKind::Return.to_canonical(), Some(TokenKind::Send));
    /// assert_eq!(TokenKind::Make.to_canonical(), None); // becomes plain assignment
    /// assert_eq!(TokenKind::Do.to_canonical(), None);   // already canonical
    /// ```
    pub fn to_canonical(&self) -> Option<TokenKind> {
        match self {
            // Function declaration aliases
            TokenKind::Build | TokenKind::Fun | TokenKind::Function => Some(TokenKind::Do),
            // Return aliases
            TokenKind::Return | TokenKind::Give => Some(TokenKind::Send),
            // Conditional aliases
            TokenKind::If | TokenKind::Elif => Some(TokenKind::When),
            // Loop aliases
            TokenKind::While => Some(TokenKind::Repeat),
            // Error handling aliases
            TokenKind::Attempt => Some(TokenKind::Try),
            // Module aliases
            TokenKind::Import | TokenKind::From => Some(TokenKind::Use),
            // Type definition aliases
            TokenKind::Model => Some(TokenKind::Class),
            // Constant aliases
            TokenKind::Keep => Some(TokenKind::Const),
            // Null alias
            TokenKind::None => Some(TokenKind::Null),
            // Loop control aliases
            TokenKind::Stop => Some(TokenKind::Break),
            TokenKind::Skip => Some(TokenKind::Continue),
            // Iteration alias
            TokenKind::Each => Some(TokenKind::For),
            // Match alias
            TokenKind::Switch => Some(TokenKind::Match),
            // Operator aliases
            TokenKind::Be => Some(TokenKind::Equal),
            TokenKind::Equals => Some(TokenKind::EqualEqual),
            // Variable declaration aliases: Make/Let/Var become plain assignment — no token replacement
            TokenKind::Make | TokenKind::Let | TokenKind::Var => None,
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
                | TokenKind::Equals
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
            TokenKind::Keep => Some("keep"),
            TokenKind::Give => Some("give"),
            TokenKind::Stop => Some("stop"),
            TokenKind::Skip => Some("skip"),
            TokenKind::Each => Some("each"),
            TokenKind::Be => Some("be"),
            TokenKind::Equals => Some("equals"),
            TokenKind::Then => Some("then"),
            TokenKind::End => Some("end"),
            TokenKind::With => Some("with"),
            TokenKind::Typeof => Some("typeof"),
            TokenKind::Use => Some("use"),
            TokenKind::Do => Some("do"),
            TokenKind::Send => Some("send"),
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

            TokenKind::FStringStart => Some("$\""), // canonical; `f"` is a deprecated alias
            TokenKind::FStringExprStart => Some("{"),
            TokenKind::FStringExprEnd => Some("}"),
            TokenKind::FStringEnd => Some("\""),
            TokenKind::Loop => Some("loop"),
            TokenKind::Parallel => Some("parallel"),
            TokenKind::Default => Some("default"),

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
            | TokenKind::BangEqualEqual
            | TokenKind::Equals => Precedence::Equality,
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
            | TokenKind::Equals
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
            fmt::Debug::fmt(self, f)
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
        // ── Canonical 2.0 Keywords ───────────────────────────────────────────────
        "do" => Some(TokenKind::Do),
        "send" => Some(TokenKind::Send),
        "when" => Some(TokenKind::When),
        "loop" => Some(TokenKind::Loop),
        "repeat" => Some(TokenKind::Repeat),
        "for" => Some(TokenKind::For),
        "in" => Some(TokenKind::In),
        "match" => Some(TokenKind::Match),
        "case" => Some(TokenKind::Case),
        "default" => Some(TokenKind::Default),
        "try" => Some(TokenKind::Try),
        "catch" => Some(TokenKind::Catch),
        "throw" => Some(TokenKind::Throw),
        "use" => Some(TokenKind::Use),
        "class" => Some(TokenKind::Class),
        "struct" => Some(TokenKind::Struct),
        "enum" => Some(TokenKind::Enum),
        "trait" => Some(TokenKind::Trait),
        "interface" => Some(TokenKind::Interface),
        "const" => Some(TokenKind::Const),
        "null" => Some(TokenKind::Null),
        "say" => Some(TokenKind::Say),
        "ask" => Some(TokenKind::Ask),
        "break" => Some(TokenKind::Break),
        "continue" => Some(TokenKind::Continue),
        "else" => Some(TokenKind::Else),
        "async" => Some(TokenKind::Async),
        "await" => Some(TokenKind::Await),
        "parallel" => Some(TokenKind::Parallel),
        "end" => Some(TokenKind::End),
        "export" => Some(TokenKind::Export),
        "new" => Some(TokenKind::New),
        "self" => Some(TokenKind::SelfKw),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        "typeof" => Some(TokenKind::Typeof),
        "with" => Some(TokenKind::With),

        // ── Deprecated / Alias Keywords (TSW1xxx) ───────────────────────────────
        "build" => Some(TokenKind::Build),       // TSW1002 → do
        "make" => Some(TokenKind::Make),         // TSW1001 → plain assignment
        "return" => Some(TokenKind::Return),     // TSW1003 → send
        "model" => Some(TokenKind::Model),       // TSW1013 → class
        "if" => Some(TokenKind::If),             // TSW1007 → when
        "elif" => Some(TokenKind::Elif),         // TSW1007 → else when
        "while" => Some(TokenKind::While),       // TSW1008 → repeat
        "import" => Some(TokenKind::Import),     // TSW1009 → use
        "from" => Some(TokenKind::From),         // TSW1009 → use
        "let" => Some(TokenKind::Let),           // TSW1001 → plain assignment
        "var" => Some(TokenKind::Var),           // TSW1001 → plain assignment
        "fun" => Some(TokenKind::Fun),           // TSW1002 → do
        "function" => Some(TokenKind::Function), // TSW1002 → do
        "attempt" => Some(TokenKind::Attempt),   // TSW1004 → try
        "none" => Some(TokenKind::None),         // TSW1011 → null
        "keep" => Some(TokenKind::Keep),         // → const
        "give" => Some(TokenKind::Give),         // TSW1005 → send
        "stop" => Some(TokenKind::Stop),         // → break
        "skip" => Some(TokenKind::Skip),         // → continue
        "each" => Some(TokenKind::Each),         // TSW1010 → for
        "switch" => Some(TokenKind::Switch),     // → match
        "be" => Some(TokenKind::Be),
        "equals" => Some(TokenKind::Equals),
        "then" => Some(TokenKind::Then), // TSW1006 → (removed)

        // ── Reserved / Meta Keywords ───────────────────────────────────────────────
        "type" => Some(TokenKind::Type),
        "yield" => Some(TokenKind::Yield),
        "spawn" => Some(TokenKind::Spawn),
        "pub" => Some(TokenKind::Pub),
        "mut" => Some(TokenKind::Mut),

        // ── Word Operator Aliases (logical) ───────────────────────────────────────
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
