//! # TechScript Compiler Driver — Structured Logging
//!
//! Provides structured, leveled logging for the `tsc` compiler driver.
//! Supports human-readable colored output and machine-readable JSON lines.
//! The `Logger` is also an `EventListener` that prints stage progress.

use colored::Colorize;
use std::time::Duration;

use crate::events::{CompilationEvent, EventListener};

/// Log verbosity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Errors only. No progress output.
    Quiet,
    /// Errors, warnings, and build summary. Default.
    Normal,
    /// + Info about each compilation stage.
    Verbose,
    /// + Internal pipeline detail (token counts, IR sizes, cache decisions).
    Trace,
}

/// Log output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// ANSI-colored human-readable output.
    Human,
    /// Machine-readable JSON lines (one object per log entry).
    Json,
}

/// Status of a compilation stage.
pub enum StageStatus {
    Started,
    Completed { duration: Duration },
    Skipped { reason: &'static str },
    Failed { error: String },
}

/// Structured logger for the compiler driver.
pub struct Logger {
    pub level: LogLevel,
    pub format: LogFormat,
}

impl Logger {
    /// Creates a new logger with the given verbosity and format.
    pub fn new(level: LogLevel, format: LogFormat) -> Self {
        Self { level, format }
    }

    /// Creates a default logger (Normal + Human).
    pub fn default_logger() -> Self {
        Self {
            level: LogLevel::Normal,
            format: LogFormat::Human,
        }
    }

    /// Logs an error message (always shown).
    pub fn error(&self, msg: &str) {
        match self.format {
            LogFormat::Human => eprintln!("{} {}", "error:".red().bold(), msg),
            LogFormat::Json => eprintln!(
                "{}",
                serde_json::json!({ "level": "error", "message": msg })
            ),
        }
    }

    /// Logs a warning message (shown at Normal and above).
    pub fn warn(&self, msg: &str) {
        if self.level < LogLevel::Normal {
            return;
        }
        match self.format {
            LogFormat::Human => eprintln!("{} {}", "warning:".yellow().bold(), msg),
            LogFormat::Json => eprintln!(
                "{}",
                serde_json::json!({ "level": "warning", "message": msg })
            ),
        }
    }

    /// Logs an info message (shown at Verbose and above).
    pub fn info(&self, msg: &str) {
        if self.level < LogLevel::Verbose {
            return;
        }
        match self.format {
            LogFormat::Human => eprintln!("{} {}", "info:".cyan(), msg),
            LogFormat::Json => {
                eprintln!("{}", serde_json::json!({ "level": "info", "message": msg }))
            }
        }
    }

    /// Logs a trace message (shown at Trace only).
    pub fn trace(&self, msg: &str) {
        if self.level < LogLevel::Trace {
            return;
        }
        match self.format {
            LogFormat::Human => eprintln!("{} {}", "trace:".dimmed(), msg),
            LogFormat::Json => eprintln!(
                "{}",
                serde_json::json!({ "level": "trace", "message": msg })
            ),
        }
    }

    /// Prints a stage status line (shown at Verbose and above).
    pub fn stage(&self, name: &str, status: StageStatus) {
        if self.level < LogLevel::Verbose {
            return;
        }
        match self.format {
            LogFormat::Human => match status {
                StageStatus::Started => {
                    eprintln!("  {} {}...", "→".cyan(), name);
                }
                StageStatus::Completed { duration } => {
                    eprintln!(
                        "  {} {} ({})",
                        "✓".green(),
                        name,
                        format_duration(duration).dimmed()
                    );
                }
                StageStatus::Skipped { reason } => {
                    eprintln!("  {} {} [{}]", "⊘".dimmed(), name, reason.dimmed());
                }
                StageStatus::Failed { error } => {
                    eprintln!("  {} {} — {}", "✗".red(), name, error.red());
                }
            },
            LogFormat::Json => {
                let (status_str, extra) = match &status {
                    StageStatus::Started => ("started", String::new()),
                    StageStatus::Completed { duration } => {
                        ("completed", format!("{}", duration.as_millis()))
                    }
                    StageStatus::Skipped { reason } => ("skipped", reason.to_string()),
                    StageStatus::Failed { error } => ("failed", error.clone()),
                };
                eprintln!(
                    "{}",
                    serde_json::json!({
                        "level": "stage",
                        "stage": name,
                        "status": status_str,
                        "detail": extra,
                    })
                );
            }
        }
    }

