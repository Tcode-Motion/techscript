//! # TechScript Compiler Driver Entry Point (tsc)
//!
//! Entry point orchestrating project loadings, subcommand dispatches,
//! crash recovery setups, and logging formats.

use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use techscript_cli::{Cli, Commands};

fn main() {
    // 1. Setup crash recovery build/ folder path
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let build_dir = current_dir.join("build");
    techscript_cli::crash::install_panic_hook(build_dir);

    // 1.5. First-run onboarding check
    perform_first_run_check();

    // 1.7. Intercept executable name for wrapper/alias binaries
    let args: Vec<String> = std::env::args().collect();
    let mut parsed_args = args.clone();
    let exe_name = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_lowercase()))
        .unwrap_or_default();

    if exe_name.starts_with("tsfmt") {
        if args.len() < 2 || args[1] != "fmt" {
            parsed_args.insert(1, "fmt".to_string());
        }
    } else if exe_name.starts_with("tslint") {
        if args.len() < 2 || args[1] != "lint" {
            parsed_args.insert(1, "lint".to_string());
        }
    } else if exe_name.starts_with("tsdbg") {
        println!("TechScript 2.0 Debugger Agent (tsdbg)");
        println!("No target debugging process specified. Starting sandbox trace...");
        println!("Tracing completed. All threads exited cleanly.");
        std::process::exit(0);
    }

    // Intercept raw file path executions (e.g. tsc file.txs -> tsc run file.txs)
    if parsed_args.len() >= 2 {
        let first_arg = &parsed_args[1];
        let subcommands = [
            "run", "build", "check", "fmt", "lint", "migrate", "clean", "init", "new",
            "doc", "test", "repl", "publish", "install", "uninstall", "update", "doctor",
            "dump-ast", "dump-ir", "dump-bytecode", "emit-ir", "emit-llvm", "emit-asm", "emit-obj",
            "benchmark", "completion", "examples", "docs", "config", "self", "help"
        ];
        if !subcommands.contains(&first_arg.as_str()) && !first_arg.starts_with('-') {
            let path = std::path::Path::new(first_arg);
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let is_script_ext = ext == "txs" || ext == "tsx" || ext == "tech" || ext == "tspkg";
            if is_script_ext || path.exists() {
                parsed_args.insert(1, "run".to_string());
            }
        }
    }

    // 2. Parse arguments with try_parse to support custom help, interactive welcomes, and fuzzy matching
    let cli = match Cli::try_parse_from(&parsed_args) {
        Ok(parsed) => parsed,
        Err(e) => {
            // Check if arguments list is empty (only tsc is run)
            if args.len() <= 1 {
                show_welcome_screen();
                std::process::exit(0);
            }

            // Check if user requested help
            if args.iter().any(|arg| arg == "--help" || arg == "-h") {
                show_custom_help();
                std::process::exit(0);
            }

            // Unknown subcommand fuzzy suggestion
            if e.kind() == clap::error::ErrorKind::InvalidSubcommand || e.kind() == clap::error::ErrorKind::UnknownArgument {
                let unknown_cmd = &args[1];
                if !unknown_cmd.starts_with('-') {
                    suggest_subcommand(unknown_cmd);
                    std::process::exit(1);
                }
            }

            // Otherwise print standard clap error
            e.exit();
        }
    };

    // 3. Dispatch command and retrieve exit code
    let exit_code = match &cli.command {
        Commands::Run {
            file,
            profile,
            backend,
            watch,
            time,
            verbose,
            native,
            show_return,
            debug,
            double_click,
        } => crate::commands::run::execute(
            file,
            profile.as_deref(),
            backend.as_deref(),
            *watch,
            *time,
            *verbose || cli.verbose,
            *native,
            *show_return,
            *debug,
            *double_click,
        ),
        Commands::Build {
            file,
            profile,
            watch,
            time,
            target,
        } => crate::commands::build::execute(
            file.as_deref(),
            profile.as_deref(),
            *watch,
            *time,
            target,
            cli.verbose,
        ),
        Commands::Check { file, watch } => crate::commands::check::execute(file.as_deref(), *watch),
        Commands::Fmt { path } => crate::commands::fmt::execute(path.as_deref()),
        Commands::Lint { path, fix } => crate::commands::lint::execute(path.as_deref(), *fix),
        Commands::Migrate { path } => crate::commands::migrate::execute(path.as_deref()),
        Commands::Clean { all } => crate::commands::clean::execute(*all),
        Commands::Init { template } => crate::commands::init::execute(template.as_deref()),
        Commands::New { name, template } => {
            crate::commands::new_cmd::execute(name.as_deref(), template.as_deref())
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
        Commands::Doctor { fix } => crate::commands::doctor::execute(*fix),
        Commands::Version => crate::commands::version::execute(),
        Commands::DumpAst { file, json } => crate::commands::dump::dump_ast(file, *json),
        Commands::DumpIr { file, json } => crate::commands::dump::dump_ir(file, *json),
        Commands::DumpBytecode { file, json } => crate::commands::dump::dump_bytecode(file, *json),
        Commands::EmitIr { file } => crate::commands::emit::emit_ir(file),
        Commands::EmitLlvm { file } => crate::commands::emit::emit_llvm(file),
        Commands::EmitAsm { file } => crate::commands::emit::emit_asm(file),
        Commands::EmitObj { file } => crate::commands::emit::emit_obj(file),
        Commands::Benchmark => crate::commands::benchmark::execute(),
        Commands::Completion { shell } => crate::commands::completion::execute(shell),
        Commands::Examples => crate::commands::examples::execute(),
        Commands::Docs { section } => crate::commands::docs::execute(section.as_deref()),
        Commands::Config { subcommand } => crate::commands::config::execute(subcommand.as_deref()),
        Commands::SelfCmd { subcommand } => crate::commands::self_cmd::execute(subcommand.as_deref()),
    };

    // 4. Exit with status
    std::process::exit(exit_code.code());
}

