//! # tsc examples Command
//!
//! Discovers and lists all bundled language examples inside the installation package.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::PathBuf;

pub fn execute() -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Resolve home directory path candidates
    let home_path = std::env::var("TECHSCRIPT_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| current_dir.clone());

    let examples_dir = home_path.join("examples");

    if !examples_dir.exists() {
        eprintln!("{}", "Warning: Bundled examples folder not found inside TECHSCRIPT_HOME. Listing local directory examples...".yellow());
    }

    let search_dir = if examples_dir.exists() {
        examples_dir
    } else {
        current_dir.join("examples")
    };

    if !search_dir.exists() {
        eprintln!(
            "{}",
            "Error: No examples directory found on this system."
                .red()
                .bold()
        );
        return ExitCode::IoError;
    }

    println!(
        "{}",
        "=========================================================".bold()
    );
    println!("             TECHSCRIPT 2.0 BUNDLED EXAMPLES             ");
    println!(
        "{}",
        "=========================================================".bold()
    );
    println!("Browse and execute the official examples to get started:\n");

    let entries = match std::fs::read_dir(&search_dir) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("Error: Could not read examples directory.");
            return ExitCode::IoError;
        }
    };

    let mut count = 0;
    for entry in entries.flatten() {
        if entry.path().is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            // Ignore dotfiles
            if name.starts_with('.') {
                continue;
            }
            println!(
                "  • {} — Run via: tsc run {}",
                name.cyan().bold(),
                format!("examples/{}/main.txs", name).green()
            );
            count += 1;
        }
    }

    if count == 0 {
        println!("  No examples found in {:?}", search_dir);
    }

    println!("\nRelated Commands:");
    println!("  tsc run <file>    Execute a TechScript source file");
    println!("  tsc docs          Open local HTML documentation");
    println!("  tsc new <name>    Create a new project workspace");
    println!(
        "{}",
        "=========================================================".bold()
    );

    ExitCode::Success
}
