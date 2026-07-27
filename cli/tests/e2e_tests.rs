// cli/tests/e2e_tests.rs
use std::collections::HashSet;
use techscript_errors::DiagnosticReporter;
use techscript_runtime::context::Capability;
use techscript_runtime::value::RuntimeValue;

fn run_src(src: &str, capabilities: Vec<Capability>) -> Result<RuntimeValue, String> {
    let mut reporter = DiagnosticReporter::new();
    let tokens = techscript_lexer::lex_recovered(src, &mut reporter);
    let program = techscript_parser::parse_recovered(&tokens, &mut reporter);

    // Check syntax errors
    if reporter.has_errors() {
        return Err(format!("Parsing failed: {:?}", reporter.get_diagnostics()));
    }

    let mut semantic_reporter = DiagnosticReporter::new();
    let checked = techscript_semantic::analyze(program.clone(), &mut semantic_reporter);
    if checked.is_err() || semantic_reporter.has_errors() {
        return Err(format!(
            "Semantic failed: {:?}",
            semantic_reporter.get_diagnostics()
        ));
    }

    let lowered = techscript_ir::lower(&program, "main");
    let mut module = lowered.module;
    let opt_ctx = techscript_optimizer::OptimizationContext::new();
    techscript_optimizer::optimize(&mut module, &opt_ctx);

    let bytecode = techscript_bytecode::compile(&module);
    let mut vm = techscript_vm::VM::new(bytecode);

    // Set custom capabilities for sandboxing test
    vm.ctx.config.capabilities = capabilities.into_iter().collect::<HashSet<_>>();

    vm.run().map_err(|e| format!("VM error: {:?}", e))
}

#[test]
fn test_e2e_hello_world() {
    let src = r#"
        build main() {
            return "Hello, World!";
        }
    "#;
    let res = run_src(src, vec![]).unwrap();
    assert_eq!(res.try_into_string().unwrap(), "Hello, World!");
}

#[test]
fn test_e2e_calculator() {
    let src = r#"
        build main() {
            make result = 2 + 3 * 4 - 2;
            return result;
        }
    "#;
    let res = run_src(src, vec![]).unwrap();
    assert_eq!(res.try_into_int().unwrap(), 12);
}

#[test]
fn test_e2e_recursive_fibonacci() {
    let src = r#"
        fun fib(n) {
            if (n < 2) {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        }
        build main() {
            return fib(10);
        }
    "#;
    let res = run_src(src, vec![]).unwrap();
    assert_eq!(res.try_into_int().unwrap(), 55);
}

#[test]
fn test_e2e_collections_lists_and_maps() {
    let src = r#"
        build main() {
            make list = [1, 2];
            push(list, 3);

            make map = {"key": "value"};
            insert(map, "count", len(list));

            return map;
        }
    "#;
    let res = run_src(src, vec![]).unwrap();
    if let RuntimeValue::Map { entries, .. } = res {
        let borrow = entries.borrow();
        assert_eq!(
            borrow.get("key").unwrap().try_into_string().unwrap(),
            "value"
        );
        assert_eq!(borrow.get("count").unwrap().try_into_int().unwrap(), 3);
    } else {
        panic!("Expected Map");
    }
}

#[test]
fn test_e2e_json_parsing() {
    let src = r#"
        build main() {
            make obj = std.json.parse("[1, 2]");
            return obj;
        }
    "#;
    let res = run_src(src, vec![]).unwrap();
    if let RuntimeValue::List { items, .. } = res {
        let borrow = items.borrow();
        assert_eq!(borrow.len(), 2);
        assert_eq!(borrow[0].try_into_int().unwrap(), 1);
        assert_eq!(borrow[1].try_into_int().unwrap(), 2);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_e2e_fs_sandboxing_denied() {
    let src = r#"
        build main() {
            attempt {
                write_file("test.txt", "hello");
                return "success";
            } catch err {
                return "denied";
            }
        }
    "#;
    // Call with NO capabilities
    let res = run_src(src, vec![]).unwrap();
    assert_eq!(res.try_into_string().unwrap(), "denied");
}

#[test]
fn test_e2e_fs_sandboxing_granted() {
    let src = r#"
        build main() {
            attempt {
                write_file("test_e2e_temp.txt", "sandboxed_hello");
                make content = read_file("test_e2e_temp.txt");
                return content;
            } catch err {
                return "failed";
            }
        }
    "#;
    // Call with FileSystem capability
    let res = run_src(src, vec![Capability::FileSystem]).unwrap();
    assert_eq!(res.try_into_string().unwrap(), "sandboxed_hello");

    // Cleanup
    std::fs::remove_file("test_e2e_temp.txt").ok();
}
