//! # tsc completion Command
//!
//! Generates shell autocompletion scripts for popular shells.

use crate::exit_code::ExitCode;
use clap::CommandFactory;

pub fn execute(shell_str: &str) -> ExitCode {
    let mut cmd = crate::Cli::command();

    let shell = match shell_str.to_lowercase().as_str() {
        "bash" => clap_complete::Shell::Bash,
        "zsh" => clap_complete::Shell::Zsh,
        "fish" => clap_complete::Shell::Fish,
        "powershell" => clap_complete::Shell::PowerShell,
        _ => {
            eprintln!(
                "Error: Unsupported shell '{}'. Choose from: bash, zsh, fish, powershell.",
                shell_str
            );
            return ExitCode::InvalidUsage;
        }
    };

    clap_complete::generate(shell, &mut cmd, "tsc", &mut std::io::stdout());
    ExitCode::Success
}
