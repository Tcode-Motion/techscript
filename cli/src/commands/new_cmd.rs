//! # tsc new Command
//!
//! Scaffolds a new project structure in a newly created directory.
//! Launches an interactive wizard if name or template is missing.

use crate::exit_code::ExitCode;
use crate::templates::ProjectTemplate;
use std::path::PathBuf;
use std::io::{self, Write};
use colored::Colorize;

pub fn execute(name_opt: Option<&str>, template_str: Option<&str>) -> ExitCode {
    let mut resolved_name = name_opt.map(|s| s.to_string());
    let mut resolved_template = template_str.map(|s| s.to_string());

    // 1. Run interactive wizard if name is not provided
    if resolved_name.is_none() {
        println!("{}", "=========================================================".cyan().bold());
        println!("             TECHSCRIPT 2.0 PROJECT CREATOR              ");
        println!("{}", "=========================================================".cyan().bold());
        
        print!("Project Name [hello_techscript]: ");
        io::stdout().flush().ok();
        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name).ok();
        let trimmed_name = input_name.trim();
        resolved_name = Some(if trimmed_name.is_empty() {
            "hello_techscript".to_string()
        } else {
            trimmed_name.to_string()
        });

        println!("\nAvailable Templates:");
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
        println!("{}", "=========================================================\n".cyan().bold());
    }

    let name = resolved_name.as_ref().unwrap();
    let template_input = resolved_template.as_deref().unwrap_or("console");

    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_dir = current_dir.join(name);

    if target_dir.exists() {
        eprintln!("Error: Directory '{}' already exists.", name);
        return ExitCode::IoError;
    }

    let template = match ProjectTemplate::parse(template_input) {
        Some(tpl) => tpl,
        None => {
            eprintln!("Error: Unknown project template '{}'.", template_input);
            return ExitCode::InvalidUsage;
        }
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
            
            println!("\n{}", "✓ Project created successfully.".green().bold());
            println!("\nNext steps:");
            println!("  cd {}", name.cyan());
            println!("  tsc run");
            println!("  code .");
            ExitCode::Success
        }
        Err(e) => {
            eprintln!("Error scaffolding project: {}", e);
            ExitCode::IoError
        }
    }
}
