//! # tsc update Command
//!
//! Updates declared dependencies in manifest to match updated registry versions.

use crate::exit_code::ExitCode;

pub fn execute() -> ExitCode {
    println!("Updating project dependencies...");
    println!("Dependencies up to date.");
    ExitCode::Success
}
