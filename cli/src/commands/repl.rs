//! # tsc REPL Command
//!
//! Interactive Read-Eval-Print Loop shell.
//! Maintains persistent interpreter environments and exposes metacommands.

use crate::exit_code::ExitCode;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn execute() -> ExitCode {
    println!(
        "{}",
        "========================================================="
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "                 TechScript v2.0 REPL                    "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================================="
            .cyan()
            .bold()
    );
    println!("Type ':help' or ':h' for metacommands assistance.");
    println!("Type ':quit' or ':q' to exit the shell.\n");

    let mut rl = match DefaultEditor::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing REPL shell: {}", e);
            return ExitCode::InternalError;
        }
    };

    // Load persistent history
    let history_path = dirs::home_dir().map(|h| h.join(".techscript").join("repl_history"));
    if let Some(ref path) = history_path {
        std::fs::create_dir_all(path.parent().unwrap()).ok();
        rl.load_history(path).ok();
    }

    let mut interpreter = techscript_interpreter::Interpreter::new();
    let mut session_history = Vec::new();

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(trimmed);
                session_history.push(trimmed.to_string());

                // Save history incrementally
                if let Some(ref path) = history_path {
                    rl.save_history(path).ok();
                }

                if trimmed.starts_with(':') {
                    // Process REPL Metacommand
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    let cmd = parts[0];
                    match cmd {
                        ":quit" | ":q" => {
                            println!("Goodbye!");
                            break;
                        }
                        ":help" | ":h" => {
                            print_help();
                        }
                        ":clear" | ":c" => {
                            interpreter = techscript_interpreter::Interpreter::new();
                            println!("REPL environment state cleared.");
                        }
                        ":history" => {
                            for (i, h) in session_history.iter().enumerate() {
                                println!("{:>4}: {}", i + 1, h);
                            }
                        }
                        ":type" => {
                            if parts.len() < 2 {
                                println!("Usage: :type <expr>");
                            } else {
                                let expr = parts[1..].join(" ");
                                evaluate_type(&expr, &mut interpreter);
                            }
                        }
                        ":ast" => {
                            if parts.len() < 2 {
                                println!("Usage: :ast <expr>");
                            } else {
                                let expr = parts[1..].join(" ");
                                dump_repl_ast(&expr);
                            }
                        }
                        ":ir" => {
                            if parts.len() < 2 {
                                println!("Usage: :ir <expr>");
                            } else {
                                let expr = parts[1..].join(" ");
                                dump_repl_ir(&expr);
                            }
                        }
                        ":bytecode" => {
                            if parts.len() < 2 {
                                println!("Usage: :bytecode <expr>");
                            } else {
                                let expr = parts[1..].join(" ");
                                dump_repl_bytecode(&expr);
                            }
                        }
                        ":load" => {
                            if parts.len() < 2 {
                                println!("Usage: :load <file>");
                            } else {
                                let file = parts[1];
                                if let Ok(content) = std::fs::read_to_string(file) {
                                    eval_code(&content, &mut interpreter);
                                } else {
                                    println!("Error: Could not read file '{}'.", file);
                                }
                            }
                        }
                        ":save" => {
                            if parts.len() < 2 {
                                println!("Usage: :save <file>");
                            } else {
                                let file = parts[1];
                                let session_data = session_history.join("\n");
                                if std::fs::write(file, session_data).is_ok() {
                                    println!("Session history saved to '{}'.", file);
                                } else {
                                    println!("Error: Could not write to file '{}'.", file);
                                }
                            }
                        }
                        other => {
                            println!(
                                "Unknown command '{}'. Type ':help' for instructions.",
                                other
                            );
                        }
                    }
                } else {
                    // Regular statement/expression evaluation
                    eval_code(trimmed, &mut interpreter);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted. Ctrl+C pressed.");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("EOF. Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("REPL Read Error: {:?}", err);
                break;
            }
        }
    }

    ExitCode::Success
}

fn print_help() {
    println!("TechScript REPL Shell Metacommands:");
    println!("  :help, :h      Show this help text");
    println!("  :quit, :q      Exit the REPL shell");
    println!("  :clear, :c     Reset REPL environment state");
    println!("  :type <expr>   Show the type of an expression");
    println!("  :history       Show input history of this session");
    println!("  :load <file>   Load and execute a file inside current REPL");
    println!("  :save <file>   Save input history to a file");
    println!("  :ast <expr>    Parse expression and dump AST");
    println!("  :ir <expr>     Parse expression and dump IR representation");
    println!("  :bytecode <e>  Compile expression and disassemble bytecode");
}

fn eval_code(code: &str, interpreter: &mut techscript_interpreter::Interpreter) {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = match techscript_lexer::lex(code, &mut reporter) {
        Ok(t) => t,
        Err(_) => return,
    };
    let program = match techscript_parser::parse(&tokens, &mut reporter) {
        Ok(p) => p,
        Err(_) => return,
    };
    let checked = match techscript_semantic::analyze(program, &mut reporter) {
        Ok(c) => c,
        Err(_) => return,
    };

    match interpreter.interpret(&checked.program) {
        Ok(val) => {
            if val != techscript_runtime::RuntimeValue::Null {
                println!("{:?}", val);
            }
        }
        Err(e) => {
            println!("Runtime Error: {:?}", e);
        }
    }
}

fn evaluate_type(expr: &str, interpreter: &mut techscript_interpreter::Interpreter) {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    if let Ok(tokens) = techscript_lexer::lex(expr, &mut reporter) {
        if let Ok(program) = techscript_parser::parse(&tokens, &mut reporter) {
            if let Ok(checked) = techscript_semantic::analyze(program, &mut reporter) {
                if let Ok(val) = interpreter.interpret(&checked.program) {
                    println!("Type: {:?}", val.runtime_type());
                }
            }
        }
    }
}

fn dump_repl_ast(expr: &str) {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    if let Ok(tokens) = techscript_lexer::lex(expr, &mut reporter) {
        if let Ok(program) = techscript_parser::parse(&tokens, &mut reporter) {
            println!("AST: {:?}", program);
        }
    }
}

fn dump_repl_ir(expr: &str) {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    if let Ok(tokens) = techscript_lexer::lex(expr, &mut reporter) {
        if let Ok(program) = techscript_parser::parse(&tokens, &mut reporter) {
            if let Ok(checked) = techscript_semantic::analyze(program, &mut reporter) {
                let lowered = techscript_ir::lower(&checked.program, "repl");
                println!("IR: {:?}", lowered.module);
            }
        }
    }
}

fn dump_repl_bytecode(expr: &str) {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    if let Ok(tokens) = techscript_lexer::lex(expr, &mut reporter) {
        if let Ok(program) = techscript_parser::parse(&tokens, &mut reporter) {
            if let Ok(checked) = techscript_semantic::analyze(program, &mut reporter) {
                let lowered = techscript_ir::lower(&checked.program, "repl");
                let bc = techscript_bytecode::compile(&lowered.module);
                println!("Bytecode function: {:?}", bc.functions);
            }
        }
    }
}

mod dirs {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        #[allow(deprecated)]
        std::env::home_dir()
    }
}
