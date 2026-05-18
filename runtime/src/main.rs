use std::fs;
use std::path::PathBuf;

mod process {
    pub fn exit(code: i32) -> ! {
        techscript::run::exit(code);
    }
}
use std::sync::mpsc;
use std::time::Duration;

use clap::{Parser, Subcommand};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use techscript::disasm;
use techscript::doctor::DoctorReport;
use techscript::run::{self, VERSION};
use techscript::scaffold;
use techscript::repl::start_repl;
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "tech",
    about = "TechScript — a friendly programming language (.txs)",
    version = VERSION,
    disable_help_subcommand = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(value_name = "FILE")]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a TechScript file
    Run {
        file: String,
        #[arg(long)]
        debug: bool,
        #[arg(long)]
        watch: bool,
        #[arg(long)]
        double_click: bool,
    },
    /// Compile to bytecode (.txbc) or native binary
    Build {
        file: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long)]
        native: bool,
    },
    /// Syntax check only
    Check { file: String },
    /// Evaluate inline code
    Eval { code: String },
    /// Start interactive REPL
    Repl,
    /// Show version
    Version,
    /// Initialize a new project
    Init {
        name: String,
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Format source code
    Fmt { file: String },
    /// Lint for code quality
    Lint { file: String },
    /// Generate Markdown documentation
    Doc { file: String },
    /// Install a package using Git
    Install { pkg: String, url: Option<String> },
    /// System diagnostics
    Doctor,
    /// Run test files
    Test {
        #[arg(default_value = ".")]
        dir: String,
    },
    /// Run with debug tracing
    Debug { file: String },
    /// Package manager commands
    Pkg {
        #[command(subcommand)]
        cmd: PkgCommands,
    },
    /// Check for updates
    Update,
    /// Launch the native visual IDE
    Studio,
}

#[derive(Subcommand)]
enum PkgCommands {
    Init { name: String },
    Install,
    List,
}

