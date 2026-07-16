use crate::types::TypeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resolved symbol metadata detailing kind, mutability, and type references.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub is_constant: bool,
    pub is_function: bool,
    pub is_type: bool,
    pub type_id: TypeId,
}

impl Symbol {
    pub fn new(
        name: String,
        is_constant: bool,
        is_function: bool,
        is_type: bool,
        type_id: TypeId,
    ) -> Self {
        Self {
            name,
            is_constant,
            is_function,
            is_type,
            type_id,
        }
    }
}

/// A scope block frame keeping variable maps.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
}

/// Multi-layered symbol table mapping namespaces.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()], // Always start with a global scope
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn register(&mut self, name: String, symbol: Symbol) {
        if let Some(current) = self.scopes.last_mut() {
            current.symbols.insert(name, symbol);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.symbols.get_mut(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn check_shadowing(&self, name: &str) -> bool {
        if self.scopes.len() <= 1 {
            return false;
        }
        // Check only parent scopes (excluding the last/current scope)
        for scope in self.scopes.iter().take(self.scopes.len() - 1) {
            if scope.symbols.contains_key(name) {
                return true;
            }
        }
        false
    }
}
