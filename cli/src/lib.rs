//! # TechScript CLI Library
//!
//! Exposes command-line parsing structures for testing and integration.

use clap::{Parser as ClapParser, Subcommand};

#[derive(ClapParser, Debug)]
#[command(name = "tech")]
#[command(about = "TechScript 2.0 compiler and runtime single-binary executable", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Executes a TechScript source file (.txs)
    Run {
        /// Source file path ending in .txs
        file: String,
        #[arg(long)]
        verbose: bool,
    },
    /// Starts the interactive REPL shell
    Repl,
    /// Runs semantic analysis checks without executing
    Check { file: String },
    /// Formats .txs source files in-place
    Fmt { file: String },
    /// Lints source files and optionally fixes deprecated keywords
    Lint {
        file: String,
        #[arg(long)]
        fix: bool,
    },
    /// Discovers and runs integration tests (*_test.txs)
    Test { dir: Option<String> },
    /// Prints version details
    Version,
    /// Scaffolds a new project structure
    New { name: String },
}
