// cli/src/commands/emit.rs

use crate::exit_code::ExitCode;
use std::fs;
use std::path::{Path, PathBuf};

fn compile_to_ir_module(file_path: &str) -> Result<techscript_ir::Module, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("Source file does not exist: {}", file_path));
    }

    let source =
        fs::read_to_string(path).map_err(|e| format!("Failed to read source file: {}", e))?;

    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = techscript_lexer::lex(&source, &mut reporter)
        .map_err(|_| "Lexical analysis failed".to_string())?;

    let program = techscript_parser::parse(&tokens, &mut reporter)
        .map_err(|_| "Parsing failed".to_string())?;

    let checked = techscript_semantic::analyze(program, &mut reporter)
        .map_err(|_| "Semantic analysis failed".to_string())?;

    let lowered = techscript_ir::lower(&checked.program, "main");
    let mut module = lowered.module;

    // Run TechScript IR optimizer
    let opt_ctx = techscript_optimizer::OptimizationContext::new();
    let _ = techscript_optimizer::optimize(&mut module, &opt_ctx);

    Ok(module)
}

pub fn emit_ir(file_path: &str) -> ExitCode {
    match compile_to_ir_module(file_path) {
        Ok(module) => {
            let path = Path::new(file_path);
            let out_path = path.with_extension("ir");
            let content = format!("{:#?}", module);
            if fs::write(&out_path, content).is_ok() {
                println!("Successfully wrote TechScript IR to {:?}", out_path);
                ExitCode::Success
            } else {
                eprintln!("Error: Failed to write IR to {:?}", out_path);
                ExitCode::IoError
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::CompilationError
        }
    }
}

pub fn emit_llvm(file_path: &str) -> ExitCode {
    match compile_to_ir_module(file_path) {
        Ok(module) => {
            let path = Path::new(file_path);
            let out_path = path.with_extension("ll");
            match techscript_llvm_backend::LLVMBackend::emit_llvm_ir(&module, &out_path) {
                Ok(_) => {
                    println!("Successfully wrote textual LLVM IR to {:?}", out_path);
                    ExitCode::Success
                }
                Err(e) => {
                    eprintln!("Error: LLVM IR emission failed: {}", e);
                    ExitCode::CompilationError
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::CompilationError
        }
    }
}

pub fn emit_asm(file_path: &str) -> ExitCode {
    match compile_to_ir_module(file_path) {
        Ok(module) => {
            let path = Path::new(file_path);
            let out_path = path.with_extension("s");
            let opts = techscript_llvm_backend::LLVMBackendOptions {
                target_triple: crate::pipeline::get_host_target_triple(),
                opt_level: techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
                debug_symbols: true,
            };
            match techscript_llvm_backend::LLVMBackend::emit_asm(&module, &opts, &out_path) {
                Ok(_) => {
                    println!("Successfully wrote Assembly file to {:?}", out_path);
                    ExitCode::Success
                }
                Err(e) => {
                    eprintln!("Error: Assembly emission failed: {}", e);
                    ExitCode::CompilationError
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::CompilationError
        }
    }
}

pub fn emit_obj(file_path: &str) -> ExitCode {
    match compile_to_ir_module(file_path) {
        Ok(module) => {
            let path = Path::new(file_path);
            let ext = if cfg!(windows) { "obj" } else { "o" };
            let out_path = path.with_extension(ext);
            let opts = techscript_llvm_backend::LLVMBackendOptions {
                target_triple: crate::pipeline::get_host_target_triple(),
                opt_level: techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
                debug_symbols: true,
            };
            match techscript_llvm_backend::LLVMBackend::compile(&module, &opts, &out_path) {
                Ok(_) => {
                    println!("Successfully wrote Object file to {:?}", out_path);
                    ExitCode::Success
                }
                Err(e) => {
                    eprintln!("Error: Object file emission failed: {}", e);
                    ExitCode::CompilationError
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::CompilationError
        }
    }
}
