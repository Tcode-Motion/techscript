// ── TechScript CLI ───────────────────────────────────────────────────
// The `tech` binary — run, build, check, fmt, lint, repl, test, and more.
// Zero external dependencies — pure Rust.

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use techscript_core::ansi::Color;
use techscript_core::lexer::Lexer;
use techscript_core::parser;
use techscript_core::compiler::Compiler;
use techscript_core::vm::VM;
use techscript_core::error::format_error;
use techscript_core::formatter;
use techscript_core::linter;
use techscript_core::bytecode_file;

mod pkg;

const VERSION: &str = "1.0.5";

fn main() {
    // Enable ANSI colors on Windows
    #[cfg(windows)]
    { let _ = enable_virtual_terminal(); }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_banner();
        return;
    }

    let cmd = args[1].as_str();

    match cmd {
        "run" => {
            require_arg(&args, 2, "run <file>");
            let debug = args.iter().any(|a| a == "--debug");
            let show_ast = args.iter().any(|a| a == "--ast");
            let show_bytecode = args.iter().any(|a| a == "--bytecode");
            let trace = args.iter().any(|a| a == "--trace");
            run_file(&args[2], debug, show_ast, show_bytecode, trace);
        }
        "build" => {
            require_arg(&args, 2, "build <file>");
            let output = find_flag_value(&args, "-o").or_else(|| find_flag_value(&args, "--output"));
            let native = args.iter().any(|a| a == "--native");
            build_file(&args[2], output, native);
        }
        "check" => {
            require_arg(&args, 2, "check <file>");
            check_file(&args[2]);
        }
        "fmt" | "format" => {
            require_arg(&args, 2, "fmt <file>");
            let check_only = args.iter().any(|a| a == "--check");
            fmt_file(&args[2], check_only);
        }
        "lint" => {
            require_arg(&args, 2, "lint <file>");
            lint_file(&args[2]);
        }
        "debug" => {
            require_arg(&args, 2, "debug <file>");
            let show_ast = args.iter().any(|a| a == "--ast");
            let show_bytecode = args.iter().any(|a| a == "--bytecode");
            let trace = true;
            run_file(&args[2], true, show_ast, show_bytecode, trace);
        }
        "eval" => {
            require_arg(&args, 2, "eval <code>");
            run_inline(&args[2]);
        }
        "repl" => start_repl(),
        "version" | "--version" | "-v" => println!("TechScript v{}", VERSION),
        "doctor" => run_doctor(),
        "test" => {
            let dir = if args.len() > 2 { &args[2] } else { "." };
            run_tests(dir);
        }
        "init" => {
            let name = if args.len() > 2 { Some(args[2].clone()) } else { None };
            init_project(name);
        }
        "doc" => {
            require_arg(&args, 2, "doc <file>");
            generate_docs(&args[2]);
        }
        "install" => {
            require_arg(&args, 2, "install <name> [url]");
            let name = args[2].clone();
            let url = if args.len() > 3 && !args[3].starts_with('-') { Some(args[3].clone()) } else { None };
            pkg::install_package(&name, url.as_deref());
        }
        "--help" | "-h" | "help" => print_banner(),
        _ => {
            // Shorthand: `tech hello.txs` = `tech run hello.txs`
            if cmd.ends_with(".txs") || cmd.ends_with(".tx") || cmd.ends_with(".txc") {
                run_file(cmd, false, false, false, false);
            } else if cmd.starts_with("[[[") && cmd.ends_with("]]]") {
                let inner = &cmd[3..cmd.len()-3];
                run_inline(inner);
            } else {
                eprintln!("{} Unknown command '{}'. Run `tech --help` for usage.", Color::bold_red("error:"), cmd);
                process::exit(1);
            }
        }
    }
}

fn require_arg(args: &[String], index: usize, usage: &str) {
    if args.len() <= index || args[index].starts_with("-") {
        eprintln!("{} Missing argument. Usage: tech {}", Color::bold_red("error:"), usage);
        process::exit(1);
    }
}

