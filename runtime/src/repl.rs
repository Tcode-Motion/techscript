// ── TechScript REPL ──────────────────────────────────────────────────
use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;

pub fn start_repl() {
    println!("🐉 TechScript v{} — Interactive REPL", crate::VERSION);
    println!("Type 'exit' or press Ctrl+C to quit.\n");

    let mut vm = VM::new();

    loop {
        print!("txs> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() { continue; }
        if input == "exit" || input == "quit" { break; }

        match run_line(&mut vm, input) {
            Ok(()) => {}
            Err(e) => eprintln!("{}", e),
        }
    }
    println!("Goodbye! 🐉");
}

fn run_line(vm: &mut VM, input: &str) -> Result<(), String> {
    let tokens = Lexer::new(input, "<repl>").tokenize().map_err(|e| e.to_string())?;
    let program = Parser::new(tokens, "<repl>").parse().map_err(|e| e.to_string())?;
    
    let mut compiler = Compiler::new();
    compiler.is_repl = true;
    let function = compiler.compile(&program).map_err(|e| e.to_string())?;
    
    vm.run(function).map_err(|e| e.to_string())?;
    
    let result = vm.pop();
    vm.clear_stack();
    
    if !matches!(result, crate::value::Value::None) {
        println!("{}", result.display_string());
    }
    
    Ok(())
}