    /// Prints a bold header (Normal and above).
    pub fn header(&self, msg: &str) {
        if self.level < LogLevel::Normal {
            return;
        }
        if self.format == LogFormat::Human {
            println!("{}", msg.bold());
        }
    }

    /// Prints a success summary (Normal and above).
    pub fn success(&self, msg: &str) {
        if self.level < LogLevel::Normal {
            return;
        }
        match self.format {
            LogFormat::Human => println!("{} {}", "✓".green().bold(), msg.green()),
            LogFormat::Json => {
                println!(
                    "{}",
                    serde_json::json!({ "level": "success", "message": msg })
                )
            }
        }
    }
}

/// Formats a duration as a human-readable string.
pub fn format_duration(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1 {
        format!("{} µs", d.as_micros())
    } else if ms < 1000 {
        format!("{} ms", ms)
    } else {
        format!("{:.2} s", d.as_secs_f64())
    }
}

/// Logger is an EventListener — prints stage progress.
impl EventListener for Logger {
    fn on_event(&mut self, event: &CompilationEvent) {
        match event {
            CompilationEvent::BeforeLex { path } => {
                self.stage(&format!("Lexing {}", path.display()), StageStatus::Started);
            }
            CompilationEvent::AfterLex {
                path,
                token_count,
                duration,
            } => {
                self.trace(&format!("{}: {} tokens", path.display(), token_count));
                self.stage(
                    &format!("Lexing {}", path.display()),
                    StageStatus::Completed {
                        duration: *duration,
                    },
                );
            }
            CompilationEvent::BeforeParse { path } => {
                self.stage(&format!("Parsing {}", path.display()), StageStatus::Started);
            }
            CompilationEvent::AfterParse {
                path,
                node_count,
                duration,
            } => {
                self.trace(&format!("{}: {} AST nodes", path.display(), node_count));
                self.stage(
                    &format!("Parsing {}", path.display()),
                    StageStatus::Completed {
                        duration: *duration,
                    },
                );
            }
            CompilationEvent::BeforeSemantic { path } => {
                self.stage(
                    &format!("Semantic analysis {}", path.display()),
                    StageStatus::Started,
                );
            }
            CompilationEvent::AfterSemantic {
                path,
                symbol_count,
                duration,
            } => {
                self.trace(&format!("{}: {} symbols", path.display(), symbol_count));
                self.stage(
                    &format!("Semantic analysis {}", path.display()),
                    StageStatus::Completed {
                        duration: *duration,
                    },
                );
            }
            CompilationEvent::BeforeLowering { path } => {
                self.stage(
                    &format!("IR lowering {}", path.display()),
                    StageStatus::Started,
                );
            }
            CompilationEvent::AfterLowering {
                path,
                function_count,
                duration,
            } => {
                self.trace(&format!(
                    "{}: {} IR functions",
                    path.display(),
                    function_count
                ));
                self.stage(
                    &format!("IR lowering {}", path.display()),
                    StageStatus::Completed {
                        duration: *duration,
                    },
                );
            }
            CompilationEvent::BeforeOptimize { path } => {
                self.stage(
                    &format!("Optimization {}", path.display()),
                    StageStatus::Started,
                );
            }
            CompilationEvent::AfterOptimize {
                path,
                passes_run,
                duration,
            } => {
                self.trace(&format!(
                    "{}: {} optimization passes",
                    path.display(),
                    passes_run
                ));
                self.stage(
                    &format!("Optimization {}", path.display()),
                    StageStatus::Completed {
                        duration: *duration,
                    },
                );
            }
            CompilationEvent::BeforeBytecode { path } => {
                self.stage(
                    &format!("Bytecode generation {}", path.display()),
                    StageStatus::Started,
                );
            }
            CompilationEvent::AfterBytecode {
                path,
                instruction_count,
                duration,
            } => {
                self.trace(&format!(
                    "{}: {} instructions",
                    path.display(),
                    instruction_count
                ));
                self.stage(
                    &format!("Bytecode generation {}", path.display()),
                    StageStatus::Completed {
                        duration: *duration,
                    },
                );
            }
            CompilationEvent::BuildStarted { unit_count } => {
                self.info(&format!(
                    "Compiling {} compilation unit{}",
                    unit_count,
                    if *unit_count == 1 { "" } else { "s" }
                ));
            }
            CompilationEvent::BuildFinished { stats } => {
                if self.level >= LogLevel::Normal && self.format == LogFormat::Human {
                    println!("{}", stats.render_human());
                }
            }
        }
    }
}