fn find_flag_value(args: &[String], flag: &str) -> Option<String> {
    for (i, a) in args.iter().enumerate() {
        if a == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn print_banner() {
    println!("{}", Color::bold_cyan("  ████████╗██╗  ██╗"));
    println!("{}", Color::bold_cyan("  ╚══██╔══╝╚██╗██╔╝"));
    println!("{}", Color::bold_cyan("     ██║    ╚███╔╝ "));
    println!("{}", Color::bold_cyan("     ██║    ██╔██╗ "));
    println!("{}", Color::bold_cyan("     ██║   ██╔╝ ██╗"));
    println!("{}", Color::bold_cyan("     ╚═╝   ╚═╝  ╚═╝"));
    println!();
    println!("{}", Color::bold_cyan("TechScript v1.0.5 — A friendly, high-performance programming language"));
    println!();
    println!("{}", Color::bold_white("Usage:"));
    println!("  tech run <file.txs>      Run a TechScript file");
    println!("  tech <file.txs>          Shorthand for run");
    println!("  tech build <file.txs>          Compile to bytecode (.txc)");
    println!("  tech build --native <file.txs> Compile to standalone .exe binary");
    println!("  tech check <file.txs>    Syntax check only");
    println!("  tech fmt <file.txs>      Format source code");
    println!("  tech lint <file.txs>     Lint for code quality");
    println!("  tech repl                Start interactive REPL");
    println!("  tech test [dir]          Run test files");
    println!("  tech init [name]         Initialize a new project");
    println!("  tech doc <file.txs>      Generate Markdown documentation");
    println!("  tech install <pkg> [url] Install a package using Git");
    println!("  tech eval \"say 42\"       Evaluate inline code");
    println!("  tech debug <file.txs>    Run with debug tracing");
    println!("    --ast                 Also print parsed AST");
    println!("    --bytecode            Also print bytecode disassembly");
    println!("    --trace               Print VM instruction trace");
    println!("  tech doctor              System diagnostics");
    println!("  tech version             Show version");
    println!();
    println!("{}", Color::bold_white("Modules:"));
    println!("  use math | use fs | use os | use json | use crypto | use date");
    println!("  use api  | use web | use gui | use 3d  | use anime");
    println!();
    println!("{}", Color::bold_white("Examples:"));
    println!("  tech run hello.txs");
    println!("  tech repl");
    println!("  tech build hello.txs");
}

// ── Run ──────────────────────────────────────────────────────────────

fn run_file(filepath: &str, debug: bool, show_ast: bool, show_bytecode: bool, trace: bool) {
    if filepath.ends_with(".txc") {
        run_bytecode_file(filepath);
        return;
    }

    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", Color::bold_red("error:"), filepath, e);
            process::exit(1);
        }
    };

    let source_lines: Vec<&str> = source.lines().collect();

    let tokens = match Lexer::new(&source, filepath).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    if debug {
        eprintln!("{}", Color::dim("── Tokens ──"));
        for t in &tokens {
            eprintln!("  {}", t);
        }
        eprintln!("{}", Color::dim("────────────"));
    }

    let program = match parser::Parser::new(tokens, filepath).parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    if debug && show_ast {
        eprintln!("{}", Color::dim("── AST ──"));
        eprintln!("{:#?}", program);
        eprintln!("{}", Color::dim("─────────"));
    }

    let function = match Compiler::new().compile(&program) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    };

    if debug && show_bytecode {
        eprintln!("{}", Color::dim("── Bytecode ──"));
        eprintln!("{}", function.chunk.disassemble(&function.name));
        eprintln!("{}", Color::dim("──────────────"));
    }

    let mut vm = VM::new();
    if trace {
        std::env::set_var("TRACE", "1");
    }
    if let Err(e) = vm.run(function) {
        eprintln!("{}", format_error(&e, &source_lines));
        process::exit(1);
    }
    if let Err(e) = vm.run_event_loop() {
        eprintln!("{}", format_error(&e, &source_lines));
        process::exit(1);
    }
}

