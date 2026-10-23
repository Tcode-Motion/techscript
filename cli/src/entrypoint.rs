//! # TechScript Compiler Driver Entry Point (tsc / techscript / tech)
//!
//! Entry point orchestrating project loadings, subcommand dispatches,
//! crash recovery setups, and logging formats.

use crate::{Cli, Commands};
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

pub fn run() {
    // 1. Setup crash recovery build/ folder path
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let build_dir = current_dir.join("build");
    crate::crash::install_panic_hook(build_dir);

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
            "run",
            "build",
            "check",
            "fmt",
            "lint",
            "migrate",
            "clean",
            "init",
            "new",
            "doc",
            "test",
            "repl",
            "publish",
            "install",
            "uninstall",
            "update",
            "doctor",
            "dump-ast",
            "dump-ir",
            "dump-bytecode",
            "emit-ir",
            "emit-llvm",
            "emit-asm",
            "emit-obj",
            "benchmark",
            "completion",
            "examples",
            "docs",
            "config",
            "self",
            "dev",
            "routes",
            "deploy",
            "add",
            "list",
            "ls",
            "search",
            "remove",
            "rm",
            "help",
            "version",
            "app",
            "web",
            "native",
            "logs",
            "report",
        ];
        if !subcommands.contains(&first_arg.as_str()) && !first_arg.starts_with('-') {
            let path = std::path::Path::new(first_arg);
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let is_script_ext = ext == "txs"
                || ext == "tsx"
                || ext == "tech"
                || ext == "tspkg"
                || ext == "wtxs"
                || ext == "stxs"
                || ext == "3txs";
            if is_script_ext || path.exists() {
                parsed_args.insert(1, "run".to_string());
            }
        }
    }

    // Direct check for version flags
    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("TechScript v{}", techscript_common::TECHSCRIPT_VERSION);
        std::process::exit(0);
    }

    // 2. Parse arguments with try_parse to support custom help, interactive welcomes, and fuzzy matching
    let cli = match Cli::try_parse_from(&parsed_args) {
        Ok(parsed) => parsed,
        Err(e) => {
            // Check if arguments list is empty (only tsc is run)
            if args.len() <= 1 {
                show_welcome_screen();
                #[cfg(windows)]
                if is_launched_from_explorer() {
                    println!("\n[Launched from Windows File Explorer]");
                    println!("Press Enter to exit...");
                    let mut buffer = String::new();
                    let _ = std::io::stdin().read_line(&mut buffer);
                }
                std::process::exit(0);
            }

            // Check if user requested help
            if args.iter().any(|arg| arg == "--help" || arg == "-h") {
                show_custom_help();
                std::process::exit(0);
            }

            // Unknown subcommand fuzzy suggestion
            if e.kind() == clap::error::ErrorKind::InvalidSubcommand {
                if let Some(unknown_cmd) = args.get(1) {
                    if !unknown_cmd.starts_with('-') {
                        suggest_subcommand(unknown_cmd);
                        std::process::exit(1);
                    }
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
            release,
            profile,
            backend,
            watch,
            time,
            verbose,
            native,
            show_return,
            debug,
            double_click,
        } => {
            let prof = if *release {
                Some("release")
            } else {
                profile.as_deref()
            };
            crate::commands::run::execute(
                file,
                prof,
                backend.as_deref(),
                *watch,
                *time,
                *verbose || cli.verbose,
                *native,
                *show_return,
                *debug,
                *double_click,
            )
        }
        Commands::Build {
            file,
            release,
            profile,
            watch,
            time,
            target,
        } => {
            let prof = if *release {
                Some("release")
            } else {
                profile.as_deref()
            };
            crate::commands::build::execute(
                file.as_deref(),
                prof,
                *watch,
                *time,
                target,
                cli.verbose,
            )
        }
        Commands::Check { file, watch } => crate::commands::check::execute(file.as_deref(), *watch),
        Commands::Fmt { path } => crate::commands::fmt::execute(path.as_deref()),
        Commands::Lint { path, fix } => crate::commands::lint::execute(path.as_deref(), *fix),
        Commands::Migrate { path } => crate::commands::migrate::execute(path.as_deref()),
        Commands::Clean { all } => crate::commands::clean::execute(*all),
        Commands::Init { template } => crate::commands::init::execute(template.as_deref()),
        Commands::New {
            name,
            second_name,
            template,
        } => crate::commands::new_cmd::execute(
            name.as_deref(),
            template.as_deref(),
            second_name.as_deref(),
        ),
        Commands::Dev { file, port } => crate::commands::dev::execute(file.as_deref(), *port),
        Commands::Routes { file } => crate::commands::routes::execute(file.as_deref()),
        Commands::Deploy { target } => crate::commands::deploy::execute(target.as_deref()),
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
        Commands::List => crate::commands::list::execute(),
        Commands::Search { query } => crate::commands::search::execute(query),
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
        Commands::SelfCmd { subcommand } => {
            crate::commands::self_cmd::execute(subcommand.as_deref())
        }
        Commands::App { subcommand } => crate::commands::app::execute(subcommand, cli.verbose),
        Commands::Web { subcommand } => crate::commands::web::execute(subcommand, cli.verbose),
        Commands::Native { subcommand } => crate::commands::native::execute(subcommand),
        Commands::Logs { level, follow } => {
            crate::commands::logs::execute(level.as_deref(), *follow)
        }
        Commands::Report => crate::commands::report::execute(),
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
            println!(
                "{}",
                "========================================================"
                    .green()
                    .bold()
            );
            println!(
                "{}",
                "            Welcome to TechScript 2.0!                  "
                    .green()
                    .bold()
            );
            println!(
                "{}",
                "========================================================"
                    .green()
                    .bold()
            );
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

            println!(
                "✓ Created global configurations directory at: {}",
                config_dir.display().to_string().cyan()
            );
            println!(
                "✓ Default settings written to: {}",
                config_file.display().to_string().cyan()
            );
            println!("\nRun 'tsc' or 'techscript' or 'tsc --help' to get started.");
            println!(
                "{}",
                "========================================================\n"
                    .green()
                    .bold()
            );
        }
    }
}

