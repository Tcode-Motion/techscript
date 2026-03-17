#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::compiler::Compiler;
    use crate::vm::VM;

    fn eval_code(code: &str) -> Result<(), String> {
        let tokens = Lexer::new(code, "test").tokenize().map_err(|e| e.to_string())?;
        let program = Parser::new(tokens, "test").parse().map_err(|e| e.to_string())?;
        let function = Compiler::new().compile(&program).map_err(|e| e.to_string())?;
        let mut vm = VM::new();
        vm.run(function).map_err(|e| e.to_string())
    }

    #[test]
    fn test_lexer() {
        let code = "say 1 + 2 * 3";
        let tokens = Lexer::new(code, "test").tokenize().unwrap();
        assert_eq!(tokens.len(), 7); // say, 1, +, 2, *, 3, EOF
    }

    #[test]
    fn test_parser() {
        let code = "x = 42";
        let builder = Lexer::new(code, "test").tokenize().unwrap();
        let program = Parser::new(builder, "test").parse().unwrap();
        assert_eq!(program.body.len(), 1);
    }

    #[test]
    fn test_compiler_and_vm_math() {
        // Test basic assertion evaluating correctly
        let code = "assert(1 + 2 * 3 == 7, \"Math precedence\")";
        assert!(eval_code(code).is_ok());
    }

    #[test]
    fn test_vm_function() {
        let code = r#"
        build add(a, b) {
            send a + b
        }
        assert(add(5, 5) == 10, "Function call")
        "#;
        assert!(eval_code(code).is_ok());
    }

    #[test]
    fn test_vm_loop() {
        let code = r#"
        make x = 0
        each i in 0..5 {
            x = x + i
        }
        assert(x == 10, "Loop logic")
        "#;
        assert!(eval_code(code).is_ok());
    }
}