fn run_bytecode_file(filepath: &str) {
    let data = match fs::read(filepath) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", Color::bold_red("error:"), filepath, e);
            process::exit(1);
        }
    };

    let function = match bytecode_file::deserialize(&data) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} {}", Color::bold_red("error:"), e);
            process::exit(1);
        }
    };

    let mut vm = VM::new();
    if let Err(e) = vm.run(function) {
        eprintln!("{} {}", Color::bold_red("runtime error:"), e);
        process::exit(1);
    }
    if let Err(e) = vm.run_event_loop() {
        eprintln!("{} {}", Color::bold_red("runtime error:"), e);
        process::exit(1);
    }
}

fn run_inline(code: &str) {
    let source_lines: Vec<&str> = code.lines().collect();

    let tokens = match Lexer::new(code, "<inline>").tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("{}", format_error(&e, &source_lines)); process::exit(1); }
    };

    let program = match parser::Parser::new(tokens, "<inline>").parse() {
        Ok(p) => p,
        Err(e) => { eprintln!("{}", format_error(&e, &source_lines)); process::exit(1); }
    };

    let function = match Compiler::new().compile(&program) {
        Ok(f) => f,
        Err(e) => { eprintln!("{}", format_error(&e, &source_lines)); process::exit(1); }
    };

    let mut vm = VM::new();
    if let Err(e) = vm.run(function) {
        eprintln!("{}", format_error(&e, &source_lines));
        process::exit(1);
    }
    if let Err(e) = vm.run_event_loop() {
        eprintln!("{}", format_error(&e, &source_lines));
        process::exit(1);
    }
}

// ── Build ────────────────────────────────────────────────────────────

fn build_file(filepath: &str, output: Option<String>, native: bool) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", Color::bold_red("error:"), filepath, e);
            process::exit(1);
        }
    };

    let source_lines: Vec<&str> = source.lines().collect();

    let tokens = match Lexer::new(&source, filepath).tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("{}", format_error(&e, &source_lines)); process::exit(1); }
    };

    let program = match parser::Parser::new(tokens, filepath).parse() {
        Ok(p) => p,
        Err(e) => { eprintln!("{}", format_error(&e, &source_lines)); process::exit(1); }
    };

    let function = match Compiler::new().compile(&program) {
        Ok(f) => f,
        Err(e) => { eprintln!("{}", format_error(&e, &source_lines)); process::exit(1); }
    };

    let bytecode = bytecode_file::serialize(&function);

    if native {
        build_native_executable(filepath, &bytecode, output);
    } else {
        let out_path = output.unwrap_or_else(|| bytecode_file::txc_path(filepath));
        match fs::write(&out_path, &bytecode) {
            Ok(_) => {
                println!("{} {} → {} ({} bytes bytecode)",
                    Color::bold_green("✓"),
                    filepath,
                    out_path,
                    bytecode.len()
                );
            }
            Err(e) => {
                eprintln!("{} Cannot write '{}': {}", Color::bold_red("error:"), out_path, e);
                process::exit(1);
            }
        }
    }
}

