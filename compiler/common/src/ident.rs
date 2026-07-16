//! Named identifiers with source locations for the TechScript 2.0 compiler.
//!
//! An [`Ident`] pairs a variable, function, or model name with the [`Span`]
//! where it appears in source code. Identifiers are used throughout the AST,
//! symbol tables, and diagnostic messages.

use crate::span::Span;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// A named identifier with its source location.
///
/// `Ident` is used for variable names, function names, model names, parameter
/// names, and any other user-defined symbol in TechScript source code.
///
/// # Equality and Hashing
///
/// `Ident` implements [`PartialEq`] and [`Eq`] comparing **both** the name and
/// span (for AST structural comparison). However, [`Hash`] is implemented to
/// hash **only the name**, allowing `Ident` values to be used as keys in hash
/// maps where the source location is irrelevant (e.g., symbol tables).
///
/// # Examples
///
/// ```
/// use techscript_common::{Ident, Span};
///
/// let ident = Ident::new("counter".to_string(), Span::new(5, 12));
/// assert_eq!(ident.name, "counter");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ident {
    /// The identifier name as it appears in source code.
    pub name: String,
    /// The source location of this identifier.
    pub span: Span,
}

impl Ident {
    /// Creates a new `Ident` with the given name and source span.
    #[inline]
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }

    /// Creates a dummy `Ident` with the given name and a zero-length span.
    ///
    /// Useful for synthetic identifiers in tests or compiler-generated nodes.
    #[inline]
    pub fn dummy(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            span: Span::dummy(),
        }
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Hash for Ident {
    /// Hashes only the identifier name, ignoring the source span.
    ///
    /// This allows identifiers from different source locations to hash
    /// identically when they share the same name, which is the correct
    /// behavior for symbol table lookups.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
