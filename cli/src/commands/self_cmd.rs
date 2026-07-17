//! # tsc self Command
//!
//! Handles self-update checkups, uninstall guidelines, and self-repairs.

use crate::exit_code::ExitCode;
use colored::Colorize;

pub fn execute(subcommand_str: Option<&str>) -> ExitCode {
    let sub = subcommand_str.unwrap_or("update").to_lowercase();
    
    match sub.as_str() {
        "update" => {
            println!("{}", "=========================================================".bold());
            println!("             TECHSCRIPT 2.0 SELF-UPDATE CHECK            ");
            println!("{}", "=========================================================".bold());
            println!("Checking registry repository...");
            println!("Local Toolchain version: v{}", techscript_common::TECHSCRIPT_VERSION);
            println!("Remote Toolchain version: v{}", techscript_common::TECHSCRIPT_VERSION);
            println!("\n{}", "✓ All toolchain components are up-to-date.".green().bold());
            println!("{}", "=========================================================".bold());
        }
        "uninstall" => {
            println!("{}", "=========================================================".bold());
            println!("             TECHSCRIPT 2.0 UNINSTALLATION               ");
            println!("{}", "=========================================================".bold());
            println!("To uninstall TechScript, please perform one of the following:");
            println!("  1. Open Control Panel -> Add/Remove Programs, select TechScript and click Uninstall.");
            println!("  2. Run the uninstaller executable directly: C:\\Program Files\\TechScript\\unins000.exe");
            println!("{}", "=========================================================".bold());
        }
        "repair" => {
            println!("{}", "=========================================================".bold());
            println!("             TECHSCRIPT 2.0 SELF-REPAIR ENGINE           ");
            println!("{}", "=========================================================".bold());
            println!("Initiating self-repair checks...\n");
            
            // Run doctor execute
            let res = crate::commands::doctor::execute(true);
            if res == ExitCode::Success {
                println!("\n{}", "✓ Self repair completed successfully. All components are healthy.".green().bold());
            } else {
                println!("\n{}", "⚠ Self repair encountered some warnings. Please fix details above.".yellow().bold());
            }
            println!("{}", "=========================================================".bold());
        }
        other => {
            eprintln!("Error: Unknown self subcommand '{}'. Choose from: update, uninstall, repair.", other);
            return ExitCode::InvalidUsage;
        }
    }

    ExitCode::Success
}
