use techscript_cli::{Cli, Commands};
use clap::Parser;

#[test]
fn test_cli_parsing() {
    let args = vec!["tech", "version"];
    let cli = Cli::try_parse_from(args).expect("args parse successfully");
    match cli.command {
        Commands::Version => {}
        _ => panic!("Expected Version subcommand"),
    }
}
