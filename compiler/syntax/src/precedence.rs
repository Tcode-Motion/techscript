//! Operator precedence and associativity levels for Pratt parsing.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Precedence binding power levels for TechScript 2.0 parsing.
///
/// Order is from lowest binding power (`None`) to highest (`Call`).
///
/// # Examples
///
/// ```
/// use techscript_syntax::Precedence;
///
/// assert!(Precedence::Call > Precedence::Factor);
/// assert!(Precedence::Term < Precedence::Factor);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Precedence {
    /// Lowest precedence, used for non-operator expressions or statements.
    None,
    /// Variable assignments (e.g., `=`, `+=`, `-=`, `*=`, `/=`, `%=`).
    Assignment,
    /// Null-coalescing operator (`??`).
    NullCoalescing,
    /// Logical OR operator (`or`, `||`).
    Or,
    /// Logical AND operator (`and`, `&&`).
    And,
    /// Equality operators (`==`, `!=`, `===`, `!==`).
    Equality,
    /// Relational comparisons and membership checks (e.g., `<`, `>`, `<=`, `>=`, `is`, `in`).
    Comparison,
    /// Range operators (`..`, `..=`).
    Range,
    /// Bitwise OR operator (`|`).
    BitwiseOr,
    /// Bitwise XOR operator (`^`).
    BitwiseXor,
    /// Bitwise AND operator (`&`).
    BitwiseAnd,
    /// Bitwise shift operators (`<<`, `>>`).
    Shift,
    /// Additive operators (e.g., binary `+`, `-`).
    Term,
    /// Multiplicative operators (e.g., `*`, `/`, `//`, `%`).
    Factor,
    /// Exponentiation operator (`**`).
    Exponent,
    /// Unary prefix operators (e.g., unary `-`, `+`, `!`, `not`).
    Unary,
    /// Calls, member accesses, and index accesses (e.g., `()`, `.`, `?.`, `[]`).
    Call,
}

impl fmt::Display for Precedence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Precedence::None => "None",
            Precedence::Assignment => "Assignment",
            Precedence::NullCoalescing => "NullCoalescing",
            Precedence::Or => "Or",
            Precedence::And => "And",
            Precedence::Equality => "Equality",
            Precedence::Comparison => "Comparison",
            Precedence::Range => "Range",
            Precedence::BitwiseOr => "BitwiseOr",
            Precedence::BitwiseXor => "BitwiseXor",
            Precedence::BitwiseAnd => "BitwiseAnd",
            Precedence::Shift => "Shift",
            Precedence::Term => "Term",
            Precedence::Factor => "Factor",
            Precedence::Exponent => "Exponent",
            Precedence::Unary => "Unary",
            Precedence::Call => "Call",
        };
        write!(f, "{}", label)
    }
}

/// Associativity direction for binary operators.
///
/// Dictates how operators of the same precedence level group when chained.
///
/// # Examples
///
/// ```
/// use techscript_syntax::Associativity;
///
/// let assoc = Associativity::Left;
/// assert_eq!(format!("{}", assoc), "Left");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Associativity {
    /// No associativity (chaining results in compile error, e.g., range `a..b..c`).
    None,
    /// Left-to-right grouping (e.g., `a - b - c` groups as `(a - b) - c`).
    Left,
    /// Right-to-left grouping (e.g., `a ** b ** c` groups as `a ** (b ** c)`).
    Right,
}

impl fmt::Display for Associativity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Associativity::None => "None",
            Associativity::Left => "Left",
            Associativity::Right => "Right",
        };
        write!(f, "{}", label)
    }
}
