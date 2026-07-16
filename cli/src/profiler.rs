//! # TechScript Compiler Driver — Timing Profiler & Memory Statistics
//!
//! Records per-stage durations and aggregate memory statistics.
//! The `TimingProfiler` is an `EventListener` so it receives events
//! automatically from the compilation pipeline.

use std::time::{Duration, Instant};

use colored::Colorize;

use crate::events::{CompilationEvent, EventListener};
use crate::logging::format_duration;

// ─── StageTimings ────────────────────────────────────────────────────────────

/// Timing record for a single pipeline stage.
#[derive(Debug, Clone)]
pub struct StageTimings {
    pub stage: String,
    pub duration: Duration,
    pub item_count: usize,
    pub item_label: &'static str,
}

// ─── MemoryStats ─────────────────────────────────────────────────────────────

/// Aggregate memory and size statistics for a compilation.
#[derive(Debug, Default, Clone)]
pub struct MemoryStats {
    pub peak_memory_bytes: usize,
    pub ir_size_bytes: usize,
    pub bytecode_size_bytes: usize,
    pub ast_node_count: usize,
    pub symbol_count: usize,
    pub function_count: usize,
    pub basic_block_count: usize,
}

// ─── TimingProfiler ──────────────────────────────────────────────────────────

/// Records stage timings and memory stats for the `--time` flag.
pub struct TimingProfiler {
    pub stages: Vec<StageTimings>,
    pub memory: MemoryStats,
    pub total_start: Option<Instant>,
    pub enabled: bool,

    // Pending stage start times (keyed by a simple index)
    pending: std::collections::HashMap<String, Instant>,
}

impl Default for TimingProfiler {
    fn default() -> Self {
        Self::new(false)
    }
}

impl TimingProfiler {
    /// Creates a profiler. Pass `enabled = true` when `--time` flag is set.
    pub fn new(enabled: bool) -> Self {
        Self {
            stages: Vec::new(),
            memory: MemoryStats::default(),
            total_start: None,
            enabled,
            pending: std::collections::HashMap::new(),
        }
    }

    /// Records an already-measured stage timing.
    pub fn record_stage(
        &mut self,
        stage: impl Into<String>,
        duration: Duration,
        item_count: usize,
        item_label: &'static str,
    ) {
        self.stages.push(StageTimings {
            stage: stage.into(),
            duration,
            item_count,
            item_label,
        });
    }

    /// Records memory statistics.
    pub fn record_memory(&mut self, stats: MemoryStats) {
        self.memory = stats;
    }

    /// Renders the timing table as a human-readable string.
    pub fn render(&self) -> String {
        if self.stages.is_empty() {
            return String::new();
        }

        let total: Duration = self.stages.iter().map(|s| s.duration).sum();
        let mut out = String::new();

        out.push_str(&format!(
            "{}\n",
            "─── Compilation Timings ────────────────────────────────".dimmed()
        ));
        out.push_str(&format!(
            "{:<24} {:>10}   {}\n",
            "Stage".bold(),
            "Duration".bold(),
            "Items".bold()
        ));
        out.push_str(&format!("{}\n", "─".repeat(50).dimmed()));

        for s in &self.stages {
            out.push_str(&format!(
                "{:<24} {:>10}   {} {}\n",
                s.stage,
                format_duration(s.duration).cyan().to_string(),
                s.item_count,
                s.item_label.dimmed()
            ));
        }

        out.push_str(&format!("{}\n", "─".repeat(50).dimmed()));
        out.push_str(&format!(
            "{:<24} {:>10}\n\n",
            "Total".bold(),
            format_duration(total).green().bold().to_string()
        ));

        // Memory section
        let mem = &self.memory;
        if mem.peak_memory_bytes > 0 || mem.ast_node_count > 0 {
            out.push_str(&format!(
                "{}\n",
                "─── Memory Statistics ──────────────────────────────────".dimmed()
            ));
            out.push_str(&format!(
                "{:<24} {:>10}\n",
                "Peak memory",
                format_bytes(mem.peak_memory_bytes).cyan().to_string()
            ));
            if mem.ir_size_bytes > 0 {
                out.push_str(&format!(
                    "{:<24} {:>10}\n",
                    "IR module",
                    format_bytes(mem.ir_size_bytes).cyan().to_string()
                ));
            }
            if mem.bytecode_size_bytes > 0 {
                out.push_str(&format!(
                    "{:<24} {:>10}\n",
                    "Bytecode",
                    format_bytes(mem.bytecode_size_bytes).cyan().to_string()
                ));
            }
            if mem.ast_node_count > 0 {
                out.push_str(&format!("{:<24} {:>10}\n", "AST nodes", mem.ast_node_count));
            }
            if mem.function_count > 0 {
                out.push_str(&format!(
                    "{:<24} {:>10}\n",
                    "IR functions", mem.function_count
                ));
            }
            if mem.basic_block_count > 0 {
                out.push_str(&format!(
                    "{:<24} {:>10}\n",
                    "Basic blocks", mem.basic_block_count
                ));
            }
        }

        out
    }

