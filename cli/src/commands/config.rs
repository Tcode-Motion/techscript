//! # tsc config Command
//!
//! Manages global user and project workspace settings.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::PathBuf;

pub fn execute(subcommand_str: Option<&str>) -> ExitCode {
    let sub = subcommand_str.unwrap_or("show").to_lowercase();
    
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            eprintln!("Error: Home directory not found.");
            return ExitCode::IoError;
        }
    };

    let config_dir = home.join(".techscript");
    let config_file = config_dir.join("config.toml");

    match sub.as_str() {
        "show" => {
            println!("{}", "=========================================================".bold());
            println!("             TECHSCRIPT 2.0 ACTIVE CONFIGURATION         ");
            println!("{}", "=========================================================".bold());
            println!("Config Path: {}\n", config_file.display().to_string().cyan());

            if config_file.exists() {
                match std::fs::read_to_string(&config_file) {
                    Ok(content) => println!("{}", content),
                    Err(e) => {
                        eprintln!("Error reading config file: {}", e);
                        return ExitCode::IoError;
                    }
                }
            } else {
                println!("No global configuration file found. Using default values.");
            }
            println!("{}", "=========================================================".bold());
        }
        "edit" => {
            if !config_file.exists() {
                // Bootstrapping default config
                if std::fs::create_dir_all(&config_dir).is_err() || std::fs::write(&config_file, default_toml_config()).is_err() {
                    eprintln!("Error writing default config file.");
                    return ExitCode::IoError;
                }
            }
            println!("Opening configuration editor: {}", config_file.display().to_string().cyan());
            
            let status = if cfg!(windows) {
                std::process::Command::new("notepad")
                    .arg(&config_file)
                    .status()
            } else {
                let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
                std::process::Command::new(editor)
                    .arg(&config_file)
                    .status()
            };

            if let Err(e) = status {
                eprintln!("Error launching editor: {}", e);
                return ExitCode::IoError;
            }
        }
        "reset" => {
            if std::fs::create_dir_all(&config_dir).is_err() || std::fs::write(&config_file, default_toml_config()).is_err() {
                eprintln!("Error writing default config file.");
                return ExitCode::IoError;
            }
            println!("{}", "✓ Global configuration reset to default settings.".green().bold());
        }
        other => {
            eprintln!("Error: Unknown config subcommand '{}'. Choose from: show, edit, reset.", other);
            return ExitCode::InvalidUsage;
        }
    }

    ExitCode::Success
}

fn default_toml_config() -> &'static str {
    r#"# TechScript 2.0 Global Configuration File
# Located at ~/.techscript/config.toml

[config]
optimization_level = "O2"
debug_symbols = false
source_maps = false
strict_mode = false
max_recursion = 1000
log_level = "Normal"
output_format = "Plain"
parallel_jobs = 4
capabilities = ["FileSystem", "Environment", "Process", "Network"]
"#
}
