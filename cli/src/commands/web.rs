//! # `tsc web` Webforge Command Dispatcher
//!
//! Exposes dedicated Webforge commands for creating, developing, building,
//! and testing websites and full-stack web applications.

use crate::exit_code::ExitCode;
use crate::WebCommands;

pub fn execute(cmd: &WebCommands, verbose: bool) -> ExitCode {
    match cmd {
        WebCommands::Create { name } => {
            println!("Scaffolding new Webforge website: '{}'...", name);
            crate::commands::new_cmd::execute(Some("website"), Some("website"), Some(name))
        }
        WebCommands::Dev { file, port } => crate::commands::dev::execute(file.as_deref(), *port),
        WebCommands::Build { file, release } => crate::commands::build::execute(
            file.as_deref(),
            Some(if *release { "release" } else { "debug" }),
            false,
            false,
            "web",
            verbose,
        ),
        WebCommands::Test => {
            println!("Running Webforge test suites (*_test.txs, *test.wtxs)...");
            crate::commands::test::execute(None, None, false, false)
        }
    }
}
