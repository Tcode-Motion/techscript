//! # tsc dev Command
//!
//! Starts the TechScript WebForge development server with live SQLite persistence,
//! Three.js 3D engine, and toast event handlers.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn execute(file_opt: Option<&str>, port_opt: Option<u16>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // 1. Locate entry file (canonical .txs takes priority over specialized and legacy)
    let mut manifest_entry: Option<String> = None;
    if let Some(mf_path) = techscript_package_manager::find_manifest_in(&current_dir) {
        if let Ok(content) = std::fs::read_to_string(&mf_path) {
            if let Ok(mf) = toml::from_str::<techscript_package_manager::Manifest>(&content) {
                manifest_entry = Some(mf.package.entry);
            }
        }
    }

    let candidate_files = [
        file_opt,
        manifest_entry.as_deref(),
        Some("website/src/main.txs"),
        Some("website/app.txs"),
        Some("website/main.txs"),
        Some("src/main.txs"),
        Some("src/app.txs"),
        Some("app.txs"),
        Some("main.txs"),
        Some("src/main.wtxs"),
        Some("main.wtxs"),
        Some("src/main.stxs"),
        Some("main.stxs"),
        Some("src/main.tech"),
        Some("main.tech"),
    ];
    let mut resolved_path: Option<PathBuf> = None;
    for cand in candidate_files.into_iter().flatten() {
        let p = current_dir.join(cand);
        if p.exists() {
            resolved_path = Some(p);
            break;
        }
    }

    let entry_path = match resolved_path {
        Some(p) => p,
        None => {
            eprintln!(
                "{}",
                "Error: No TechScript WebForge project found in the current directory."
                    .red()
                    .bold()
            );
            eprintln!(
                "Expected entry file: 'app.txs', 'src/main.txs', 'main.txs', or 'src/main.wtxs'"
            );
            eprintln!(
                "Run '{}' to create a new website project.",
                "webforge new website <name>".cyan()
            );
            return ExitCode::IoError;
        }
    };

    if let Err(e) = techscript_common::validate_source_file(&entry_path) {
        eprintln!("{}", format!("Error: {}", e).red().bold());
        return ExitCode::Failure;
    }

    if let Some(desc) = techscript_common::TechScriptExtension::from_path(&entry_path) {
        if desc.is_legacy() {
            eprintln!(
                "{}",
                "Notice: '.tech' is a legacy extension. TechScript official source files use '.txs'. You can migrate via 'webforge migrate'."
                    .yellow()
            );
        }
    }

    let requested_port = port_opt.unwrap_or(3000);
    let mut port = requested_port;
    if port_opt.is_none() {
        for try_port in requested_port..requested_port.saturating_add(20) {
            if std::net::TcpListener::bind(("0.0.0.0", try_port)).is_ok() {
                if try_port != requested_port {
                    println!(
                        "ℹ️ Port {} was in use; automatically selected port {}",
                        requested_port, try_port
                    );
                }
                port = try_port;
                break;
            }
        }
    }
    println!(
        "{}",
        "========================================================="
            .cyan()
            .bold()
    );
    println!("            TECHSCRIPT WEBFORGE DEV SERVER               ");
    println!(
        "{}",
        "========================================================="
            .cyan()
            .bold()
    );
    println!("Project entry: {}", entry_path.display());
    println!("Server port:   {}", port);

    // Read source code
    let source = match std::fs::read_to_string(&entry_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading source file {}: {}", entry_path.display(), e);
            return ExitCode::IoError;
        }
    };

    // Lex and Parse AST
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = match techscript_lexer::lex(&source, &mut reporter) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer errors in {}: {:?}", entry_path.display(), e);
            return ExitCode::CompilationError;
        }
    };
    let program = match techscript_parser::parse(&tokens, &mut reporter) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Syntax parsing error in {}: {:?}", entry_path.display(), e);
            return ExitCode::CompilationError;
        }
    };

    let mut interpreter = techscript_interpreter::Interpreter::new();
    if let Err(e) = interpreter.interpret(&program) {
        eprintln!("Runtime Evaluation Error: {:?}", e);
        return ExitCode::Failure;
    }

    // Evaluate modular specialized files if present (pages/, server/, scenes/ across root or src/)
    let mut scanned_dirs = std::collections::HashSet::new();
    let parent_dir = entry_path.parent().unwrap_or(&current_dir);
    for sub in &["pages", "server"] {
        for base in &[parent_dir, &current_dir, &current_dir.join("src")] {
            let sub_dir = base.join(sub);
            if sub_dir.is_dir() && scanned_dirs.insert(sub_dir.clone()) {
                if let Ok(entries) = std::fs::read_dir(&sub_dir) {
                    for entry in entries.flatten() {
                        let sub_path = entry.path();
                        if techscript_common::is_supported_source_file(&sub_path) {
                            if let Ok(sub_content) = std::fs::read_to_string(&sub_path) {
                                let mut sub_reporter = techscript_errors::DiagnosticReporter::new();
                                if let Ok(sub_tokens) =
                                    techscript_lexer::lex(&sub_content, &mut sub_reporter)
                                {
                                    if let Ok(sub_prog) =
                                        techscript_parser::parse(&sub_tokens, &mut sub_reporter)
                                    {
                                        let _ = interpreter.interpret(&sub_prog);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Render the complete WebForge HTML document and initialize the server
    let html = techscript_stdlib::web::web_render_html_pub(&interpreter.ctx);
    *techscript_stdlib::web::server::PAGE_CONTENT.lock().unwrap() = html;
    if let Err(e) = techscript_stdlib::web::server::start_webforge_server(port) {
        eprintln!("Failed to start dev server on port {}: {}", port, e);
        return ExitCode::Failure;
    }

    println!(
        "\n{}",
        "✓ WebForge application initialized successfully."
            .green()
            .bold()
    );
    println!(
        "🌐 Local address: {}",
        format!("http://localhost:{}", port)
            .cyan()
            .bold()
            .underline()
    );
    println!("📁 SQLite persistence: {}", "webforge.db".yellow());
    println!("💡 Press Ctrl+C to terminate development server.\n");

    // Keep server thread alive
    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
