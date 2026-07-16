//! # TechScript Compiler Driver — Events
//!
//! Event-driven compilation pipeline hooks.
//! The `EventBus` distributes `CompilationEvent`s to all registered listeners.
//! Built-in listeners: `TimingProfiler`, `Logger`.
//! External listeners: compiler plugins.

use std::path::Path;
use std::time::Duration;

use crate::diagnostics::DiagnosticStats;

/// Events emitted at each pipeline stage boundary during compilation.
pub enum CompilationEvent<'a> {
    /// Emitted before lexing a file.
    BeforeLex { path: &'a Path },
    /// Emitted after lexing completes successfully.
    AfterLex {
        path: &'a Path,
        token_count: usize,
        duration: Duration,
    },
    /// Emitted before parsing.
    BeforeParse { path: &'a Path },
    /// Emitted after parsing completes.
    AfterParse {
        path: &'a Path,
        node_count: usize,
        duration: Duration,
    },
    /// Emitted before semantic analysis.
    BeforeSemantic { path: &'a Path },
    /// Emitted after semantic analysis completes.
    AfterSemantic {
        path: &'a Path,
        symbol_count: usize,
        duration: Duration,
    },
    /// Emitted before IR lowering.
    BeforeLowering { path: &'a Path },
    /// Emitted after IR lowering completes.
    AfterLowering {
        path: &'a Path,
        function_count: usize,
        duration: Duration,
    },
    /// Emitted before the optimization pass.
    BeforeOptimize { path: &'a Path },
    /// Emitted after optimization completes.
    AfterOptimize {
        path: &'a Path,
        passes_run: usize,
        duration: Duration,
    },
    /// Emitted before bytecode generation.
    BeforeBytecode { path: &'a Path },
    /// Emitted after bytecode generation completes.
    AfterBytecode {
        path: &'a Path,
        instruction_count: usize,
        duration: Duration,
    },
    /// Emitted when a build starts (entire project).
    BuildStarted { unit_count: usize },
    /// Emitted when the entire build finishes.
    BuildFinished { stats: &'a DiagnosticStats },
}

/// Trait for objects that receive compilation events.
pub trait EventListener: Send {
    fn on_event(&mut self, event: &CompilationEvent);
}

/// Central event bus distributing events to all registered listeners.
pub struct EventBus {
    listeners: Vec<Box<dyn EventListener>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// Creates an empty event bus.
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    /// Registers an event listener.
    pub fn subscribe(&mut self, listener: Box<dyn EventListener>) {
        self.listeners.push(listener);
    }

    /// Emits an event to all registered listeners in registration order.
    pub fn emit(&mut self, event: &CompilationEvent) {
        for listener in &mut self.listeners {
            listener.on_event(event);
        }
    }
}
