//! # tsc new Command
//!
//! Scaffolds a new project structure in a newly created directory.

use crate::exit_code::ExitCode;
use crate::templates::ProjectTemplate;
use std::path::PathBuf;

pub fn execute(name: &str, template_str: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_dir = current_dir.join(name);

    if target_dir.exists() {
        eprintln!("Error: Directory '{}' already exists.", name);
        return ExitCode::IoError;
    }

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

    println!(
        "Scaffolding new project '{}' in {}...",
        name,
        target_dir.display()
    );

    match template.scaffold(name, &target_dir) {
        Ok(paths) => {
            for path in paths {
                println!("  Created: {}", path.display());
            }
            println!("Project '{}' created successfully.", name);
            ExitCode::Success
        }
        Err(e) => {
            eprintln!("Error scaffolding project: {}", e);
            ExitCode::IoError
        }
    }
}
