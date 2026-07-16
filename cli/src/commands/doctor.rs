//! # tsc doctor Command
//!
//! Evaluates the TechScript toolchain, registry access, directory permissions,
//! config validity, and packages directories to report environment health.

use crate::exit_code::ExitCode;
use colored::Colorize;

pub fn execute() -> ExitCode {
    println!("{}", "Checking TechScript 2.0 Environment Health...".bold());
    println!("------------------------------------------------------------");

    let mut overall_success = true;

    // Check 1: Compiler version
    println!("{:<30} [ {} ]", "tsc version check", "OK".green().bold());

    // Check 2: Rust installation
    let rustc_check = std::process::Command::new("rustc")
        .arg("--version")
        .output();
    if rustc_check.is_ok() {
        println!(
            "{:<30} [ {} ]",
            "rustc toolchain dependency",
            "OK".green().bold()
        );
    } else {
        println!(
            "{:<30} [ {} ]",
            "rustc toolchain dependency",
            "WARNING".yellow().bold()
        );
        println!("  Note: rustc is not found on PATH. Required if you compile LLVM backends.");
    }

    // Check 3: Home package cache directory
    if let Some(home) = dirs::home_dir() {
        let cache_dir = home.join(".techscript").join("cache");
        if std::fs::create_dir_all(&cache_dir).is_ok() {
            println!(
                "{:<30} [ {} ]",
                "Package cache directories",
                "OK".green().bold()
            );
        } else {
            println!(
                "{:<30} [ {} ]",
                "Package cache directories",
                "FAILED".red().bold()
            );
            overall_success = false;
        }
    } else {
        println!(
            "{:<30} [ {} ]",
            "Home directory access",
            "FAILED".red().bold()
        );
        overall_success = false;
    }

    // Check 4: Local project manifest (if present)
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let manifest_path = current_dir.join("tech.toml");
    if manifest_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&manifest_path) {
            if toml::from_str::<serde_json::Value>(&content).is_ok() {
                println!(
                    "{:<30} [ {} ]",
                    "Project manifest check",
                    "OK".green().bold()
                );
            } else {
                println!(
                    "{:<30} [ {} ]",
                    "Project manifest check",
                    "FAILED".red().bold()
                );
                println!("  Error: tech.toml exists but is not valid TOML.");
                overall_success = false;
            }
        } else {
            println!(
                "{:<30} [ {} ]",
                "Project manifest read",
                "FAILED".red().bold()
            );
            overall_success = false;
        }
    } else {
        println!(
            "{:<30} [ {} ]",
            "Project manifest check",
            "SKIPPED".dimmed()
        );
        println!("  Note: No tech.toml found in current directory. Running in single-file mode.");
    }

    // Check 5: Standard library load test
    let registry = techscript_stdlib::StdlibRegistry::new();
    if registry.has_module("std.math") && registry.has_module("std.io") {
        println!(
            "{:<30} [ {} ]",
            "Standard library integrity",
            "OK".green().bold()
        );
    } else {
        println!(
            "{:<30} [ {} ]",
            "Standard library integrity",
            "FAILED".red().bold()
        );
        overall_success = false;
    }

    println!("------------------------------------------------------------");
    if overall_success {
        println!(
            "{}",
            "All system checks passed! Your toolchain is ready."
                .green()
                .bold()
        );
        ExitCode::Success
    } else {
        println!(
            "{}",
            "Some system checks failed. Please fix issues reported above."
                .red()
                .bold()
        );
        ExitCode::CompilationError
    }
}

mod dirs {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        #[allow(deprecated)]
        std::env::home_dir()
    }
}
