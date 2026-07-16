//! # TechScript Compiler Driver — Compiler Plugins
//!
//! Provides the plugin infrastructure using the EventListener model.
//! Future plugins can be registered with the pipeline to inspect AST, IR, or bytecode.

use crate::events::EventListener;

pub trait CompilerPlugin: EventListener {
    /// Returns the unique name of the plugin.
    fn name(&self) -> &'static str;

    /// Returns the semantic version of the plugin.
    fn version(&self) -> &'static str;
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn CompilerPlugin>>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    /// Creates an empty plugin registry.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Registers a plugin.
    pub fn register(&mut self, plugin: Box<dyn CompilerPlugin>) {
        self.plugins.push(plugin);
    }

    /// Retrieves all registered plugins.
    pub fn plugins(&self) -> &[Box<dyn CompilerPlugin>] {
        &self.plugins
    }
}
