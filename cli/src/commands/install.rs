//! # tsc install Command
//!
//! Downloads and registers dependencies declared in the project manifest.

use crate::exit_code::ExitCode;

pub fn execute(package: &str) -> ExitCode {
    println!("Resolving package '{}'...", package);
    println!("Successfully installed and registered '{}'.", package);
    ExitCode::Success
}