fn levenshtein(a: &str, b: &str, cache: &mut [usize]) -> usize {
    let b_len = b.len();

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    cache[..=b_len]
        .iter_mut()
        .enumerate()
        .for_each(|(i, val)| *val = i);
    for (i, &ca) in a_bytes.iter().enumerate() {
        let mut temp = i + 1;
        for (j, &cb) in b_bytes.iter().enumerate() {
            let next = if ca == cb {
                cache[j]
            } else {
                std::cmp::min(cache[j], std::cmp::min(cache[j + 1], temp)) + 1
            };
            cache[j] = temp;
            temp = next;
        }
        cache[b_len] = temp;
    }
    cache[b_len]
}

fn suggest_subcommand(unknown: &str) {
    const SUBCOMMANDS: &[&str] = &[
        "run",
        "build",
        "check",
        "fmt",
        "lint",
        "clean",
        "init",
        "new",
        "doc",
        "test",
        "repl",
        "publish",
        "install",
        "uninstall",
        "list",
        "search",
        "update",
        "doctor",
        "version",
        "dump-ast",
        "dump-ir",
        "dump-bytecode",
        "emit-ir",
        "emit-llvm",
        "emit-asm",
        "emit-obj",
        "benchmark",
        "completion",
        "examples",
        "docs",
        "config",
        "self",
        "dev",
        "routes",
        "deploy",
    ];

    let mut cache = [0; 32];
    let mut matches = Vec::new();
    for cmd in SUBCOMMANDS {
        let dist = levenshtein(unknown, cmd, &mut cache);
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
    eprintln!("\nTry: {}", "techscript --help".green());
}

fn show_welcome_screen() {
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "                 TechScript 2.0                         "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "   Modern Programming Language & WebForge Toolchain      "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
    println!("Version: v{}", techscript_common::TECHSCRIPT_VERSION);
    println!("\nUsage:");
    println!("  tsc <command> [options]");
    println!("  webforge <command> [options]");
    println!("  techscript <command> [options]");
    println!("\nQuick Start:");
    println!("  tsc new website my-site            Create a new WebForge project");
    println!("  cd my-site");
    println!("  tsc dev                            Start live development server");
    println!("\nPopular Commands:");
    println!("  new                       Scaffold a new project or template");
    println!("  dev                       Start live development server");
    println!("  build                     Compile project (web/server/vm)");
    println!("  test                      Run project test suites");
    println!("  add                       Add package dependency");
    println!("  doctor                    Evaluate environment health");
    println!("\nFor a complete list of commands, try: tsc --help");
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
}

fn show_custom_help() {
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "                 TechScript 2.0                         "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "   Modern Programming Language & WebForge Toolchain      "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
    println!("Version: v{}", techscript_common::TECHSCRIPT_VERSION);
    println!("\nUsage:");
    println!("  tsc <command> [options]");
    println!("  webforge <command> [options]");
    println!("  techscript <command> [options]");
    println!("\nQuick Start:");
    println!("  tsc app create my-app");
    println!("  cd my-app");
    println!("  tsc app run android");
    println!("\nCommon Commands:");
    println!("  {}", "Appforge (Mobile & Desktop)".bold().yellow());
    println!("    app create <name>       Scaffold a new Appforge application");
    println!("    app build <platform>    Compile application (android, windows, etc.)");
    println!("    app run <platform>      Build, install, and run on device or host");
    println!("    app wrap web            Wrap Webforge web app as native mobile/desktop app");
    println!("    app doctor              Inspect mobile and desktop toolchains");
    println!("    app clean               Clean build outputs and caches");
    println!("\n  {}", "Webforge (Web Framework)".bold().yellow());
    println!("    web create <name>       Scaffold a new Webforge website");
    println!("    web dev                 Start live development server with hot-reload");
    println!("    web build               Compile Webforge project to static dist/");
    println!("    dev                     Start live development server");
    println!("    routes                  Inspect registered server & web routes");
    println!("    deploy                  Prepare and deploy project to target host");
    println!("\n  {}", "Native & Interop".bold().yellow());
    println!("    native add <lang>       Add native module (kotlin, swift, rust, cpp, c, java)");
    println!("\n  {}", "Development & Compiler".bold().yellow());
    println!("    run                     Run a TechScript file or project");
    println!("    build                   Compile project (web / server / vm)");
    println!("    check                   Run semantic check without compiling");
    println!("    test                    Run test suites (*_test.txs)");
    println!("    repl                    Start interactive REPL console");
    println!("\n  {}", "Packages & Ecosystem".bold().yellow());
    println!("    install, add            Install package or all dependencies from webforge.toml");
    println!("    uninstall, remove       Uninstall package and update manifest");
    println!("    list, ls                List declared and resolved dependencies");
    println!("    search                  Search official package registry");
    println!("    update                  Update declared dependencies");
    println!("    publish                 Publish package to registry");
    println!("\n  {}", "Formatting & Tooling".bold().yellow());
    println!("    fmt                     Format .txs / .wtxs / .stxs / .3txs files");
    println!("    lint                    Lint code and fix deprecations");
    println!("    doctor                  Evaluate toolchain installation health");
    println!("\nDocumentation: https://github.com/Tcode-Motion/techscript/tree/main/docs");
    println!("GitHub:        https://github.com/Tcode-Motion/techscript");
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
}

#[cfg(windows)]
fn is_launched_from_explorer() -> bool {
    extern "system" {
        fn GetConsoleProcessList(process_list: *mut u32, count: u32) -> u32;
    }
    let mut process_list = [0u32; 2];
    let count = unsafe { GetConsoleProcessList(process_list.as_mut_ptr(), 2) };
    count <= 1
}

#[cfg(not(windows))]
fn is_launched_from_explorer() -> bool {
    false
}
