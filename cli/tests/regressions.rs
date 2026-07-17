// cli/tests/regressions.rs
use std::collections::HashSet;
use techscript_errors::DiagnosticReporter;
use techscript_runtime::value::RuntimeValue;
use techscript_runtime::context::Capability;

fn run_src(src: &str, capabilities: Vec<Capability>) -> Result<RuntimeValue, String> {
    let mut reporter = DiagnosticReporter::new();
    let tokens = techscript_lexer::lex_recovered(src, &mut reporter);
    let program = techscript_parser::parse_recovered(&tokens, &mut reporter);
    
    if reporter.has_errors() {
        return Err(format!("Parsing failed: {:?}", reporter.get_diagnostics()));
    }

    let mut semantic_reporter = DiagnosticReporter::new();
    let checked = techscript_semantic::analyze(program.clone(), &mut semantic_reporter);
    if checked.is_err() || semantic_reporter.has_errors() {
        return Err(format!("Semantic failed: {:?}", semantic_reporter.get_diagnostics()));
    }

    let lowered = techscript_ir::lower(&program, "main");
    let mut module = lowered.module;
    let opt_ctx = techscript_optimizer::OptimizationContext::new();
    techscript_optimizer::optimize(&mut module, &opt_ctx);

    let bytecode = techscript_bytecode::compile(&module);
    let mut vm = techscript_vm::VM::new(bytecode);
    vm.ctx.config.capabilities = capabilities.into_iter().collect::<HashSet<_>>();

    vm.run().map_err(|e| format!("VM error: {:?}", e))
}

#[test]
fn test_regression_await_precedence() {
    let src = r#"
        fun get_value() {
            return 100;
        }

        build main() {
            make fut = spawn_async(get_value);
            make val = await fut;
            return val;
        }
    "#;
    let res = run_src(src, vec![]).unwrap();
    assert_eq!(res.try_into_int().unwrap(), 100);
}

#[test]
fn test_regression_sandboxed_environment_denied() {
    let src = r#"
        build main() {
            attempt {
                make env_val = std.env.get("PATH");
                return "granted";
            } catch e {
                return "denied";
            }
        }
    "#;
    let res = run_src(src, vec![]).unwrap();
    assert_eq!(res.try_into_string().unwrap(), "denied");
}

#[test]
fn test_regression_sandboxed_environment_granted() {
    let src = r#"
        build main() {
            attempt {
                std.env.set("TS_REG_TEST", "verified");
                make val = std.env.get("TS_REG_TEST");
                return val;
            } catch e {
                return "denied";
            }
        }
    "#;
    let res = run_src(src, vec![Capability::Environment]).unwrap();
    assert_eq!(res.try_into_string().unwrap(), "verified");
}
