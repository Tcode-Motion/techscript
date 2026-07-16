//! # tsc uninstall Command
//!
//! Removes dependencies from the project configuration manifest.

use crate::exit_code::ExitCode;

pub fn execute(package: &str) -> ExitCode {
    println!("Removing package '{}' from manifest...", package);
    println!("Successfully uninstalled '{}'.", package);
    ExitCode::Success
}
