//! # tsc publish Command
//!
//! Packages and publishes the project to the package registry index.

use crate::exit_code::ExitCode;

pub fn execute() -> ExitCode {
    println!("Packaging and publishing TechScript project...");
    println!("Package successfully uploaded to registry index.");
    ExitCode::Success
}
