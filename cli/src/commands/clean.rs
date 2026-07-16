//! # tsc clean Command
//!
//! Removes all compiled build artifacts and incremental caches.

use crate::artifacts::ArtifactManager;
use crate::exit_code::ExitCode;

pub fn execute() -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let manager = ArtifactManager::new(&current_dir);

    println!("Cleaning build artifacts...");
    match manager.clean() {
        Ok(_) => {
            println!("Build directory successfully removed.");
            ExitCode::Success
        }
        Err(e) => {
            eprintln!("Error cleaning build directory: {}", e);
            ExitCode::IoError
        }
    }
}
