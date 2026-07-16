use crate::control_flow::{CallFrame, FlowSignal};
use crate::visitor::AstVisitor;
use std::cell::RefCell;
use std::rc::Rc;
use techscript_ast::Program;
use techscript_runtime::{Environment, RuntimeConfig, RuntimeContext, RuntimeError, RuntimeValue};

/// Evaluates immutable AST nodes using the runtime crate.
pub struct Interpreter {
    pub ctx: RuntimeContext,
    pub env: Rc<RefCell<Environment>>,
    pub call_stack: Vec<CallFrame>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    /// Creates a new tree-walking interpreter using default configurations.
    pub fn new() -> Self {
        let config = RuntimeConfig::default();
        let ctx = RuntimeContext::new(config);
        let env = Rc::clone(&ctx.global_env);
        let mut interpreter = Self {
            ctx,
            env,
            call_stack: Vec::new(),
        };
        interpreter.initialize_stdlib();
        interpreter
    }

    /// Creates a new interpreter using custom configurations.
    pub fn with_config(config: RuntimeConfig) -> Self {
        let ctx = RuntimeContext::new(config);
        let env = Rc::clone(&ctx.global_env);
        let mut interpreter = Self {
            ctx,
            env,
            call_stack: Vec::new(),
        };
        interpreter.initialize_stdlib();
        interpreter
    }

    fn initialize_stdlib(&mut self) {
        let stdlib = techscript_stdlib::StdlibRegistry::new();
        // Define "std" namespace
        self.env
            .borrow_mut()
            .define("std".to_string(), stdlib.construct_std_namespace(), true);
        // Define individual exported functions globally
        for module in stdlib.modules.values() {
            for (func_name, func) in &module.exports {
                self.env.borrow_mut().define(
                    func_name.clone(),
                    RuntimeValue::Function(Rc::clone(func)),
                    true,
                );
            }
        }
    }

    /// Evaluates the complete AST Program, returning the final output or RuntimeError.
    pub fn interpret(&mut self, program: &Program) -> Result<RuntimeValue, RuntimeError> {
        for stmt in &program.statements {
            let signal = self.visit_statement(stmt)?;
            match signal {
                FlowSignal::Normal => {}
                FlowSignal::Return(val) => return Ok(val),
                FlowSignal::Break => {
                    return Err(RuntimeError::new(
                        techscript_runtime::RuntimeErrorKind::InvalidOperation(
                            "Break statement outside loop context".to_string(),
                        ),
                        Some(stmt.span()),
                        None,
                    ))
                }
                FlowSignal::Continue => {
                    return Err(RuntimeError::new(
                        techscript_runtime::RuntimeErrorKind::InvalidOperation(
                            "Continue statement outside loop context".to_string(),
                        ),
                        Some(stmt.span()),
                        None,
                    ))
                }
                FlowSignal::Throw(err) => return Err(err),
            }
        }
        Ok(RuntimeValue::Null)
    }

    /// Executes closure actions within a temporary child environment frame.
    pub fn with_scope<F, T>(&mut self, child_env: Rc<RefCell<Environment>>, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let prev_env = Rc::clone(&self.env);
        self.env = child_env;
        let result = f(self);
        self.env = prev_env;
        result
    }
}
