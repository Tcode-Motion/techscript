//! # TechScript Compiler Driver Subcommand Handlers
//!
//! Exposes modular execution handlers for each of the 21 `tsc` compiler driver commands.

pub mod benchmark;
pub mod build;
pub mod check;
pub mod clean;
pub mod completion;
pub mod config;
pub mod doc;
pub mod docs;
pub mod doctor;
pub mod dump;
pub mod emit;
pub mod examples;
pub mod fmt;
pub mod init;
pub mod install;
pub mod lint;
pub mod migrate;
pub mod new_cmd;
pub mod publish;
pub mod repl;
pub mod run;
pub mod self_cmd;
pub mod test;
pub mod uninstall;
pub mod update;
pub mod version;
