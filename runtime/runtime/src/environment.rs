use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::value::RuntimeValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Stores a bound runtime value and its mutability flag.
#[derive(Debug, Clone)]
pub struct Binding {
    pub value: RuntimeValue,
    pub is_constant: bool,
}

/// Lexical scope environment mapping identifier names to runtime values.
#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    bindings: HashMap<String, Binding>,
}

impl Environment {
    /// Creates a new environment instance, optionally pointing to a parent scope.
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            parent,
            bindings: HashMap::new(),
        }
    }

    /// Registers a variable or constant binding in the current local scope.
    pub fn define(&mut self, name: String, value: RuntimeValue, is_constant: bool) {
        self.bindings.insert(name, Binding { value, is_constant });
    }

    /// Assigns a new value to an existing variable binding by traversing scope parents.
    pub fn assign(&mut self, name: &str, value: RuntimeValue) -> Result<(), RuntimeError> {
        if let Some(binding) = self.bindings.get_mut(name) {
            if binding.is_constant {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation(format!(
                        "Cannot reassign constant variable '{}'",
                        name
                    )),
                    None,
                    None,
                ));
            }
            binding.value = value;
            return Ok(());
        }

        if let Some(ref parent) = self.parent {
            if parent.borrow().lookup(name).is_ok() {
                return parent.borrow_mut().assign(name, value);
            }
        }

        // First assignment in this scope: define it
        self.define(name.to_string(), value, false);
        Ok(())
    }

    /// Looks up a variable value by recursively searching scope parents.
    pub fn lookup(&self, name: &str) -> Result<RuntimeValue, RuntimeError> {
        if let Some(binding) = self.bindings.get(name) {
            return Ok(binding.value.clone());
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow().lookup(name);
        }

        Err(RuntimeError::new(
            RuntimeErrorKind::UndefinedVariable(name.to_string()),
            None,
            None,
        ))
    }

    /// Retrieves a cloned reference pointer to the parent environment frame.
    pub fn parent(&self) -> Option<Rc<RefCell<Environment>>> {
        self.parent.clone()
    }
}