fn print_banner() {
    println!("{}", r#"  ████████╗██╗  ██╗
  ╚══██╔══╝╚██╗██╔╝
     ██║    ╚███╔╝ 
     ██║    ██╔██╗ 
     ██║   ██╔╝ ██╗
     ╚═╝   ╚═╝  ╚═╝"#.bright_blue().bold());
    println!("\nTechScript v{} — A friendly, high-performance programming language\n", VERSION.bright_green());
}

fn print_help() {
    print_banner();
    println!("{}", "Usage:".bold());
    println!("  {}      Run a TechScript file", "tech run <file.txs>".cyan());
    println!("  {}          Shorthand for run", "tech <file.txs>".cyan());
    println!("  {}    Compile to bytecode (.txbc)", "tech build <file.txs>".cyan());
    println!("  {} Compile to standalone .exe binary", "tech build --native <file.txs>".cyan());
    println!("  {}    Syntax check only", "tech check <file.txs>".cyan());
    println!("  {}      Format source code", "tech fmt <file.txs>".cyan());
    println!("  {}     Lint for code quality", "tech lint <file.txs>".cyan());
    println!("  {}                Start interactive REPL", "tech repl".cyan());
    println!("  {}          Run test files", "tech test [dir]".cyan());
    println!("  {}         Initialize a new project", "tech init [name]".cyan());
    println!("  {}      Generate Markdown documentation", "tech doc <file.txs>".cyan());
    println!("  {} Install a package using Git", "tech install <pkg> [url]".cyan());
    println!("  {}       Evaluate inline code", "tech eval \"say 42\"".cyan());
    println!("  {}    Run with debug tracing", "tech debug <file.txs>".cyan());
    println!("  {}              System diagnostics", "tech doctor".cyan());
    println!("  {}             Show version", "tech version".cyan());
    println!("  {}              Launch the native visual IDE", "tech studio".cyan());

    println!("\n{}", "Modules:".bold());
    println!("  {} | {} | {} | {} | {} | {}", "use math".bright_black(), "use fs".bright_black(), "use os".bright_black(), "use json".bright_black(), "use crypto".bright_black(), "use date".bright_black());
    println!("  {}  | {} | {} | {}  | {}", "use api".bright_black(), "use web".bright_black(), "use gui".bright_black(), "use 3d".bright_black(), "use anime".bright_black());

    println!("\n{}", "Examples:".bold());
    println!("  {}", "tech run hello.txs".bright_black());
    println!("  {}", "tech repl".bright_black());
    println!("  {}\n", "tech build hello.txs".bright_black());
}

fn main() {
    init_logging();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || (args.len() == 2 && (args[1] == "-h" || args[1] == "--help")) {
        print_help();
        return;
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { file, debug, watch, double_click }) => {
            if watch { watch_file(&file, debug); } else { run_file(&file, debug, double_click); }
        }
        Some(Commands::Build { file, output, native }) => {
            if native {
                println!("{} Native binary compilation is coming soon!", "🚧 [WIP]".bright_yellow());
            } else {
                build_file(&file, output.as_deref());
            }
        }
        Some(Commands::Check { file }) => check_file(&file),
        Some(Commands::Eval { code }) => run_inline(&code, false),
        Some(Commands::Repl) => start_repl(),
        Some(Commands::Version) => println!("TechScript v{}", VERSION),
        Some(Commands::Init { name, path }) => match scaffold::create_project(&name, path.as_deref()) {
            Ok(dir) => println!("Created project in {}", dir.bright_green()),
            Err(e) => { eprintln!("{} {}", "Error:".red().bold(), e); process::exit(1); }
        },
        Some(Commands::Fmt { file }) => println!("{} Formatting for {} coming soon!", "🚧 [WIP]".bright_yellow(), file.cyan()),
        Some(Commands::Lint { file }) => println!("{} Linting for {} coming soon!", "🚧 [WIP]".bright_yellow(), file.cyan()),
        Some(Commands::Doc { file }) => println!("{} Documentation generation for {} coming soon!", "🚧 [WIP]".bright_yellow(), file.cyan()),
        Some(Commands::Install { pkg, .. }) => println!("{} Package installation for {} coming soon!", "🚧 [WIP]".bright_yellow(), pkg.cyan()),
        Some(Commands::Doctor) => {
            let report = DoctorReport::run();
            print!("{}", report.format());
            if !report.all_ok() { process::exit(1); }
        }
        Some(Commands::Test { dir }) => run_tests(&dir),
        Some(Commands::Debug { file }) => run_file(&file, true, false),
        Some(Commands::Pkg { cmd }) => match cmd {
            PkgCommands::Init { name } => {
                if let Err(e) = techscript::pkg::manifest::pkg_init(&name) {
                    eprintln!("{} {}", "Error:".red().bold(), e); process::exit(1);
                }
                println!("Created techscript.toml for {}", name.bright_green());
            }
            PkgCommands::Install => {
                if let Err(e) = techscript::pkg::manifest::pkg_install() {
                    eprintln!("{} {}", "Error:".red().bold(), e); process::exit(1);
                }
                println!("Dependencies installed.");
            }
            PkgCommands::List => {
                match techscript::pkg::manifest::pkg_list() {
                    Ok(s) => print!("{}", s),
                    Err(e) => { eprintln!("{} {}", "Error:".red().bold(), e); process::exit(1); }
                }
            }
        },
        Some(Commands::Update) => {
            println!("TechScript v{} is installed.", VERSION.bright_green());
            println!("Visit {} for updates.", "https://github.com/techscript/techscript/releases".bright_blue().underline());
        }
        Some(Commands::Studio) => {
            // Check if tech_studio.exe exists in the same folder as this executable
            let current_exe = std::env::current_exe().ok();
            let mut studio_path = None;
            if let Some(ref path) = current_exe {
                if let Some(parent) = path.parent() {
                    let mut p = parent.join("tech_studio.exe");
                    if !p.exists() {
                        p = parent.join("tech_studio");
                    }
                    if p.exists() {
                        studio_path = Some(p);
                    }
                }
            }

            if let Some(path) = studio_path {
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    // Spawn tech_studio.exe as a detached background GUI process (creation flag 0x00000008)
                    let mut cmd = std::process::Command::new(path);
                    cmd.creation_flags(0x00000008);
                    if cmd.spawn().is_ok() {
                        // Exit the CLI console process immediately to close any open CMD window
                        return;
                    }
                }

                #[cfg(not(target_os = "windows"))]
                {
                    let mut cmd = std::process::Command::new(path);
                    if cmd.spawn().is_ok() {
                        return;
                    }
                }
            }

            // Fallback to in-process launch if tech_studio binary is not found in the path
            techscript::studio::start_studio();
        }
        None => {
            if let Some(ref arg) = cli.file {
                if arg.starts_with("[[[") && arg.ends_with("]]]") {
                    run_inline(&arg[3..arg.len() - 3], false);
                } else if arg.ends_with(".txs") || arg.ends_with(".tx") {
                    run_file(arg, false, false);
                } else {
                    eprintln!("{} Unknown command '{}'. Run `tech --help`.", "Error:".red().bold(), arg.yellow());
                    process::exit(1);
                }
            } else {
                start_repl();
            }
        }
    }
    process::exit(0);
}

fn init_logging() {
    if std::env::var("TECH_LOG").is_ok() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();
    }
}


