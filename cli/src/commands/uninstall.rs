//! # tsc uninstall Command
//!
//! Removes dependencies from the project configuration manifest.

use crate::exit_code::ExitCode;
use std::fs;
use std::path::PathBuf;
use techscript_package_manager::{Lockfile, Manifest};

pub fn execute(package: &str) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest_path = match techscript_package_manager::find_manifest_in(&current_dir) {
        Some(p) => p,
        None => {
            eprintln!("Error: Manifest (webforge.toml, techscript.toml, or tech.toml) not found in current directory.");
            return ExitCode::Failure;
        }
    };

    let manifest_content = match fs::read_to_string(&manifest_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read manifest: {}", e);
            return ExitCode::Failure;
        }
    };

    let mut manifest: Manifest = match toml::from_str(&manifest_content) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: Failed to parse manifest: {}", e);
            return ExitCode::Failure;
        }
    };

    println!("Removing package '{}' from manifest...", package);

    let mut deps = manifest.dependencies.unwrap_or_default();
    if deps.remove(package).is_none() {
        println!(
            "Warning: Package '{}' not found in dependencies list.",
            package
        );
    }
    manifest.dependencies = Some(deps);

    // Save updated manifest
    let updated_toml = match toml::to_string(&manifest) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error generating manifest TOML: {}", e);
            return ExitCode::Failure;
        }
    };
    if let Err(e) = fs::write(&manifest_path, updated_toml) {
        eprintln!("Error writing manifest: {}", e);
        return ExitCode::Failure;
    }

    // Delete packages directory
    let package_dir = current_dir.join("packages").join(package);
    if package_dir.exists() {
        fs::remove_dir_all(package_dir).ok();
    }

    // Update lockfile
    let lockfile_name = if manifest_path.file_name().and_then(|f| f.to_str()) == Some("webforge.toml") {
        "webforge.lock"
    } else if manifest_path.file_name().and_then(|f| f.to_str()) == Some("tech.toml") {
        "tech.lock"
    } else {
        "techscript.lock"
    };
    let lockfile_path = current_dir.join(lockfile_name);
    if lockfile_path.exists() {
        if let Ok(lockfile_content) = fs::read_to_string(&lockfile_path) {
            if let Ok(mut lockfile) = toml::from_str::<Lockfile>(&lockfile_content) {
                lockfile.package.retain(|p| p.name != package);
                if let Ok(updated_lock) = toml::to_string(&lockfile) {
                    fs::write(&lockfile_path, updated_lock).ok();
                }
            }
        }
    }

    println!("Successfully uninstalled '{}'.", package);
    ExitCode::Success
}