fn build_native_executable(filepath: &str, bytecode: &[u8], output: Option<String>) {
    // Determine output executable name
    let out_name = output.unwrap_or_else(|| {
        let path = Path::new(filepath);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        format!("{}{}", stem, env::consts::EXE_SUFFIX)
    });

    println!("{} Building standalone native executable: {}", Color::bold_cyan("⚙"), out_name);
    
    // We need rustc to compile a simple runner that embeds the bytecode.
    // Ensure rustc is available
    if !process::Command::new("rustc").arg("--version").output().is_ok() {
        eprintln!("{} 'rustc' is required for native compilation but was not found in PATH.", Color::bold_red("error:"));
        process::exit(1);
    }

    // Convert bytecode to a rust array string: [0x01, 0x02, ...]
    let byte_arr = bytecode.iter().map(|b| format!("0x{:02x}", b)).collect::<Vec<_>>().join(", ");
    
    let runner_src = format!(r#"
fn main() {{
    let bytecode: &[u8] = &[{byte_arr}];
    match techscript_core::bytecode_file::deserialize(bytecode) {{
        Ok(function) => {{
            let mut vm = techscript_core::vm::VM::new();
            if let Err(e) = vm.run(function) {{
                eprintln!("{{}}", e);
                std::process::exit(1);
            }}
        }}
        Err(e) => {{
            eprintln!("Failed to load embedded bytecode: {{}}", e);
            std::process::exit(1);
        }}
    }}
}}
"#);

    // Create a temporary directory to build the runner
    let temp_dir = env::temp_dir().join("techscript_native_build");
    fs::create_dir_all(&temp_dir).ok();
    
    let runner_path = temp_dir.join("runner.rs");
    if let Err(e) = fs::write(&runner_path, runner_src) {
        eprintln!("{} Failed to write temporary runner source: {}", Color::bold_red("error:"), e);
        process::exit(1);
    }

    // Assume techscript_core is available in the current workspace or as a crate.
    // For a robust distribution, tech engine should be a published crate, but for local 
    // workspace we pass the --extern flag if we can find the core library.
    // We will attempt to run `cargo build` in a generated temp crate.
    
    let cargo_toml = format!(r#"
[package]
name = "runner"
version = "1.0.0"
edition = "2021"

[dependencies]
techscript-core = {{ path = r"{}" }}
"#, env::current_dir().unwrap().join("crates").join("techscript-core").display().to_string().replace("\\", "\\\\"));

    let manifest_path = temp_dir.join("Cargo.toml");
    let src_dir = temp_dir.join("src");
    fs::create_dir_all(&src_dir).ok();
    fs::write(&manifest_path, cargo_toml).ok();
    fs::copy(&runner_path, src_dir.join("main.rs")).ok();

    println!("{} Compiling optimized binary via Cargo...", Color::dim("▸"));
    let status = process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match status {
        Ok(s) if s.success() => {
            let compiled_exe = temp_dir.join("target").join("release").join(format!("runner{}", env::consts::EXE_SUFFIX));
            if fs::copy(&compiled_exe, &out_name).is_ok() {
                println!("{} Successfully compiled native binary: {}", Color::bold_green("✓"), out_name);
            } else {
                eprintln!("{} Failed to copy compiled executable to destination.", Color::bold_red("error:"));
            }
        }
        _ => {
            eprintln!("{} Native compilation failed. Ensure the Cargo workspace is accessible.", Color::bold_red("error:"));
        }
    }
    
    // Cleanup Temp
    let _ = fs::remove_dir_all(temp_dir);
}

// ── Check ────────────────────────────────────────────────────────────

fn check_file(filepath: &str) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", Color::bold_red("error:"), filepath, e);
            process::exit(1);
        }
    };

    let source_lines: Vec<&str> = source.lines().collect();

    let tokens = match Lexer::new(&source, filepath).tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("{}", format_error(&e, &source_lines)); process::exit(1); }
    };

    match parser::Parser::new(tokens, filepath).parse() {
        Ok(program) => {
            match Compiler::new().compile(&program) {
                Ok(_) => println!("{} {}: No errors found.", Color::bold_green("✓"), filepath),
                Err(e) => {
                    eprintln!("{}", format_error(&e, &source_lines));
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", format_error(&e, &source_lines));
            process::exit(1);
        }
    }
}

// ── Format ───────────────────────────────────────────────────────────

fn fmt_file(filepath: &str, check_only: bool) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", Color::bold_red("error:"), filepath, e);
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(&source, filepath).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{} Cannot format — syntax error: {}", Color::bold_red("error:"), e);
            process::exit(1);
        }
    };

    let program = match parser::Parser::new(tokens, filepath).parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} Cannot format — parse error: {}", Color::bold_red("error:"), e);
            process::exit(1);
        }
    };

    let config = formatter::FmtConfig::default();
    let formatted = formatter::format_program(&program, &config);

    if check_only {
        if formatted == source {
            println!("{} {}: Already formatted.", Color::bold_green("✓"), filepath);
        } else {
            println!("{} {}: Would reformat.", Color::bold_yellow("✗"), filepath);
            process::exit(1);
        }
    } else {
        match fs::write(filepath, &formatted) {
            Ok(_) => println!("{} Formatted: {}", Color::bold_green("✓"), filepath),
            Err(e) => {
                eprintln!("{} Cannot write '{}': {}", Color::bold_red("error:"), filepath, e);
                process::exit(1);
            }
        }
    }
}

