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
        let mut global_scope = Scope::new();
        let any_type = TypeId(0);
        global_scope.symbols.insert("len".to_string(), Symbol::new("len".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("range".to_string(), Symbol::new("range".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("ask".to_string(), Symbol::new("ask".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("push".to_string(), Symbol::new("push".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("insert".to_string(), Symbol::new("insert".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("parse".to_string(), Symbol::new("parse".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("write_file".to_string(), Symbol::new("write_file".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("read_file".to_string(), Symbol::new("read_file".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("spawn_async".to_string(), Symbol::new("spawn_async".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("print".to_string(), Symbol::new("print".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("println".to_string(), Symbol::new("println".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("std".to_string(), Symbol::new("std".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("get".to_string(), Symbol::new("get".to_string(), true, true, false, any_type));
        global_scope.symbols.insert("set".to_string(), Symbol::new("set".to_string(), true, true, false, any_type));

        Self {
            scopes: vec![global_scope],
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