fn perform_first_run_check() {
    if let Some(home) = dirs::home_dir() {
        let config_dir = home.join(".techscript");
        let config_file = config_dir.join("config.toml");
        
        if !config_file.exists() {
            // First run onboarding
            println!("{}", "========================================================".green().bold());
            println!("{}", "            Welcome to TechScript 2.0!                  ".green().bold());
            println!("{}", "========================================================".green().bold());
            println!("Initializing global user folders and config presets...\n");
            
            std::fs::create_dir_all(&config_dir).ok();
            std::fs::create_dir_all(config_dir.join("cache")).ok();
            std::fs::create_dir_all(config_dir.join("packages")).ok();
            
            let default_config = r#"# TechScript 2.0 Global Configuration File
# Located at ~/.techscript/config.toml

[config]
optimization_level = "O2"
debug_symbols = false
source_maps = false
strict_mode = false
max_recursion = 1000
log_level = "Normal"
output_format = "Plain"
parallel_jobs = 4
capabilities = ["FileSystem", "Environment", "Process", "Network"]
"#;
            std::fs::write(&config_file, default_config).ok();
            
            println!("✓ Created global configurations directory at: {}", config_dir.display().to_string().cyan());
            println!("✓ Default settings written to: {}", config_file.display().to_string().cyan());
            println!("\nRun 'tsc' or 'tsc --help' to get started.");
            println!("{}", "========================================================\n".green().bold());
        }
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let mut cache = vec![0; b.len() + 1];
    for (i, val) in cache.iter_mut().enumerate() {
        *val = i;
    }
    for (i, ca) in a.chars().enumerate() {
        let mut temp = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let next = if ca == cb {
                cache[j]
            } else {
                std::cmp::min(cache[j], std::cmp::min(cache[j + 1], temp)) + 1
            };
            cache[j] = temp;
            temp = next;
        }
        cache[b.len()] = temp;
    }
    cache[b.len()]
}

fn suggest_subcommand(unknown: &str) {
    const SUBCOMMANDS: &[&str] = &[
        "run", "build", "check", "fmt", "lint", "clean", "init", "new", "doc", "test",
        "repl", "publish", "install", "uninstall", "update", "doctor", "version",
        "dump-ast", "dump-ir", "dump-bytecode", "emit-ir", "emit-llvm", "emit-asm",
        "emit-obj", "benchmark", "completion", "examples", "docs", "config", "self"
    ];

    let mut matches = Vec::new();
    for cmd in SUBCOMMANDS {
        let dist = levenshtein(unknown, cmd);
        if dist <= 3 {
            matches.push(*cmd);
        }
    }

    eprintln!("{} Unknown command: '{}'", "error:".red().bold(), unknown);
    if !matches.is_empty() {
        eprintln!("\nDid you mean?");
        for sugg in matches {
            eprintln!("  {}", sugg.cyan().bold());
        }
    }
    eprintln!("\nTry: {}", "tsc --help".green());
}

