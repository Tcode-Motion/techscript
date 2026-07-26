//! Exhaustive unit tests for the `techscript_syntax` crate.
//!
//! Verifies keyword lookups, canonical conversions, operator precedence,
//! associativity, literal categorization, token construction, and serialization.

use std::collections::HashSet;
use techscript_common::Span;
use techscript_syntax::{
    lookup_keyword, numeric_literal_kind, Associativity, LiteralKind, NumericLiteralKind,
    Precedence, Token, TokenKind,
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Precedence & Associativity Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_precedence_ordering() {
    assert!(Precedence::Call > Precedence::Unary);
    assert!(Precedence::Unary > Precedence::Exponent);
    assert!(Precedence::Exponent > Precedence::Factor);
    assert!(Precedence::Factor > Precedence::Term);
    assert!(Precedence::Term > Precedence::Shift);
    assert!(Precedence::Shift > Precedence::BitwiseAnd);
    assert!(Precedence::BitwiseAnd > Precedence::BitwiseXor);
    assert!(Precedence::BitwiseXor > Precedence::BitwiseOr);
    assert!(Precedence::BitwiseOr > Precedence::Comparison);
    assert!(Precedence::Comparison > Precedence::Equality);
    assert!(Precedence::Equality > Precedence::And);
    assert!(Precedence::And > Precedence::Or);
    assert!(Precedence::Or > Precedence::NullCoalescing);
    assert!(Precedence::NullCoalescing > Precedence::Assignment);
    assert!(Precedence::Assignment > Precedence::None);
}

#[test]
fn test_precedence_display() {
    assert_eq!(format!("{}", Precedence::Call), "Call");
    assert_eq!(format!("{}", Precedence::None), "None");
}

#[test]
fn test_associativity_display() {
    assert_eq!(format!("{}", Associativity::Left), "Left");
    assert_eq!(format!("{}", Associativity::Right), "Right");
    assert_eq!(format!("{}", Associativity::None), "None");
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Keyword Lookup & Conversion Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_canonical_keywords_lookup() {
    let canonicals = [
        ("do", TokenKind::Do),
        ("send", TokenKind::Send),
        ("when", TokenKind::When),
        ("loop", TokenKind::Loop),
        ("repeat", TokenKind::Repeat),
        ("for", TokenKind::For),
        ("in", TokenKind::In),
        ("match", TokenKind::Match),
        ("case", TokenKind::Case),
        ("default", TokenKind::Default),
        ("try", TokenKind::Try),
        ("catch", TokenKind::Catch),
        ("throw", TokenKind::Throw),
        ("use", TokenKind::Use),
        ("class", TokenKind::Class),
        ("struct", TokenKind::Struct),
        ("enum", TokenKind::Enum),
        ("trait", TokenKind::Trait),
        ("interface", TokenKind::Interface),
        ("const", TokenKind::Const),
        ("null", TokenKind::Null),
        ("say", TokenKind::Say),
        ("ask", TokenKind::Ask),
        ("break", TokenKind::Break),
        ("continue", TokenKind::Continue),
        ("else", TokenKind::Else),
        ("async", TokenKind::Async),
        ("await", TokenKind::Await),
        ("parallel", TokenKind::Parallel),
        ("end", TokenKind::End),
        ("export", TokenKind::Export),
        ("new", TokenKind::New),
        ("self", TokenKind::SelfKw),
        ("true", TokenKind::True),
        ("false", TokenKind::False),
        ("typeof", TokenKind::Typeof),
        ("with", TokenKind::With),
    ];

    for (lexeme, expected_kind) in canonicals {
        assert_eq!(lookup_keyword(lexeme), Some(expected_kind));
        assert!(expected_kind.is_keyword());
        assert!(expected_kind.is_canonical_keyword());
        assert!(!expected_kind.is_alias_keyword());
        assert!(!expected_kind.is_future_reserved_keyword());
        assert_eq!(expected_kind.static_lexeme(), Some(lexeme));
    }
}

#[test]
fn test_alias_keywords_lookup_and_conversion() {
    let aliases = [
        ("build", TokenKind::Build, TokenKind::Do),
        ("fun", TokenKind::Fun, TokenKind::Do),
        ("function", TokenKind::Function, TokenKind::Do),
        ("return", TokenKind::Return, TokenKind::Send),
        ("give", TokenKind::Give, TokenKind::Send),
        ("if", TokenKind::If, TokenKind::When),
        ("while", TokenKind::While, TokenKind::Repeat),
        ("attempt", TokenKind::Attempt, TokenKind::Try),
        ("none", TokenKind::None, TokenKind::Null),
        ("model", TokenKind::Model, TokenKind::Class),
        ("keep", TokenKind::Keep, TokenKind::Const),
        ("stop", TokenKind::Stop, TokenKind::Break),
        ("skip", TokenKind::Skip, TokenKind::Continue),
        ("each", TokenKind::Each, TokenKind::For),
        ("switch", TokenKind::Switch, TokenKind::Match),
    ];

    for (lexeme, alias_kind, canonical_kind) in aliases {
        assert_eq!(lookup_keyword(lexeme), Some(alias_kind));
        assert!(alias_kind.is_keyword());
        assert!(alias_kind.is_alias_keyword());
        assert!(!alias_kind.is_canonical_keyword());
        assert!(!alias_kind.is_future_reserved_keyword());
        assert_eq!(alias_kind.static_lexeme(), Some(lexeme));
        assert_eq!(alias_kind.to_canonical(), Some(canonical_kind));
    }
}

#[test]
fn test_future_reserved_keywords_lookup() {
    let reserved = [
        ("type", TokenKind::Type),
        ("yield", TokenKind::Yield),
        ("spawn", TokenKind::Spawn),
        ("pub", TokenKind::Pub),
        ("mut", TokenKind::Mut),
    ];

    for (lexeme, expected_kind) in reserved {
        assert_eq!(lookup_keyword(lexeme), Some(expected_kind));
        assert!(expected_kind.is_keyword());
        assert!(expected_kind.is_future_reserved_keyword());
        assert!(!expected_kind.is_canonical_keyword());
        assert!(!expected_kind.is_alias_keyword());
        assert_eq!(expected_kind.static_lexeme(), Some(lexeme));
    }
}

#[test]
fn test_word_logical_operators_lookup() {
    assert_eq!(lookup_keyword("and"), Some(TokenKind::And));
    assert_eq!(lookup_keyword("or"), Some(TokenKind::Or));
    assert_eq!(lookup_keyword("not"), Some(TokenKind::Not));
}

#[test]
fn test_non_keyword_lookup() {
    assert_eq!(lookup_keyword("hello"), None);
    assert_eq!(lookup_keyword(""), None);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Operator Classification & Pratt Properties Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_operator_categories() {
    // Arithmetic
    let arithmetic = [
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::DoubleSlash,
        TokenKind::Percent,
        TokenKind::DoubleStar,
    ];
    for op in arithmetic {
        assert!(op.is_operator());
        assert!(!op.is_assignment_operator());
        assert!(!op.is_comparison_operator());
        assert!(!op.is_logical_operator());
    }

    // Assignment
    let assignments = [
        TokenKind::Equal,
        TokenKind::PlusEqual,
        TokenKind::MinusEqual,
        TokenKind::StarEqual,
        TokenKind::SlashEqual,
        TokenKind::PercentEqual,
    ];
    for op in assignments {
        assert!(op.is_operator());
        assert!(op.is_assignment_operator());
        assert!(!op.is_comparison_operator());
        assert!(!op.is_logical_operator());
        assert_eq!(op.precedence(), Precedence::Assignment);
        assert_eq!(op.associativity(), Associativity::Right);
    }

    // Comparisons
    let comparisons = [
        TokenKind::EqualEqual,
        TokenKind::BangEqual,
        TokenKind::TripleEqual,
        TokenKind::BangEqualEqual,
        TokenKind::Less,
        TokenKind::Greater,
        TokenKind::LessEqual,
        TokenKind::GreaterEqual,
        TokenKind::Is,
        TokenKind::In,
    ];
    for op in comparisons {
        assert!(op.is_operator());
        assert!(!op.is_assignment_operator());
        assert!(op.is_comparison_operator());
        assert!(!op.is_logical_operator());
    }
}

#[test]
fn test_unary_vs_binary() {
    assert!(TokenKind::Minus.is_unary_operator());
    assert!(TokenKind::Plus.is_unary_operator());
    assert!(TokenKind::Not.is_unary_operator());
    assert!(!TokenKind::Star.is_unary_operator());

    assert!(TokenKind::Plus.is_binary_operator());
    assert!(TokenKind::Minus.is_binary_operator());
    assert!(TokenKind::Star.is_binary_operator());
    assert!(!TokenKind::Not.is_binary_operator());
}

#[test]
fn test_precedence_and_associativity_mappings() {
    assert_eq!(TokenKind::Plus.precedence(), Precedence::Term);
    assert_eq!(TokenKind::Plus.associativity(), Associativity::Left);

    assert_eq!(TokenKind::Star.precedence(), Precedence::Factor);
    assert_eq!(TokenKind::Star.associativity(), Associativity::Left);

    assert_eq!(TokenKind::DoubleStar.precedence(), Precedence::Exponent);
    assert_eq!(TokenKind::DoubleStar.associativity(), Associativity::Right);

    assert_eq!(
        TokenKind::QuestionQuestion.precedence(),
        Precedence::NullCoalescing
    );
    assert_eq!(
        TokenKind::QuestionQuestion.associativity(),
        Associativity::Right
    );

    assert_eq!(TokenKind::DotDot.precedence(), Precedence::Range);
    assert_eq!(TokenKind::DotDot.associativity(), Associativity::None);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Literal Categorization Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_literal_kind_resolution() {
    assert_eq!(TokenKind::IntLiteral.literal_kind(), Some(LiteralKind::Int));
    assert_eq!(
        TokenKind::FloatLiteral.literal_kind(),
        Some(LiteralKind::Float)
    );
    assert_eq!(
        TokenKind::StringLiteral.literal_kind(),
        Some(LiteralKind::Str)
    );
    assert_eq!(TokenKind::True.literal_kind(), Some(LiteralKind::Bool));
    assert_eq!(TokenKind::False.literal_kind(), Some(LiteralKind::Bool));
    assert_eq!(TokenKind::Null.literal_kind(), Some(LiteralKind::Null));
    assert_eq!(TokenKind::None.literal_kind(), Some(LiteralKind::Null));

    assert_eq!(TokenKind::Identifier.literal_kind(), None);
}

#[test]
fn test_numeric_literal_format_analysis() {
    assert_eq!(
        numeric_literal_kind("42"),
        Some(NumericLiteralKind::Decimal)
    );
    assert_eq!(
        numeric_literal_kind("12_345"),
        Some(NumericLiteralKind::Decimal)
    );
    assert_eq!(
        numeric_literal_kind("3.14159"),
        Some(NumericLiteralKind::Decimal)
    );
    assert_eq!(
        numeric_literal_kind("1.5e-10"),
        Some(NumericLiteralKind::Decimal)
    );
    assert_eq!(numeric_literal_kind("0xFF"), Some(NumericLiteralKind::Hex));
    assert_eq!(
        numeric_literal_kind("0b1010"),
        Some(NumericLiteralKind::Binary)
    );
    assert_eq!(
        numeric_literal_kind("0o755"),
        Some(NumericLiteralKind::Octal)
    );

    // Invalid numbers
    assert_eq!(numeric_literal_kind(""), None);
    assert_eq!(numeric_literal_kind("abc"), None);
    assert_eq!(numeric_literal_kind("1.2.3"), None);
    assert_eq!(numeric_literal_kind("1e2e3"), None);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Token Struct Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_token_creation() {
    let span = Span::new(5, 10);
    let token = Token::new(TokenKind::Identifier, "my_var".to_string(), span);
    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.lexeme, "my_var");
    assert_eq!(token.span, span);
}

#[test]
fn test_token_to_canonical() {
    let span = Span::new(0, 5);
    let token = Token::new(TokenKind::Build, "build".to_string(), span);
    let canonical = token.to_canonical();
    assert_eq!(canonical.kind, TokenKind::Do);
    assert_eq!(canonical.lexeme, "build"); // lexeme remains unchanged
}

#[test]
fn test_token_display() {
    let token = Token::new(TokenKind::Do, "do".to_string(), Span::new(0, 2));
    let display = format!("{}", token);
    assert!(display.contains("do"));
    assert!(display.contains("0..2"));
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Serialization & Roundtrip Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_serde_roundtrips() {
    // TokenKind
    let kind = TokenKind::Do;
    let kind_json = serde_json::to_string(&kind).unwrap();
    let deserialized_kind: TokenKind = serde_json::from_str(&kind_json).unwrap();
    assert_eq!(kind, deserialized_kind);

    // Token
    let token = Token::new(TokenKind::IntLiteral, "42".to_string(), Span::new(0, 2));
    let token_json = serde_json::to_string(&token).unwrap();
    let deserialized_token: Token = serde_json::from_str(&token_json).unwrap();
    assert_eq!(token, deserialized_token);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Uniqueness Verification
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_token_uniqueness() {
    // Ensure all TokenKind static_lexemes are unique
    let mut lexemes = HashSet::new();
    let kinds = [
        TokenKind::Do,
        TokenKind::Send,
        TokenKind::When,
        TokenKind::Loop,
        TokenKind::Repeat,
        TokenKind::For,
        TokenKind::In,
        TokenKind::Match,
        TokenKind::Case,
        TokenKind::Default,
        TokenKind::Try,
        TokenKind::Catch,
        TokenKind::Throw,
        TokenKind::Use,
        TokenKind::Class,
        TokenKind::Struct,
        TokenKind::Enum,
        TokenKind::Trait,
        TokenKind::Interface,
        TokenKind::Const,
        TokenKind::Null,
        TokenKind::Say,
        TokenKind::Ask,
        TokenKind::Break,
        TokenKind::Continue,
        TokenKind::Else,
        TokenKind::Async,
        TokenKind::Await,
        TokenKind::Parallel,
        TokenKind::End,
        TokenKind::Export,
        TokenKind::New,
        TokenKind::SelfKw,
        TokenKind::True,
        TokenKind::False,
        TokenKind::Typeof,
        TokenKind::With,
        TokenKind::Build,
        TokenKind::Make,
        TokenKind::Return,
        TokenKind::Model,
        TokenKind::If,
        TokenKind::Elif,
        TokenKind::While,
        TokenKind::Import,
        TokenKind::From,
        TokenKind::Let,
        TokenKind::Var,
        TokenKind::Fun,
        TokenKind::Function,
        TokenKind::Attempt,
        TokenKind::None,
        TokenKind::Keep,
        TokenKind::Give,
        TokenKind::Stop,
        TokenKind::Skip,
        TokenKind::Each,
        TokenKind::Switch,
        TokenKind::Be,
        TokenKind::Equals,
        TokenKind::Then,
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::DoubleSlash,
        TokenKind::Percent,
        TokenKind::DoubleStar,
        TokenKind::EqualEqual,
        TokenKind::BangEqual,
        TokenKind::TripleEqual,
        TokenKind::BangEqualEqual,
        TokenKind::Less,
        TokenKind::Greater,
        TokenKind::LessEqual,
        TokenKind::GreaterEqual,
        TokenKind::Equal,
        TokenKind::PlusEqual,
        TokenKind::MinusEqual,
        TokenKind::StarEqual,
        TokenKind::SlashEqual,
        TokenKind::PercentEqual,
        TokenKind::DotDot,
        TokenKind::DotDotEqual,
        TokenKind::QuestionDot,
        TokenKind::QuestionQuestion,
        TokenKind::Arrow,
        TokenKind::LeftParen,
        TokenKind::RightParen,
        TokenKind::LeftBrace,
        TokenKind::RightBrace,
        TokenKind::LeftBracket,
        TokenKind::RightBracket,
        TokenKind::Comma,
        TokenKind::Dot,
        TokenKind::Colon,
        TokenKind::Semicolon,
    ];

    for kind in kinds {
        let lexeme = kind.static_lexeme().unwrap();
        assert!(
            lexemes.insert(lexeme),
            "Duplicate static lexeme detected for: {:?}",
            kind
        );
    }
}
