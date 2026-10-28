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

#[test]
fn test_cli_lint_fix() {
    let args = vec!["tech", "lint", "src/main.txs", "--fix"];
    let cli = Cli::try_parse_from(args).expect("args parse successfully");
    match cli.command {
        Commands::Lint { path, fix } => {
            assert_eq!(path.as_deref(), Some("src/main.txs"));
            assert!(fix);
        }
        _ => panic!("Expected Lint subcommand"),
    }

    // Verify actual lint execute with --fix on a temporary file with deprecated syntax
    let temp_dir = std::env::temp_dir().join("techscript_lint_test");
    let _ = std::fs::create_dir_all(&temp_dir);
    let test_file = temp_dir.join("deprecated.txs");
    std::fs::write(&test_file, "let x = 10;").expect("write test file");

    let status = techscript_cli::commands::lint::execute(Some(test_file.to_str().unwrap()), true);
    assert_eq!(status, techscript_cli::exit_code::ExitCode::Success);

    let content = std::fs::read_to_string(&test_file).expect("read test file");
    assert_eq!(content, "x = 10");

    let _ = std::fs::remove_dir_all(&temp_dir);
}