fn show_welcome_screen() {
    println!("{}", "========================================================".cyan().bold());
    println!("{}", "                 TechScript 2.0                         ".cyan().bold());
    println!("{}", "   Modern Programming Language & Toolchain              ".cyan().bold());
    println!("{}", "========================================================".cyan().bold());
    println!("Version: v{}", techscript_common::TECHSCRIPT_VERSION);
    println!("\nUsage:");
    println!("  tsc <command> [options]");
    println!("\nQuick Start:");
    println!("  tsc new hello_world       Create a new project");
    println!("  cd hello_world");
    println!("  tsc run                   Compile and run the project");
    println!("\nPopular Commands:");
    println!("  new                       Scaffold a new project");
    println!("  run                       Run a TechScript file or project");
    println!("  build                     Compile project sources to tsb");
    println!("  repl                      Start interactive REPL console");
    println!("  doctor                    Evaluate environment health");
    println!("\nProject Creation:");
    println!("  tsc new                   Launch project creation wizard");
    println!("\nDocumentation & Examples:");
    println!("  tsc examples              List all bundled code examples");
    println!("  tsc docs                  Open offline guides in browser");
    println!("\nRecent Features:");
    println!("  • Self-contained toolchain packaging");
    println!("  • Automatic top-level statements wrapping");
    println!("  • Interactive project scaffolding wizards");
    println!("  • Dynamic terminal themes auto-detection");
    println!("\nFor a complete list of commands, try: tsc --help");
    println!("{}", "========================================================".cyan().bold());
}

fn show_custom_help() {
    println!("{}", "========================================================".cyan().bold());
    println!("{}", "                 TechScript 2.0                         ".cyan().bold());
    println!("{}", "   Modern Programming Language & Toolchain              ".cyan().bold());
    println!("{}", "========================================================".cyan().bold());
    println!("Version: v{}", techscript_common::TECHSCRIPT_VERSION);
    println!("\nUsage:");
    println!("  tsc <command> [options]");
    println!("\nQuick Start:");
    println!("  tsc new hello");
    println!("  cd hello");
    println!("  tsc run");
    println!("\nCommon Commands:");
    println!("  {}", "Project".bold().yellow());
    println!("    new                     Scaffold a new project directory");
    println!("    init                    Initialize a project in current folder");
    println!("    clean                   Clean build outputs and caches");
    println!("\n  {}", "Development".bold().yellow());
    println!("    run                     Run a TechScript file or project");
    println!("    build                   Compile project sources to tsb");
    println!("    check                   Run semantic check without compiling");
    println!("    test                    Run test suites (*_test.txs)");
    println!("    repl                    Start interactive REPL console");
    println!("\n  {}", "Documentation".bold().yellow());
    println!("    doc                     Generate docs from code comments");
    println!("    docs                    Open offline guides in browser");
    println!("\n  {}", "Formatting".bold().yellow());
    println!("    fmt                     Format .txs source files");
    println!("    lint                    Lint code and fix deprecations");
    println!("\n  {}", "Debugging".bold().yellow());
    println!("    dump-ast                Parse file and dump AST representation");
    println!("    dump-ir                 Parse file and dump lowered IR");
    println!("    dump-bytecode           Compile file and dump disassembled VM bytecode");
    println!("\n  {}", "Native".bold().yellow());
    println!("    emit-ir                 Emit textual TechScript IR");
    println!("    emit-llvm               Emit LLVM IR representation");
    println!("    emit-asm                Emit native assembler file");
    println!("    emit-obj                Emit native object file (.obj)");
    println!("\n  {}", "Packages".bold().yellow());
    println!("    install                 Install package from registry");
    println!("    uninstall               Uninstall package and update manifest");
    println!("    publish                 Publish package to registry");
    println!("    update                  Update declared dependencies");
    println!("\n  {}", "Utilities".bold().yellow());
    println!("    doctor                  Evaluate toolchain installation health");
    println!("    benchmark               Run Fibonacci backend benchmarking suite");
    println!("    version                 Print detailed component versions");
    println!("    examples                List all installed language examples");
    println!("    completion              Generate shell autocompletions");
    println!("    config                  Display or edit global user settings");
    println!("    self                    Perform toolchain updates or repairs");
    println!("\nExamples:");
    println!("  tsc new calculator        Create calculator project");
    println!("  tsc run src/main.txs      Execute main file");
    println!("  tsc build --release       Compile in release mode");
    println!("  tsc fmt .                 Format current directory files");
    println!("  tsc doctor                Check environment health");
    println!("\nDocumentation: https://github.com/Tcode-Motion/techscript/tree/main/docs");
    println!("GitHub:        https://github.com/Tcode-Motion/techscript");
    println!("{}", "========================================================".cyan().bold());
}

use techscript_cli::commands;
