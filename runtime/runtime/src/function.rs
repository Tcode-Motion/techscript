use crate::context::RuntimeContext;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::value::RuntimeValue;
use std::cell::RefCell;
use std::rc::Rc;

/// Trait defining the calling contract for any callable object (AST, Native, or VM function).
pub trait Callable {
    /// Returns the function or method name identifier.
    fn name(&self) -> &str;

    /// Returns the number of arguments expected by the callable.
    fn arity(&self) -> usize;

    /// Whether this callable accepts a particular argument count.  Fixed-arity
    /// callables retain the historical behaviour; user functions can widen
    /// this range when they declare default parameters.
    fn accepts_arity(&self, count: usize) -> bool {
        count == self.arity()
    }

    /// Executes the function call using the given context and argument values.
    fn call(
        &self,
        ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError>;
}

/// Abstract representation of a function body to allow future bytecode/VM/LLVM extension.
#[derive(Debug, Clone)]
pub enum FunctionBody {
    Ast(techscript_ast::Block),
    Bytecode(Vec<u8>),
}

/// Dynamic representation of a user-defined function closure.
pub struct UserFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: FunctionBody,
    pub closure: Rc<RefCell<Environment>>,
}

impl Callable for UserFunction {
    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        _args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        // Crate is responsible ONLY for function definition infrastructure.
        // Interpreter will intercept and traverse AST.
        Ok(RuntimeValue::Null)
    }
}
