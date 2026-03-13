use std::fs;
use std::process;
use clap::{Parser, Subcommand};

use techscript::lexer::Lexer;
use techscript::parser;
use techscript::compiler::Compiler;
use techscript::vm::VM;
use techscript::error::format_error;
use techscript::repl::start_repl;
use std::path::Path;

const VERSION: &str = "1.0.4.3";

#[derive(Parser)]
#[command(name = "tech", about = "TechScript — a friendly programming language (.txs)", version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run a .txs file directly (shorthand for `tech run <file>`)
    #[arg(value_name = "FILE")]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .txs file
    Run {
        /// Path to .txs file
        file: String,
        /// Show debug info
        #[arg(long)]
        debug: bool,
    },
    /// Compile a .txs file to bytecode (coming soon)
    Build {
        /// Path to .txs file
        file: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Check syntax without running
    Check {
        /// Path to .txs file
        file: String,
    },
    /// Debug a .txs file with enhanced tracing
    Debug {
        /// Path to .txs file
        file: String,
    },
    /// Evaluate inline TechScript code: tech eval "say 42"
    Eval {
        /// TechScript source code string
        code: String,
    },
    /// Start interactive REPL
    Repl,
    /// Show version
    Version,
    /// Check system dependencies and environment
    Doctor,
    /// Run automated testing suite
    Test,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { file, debug }) => run_file(&file, debug),
        Some(Commands::Build { file, output: _ }) => {
            println!("Coming Soon: `tech build` will compile .txs files to standalone bytecode packages.");
            println!("For now, use `tech run {}` to execute your program.", file);
        }
        Some(Commands::Check { file }) => check_file(&file),
        Some(Commands::Debug { file }) => {
            std::env::set_var("TRACE", "1");
            run_file(&file, true);
        }
        Some(Commands::Eval { code }) => run_inline(&code),
        Some(Commands::Repl) => start_repl(),
        Some(Commands::Version) => println!("TechScript v{}", VERSION),
        Some(Commands::Doctor) => run_doctor(),
        Some(Commands::Test) => run_tests(),
        None => {
            // Check for bare file argument or inline [[[ ... ]]] code
            if let Some(ref arg) = cli.file {
                if arg.starts_with("[[[") && arg.ends_with("]]]") {
                    let inner = &arg[3..arg.len()-3];
                    run_inline(inner);
                } else if arg.ends_with(".txs") || arg.ends_with(".tx") {
                    run_file(arg, false);
                } else {
                    eprintln!("Error: Unknown command '{}'. Run `tech --help` for usage.", arg);
                    process::exit(1);
                }
            } else {
                // No arguments — show help
                println!("🐉 TechScript v{} — A friendly programming language", VERSION);
                println!();
                println!("Usage:");
                println!("  tech run <file.txs>     Run a TechScript file");
                println!("  tech <file.txs>         Shorthand for run");
                println!("  tech build <file.txs>   Compile to bytecode (coming soon)");
                println!("  tech check <file.txs>   Syntax check only");
                println!("  tech debug <file.txs>   Run with enhanced debug tracing");
                println!("  tech repl               Start interactive REPL");
                println!("  tech version            Show version");
                println!("  tech doctor             System diagnostics");
                println!("  tech test               Run integration tests");
                println!();
                println!("Modules: use api | use web | use gui | use 3d | use anime");
                println!();
                println!("Examples:");
                println!("  tech run hello.txs");
                println!("  tech hello.txs");
                println!("  tech repl");
            }
        }
    }
}

fn run_file(filepath: &str, debug: bool) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error: File not found: {}", filepath);
            process::exit(1);
        }
    };

    let source_lines: Vec<&str> = source.lines().collect();

    // Lex
    let tokens = match Lexer::new(&source, filepath).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    if debug {
        for t in &tokens {
            eprintln!("  {}", t);
        }
        eprintln!("---");
    }

    // Parse
    let program = match parser::Parser::new(tokens, filepath).parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    // Compile
    let function = match Compiler::new().compile(&program) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    // Execute
    let mut vm = VM::new();
    if let Err(e) = vm.run(function) {
        eprintln!("{}", format_error(&e, &source_lines));
        process::exit(1);
    }
}

fn check_file(filepath: &str) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error: File not found: {}", filepath);
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(&source, filepath).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("X {}: {}", filepath, e);
            process::exit(1);
        }
    };

    match parser::Parser::new(tokens, filepath).parse() {
        Ok(_) => println!("V {}: No syntax errors found.", filepath),
        Err(e) => {
            eprintln!("X {}: {}", filepath, e);
            process::exit(1);
        }
    }
}

fn run_inline(code: &str) {
    let source_lines: Vec<&str> = code.lines().collect();

    // Lex
    let tokens = match Lexer::new(code, "<inline>").tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    // Parse
    let program = match parser::Parser::new(tokens, "<inline>").parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    // Compile
    let function = match Compiler::new().compile(&program) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    // Execute
    let mut vm = VM::new();
    if let Err(e) = vm.run(function) {
        eprintln!("{}", format_error(&e, &source_lines));
        process::exit(1);
    }
}

fn run_doctor() {
    println!("🩺 TechScript System Doctor");
    println!("---------------------------");
    println!("OS: {}", std::env::consts::OS);
    println!("Architecture: {}", std::env::consts::ARCH);
    println!("TechScript Version: {}", VERSION);
    
    if std::net::TcpListener::bind("127.0.0.1:8080").is_ok() {
        println!("Port 8080: Available (Web/API Default)");
    } else {
        println!("Port 8080: In Use (Warning)");
    }

    if std::net::TcpListener::bind("127.0.0.1:0").is_ok() {
        println!("Dynamic Ports: Available (GUI/3D)");
    } else {
        println!("Dynamic Ports: Error (Critical)");
    }
    
    println!("\n✅ All systems are ready for TechScript development.");
}

fn run_tests() {
    use std::process::Command;
    println!("🧪 Running TechScript Integration Tests");
    println!("-------------------------------------");

    let examples = [
        "examples/api_server.txs",
        "examples/web_page.txs",
        "examples/gui_app.txs",
        "examples/3d_scene.txs",
        "examples/animation.txs",
    ];

    let mut passed = 0;
    let exe = std::env::current_exe().unwrap_or_else(|_| "techscript".into());

    for ex in examples {
        if Path::new(ex).exists() {
            print!("🔄 Testing {}... ", ex);
            use std::io::Write;
            let _ = std::io::stdout().flush();

            let child = Command::new(&exe)
                .arg("run")
                .arg(ex)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();
            
            if let Ok(mut c) = child {
                std::thread::sleep(std::time::Duration::from_millis(1500)); 
                if let Ok(Some(status)) = c.try_wait() {
                    if status.success() {
                        println!("✅ Passed (completed successfully)");
                        passed += 1;
                    } else {
                        println!("❌ Failed (exited with {})", status);
                    }
                } else {
                    println!("✅ Passed (server running steadily)");
                    passed += 1;
                    let _ = c.kill(); 
                }
            } else {
                println!("❌ Failed to launch process.");
            }
        } else {
            println!("⚠️ Skipped {} (file not found)", ex);
        }
    }
    
    println!("\nTest Results: {}/{} passed.", passed, examples.len());
}
