//! # tsc init Command
//!
//! Initializes a new project layout in the current working directory.

use crate::exit_code::ExitCode;
use crate::templates::ProjectTemplate;
use std::path::PathBuf;

pub fn execute(template_str: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let template = match template_str {
        Some(t) => match ProjectTemplate::parse(t) {
            Some(tpl) => tpl,
            None => {
                eprintln!("Error: Unknown project template '{}'.", t);
                return ExitCode::InvalidUsage;
            }
        },
        None => ProjectTemplate::Console,
    };

    let name = current_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    println!(
        "Initializing project '{}' inside {:?}...",
        name, current_dir
    );

    match template.scaffold(&name, &current_dir) {
        Ok(paths) => {
            for path in paths {
                println!("  Created: {}", path.display());
            }
            println!("Project initialized successfully.");
            ExitCode::Success
        }
        Err(e) => {
            eprintln!("Error initializing project: {}", e);
            ExitCode::IoError
        }
    }
}
