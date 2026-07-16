//! # tsc dump-ast, dump-ir, dump-bytecode Subcommands
//!
//! Dumps intermediate compiler stages for debugging.

use crate::exit_code::ExitCode;
use std::path::{Path, PathBuf};

pub fn dump_ast(file: &str, json: bool) -> ExitCode {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let content = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::IoError;
        }
    };

    let tokens = match techscript_lexer::lex(&content, &mut reporter) {
        Ok(t) => t,
        Err(_) => return ExitCode::CompilationError,
    };

    let program = match techscript_parser::parse(&tokens, &mut reporter) {
        Ok(p) => p,
        Err(_) => return ExitCode::CompilationError,
    };

    if json {
        match serde_json::to_string_pretty(&program) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("Serialization error: {}", e),
        }
    } else {
        println!("{:#?}", program);
    }

    ExitCode::Success
}

pub fn dump_ir(file: &str, json: bool) -> ExitCode {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let content = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::IoError;
        }
    };

    let tokens = match techscript_lexer::lex(&content, &mut reporter) {
        Ok(t) => t,
        Err(_) => return ExitCode::CompilationError,
    };

    let program = match techscript_parser::parse(&tokens, &mut reporter) {
        Ok(p) => p,
        Err(_) => return ExitCode::CompilationError,
    };

    let checked = match techscript_semantic::analyze(program, &mut reporter) {
        Ok(c) => c,
        Err(_) => return ExitCode::CompilationError,
    };

    let lowered = techscript_ir::lower(&checked.program, "main");

    if json {
        match serde_json::to_string_pretty(&lowered.module) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("Serialization error: {}", e),
        }
    } else {
        println!("{:#?}", lowered.module);
    }

    ExitCode::Success
}

pub fn dump_bytecode(file: &str, json: bool) -> ExitCode {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let content = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::IoError;
        }
    };

    let tokens = match techscript_lexer::lex(&content, &mut reporter) {
        Ok(t) => t,
        Err(_) => return ExitCode::CompilationError,
    };

    let program = match techscript_parser::parse(&tokens, &mut reporter) {
        Ok(p) => p,
        Err(_) => return ExitCode::CompilationError,
    };

    let checked = match techscript_semantic::analyze(program, &mut reporter) {
        Ok(c) => c,
        Err(_) => return ExitCode::CompilationError,
    };

    let lowered = techscript_ir::lower(&checked.program, "main");

    let mut module = lowered.module;
    let opt_ctx = techscript_optimizer::OptimizationContext::new();
    let _ = techscript_optimizer::optimize(&mut module, &opt_ctx);

    let bytecode_module = techscript_bytecode::compile(&module);

    if json {
        match serde_json::to_string_pretty(&bytecode_module) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("Serialization error: {}", e),
        }
    } else {
        println!("{:#?}", bytecode_module);
    }

    ExitCode::Success
}
