use crate::control_flow::{CallFrame, FlowSignal};
use crate::visitor::AstVisitor;
use indexmap::IndexMap;
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
        // Define "std" namespace (std.math.sqrt(), etc.)
        self.env
            .borrow_mut()
            .define("std".to_string(), stdlib.construct_std_namespace(), true);
        // Define individual exported functions globally (sqrt(), randint(), etc.)
        for module in stdlib.modules.values() {
            for (func_name, func) in &module.exports {
                self.env.borrow_mut().define(
                    func_name.clone(),
                    RuntimeValue::Function(Rc::clone(func)),
                    true,
                );
            }
        }
        // v1.0.8: also bind each module as a top-level namespace so that
        // `math.sqrt(x)` and `random.randint(1,10)` work without `import`.
        for (mod_key, module) in &stdlib.modules {
            let short_name = mod_key.strip_prefix("std.").unwrap_or(mod_key.as_str());
            let mut module_map = IndexMap::new();
            for (func_name, func) in &module.exports {
                module_map.insert(func_name.clone(), RuntimeValue::Function(Rc::clone(func)));
            }
            self.env.borrow_mut().define(
                short_name.to_string(),
                RuntimeValue::Map {
                    entries: Rc::new(RefCell::new(module_map)),
                    is_const: true,
                },
                true,
            );
        }
        // Bridge native-registry built-ins (type_of, len, range, to_int,
        // to_str, to_bool, to_float, assert, exit) into the global env so
        // they are callable as plain identifiers in TechScript code.
        // We collect into a Vec first to avoid holding an immutable borrow
        // on `self.ctx` while mutably borrowing `self.env`.
        let natives: Vec<(String, Rc<dyn techscript_runtime::Callable>)> = self
            .ctx
            .registry
            .iter()
            .map(|(name, func)| (name.to_string(), Rc::clone(func)))
            .collect();
        for (name, func) in natives {
            self.env
                .borrow_mut()
                .define(name, RuntimeValue::Function(func), true);
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

        // Auto-execute main function if defined in environment
        let main_val = self.env.borrow().lookup("main");
        if let Ok(RuntimeValue::Function(main_func)) = main_val {
            self.call_stack.push(crate::control_flow::CallFrame::new(
                "main".to_string(),
                None,
            ));
            let res = main_func.call(&mut self.ctx, vec![]);
            self.call_stack.pop();
            return res;
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
