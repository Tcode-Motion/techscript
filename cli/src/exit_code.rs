//! # TechScript Compiler Driver — Exit Codes
//!
//! Typed exit codes for the `tsc` compiler driver.
//! All commands return `ExitCode` instead of raw integers.

/// Typed exit codes for the `tsc` compiler driver.
///
/// Follows POSIX conventions:
/// - 0 = success
/// - 1–10 = domain-specific errors
/// - 64 = command-line usage error (EX_USAGE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Command completed successfully.
    Success = 0,
    /// One or more compilation errors occurred.
    CompilationError = 1,
    /// Configuration file is invalid or missing.
    ConfigError = 2,
    /// Filesystem I/O operation failed.
    IoError = 3,
    /// One or more tests failed.
    TestFailure = 4,
    /// Lint rules found violations.
    LintFailure = 5,
    /// Internal compiler error (panic / ICE).
    InternalError = 10,
    /// Invalid command-line usage.
    InvalidUsage = 64,
}

impl ExitCode {
    /// Returns the underlying integer exit code.
    pub fn code(self) -> i32 {
        self as i32
    }

    /// Returns a human-readable description.
    pub fn description(self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::CompilationError => "Compilation failed",
            Self::ConfigError => "Configuration error",
            Self::IoError => "I/O error",
            Self::TestFailure => "Tests failed",
            Self::LintFailure => "Lint violations found",
            Self::InternalError => "Internal compiler error",
            Self::InvalidUsage => "Invalid command usage",
        }
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description(), self.code())
    }
}