// ── Lint ─────────────────────────────────────────────────────────────

fn lint_file(filepath: &str) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", Color::bold_red("error:"), filepath, e);
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(&source, filepath).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{} Syntax error prevents linting: {}", Color::bold_red("error:"), e);
            process::exit(1);
        }
    };

    let program = match parser::Parser::new(tokens, filepath).parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} Parse error prevents linting: {}", Color::bold_red("error:"), e);
            process::exit(1);
        }
    };

    let warnings = linter::lint_program(&program);

    if warnings.is_empty() {
        println!("{} {}: No lint warnings.", Color::bold_green("✓"), filepath);
    } else {
        println!("{} {} — {} warning(s):", Color::bold_yellow("⚠"), filepath, warnings.len());
        for w in &warnings {
            match w.severity {
                linter::Severity::Error => eprintln!("  {} {}", Color::red("✗"), w),
                linter::Severity::Warning => eprintln!("  {} {}", Color::yellow("⚠"), w),
                linter::Severity::Info => eprintln!("  {} {}", Color::blue("ℹ"), w),
            }
        }
    }
}

// ── REPL ─────────────────────────────────────────────────────────────

fn start_repl() {
    use std::io::{self, Write};

    println!("{}", Color::bold_cyan("  TX REPL v1.0.5 — Interactive Mode"));
    println!("Type {} or press {} to quit. Type {} for help.\n",
        Color::dim("'exit'"), Color::dim("Ctrl+C"), Color::dim("'.help'"));

    let mut vm = VM::new();

    loop {
        print!("{} ", Color::bold_green("txs>"));
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() { continue; }
        if input == "exit" || input == "quit" { break; }

        // Magic commands
        if input.starts_with('.') {
            match input {
                ".help" => {
                    println!("{}", Color::bold_white("REPL Commands:"));
                    println!("  .help     Show this help");
                    println!("  .clear    Clear the screen");
                    println!("  .version  Show version");
                    println!("  exit      Quit the REPL");
                    continue;
                }
                ".clear" => {
                    print!("\x1B[2J\x1B[1;1H");
                    continue;
                }
                ".version" => {
                    println!("TechScript v{}", VERSION);
                    continue;
                }
                _ => {
                    println!("{} Unknown command: {}", Color::yellow("?"), input);
                    continue;
                }
            }
        }

        // Multi-line detection: if line has unbalanced braces
        let mut full_input = input.to_string();
        let mut depth = count_braces(&full_input);

        while depth > 0 {
            print!("{} ", Color::green("..."));
            io::stdout().flush().unwrap();

            let mut continuation = String::new();
            if io::stdin().read_line(&mut continuation).is_err() {
                break;
            }
            let cont_trimmed = continuation.trim_end();
            full_input.push('\n');
            full_input.push_str(cont_trimmed);
            depth = count_braces(&full_input);
        }

        match run_repl_line(&mut vm, &full_input) {
            Ok(()) => {}
            Err(e) => eprintln!("{} {}", Color::bold_red("error:"), e),
        }
    }

    println!("{}", Color::bold_cyan("Goodbye! // TX"));
}

fn count_braces(s: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escape = false;
    for ch in s.chars() {
        if escape { escape = false; continue; }
        if ch == '\\' { escape = true; continue; }
        if ch == '"' { in_string = !in_string; continue; }
        if in_string { continue; }
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
    }
    depth
}

fn run_repl_line(vm: &mut VM, input: &str) -> Result<(), String> {
    let tokens = Lexer::new(input, "<repl>").tokenize().map_err(|e| e.to_string())?;
    let program = parser::Parser::new(tokens, "<repl>").parse().map_err(|e| e.to_string())?;
    let function = Compiler::new().compile(&program).map_err(|e| e.to_string())?;
    vm.run(function).map_err(|e| e.to_string())
}

