//! # TechScript CLI Binary
//!
//! Command-line entry point parsing and subcommand execution.
//! Orchestrates lexer, parser, semantic analyzer, and interpreter flows.

use clap::Parser;
use techscript_cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file, verbose } => {
            println!("Running file {} (verbose: {})", file, verbose);
        }
        Commands::Repl => {
            println!("Starting REPL shell...");
        }
        Commands::Check { file } => {
            println!("Checking file {}...", file);
        }
        Commands::Fmt { file } => {
            println!("Formatting file {}...", file);
        }
        Commands::Lint { file, fix } => {
            println!("Linting file {} (fix: {})...", file, fix);
        }
        Commands::Test { dir } => {
            println!("Running tests in {:?}", dir);
        }
        Commands::Version => {
            println!("TechScript version 2.0.0 (Rust binary)");
        }
        Commands::New { name } => {
            println!("Scaffolding new project {}...", name);
        }
    }
}
