#![allow(unused_imports, unused_variables, unused_mut, dead_code)]

//! # TechScript CLI Library
//!
//! Exposes command-line parsing structures for testing and integration.

pub mod artifacts;
pub mod cache;
pub mod commands;
pub mod compile_db;
pub mod config;
pub mod crash;
pub mod diagnostics;
pub mod events;
pub mod exit_code;
pub mod logging;
pub mod pipeline;
pub mod plugin;
pub mod profiler;
pub mod project;
pub mod scheduler;
pub mod templates;
pub mod watch;

use clap::{Parser as ClapParser, Subcommand};

#[derive(ClapParser, Debug)]
#[command(name = "tsc")]
#[command(about = "TechScript 2.0 compiler driver", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Set verbosity to quiet (errors only)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Set verbosity to verbose
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Set verbosity to trace
    #[arg(long, global = true)]
    pub trace: bool,

    /// Set log format to JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Set parallel compilation jobs
    #[arg(short, long, global = true)]
    pub jobs: Option<usize>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Executes a TechScript source file (.txs)
    Run {
        /// Source file path ending in .txs or .ts
        file: String,

        /// Profile to use: debug, release, release-fast, release-small
        #[arg(short, long)]
        profile: Option<String>,

        /// Execution backend to use: vm, interpreter
        #[arg(short, long)]
        backend: Option<String>,

        /// Run in watch mode, re-executing on change
        #[arg(short, long)]
        watch: bool,

        /// Enable timing and memory profiling outputs
        #[arg(long)]
        time: bool,

        /// Verbose execution details
        #[arg(short, long)]
        verbose: bool,
    },

    /// Compiles project sources to a bytecode bundle
    Build {
        /// Optional input file path (defaults to project entry file)
        file: Option<String>,

        /// Profile to use: debug, release, release-fast, release-small
        #[arg(short, long)]
        profile: Option<String>,

        /// Run in watch mode, rebuilding on change
        #[arg(short, long)]
        watch: bool,

        /// Enable timing profiling outputs
        #[arg(long)]
        time: bool,
    },

    /// Runs semantic analysis checks without compiling or executing
    Check {
        /// Optional input file path
        file: Option<String>,

        /// Run in watch mode, checking on change
        #[arg(short, long)]
        watch: bool,
    },

    /// Formats .txs source files in-place
    Fmt {
        /// Optional file or directory path (defaults to current directory)
        path: Option<String>,
    },

    /// Lints source files and optionally fixes deprecated keywords
    Lint {
        /// Optional file or directory path
        path: Option<String>,

        /// Attempt to automatically fix lint issues
        #[arg(long)]
        fix: bool,
    },

    /// Cleans build outputs and incremental caches
    Clean,

    /// Initializes a new project in the current directory
    Init {
        /// Project template: console, library, package, workspace, empty
        #[arg(short, long)]
        template: Option<String>,
    },

    /// Scaffolds a new project structure in a new directory
    New {
        /// Name of the new project directory
        name: String,

        /// Project template: console, library, package, workspace, empty
        #[arg(short, long)]
        template: Option<String>,
    },

    /// Generates documentation from triple-slash (///) comments
    Doc {
        /// Optional path to extract docs from
        path: Option<String>,
    },

    /// Discovers and runs integration tests (*_test.txs)
    Test {
        /// Directory containing tests (defaults to current directory)
        dir: Option<String>,

        /// Filter test names containing the pattern
        #[arg(short, long)]
        filter: Option<String>,

        /// Run tests in parallel
        #[arg(short, long)]
        parallel: bool,

        /// Run ignored tests as well
        #[arg(long)]
        ignored: bool,
    },

    /// Starts the interactive REPL shell
    Repl,

    /// Packages and publishes package to the registry
    Publish,

    /// Installs a package from the registry
    Install {
        /// Package name to install
        package: String,
    },

    /// Uninstalls a package and updates manifest
    Uninstall {
        /// Package name to uninstall
        package: String,
    },

    /// Updates declared dependencies in manifest
    Update,

    /// Evaluates toolchain installation and environment health
    Doctor,

    /// Prints detailed version and environment information
    Version,

    /// Parses file and dumps AST representation
    DumpAst {
        file: String,

        /// Format output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Parses file and dumps lowered IR representation
    DumpIr {
        file: String,

        /// Format output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Compiles file and dumps disassembled bytecode
    DumpBytecode {
        file: String,

        /// Format output as JSON
        #[arg(long)]
        json: bool,
    },
}