// ── Test Runner ──────────────────────────────────────────────────────

fn run_tests(dir: &str) {
    use std::io::Write;

    println!("{}", Color::bold_cyan("🧪 TechScript Test Runner"));
    println!("{}", Color::dim(&"─".repeat(40)));

    let test_dir = Path::new(dir);
    if !test_dir.exists() {
        eprintln!("{} Directory not found: {}", Color::bold_red("error:"), dir);
        process::exit(1);
    }

    let mut test_files: Vec<_> = Vec::new();

    fn walk(dir: &Path, files: &mut Vec<std::path::PathBuf>, include_all_txs: bool) {
        let Ok(entries) = fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, files, include_all_txs);
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".txs") { continue; }
            if include_all_txs {
                files.push(path);
            } else if name.ends_with("_test.txs") || name.starts_with("test_") {
                files.push(path);
            }
        }
    }

    // tests: only conventional test names
    walk(test_dir, &mut test_files, false);
    // examples: run all .txs (they should be self-testing or at least not crash)
    let examples_dir = test_dir.join("examples");
    if examples_dir.exists() {
        walk(&examples_dir, &mut test_files, true);
    }

    test_files.sort();

    if test_files.is_empty() {
        println!("No test files found. Test files should be named *_test.txs or test_*.txs");
        return;
    }

    let mut passed = 0;
    let mut failed = 0;
    let start = std::time::Instant::now();

    for test_file in &test_files {
        let display_name = test_file.strip_prefix(test_dir).unwrap_or(test_file).display();
        print!("  {} {} ... ", Color::dim("▸"), display_name);
        std::io::stdout().flush().ok();

        let source = match fs::read_to_string(test_file) {
            Ok(s) => s,
            Err(e) => {
                println!("{} ({})", Color::bold_red("FAIL"), e);
                failed += 1;
                continue;
            }
        };

        let source_lines: Vec<&str> = source.lines().collect();

        let result = (|| -> Result<(), String> {
            let tokens = Lexer::new(&source, &test_file.to_string_lossy())
                .tokenize().map_err(|e| format_error(&e, &source_lines))?;
            let program = parser::Parser::new(tokens, &test_file.to_string_lossy())
                .parse().map_err(|e| format_error(&e, &source_lines))?;
            let function = Compiler::new().compile(&program)
                .map_err(|e| format_error(&e, &source_lines))?;
            let mut vm = VM::new();
            vm.run(function).map_err(|e| format_error(&e, &source_lines))?;
            vm.run_event_loop().map_err(|e| format_error(&e, &source_lines))?;
            Ok(())
        })();

        match result {
            Ok(()) => {
                println!("{}", Color::bold_green("PASS"));
                passed += 1;
            }
            Err(e) => {
                println!("{}", Color::bold_red("FAIL"));
                if let Some(first_line) = e.lines().next() {
                    println!("    {}", Color::dim(first_line));
                }
                failed += 1;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("{}", Color::dim(&"─".repeat(40)));

    if failed == 0 {
        println!("{} All {} tests passed ({:.2}s)",
            Color::bold_green("✓"), passed, elapsed.as_secs_f64());
    } else {
        println!("{} {}/{} tests passed, {} failed ({:.2}s)",
            Color::bold_red("✗"), passed, passed + failed, failed, elapsed.as_secs_f64());
        process::exit(1);
    }
}

// ── Init ─────────────────────────────────────────────────────────────

fn init_project(name: Option<String>) {
    let project_name = name.unwrap_or_else(|| {
        env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "my-project".to_string())
    });

    let toml_content = format!(
        "[project]\nname = \"{}\"\nversion = \"0.1.0\"\nauthor = \"\"\ndescription = \"\"\n\n[dependencies]\n",
        project_name
    );

    if Path::new("tech.toml").exists() {
        println!("{} tech.toml already exists. Skipping.", Color::yellow("⚠"));
    } else {
        fs::write("tech.toml", &toml_content).ok();
        println!("{} Created tech.toml", Color::bold_green("✓"));
    }

    fs::create_dir_all("src").ok();
    let main_content = format!(
        "# {} — TechScript project\n\nsay \"Hello from {}!\"\n",
        project_name, project_name
    );
    if !Path::new("src/main.txs").exists() {
        fs::write("src/main.txs", &main_content).ok();
        println!("{} Created src/main.txs", Color::bold_green("✓"));
    }

    fs::create_dir_all("tests").ok();
    let test_content = "# Test file for your project\nassert(1 + 1 == 2, \"Basic math works\")\nsay \"All tests passed!\"\n";
    if !Path::new("tests/basic_test.txs").exists() {
        fs::write("tests/basic_test.txs", test_content).ok();
        println!("{} Created tests/basic_test.txs", Color::bold_green("✓"));
    }

    println!("\nProject '{}' initialized! Run {} to get started.",
        Color::bold_cyan(&project_name), Color::bold_white("tech run src/main.txs"));
}

// ── Doctor ───────────────────────────────────────────────────────────

fn run_doctor() {
    println!("{}", Color::bold_cyan("🩺 TechScript System Doctor"));
    println!("{}", Color::dim(&"─".repeat(40)));
    println!("  OS:           {}", env::consts::OS);
    println!("  Architecture: {}", env::consts::ARCH);
    println!("  TechScript:   v{}", VERSION);
    println!("  Rust runtime: {}", Color::green("native (no Python required)"));

    if std::net::TcpListener::bind("127.0.0.1:8080").is_ok() {
        println!("  Port 8080:    {}", Color::green("available"));
    } else {
        println!("  Port 8080:    {}", Color::yellow("busy (in use)"));
    }

    let has_rustc = process::Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if has_rustc {
        println!("  Rust toolchain: {}", Color::green("found (native compilation available)"));
    } else {
        println!("  Rust toolchain: {}", Color::yellow("not found (native compilation unavailable)"));
    }

    println!("\n{} All systems are ready for TechScript development.", Color::bold_green("✓"));
}

// ── Docs ─────────────────────────────────────────────────────────────

fn generate_docs(filepath: &str) {
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", Color::bold_red("error:"), filepath, e);
            process::exit(1);
        }
    };

    println!("{} Generating docs for {}", Color::bold_cyan("📚"), filepath);
    let mut docs = String::new();
    let mut current_docblock = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("///") {
            let comment = trimmed.trim_start_matches("///").trim_start();
            current_docblock.push(comment.to_string());
        } else if !current_docblock.is_empty() && (trimmed.starts_with("build ") || trimmed.starts_with("async build ") || trimmed.starts_with("model ") || trimmed.starts_with("class ")) {
            let header = trimmed.trim_end_matches(" {").trim_end_matches("{").trim();
            docs.push_str(&format!("### `{}`\n\n", header));
            for doc_line in &current_docblock {
                docs.push_str(&format!("{}\n", doc_line));
            }
            docs.push_str("\n---\n\n");
            current_docblock.clear();
        } else if !trimmed.is_empty() {
            current_docblock.clear(); // Clear docblock if it's not followed by a function/class
        }
    }

    if docs.is_empty() {
        println!("  No documentation comments (///) found in this file.");
        return;
    }

    let out_name = filepath.replace(".txs", ".md");
    if let Err(e) = fs::write(&out_name, &docs) {
        eprintln!("{} Failed to write docs: {}", Color::bold_red("error:"), e);
        process::exit(1);
    }
    println!("{} Wrote documentation to {}", Color::bold_green("✓"), out_name);
}

// ── Windows ANSI support ─────────────────────────────────────────────

#[cfg(windows)]
fn enable_virtual_terminal() -> Result<(), ()> {
    // Enable ANSI escape sequence processing on Windows 10+
    unsafe {
        extern "system" {
            fn GetStdHandle(nStdHandle: u32) -> *mut std::ffi::c_void;
            fn GetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, lpMode: *mut u32) -> i32;
            fn SetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, dwMode: u32) -> i32;
        }
        const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5;
        const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
    Ok(())
}