    /// Renders timings as JSON.
    pub fn render_json(&self) -> String {
        let stages: Vec<_> = self
            .stages
            .iter()
            .map(|s| {
                serde_json::json!({
                    "stage": s.stage,
                    "duration_ms": s.duration.as_millis(),
                    "item_count": s.item_count,
                    "item_label": s.item_label,
                })
            })
            .collect();

        serde_json::json!({
            "stages": stages,
            "memory": {
                "peak_bytes": self.memory.peak_memory_bytes,
                "ir_bytes": self.memory.ir_size_bytes,
                "bytecode_bytes": self.memory.bytecode_size_bytes,
                "ast_nodes": self.memory.ast_node_count,
                "functions": self.memory.function_count,
                "basic_blocks": self.memory.basic_block_count,
            }
        })
        .to_string()
    }
}

/// TimingProfiler is an EventListener — records stage durations from events.
impl EventListener for TimingProfiler {
    fn on_event(&mut self, event: &CompilationEvent) {
        if !self.enabled {
            return;
        }
        match event {
            CompilationEvent::BuildStarted { .. } => {
                self.total_start = Some(Instant::now());
            }
            CompilationEvent::AfterLex {
                path,
                token_count,
                duration,
            } => {
                let key = format!("lex:{}", path.display());
                _ = self.pending.remove(&key);
                self.record_stage("Lexing", *duration, *token_count, "tokens");
            }
            CompilationEvent::AfterParse {
                path,
                node_count,
                duration,
            } => {
                let key = format!("parse:{}", path.display());
                _ = self.pending.remove(&key);
                self.record_stage("Parsing", *duration, *node_count, "AST nodes");
            }
            CompilationEvent::AfterSemantic {
                path,
                symbol_count,
                duration,
            } => {
                let key = format!("sem:{}", path.display());
                _ = self.pending.remove(&key);
                self.record_stage("Semantic analysis", *duration, *symbol_count, "symbols");
            }
            CompilationEvent::AfterLowering {
                path,
                function_count,
                duration,
            } => {
                let key = format!("lower:{}", path.display());
                _ = self.pending.remove(&key);
                self.record_stage("IR lowering", *duration, *function_count, "functions");
            }
            CompilationEvent::AfterOptimize {
                path,
                passes_run,
                duration,
            } => {
                let key = format!("opt:{}", path.display());
                _ = self.pending.remove(&key);
                self.record_stage("Optimization", *duration, *passes_run, "passes");
            }
            CompilationEvent::AfterBytecode {
                path,
                instruction_count,
                duration,
            } => {
                let key = format!("bc:{}", path.display());
                _ = self.pending.remove(&key);
                self.record_stage(
                    "Bytecode generation",
                    *duration,
                    *instruction_count,
                    "instructions",
                );
            }
            _ => {}
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
