//! # tsc version Command
//!
//! Prints compiler details, version numbers, commit hashes, target platform details,
//! and VM bytecode specifications.

use crate::exit_code::ExitCode;

pub fn execute() -> ExitCode {
    println!(
        "TechScript compiler driver (tsc) v{}",
        techscript_common::TECHSCRIPT_VERSION
    );
    println!("Bytecode Engine Version:  1.0.0");
    println!("Virtual Machine Target:  stack-based VM");
    println!("Host Architecture:       {}", std::env::consts::ARCH);
    println!("Host OS:                 {}", std::env::consts::OS);
    println!(
        "Compilation Profile:     {}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );
    ExitCode::Success
}
