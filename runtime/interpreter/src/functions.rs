use crate::control_flow::FlowSignal;
use crate::interpreter::Interpreter;
use crate::visitor::AstVisitor;
use std::cell::RefCell;
use std::rc::Rc;
use techscript_runtime::{Callable, Environment, RuntimeError, RuntimeValue, UserFunction};
use techscript_ast::Expression;

impl Interpreter {
    /// Bridges a UserFunction from the runtime crate to a Callable executing via AST visitor.
pub fn bridge_user_function(&self, user_func: UserFunction) -> BridgedFunction {
        let defaults = vec![None; user_func.params.len()];
        BridgedFunction { user_func, defaults }
    }

    pub fn bridge_declared_function(&self, user_func: UserFunction, defaults: Vec<Option<Expression>>) -> BridgedFunction {
        BridgedFunction { user_func, defaults }
    }
}

pub struct BridgedFunction {
    user_func: UserFunction,
    defaults: Vec<Option<Expression>>,
}

impl Callable for BridgedFunction {
    fn name(&self) -> &str {
        &self.user_func.name
    }

    fn arity(&self) -> usize {
        self.user_func.params.len()
    }

    fn accepts_arity(&self, count: usize) -> bool {
        let required = self.defaults.iter().filter(|default| default.is_none()).count();
        (required..=self.user_func.params.len()).contains(&count)
    }

    fn call(
        &self,
        ctx: &mut techscript_runtime::RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        // Construct a temporary interpreter sharing the caller config and global env
        let mut interpreter = Interpreter {
            ctx: techscript_runtime::RuntimeContext {
                config: ctx.config.clone(),
                global_env: Rc::clone(&ctx.global_env),
                registry: techscript_runtime::NativeRegistry::new(),
                resources: Rc::clone(&ctx.resources),
            },
            env: Rc::clone(&self.user_func.closure),
            call_stack: Vec::new(),
        };

        // Create new local scope block for the function execution
        let local_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.user_func.closure,
        )))));
        let provided = args.len();
        for (param, arg) in self.user_func.params.iter().zip(args) {
            local_env.borrow_mut().define(param.clone(), arg, false);
        }
        for (index, param) in self.user_func.params.iter().enumerate().skip(provided) {
            if let Some(default) = self.defaults.get(index).and_then(|value| value.as_ref()) {
                let value = interpreter.with_scope(Rc::clone(&local_env), |interp| interp.visit_expression(default))?;
                local_env.borrow_mut().define(param.clone(), value, false);
            }
        }

        if let techscript_runtime::FunctionBody::Ast(ref body) = self.user_func.body {
            let signal = interpreter.with_scope(local_env, |interp| interp.execute_block(body))?;
            match signal {
                FlowSignal::Return(val) => Ok(val),
                FlowSignal::Throw(err) => Err(err),
                _ => Ok(RuntimeValue::Null),
            }
        } else {
            Ok(RuntimeValue::Null)
        }
    }
}
