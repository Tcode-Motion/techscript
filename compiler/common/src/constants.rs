//! Shared compiler constants for TechScript 2.0.
//!
//! This module centralizes magic numbers and version strings so that every
//! crate in the workspace references a single authoritative source.

/// The TechScript compiler version, sourced from the workspace `Cargo.toml`.
///
/// This uses the `CARGO_PKG_VERSION` environment variable set by Cargo at
/// compile time, ensuring the version string is always synchronized with
/// the crate manifest.
pub const TECHSCRIPT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum call stack depth before the interpreter raises a stack overflow
/// error (`E1020`).
///
/// This guard prevents infinite recursion from consuming unbounded memory.
/// The default value of 1024 is generous enough for realistic programs while
/// still catching runaway recursion promptly.
pub const MAX_RECURSION_DEPTH: usize = 1024;

/// Maximum source file size in bytes (10 MiB).
///
/// Files exceeding this limit are rejected before lexing to prevent
/// pathological memory consumption. This is intentionally generous — the
/// performance budget (10,000 lines in <100ms) assumes much smaller files.
pub const MAX_SOURCE_FILE_SIZE: usize = 10 * 1024 * 1024;
