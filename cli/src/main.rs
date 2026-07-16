//! # TechScript Compiler Driver Entry Point (tsc)
//!
//! Entry point orchestrating project loadings, subcommand dispatches,
//! crash recovery setups, and logging formats.

use clap::Parser;
use std::path::PathBuf;
use techscript_cli::{Cli, Commands};

fn main() {
    // 1. Setup crash recovery build/ folder path
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let build_dir = current_dir.join("build");
    techscript_cli::crash::install_panic_hook(build_dir);

    // 2. Parse arguments
    let cli = Cli::parse();

    // 3. Dispatch command and retrieve exit code
    let exit_code = match &cli.command {
        Commands::Run {
            file,
            profile,
            backend,
            watch,
            time,
            verbose,
        } => crate::commands::run::execute(
            file,
            profile.as_deref(),
            backend.as_deref(),
            *watch,
            *time,
            *verbose || cli.verbose,
        ),
        Commands::Build {
            file,
            profile,
            watch,
            time,
        } => crate::commands::build::execute(file.as_deref(), profile.as_deref(), *watch, *time),
        Commands::Check { file, watch } => crate::commands::check::execute(file.as_deref(), *watch),
        Commands::Fmt { path } => crate::commands::fmt::execute(path.as_deref()),
        Commands::Lint { path, fix } => crate::commands::lint::execute(path.as_deref(), *fix),
        Commands::Clean => crate::commands::clean::execute(),
        Commands::Init { template } => crate::commands::init::execute(template.as_deref()),
        Commands::New { name, template } => {
            crate::commands::new_cmd::execute(name, template.as_deref())
        }
        Commands::Doc { path } => crate::commands::doc::execute(path.as_deref()),
        Commands::Test {
            dir,
            filter,
            parallel,
            ignored,
        } => crate::commands::test::execute(dir.as_deref(), filter.as_deref(), *parallel, *ignored),
        Commands::Repl => crate::commands::repl::execute(),
        Commands::Publish => crate::commands::publish::execute(),
        Commands::Install { package } => crate::commands::install::execute(package),
        Commands::Uninstall { package } => crate::commands::uninstall::execute(package),
        Commands::Update => crate::commands::update::execute(),
        Commands::Doctor => crate::commands::doctor::execute(),
        Commands::Version => crate::commands::version::execute(),
        Commands::DumpAst { file, json } => crate::commands::dump::dump_ast(file, *json),
        Commands::DumpIr { file, json } => crate::commands::dump::dump_ir(file, *json),
        Commands::DumpBytecode { file, json } => crate::commands::dump::dump_bytecode(file, *json),
    };

    // 4. Exit with status
    std::process::exit(exit_code.code());
}

use techscript_cli::commands;
