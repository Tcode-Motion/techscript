//! # tsc clean Command
//!
//! Removes all compiled build artifacts and incremental caches.
//! Optionally clears global registry caches with --all.

use crate::artifacts::ArtifactManager;
use crate::exit_code::ExitCode;
use colored::Colorize;

pub fn execute(all: bool) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let manager = ArtifactManager::new(&current_dir);

    println!("Cleaning local build artifacts...");
    let mut success = true;

    match manager.clean() {
        Ok(_) => {
            println!("{}", "✓ Build directory successfully removed.".green());
        }
        Err(e) => {
            eprintln!("Error cleaning build directory: {}", e);
            success = false;
        }
    }

    if all {
        println!("Cleaning global package caches...");
        if let Some(home) = home_dir() {
            let cache_dir = home.join(".techscript").join("cache");
            if cache_dir.exists() {
                if std::fs::remove_dir_all(&cache_dir).is_ok() {
                    std::fs::create_dir_all(&cache_dir).ok();
                    println!("{}", "✓ Global package caches successfully cleared.".green());
                } else {
                    eprintln!("Error: Could not clear global package caches.");
                    success = false;
                }
            }
        }
    }

    if success {
        ExitCode::Success
    } else {
        ExitCode::IoError
    }
}

fn home_dir() -> Option<std::path::PathBuf> {
    #[allow(deprecated)]
    std::env::home_dir()
}