fn run_file(filepath: &str, debug: bool, double_click: bool) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error: File not found: {}", filepath);
            if double_click {
                pause_for_double_click();
            }
            process::exit(1);
        }
    };

    if debug {
        run_file_debug(&source, filepath, double_click);
        return;
    }

    if let Err(e) = run::run_source(&source, filepath) {
        eprintln!("{}", run::format_run_error(&e, &source));
        if double_click {
            pause_for_double_click();
        }
        process::exit(1);
    }

    if double_click {
        pause_for_double_click();
    }
}

fn run_file_debug(source: &str, filepath: &str, double_click: bool) {
    use techscript::lexer::Lexer;

    let lines: Vec<&str> = source.lines().collect();

    eprintln!("=== TOKENS ===");
    match Lexer::new(source, filepath).tokenize() {
        Ok(tokens) => {
            for t in &tokens {
                eprintln!("  {}", t);
            }
        }
        Err(e) => {
            eprintln!("{}", run::format_run_error(&e, &source));
            if double_click {
                pause_for_double_click();
            }
            process::exit(1);
        }
    }

    eprintln!("=== COMPILE ===");
    match run::compile_source(source, filepath) {
        Ok(function) => {
            eprintln!("{}", disasm::disassemble_chunk("main", &function.chunk));
            let mut vm = techscript::vm::VM::new();
            if let Err(e) = vm.run(function) {
                eprintln!("{}", techscript::error::format_error(&e, &lines));
                if double_click {
                    pause_for_double_click();
                }
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", run::format_run_error(&e, &source));
            if double_click {
                pause_for_double_click();
            }
            process::exit(1);
        }
    }

    if double_click {
        pause_for_double_click();
    }
}

fn check_file(filepath: &str) {
    match run::check_file(filepath) {
        Ok(()) => println!("{}: No syntax errors found.", filepath),
        Err(e) => {
            eprintln!("{}", run::format_file_error(&e, filepath));
            process::exit(1);
        }
    }
}

fn run_inline(code: &str, _debug: bool) {
    if let Err(e) = run::run_source(code, "<inline>") {
        eprintln!("{}", run::format_run_error(&e, code));
        process::exit(1);
    }
}

fn build_file(filepath: &str, output: Option<&str>) {
    let source = fs::read_to_string(filepath).unwrap_or_else(|_| {
        eprintln!("Error: File not found: {}", filepath);
        process::exit(1);
    });

    let function = match run::compile_source(&source, filepath) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", run::format_run_error(&e, &source));
            process::exit(1);
        }
    };

    let out = output.map(|s| s.to_string()).unwrap_or_else(|| {
        PathBuf::from(filepath)
            .with_extension("txbc")
            .to_string_lossy()
            .to_string()
    });

    let data = techscript::bytecode::serialize_function(&function);
    fs::write(&out, data).unwrap_or_else(|e| {
        eprintln!("Error writing {}: {}", out, e);
        process::exit(1);
    });
    println!("Built {} ({} bytes bytecode)", out, function.chunk.code.len());
}

fn run_tests(dir: &str) {
    let root = PathBuf::from(dir);
    let mut passed = 0;
    let mut failed = 0;

    for entry in walkdir_simple(&root) {
        if entry.ends_with("_test.txs") || entry.contains("/tests/") && entry.ends_with(".txs") {
            print!("  {} ... ", entry);
            match run::run_file(&entry) {
                Ok(()) => { println!("ok"); passed += 1; }
                Err(e) => { println!("FAIL"); eprintln!("    {}", e); failed += 1; }
            }
        }
    }

    if passed + failed == 0 {
        println!("No test files found (*_test.txs)");
    } else {
        println!("\n{} passed, {} failed", passed, failed);
        if failed > 0 { process::exit(1); }
    }
}

fn walkdir_simple(root: &PathBuf) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir_simple(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("txs") {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
    files
}

fn watch_file(filepath: &str, debug: bool) {
    println!("Watching {} (Ctrl+C to stop)...", filepath);
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| { if let Ok(event) = res { let _ = tx.send(event); } },
        notify::Config::default(),
    ).expect("watcher");
    watcher.watch(std::path::Path::new(filepath), RecursiveMode::NonRecursive).ok();

    loop {
        run_file(filepath, debug, false);
        loop {
            if let Ok(event) = rx.recv_timeout(Duration::from_millis(500)) {
                if matches!(event.kind, EventKind::Modify(_)) {
                    println!("\n--- File changed, re-running ---");
                    break;
                }
            }
        }
    }
}

fn pause_for_double_click() {
    println!("\n{}", "[Process completed. Press Enter to exit...]".bright_black().italic());
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}
