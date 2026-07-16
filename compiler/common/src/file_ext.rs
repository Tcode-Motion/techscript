//! TechScript file extension validation.
//!
//! The TechScript 2.0 file extension `.txs` is a frozen design decision.
//! This module provides centralized constants and validation utilities so that
//! every crate in the compiler pipeline references a single definition.

use std::fmt;
use std::path::Path;

/// The TechScript source file extension without the leading dot.
pub const TECHSCRIPT_EXTENSION: &str = "txs";

/// The TechScript source file extension with the leading dot.
pub const TECHSCRIPT_DOT_EXTENSION: &str = ".txs";

/// Errors originating from the `techscript_common` crate.
///
/// This enum is intentionally kept small. Additional variants will be added
/// as the common crate grows, but domain-specific errors belong in their
/// respective crates (e.g., `techscript_errors` for compiler diagnostics).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommonError {
    /// The provided file does not have the required `.txs` extension.
    InvalidExtension {
        /// The path that failed validation.
        path: String,
        /// A human-readable description of the error.
        message: String,
    },
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonError::InvalidExtension { path, message } => {
                write!(f, "{message}: '{path}'")
            }
        }
    }
}

impl std::error::Error for CommonError {}

/// Returns `true` if the given path has the `.txs` extension.
///
/// The check is case-sensitive: `.TXS`, `.Txs`, and other variants are rejected.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use techscript_common::is_techscript_file;
///
/// assert!(is_techscript_file(Path::new("main.txs")));
/// assert!(!is_techscript_file(Path::new("main.tech")));
/// assert!(!is_techscript_file(Path::new("main.TXS")));
/// assert!(!is_techscript_file(Path::new("main")));
/// ```
pub fn is_techscript_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext == TECHSCRIPT_EXTENSION)
}

/// Validates that the given path has the `.txs` extension.
///
/// Returns `Ok(())` if the extension is valid, or a [`CommonError::InvalidExtension`]
/// describing the problem.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use techscript_common::validate_extension;
///
/// assert!(validate_extension(Path::new("main.txs")).is_ok());
/// assert!(validate_extension(Path::new("main.tech")).is_err());
/// ```
pub fn validate_extension(path: &Path) -> Result<(), CommonError> {
    if is_techscript_file(path) {
        Ok(())
    } else {
        Err(CommonError::InvalidExtension {
            path: path.display().to_string(),
            message: format!(
                "TechScript source files must use the '{TECHSCRIPT_DOT_EXTENSION}' extension"
            ),
        })
    }
}
