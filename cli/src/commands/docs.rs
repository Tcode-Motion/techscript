//! # tsc docs Command
//!
//! Opens bundled offline HTML documentation book in the default browser.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::PathBuf;

pub fn execute(section: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Resolve home directory path candidates
    let home_path = std::env::var("TECHSCRIPT_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| current_dir.clone());

    let docs_dir = home_path.join("docs").join("html").join("index.html");

    if !docs_dir.exists() {
        eprintln!(
            "{}",
            "Error: Offline documentation file index.html not found inside TECHSCRIPT_HOME."
                .red()
                .bold()
        );
        return ExitCode::IoError;
    }

    let mut url = format!("file:///{}", docs_dir.to_string_lossy().replace('\\', "/"));

    if let Some(sec) = section {
        let fragment = match sec.to_lowercase().as_str() {
            "std" => "#std",
            "compiler" => "#compiler",
            "guide" => "#guide",
            other => {
                eprintln!(
                    "Warning: Unknown docs section '{}'. Defaulting to guide index.",
                    other
                );
                ""
            }
        };
        url.push_str(fragment);
    }

    println!("Opening offline documentation: {}", url.cyan());

    if let Err(e) = open_browser(&url) {
        eprintln!("Error opening default web browser: {}", e);
        return ExitCode::IoError;
    }

    ExitCode::Success
}

fn open_browser(url: &str) -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "start", url])
            .status()?;
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(url).status()?;
    } else {
        std::process::Command::new("xdg-open").arg(url).status()?;
    }
    Ok(())
}
