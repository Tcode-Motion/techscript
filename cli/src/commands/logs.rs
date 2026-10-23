//! # `tsc logs` Centralized Log Viewer
//!
//! Inspects compiler, build engine, Gradle, and deployment execution logs.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn execute(level: Option<&str>, _follow: bool) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let logs_dir = current_dir.join("logs");

    if !logs_dir.exists() {
        println!("No build logs found in '{}'.", logs_dir.display());
        return ExitCode::Success;
    }

    println!(
        "{} Inspecting centralized build logs in '{}':\n",
        "📋".bold(),
        logs_dir.display().to_string().cyan()
    );

    let mut log_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&logs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext == "log" || ext == "json" {
                        log_files.push(path);
                    }
                }
            }
        }
    }

    log_files.sort_by(|a, b| b.cmp(a)); // newest first

    if log_files.is_empty() {
        println!("No log records found.");
        return ExitCode::Success;
    }

    // Display the most recent log file
    let latest = &log_files[0];
    println!(
        "--- Most Recent Log: {} ---",
        latest
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .yellow()
            .bold()
    );

    if let Ok(content) = fs::read_to_string(latest) {
        for line in content.lines().take(100) {
            if let Some(lvl) = level {
                if !line.to_lowercase().contains(&lvl.to_lowercase()) {
                    continue;
                }
            }
            if line.contains("ERROR") || line.contains("error:") || line.contains("FAILED") {
                println!("{}", line.red());
            } else if line.contains("WARN") || line.contains("warning:") {
                println!("{}", line.yellow());
            } else {
                println!("{}", line);
            }
        }
    }

    println!("\nTotal log files available: {}", log_files.len());
    ExitCode::Success
}
