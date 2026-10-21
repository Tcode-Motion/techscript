//! # techscript list Command
//!
//! Lists declared project dependencies from techscript.toml or tech.toml.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use techscript_package_manager::{find_manifest_in, Manifest};

pub fn execute() -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest_path = match find_manifest_in(&current_dir) {
        Some(p) => p,
        None => {
            eprintln!(
                "{}",
                "Error: No project manifest (techscript.toml or tech.toml) found."
                    .red()
                    .bold()
            );
            return ExitCode::Failure;
        }
    };

    let manifest_content = match fs::read_to_string(&manifest_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading manifest: {}", e);
            return ExitCode::Failure;
        }
    };

    let manifest: Manifest = match toml::from_str(&manifest_content) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error parsing manifest: {}", e);
            return ExitCode::Failure;
        }
    };

    println!(
        "{} {}",
        "Project:".bold(),
        manifest.package.name.cyan().bold()
    );
    println!("Version: {}", manifest.package.version);
    println!(
        "Manifest: {}",
        manifest_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );
    println!();

    let deps = manifest.dependencies.unwrap_or_default();
    if deps.is_empty() {
        println!("No dependencies declared.");
        return ExitCode::Success;
    }

    println!(
        "{:<24} {:<16} {}",
        "PACKAGE".bold(),
        "VERSION".bold(),
        "STATUS".bold()
    );
    println!(
        "{}",
        "------------------------------------------------------".dimmed()
    );

    for (name, dep) in &deps {
        let ver_str = match dep {
            techscript_package_manager::DependencyConfig::SimpleVersion(v) => v.clone(),
            techscript_package_manager::DependencyConfig::Detailed { version, path } => {
                if let Some(v) = version {
                    v.clone()
                } else if let Some(p) = path {
                    format!("path:{}", p)
                } else {
                    "*".to_string()
                }
            }
        };

        let is_installed = current_dir.join("packages").join(name).exists();
        let status = if is_installed {
            "Installed".green()
        } else {
            "Declared".yellow()
        };

        println!("{:<24} {:<16} {}", name.cyan(), ver_str, status);
    }

    println!();
    println!(
        "Total dependencies: {}",
        deps.len().to_string().green().bold()
    );
    ExitCode::Success
}
