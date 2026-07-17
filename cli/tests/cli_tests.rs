use clap::Parser;
use techscript_cli::{Cli, Commands};

#[test]
fn test_cli_parsing() {
    let args = vec!["tech", "version"];
    let cli = Cli::try_parse_from(args).expect("args parse successfully");
    match cli.command {
        Commands::Version => {}
        _ => panic!("Expected Version subcommand"),
    }
}

#[test]
fn test_cli_native_options() {
    let args = vec!["tech", "run", "main.txs", "--native"];
    let cli = Cli::try_parse_from(args).expect("args parse successfully");
    match cli.command {
        Commands::Run { native, .. } => {
            assert!(native);
        }
        _ => panic!("Expected Run subcommand"),
    }

    let args2 = vec!["tech", "build", "main.txs", "--target", "native"];
    let cli2 = Cli::try_parse_from(args2).expect("args parse successfully");
    match cli2.command {
        Commands::Build { target, .. } => {
            assert_eq!(target, "native");
        }
        _ => panic!("Expected Build subcommand"),
    }
}

#[test]
fn test_cli_emit_commands() {
    let args = vec!["tech", "emit-llvm", "main.txs"];
    let cli = Cli::try_parse_from(args).expect("args parse successfully");
    match cli.command {
        Commands::EmitLlvm { file } => {
            assert_eq!(file, "main.txs");
        }
        _ => panic!("Expected EmitLlvm subcommand"),
    }
}

