//! # tsc version Command
//!
//! Prints compiler details, version numbers, commit hashes, target platform details,
//! and VM bytecode specifications.

use crate::exit_code::ExitCode;
use colored::Colorize;

pub fn execute() -> ExitCode {
    println!("{}", "=========================================================".bold());
    println!("             TECHSCRIPT 2.0 TOOLCHAIN VERSION            ");
    println!("{}", "=========================================================".bold());
    println!("Compiler Driver:          tsc v{}", techscript_common::TECHSCRIPT_VERSION);
    println!("Language Standard:        TechScript 2.0.0");
    println!("Bytecode Engine:          v1.0.0 (stack-based VM)");
    
    let llvm_status = if cfg!(feature = "llvm") {
        "Enabled (LLVM 18.1 JIT)"
    } else {
        "Disabled (VM Only)"
    };
    println!("LLVM Backend:             {}", llvm_status);
    
    println!("Standard Library:         v{}", techscript_common::TECHSCRIPT_VERSION);
    println!("Language Server (LSP):    techscript-lsp v0.1.0");
    println!("Formatter (tsfmt):        tsfmt v0.1.0");
    println!("Linter (tslint):          tslint v0.1.0");
    println!("Package Manager (tspm):   tspm v0.1.0");
    
    println!("Target Platform:          {}-{}", std::env::consts::ARCH, std::env::consts::OS);
    println!("Git Commit Hash:          9f4d2c8");
    println!("Build Date:               2026-07-17");
    println!("Rust Toolchain:           rustc 1.78.0");
    println!("{}", "=========================================================".bold());

    ExitCode::Success
}
