//! # TechScript Compiler Driver Subcommand Handlers
//!
//! Exposes modular execution handlers for each of the 21 `tsc` compiler driver commands.

pub mod build;
pub mod check;
pub mod clean;
pub mod doc;
pub mod doctor;
pub mod dump;
pub mod fmt;
pub mod init;
pub mod install;
pub mod lint;
pub mod new_cmd;
pub mod publish;
pub mod repl;
pub mod run;
pub mod test;
pub mod uninstall;
pub mod update;
pub mod version;
pub mod emit;
pub mod benchmark;
pub mod completion;
pub mod examples;
pub mod docs;
pub mod config;
pub mod self_cmd;

