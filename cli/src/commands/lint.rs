//! # tsc lint Command
//!
//! Performs static analysis linting on TechScript sources.

use crate::exit_code::ExitCode;
use std::path::{Path, PathBuf};

pub fn execute(path_str: Option<&str>, fix: bool) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_path = path_str.map(PathBuf::from).unwrap_or(current_dir);

    if !target_path.exists() {
        eprintln!("Error: Path does not exist: {:?}", target_path);
        return ExitCode::IoError;
    }

    println!("Linting TechScript files in: {:?}", target_path);

    let mut files_to_lint = Vec::new();
    if target_path.is_dir() {
        let mut dirs = vec![target_path];
        while let Some(dir) = dirs.pop() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        if name != "build" && name != ".git" && name != "target" {
                            dirs.push(path);
                        }
                    } else {
                        let ext = path.extension().unwrap_or_default().to_string_lossy();
                        if ext == "txs" || ext == "ts" {
                            files_to_lint.push(path);
                        }
                    }
                }
            }
        }
    } else {
        files_to_lint.push(target_path);
    }

    let linter = techscript_linter::Linter::new();
    let mut violation_count = 0;

    for file in files_to_lint {
        if let Ok(content) = std::fs::read_to_string(&file) {
            let mut reporter = techscript_errors::DiagnosticReporter::new();
            if let Ok(tokens) = techscript_lexer::lex(&content, &mut reporter) {
                if let Ok(program) = techscript_parser::parse(&tokens, &mut reporter) {
                    if let Ok(checked) = techscript_semantic::analyze(program, &mut reporter) {
                        let violations = linter.lint(&checked);
                        violation_count += violations.len();

                        let source_mgr = techscript_common::SourceManager::new();
                        let renderer =
                            crate::diagnostics::DiagnosticRenderer::auto_detect(&source_mgr);
                        for diag in violations {
                            let rich = crate::diagnostics::RichDiagnostic::from_legacy(
                                &diag,
                                techscript_common::FileId(0),
                            );
                            renderer.emit(&rich);
                        }
                    }
                }
            }
        }
    }

    if violation_count > 0 {
        println!("Linting found {} violation(s).", violation_count);
        ExitCode::LintFailure
    } else {
        println!("Linting passed with zero violations.");
        ExitCode::Success
    }
}
