//! Scanned token representations containing lexemes and source positions.

use crate::token_kind::TokenKind;
use serde::{Deserialize, Serialize};
use std::fmt;
use techscript_common::Span;

/// A scanned token output from lexical analysis.
///
/// Combines the token category ([`TokenKind`]), the exact raw source slice
/// (`lexeme`), and the source location ([`Span`]) where the token was matched.
///
/// # Examples
///
/// ```
/// use techscript_syntax::{Token, TokenKind};
/// use techscript_common::Span;
///
/// let token = Token::new(TokenKind::Make, "make".to_string(), Span::new(0, 4));
/// assert_eq!(token.kind, TokenKind::Make);
/// assert_eq!(token.lexeme, "make");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    /// The category of this token.
    pub kind: TokenKind,
    /// The exact matched slice of source text.
    pub lexeme: String,
    /// The byte range in the source code.
    pub span: Span,
}

impl Token {
    /// Creates a new `Token`.
    #[inline]
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self { kind, lexeme, span }
    }

    /// Converts this token's kind to its canonical equivalent if it is an alias keyword.
    ///
    /// The matched lexeme and span remain unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use techscript_syntax::{Token, TokenKind};
    /// use techscript_common::Span;
    ///
    /// let token = Token::new(TokenKind::Let, "let".to_string(), Span::new(0, 3));
    /// let canonical = token.to_canonical();
    /// assert_eq!(canonical.kind, TokenKind::Make);
    /// assert_eq!(canonical.lexeme, "let");
    /// ```
    pub fn to_canonical(&self) -> Self {
        if let Some(canonical_kind) = self.kind.to_canonical() {
            Self {
                kind: canonical_kind,
                lexeme: self.lexeme.clone(),
                span: self.span,
            }
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (\"{}\" at span {})",
            self.kind, self.lexeme, self.span
        )
    }
}
