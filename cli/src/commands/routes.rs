//! # tsc routes Command
//!
//! Inspects project AST and reports all registered WebForge routes,
//! SQLite database tables, and client event handlers.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn execute(file_opt: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Locate entry file (canonical .txs takes priority over specialized and legacy)
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
        Some("app.txs"),
        Some("src/app.txs"),
        Some("src/main.txs"),
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
                "Error: No TechScript project found in the current directory."
                    .red()
                    .bold()
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

    let source = match std::fs::read_to_string(&entry_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading source file: {}", e);
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

    // Collect all programs (main entry + modular pages, server, scenes)
    let mut programs = vec![program];
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
                                let mut sub_rep = techscript_errors::DiagnosticReporter::new();
                                if let Ok(sub_tokens) = techscript_lexer::lex(&sub_content, &mut sub_rep) {
                                    if let Ok(sub_prog) = techscript_parser::parse(&sub_tokens, &mut sub_rep) {
                                        programs.push(sub_prog);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "{}",
        "================================================================================"
            .cyan()
            .bold()
    );
    println!("                   TECHSCRIPT WEBFORGE APPLICATION ROUTES                      ");
    println!(
        "{}",
        "================================================================================"
            .cyan()
            .bold()
    );
    println!("Project: {}", entry_path.display().to_string().yellow());
    println!();
    println!(
        "{:<8} {:<24} {:<16} {}",
        "METHOD".bold(),
        "ROUTE".bold(),
        "TYPE".bold(),
        "HANDLER / TARGET".bold()
    );
    println!("{}", "--------------------------------------------------------------------------------".dimmed());

    // Root UI Page
    println!(
        "{:<8} {:<24} {:<16} {}",
        "GET".green().bold(),
        "/".cyan(),
        "HTML Page".white(),
        "WebForge Reactive UI (Outfit + 3D Canvas)"
    );

    let mut count = 1;
    let mut printed_routes = std::collections::HashSet::new();
    printed_routes.insert("/".to_string());

    for prog in &programs {
        for stmt in &prog.statements {
            if let techscript_ast::Statement::DSL(block) = stmt {
                if block.kind == "page" {
                    let page_name = block.args.first().map(|e| match e {
                        techscript_ast::Expression::Identifier(id) => id.name.clone(),
                        techscript_ast::Expression::Literal(lit) => match &lit.value {
                            techscript_ast::LiteralVal::Str(s) => s.clone(),
                            _ => format!("{:?}", lit.value),
                        },
                        _ => "Page".to_string(),
                    }).unwrap_or_else(|| "Page".to_string());

                    let route_opt = block.args.get(1).and_then(|e| match e {
                        techscript_ast::Expression::Literal(lit) => match &lit.value {
                            techscript_ast::LiteralVal::Str(s) => Some(s.clone()),
                            _ => None,
                        },
                        _ => None,
                    });
                    let route = route_opt.unwrap_or_else(|| {
                        let n = page_name.to_lowercase();
                        if n == "home" { "/".to_string() } else { format!("/{}", n) }
                    });
                    if printed_routes.insert(route.clone()) {
                        println!(
                            "{:<8} {:<24} {:<16} {}",
                            "GET".green().bold(),
                            route.cyan(),
                            "HTML Page".white(),
                            format!("Page [{}]", page_name)
                        );
                        count += 1;
                    }
                } else if block.kind == "api" {
                    let route = block.args.first().and_then(|e| match e {
                        techscript_ast::Expression::Literal(lit) => match &lit.value {
                            techscript_ast::LiteralVal::Str(s) => Some(s.clone()),
                            _ => None,
                        },
                        _ => None,
                    }).unwrap_or_else(|| "/api".to_string());
                    if printed_routes.insert(route.clone()) {
                        println!(
                            "{:<8} {:<24} {:<16} {}",
                            "ANY".blue().bold(),
                            route.cyan(),
                            "REST API".yellow(),
                            "WebForge API Endpoint"
                        );
                        count += 1;
                    }
                } else if block.kind == "database" {
                    for child in &block.children {
                        if let techscript_ast::DSLChild::Block(table_block) = child {
                            if table_block.kind == "table" {
                                let tbl = if let Some(arg) = table_block.args.first() {
                                    match arg {
                                        techscript_ast::Expression::Identifier(id) => id.name.clone(),
                                        techscript_ast::Expression::Literal(lit) => match &lit.value {
                                            techscript_ast::LiteralVal::Str(s) => s.clone(),
                                            _ => format!("{:?}", lit.value),
                                        },
                                        _ => "notes".to_string(),
                                    }
                                } else {
                                    "notes".to_string()
                                };
                                let clean: String = tbl.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
                                let api_route = format!("/api/{}", clean);
                                if printed_routes.insert(api_route.clone()) {
                                    println!(
                                        "{:<8} {:<24} {:<16} {}",
                                        "GET".green().bold(),
                                        api_route.cyan(),
                                        "REST API".yellow(),
                                        format!("Fetch rows from SQLite [{}]", clean)
                                    );
                                    println!(
                                        "{:<8} {:<24} {:<16} {}",
                                        "POST".blue().bold(),
                                        api_route.cyan(),
                                        "REST API".yellow(),
                                        format!("Insert row into SQLite [{}]", clean)
                                    );
                                    count += 2;
                                }
                            }
                        }
                    }
                }
            } else if let techscript_ast::Statement::If(if_stmt) = stmt {
                let cond_str = format!("{:?}", if_stmt.condition);
                if cond_str.contains("clicked") || cond_str.contains("is") {
                    println!(
                        "{:<8} {:<24} {:<16} {}",
                        "EVENT".magenta().bold(),
                        "onClick".cyan(),
                        "Client Action".magenta(),
                        "Browser Toast / API dispatch"
                    );
                    count += 1;
                }
            }
        }
    }

    println!("{}", "--------------------------------------------------------------------------------".dimmed());
    println!("Total registered application routes: {}\n", count.to_string().green().bold());

    ExitCode::Success
}
