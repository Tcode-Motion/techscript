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

        // Helper closure to avoid repetition
        let mut add = |name: &str| {
            global_scope.symbols.insert(
                name.to_string(),
                Symbol::new(name.to_string(), true, true, false, any_type),
            );
        };

        // ── Core native-registry built-ins ──────────────────────────────────
        add("say");
        add("ask");
        add("len");
        add("range");
        add("type_of");
        add("to_int");
        add("to_float");
        add("to_str");
        add("to_bool");
        add("assert");
        add("exit");
        add("env");
        add("file");
        add("panic");

        // ── Type-conversion aliases used in v1.0.8 code ─────────────────────
        add("str"); // same as to_str
        add("int"); // same as to_int
        add("float"); // same as to_float
        add("bool"); // same as to_bool

        // ── Stdlib module-level globals ──────────────────────────────────────
        add("std");
        add("math");
        add("random");
        add("strings");
        add("json");
        add("io");
        add("fs");
        add("sys");
        add("net");
        add("http");
        add("xml");
        add("csv");
        add("yaml");
        add("datetime");
        add("crypto");
        add("hash");
        add("regex");
        add("path");
        add("thread");
        add("sync");
        add("testing");
        add("logging");
        add("compress");
        add("encoding");
        add("uuid");
        add("url");
        add("system");
        add("toml");
        add("database");
        add("graphics");
        add("ai");

        // ── Miscellaneous globals used in existing stdlib code ───────────────
        add("push");
        add("insert");
        add("parse");
        add("write_file");
        add("read_file");
        add("spawn_async");
        add("print");
        add("println");
        add("get");
        add("set");

        // ── Math functions re-exported at global scope ───────────────────────
        add("sqrt");
        add("floor");
        add("ceil");
        add("round");
        add("abs");
        add("pow");
        add("log");
        add("sin");
        add("cos");
        add("tan");
        add("pi");

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
