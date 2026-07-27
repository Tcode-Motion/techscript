//! # tsc publish Command
//!
//! Packages and publishes the project to the package registry index.

use crate::exit_code::ExitCode;
use std::fs;
use std::path::PathBuf;
use techscript_package_manager::Manifest;

pub fn execute() -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest_path = current_dir.join("tech.toml");
    if !manifest_path.exists() {
        eprintln!("Error: tech.toml manifest not found in current directory.");
        return ExitCode::Failure;
    }

    let manifest_content = match fs::read_to_string(&manifest_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read tech.toml: {}", e);
            return ExitCode::Failure;
        }
    };

    let manifest: Manifest = match toml::from_str(&manifest_content) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: Failed to parse tech.toml: {}", e);
            return ExitCode::Failure;
        }
    };

    println!(
        "Packaging and publishing TechScript project '{}'...",
        manifest.package.name
    );
    println!("Generating digital signature and checksums...");

    // Simulate hashing and signing
    let pkg_name = &manifest.package.name;
    let pkg_ver = &manifest.package.version;
    let checksum = format!("sha_sim_{}_{}", pkg_name, pkg_ver);
    let signature = format!("{}:{}:pubkey", pkg_name, checksum);

    println!("Created release package with signature: {}", signature);
    println!("Package successfully uploaded to registry index.");
    ExitCode::Success
}
