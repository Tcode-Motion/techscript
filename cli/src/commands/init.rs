//! # tsc init Command
//!
//! Initializes a new project layout in the current working directory.
//! Launches an interactive wizard if template choice is missing.

use crate::exit_code::ExitCode;
use crate::templates::ProjectTemplate;
use colored::Colorize;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn execute(template_str: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let mut resolved_template = template_str.map(|s| s.to_string());

    // 1. Run interactive wizard if template is not provided
    if resolved_template.is_none() {
        println!(
            "{}",
            "========================================================="
                .cyan()
                .bold()
        );
        println!("             TECHSCRIPT 2.0 INITIALIZATION               ");
        println!(
            "{}",
            "========================================================="
                .cyan()
                .bold()
        );
        println!("Select template structure for this folder:");
        println!("  1) Console   (Standard application entry point)");
        println!("  2) Library   (Shared library module logic)");
        println!("  3) Package   (Publishable standard package structure)");
        println!("  4) Workspace (Multi-project cargo/ts workspaces)");
        println!("  5) Empty     (Minimal empty code skeleton)");
        print!("Select template [1]: ");
        io::stdout().flush().ok();
        let mut input_tmpl = String::new();
        io::stdin().read_line(&mut input_tmpl).ok();
        let trimmed_tmpl = input_tmpl.trim();
        resolved_template = Some(match trimmed_tmpl {
            "2" | "library" => "library".to_string(),
            "3" | "package" => "package".to_string(),
            "4" | "workspace" => "workspace".to_string(),
            "5" | "empty" => "empty".to_string(),
            _ => "console".to_string(),
        });
        println!(
            "{}",
            "=========================================================\n"
                .cyan()
                .bold()
        );
    }

    let template_input = resolved_template.as_deref().unwrap_or("console");

    let template = match ProjectTemplate::parse(template_input) {
        Some(tpl) => tpl,
        None => {
            eprintln!("Error: Unknown project template '{}'.", template_input);
            return ExitCode::InvalidUsage;
        }
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
            println!("\n{}", "✓ Project initialized successfully.".green().bold());
            println!("\nNext steps:");
            println!("  tsc run");
            println!("  code .");
            ExitCode::Success
        }
        Err(e) => {
            eprintln!("Error initializing project: {}", e);
            ExitCode::IoError
        }
    }
}
