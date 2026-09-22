//! # tsc fmt Command
//!
//! Formats TechScript source code files recursively in the specified path.

use crate::exit_code::ExitCode;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn execute(path_str: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_path = path_str.map(PathBuf::from).unwrap_or(current_dir);

    if !target_path.exists() {
        eprintln!("Error: Path does not exist: {:?}", target_path);
        return ExitCode::IoError;
    }

    println!("Formatting TechScript files in: {:?}", target_path);

    let mut files_to_format = Vec::new();
    if target_path.is_dir() {
        let mut dirs = vec![target_path];
        while let Some(dir) = dirs.pop() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        if name != "build" && name != ".git" && name != "target" {
                            dirs.push(path);
                        }
                    } else {
                        let ext = path.extension().unwrap_or_default().to_string_lossy();
                        if ext == "txs" || ext == "ts" {
                            files_to_format.push(path);
                        }
                    }
                }
            }
        }
    } else {
        files_to_format.push(target_path);
    }

    let formatted_count = AtomicUsize::new(0);

    files_to_format.par_iter().for_each(|file| {
        let formatter = techscript_formatter::DocumentFormatter::new(4);
        match std::fs::read_to_string(file) {
            Ok(content) => {
                let formatted = formatter.format_source(&content);
                // In skeletal phase, if format_source returns empty, we just skip writing to avoid wiping out files.
                // In future phase, the formatting AST walker will produce real output.
                if !formatted.is_empty()
                    && formatted != content
                    && !formatted.contains("<stmt>")
                    && !formatted.contains("<pat>")
                {
                    if let Err(e) = std::fs::write(file, formatted) {
                        eprintln!("Error writing formatted file {:?}: {}", file, e);
                    } else {
                        println!("Formatted: {:?}", file);
                        formatted_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file {:?}: {}", file, e);
            }
        }
    });

    println!("Formatted {} files.", formatted_count.into_inner());
    ExitCode::Success
}
