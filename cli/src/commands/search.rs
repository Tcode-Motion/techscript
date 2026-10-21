//! # techscript search Command
//!
//! Searches the official TechScript package registry index.

use crate::exit_code::ExitCode;
use colored::Colorize;

pub fn execute(query: &str) -> ExitCode {
    let q = query.to_lowercase();
    println!(
        "Searching TechScript package registry for '{}'...\n",
        query.cyan()
    );

    // Official registry packages index
    let packages = [
        (
            "log",
            "1.0.0",
            "Structured logging utilities and log level handlers",
        ),
        (
            "http",
            "2.1.0",
            "HTTP client and server networking primitives",
        ),
        (
            "web",
            "2.0.0",
            "WebForge declarative web framework components",
        ),
        (
            "json",
            "1.2.0",
            "Fast streaming JSON serialization and parsing",
        ),
        (
            "sqlite",
            "1.0.0",
            "Local SQLite persistence and relational data models",
        ),
        (
            "three3d",
            "0.160.0",
            "Three.js WebGL 3D graphics integration",
        ),
        (
            "crypto",
            "1.1.0",
            "Cryptographic hashing, SHA-256, and AES helpers",
        ),
        (
            "testing",
            "1.0.0",
            "Unit and integration test assertion harness",
        ),
        (
            "colors",
            "1.0.0",
            "Terminal ANSI colorization and styling utilities",
        ),
    ];

    let mut matches = Vec::new();
    for (name, ver, desc) in packages {
        if name.to_lowercase().contains(&q) || desc.to_lowercase().contains(&q) {
            matches.push((name, ver, desc));
        }
    }

    if matches.is_empty() {
        println!("No packages matched '{}'.", query);
        return ExitCode::Success;
    }

    println!(
        "{:<18} {:<10} {}",
        "PACKAGE".bold(),
        "VERSION".bold(),
        "DESCRIPTION".bold()
    );
    println!(
        "{}",
        "--------------------------------------------------------------------------------".dimmed()
    );

    for (name, ver, desc) in &matches {
        println!("{:<18} {:<10} {}", name.cyan().bold(), ver.green(), desc);
    }

    println!();
    println!(
        "Found {} package(s). Install using '{}'.",
        matches.len().to_string().green().bold(),
        format!("techscript add {}", matches[0].0).cyan()
    );
    ExitCode::Success
}
