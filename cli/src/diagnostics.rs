//! # TechScript Compiler Driver — Diagnostics
//!
//! Rich multi-span diagnostic rendering with colored, plain, and JSON output.
//! Bridges the existing `techscript_errors::Diagnostic` to the rich format
//! and accumulates post-compilation statistics.

use colored::Colorize;
use std::sync::Arc;
use std::time::Duration;

use techscript_common::{FileId, SourceManager};
use techscript_errors::{Diagnostic, DiagnosticLevel, ErrorCode};

// ─── Severity ────────────────────────────────────────────────────────────────

/// Severity levels for rich diagnostic messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
    Hint,
}

impl Severity {
    fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
            Self::Hint => "hint",
        }
    }

    fn colored_label(self) -> colored::ColoredString {
        match self {
            Self::Error => "error".red().bold(),
            Self::Warning => "warning".yellow().bold(),
            Self::Note => "note".cyan().bold(),
            Self::Help => "help".green().bold(),
            Self::Hint => "hint".blue().bold(),
        }
    }
}

// ─── SpanLabel ───────────────────────────────────────────────────────────────

/// A source span paired with an optional label.
#[derive(Debug, Clone)]
pub struct SpanLabel {
    pub span: techscript_common::Span,
    pub file_id: FileId,
    pub label: Option<String>,
}

// ─── FixSuggestion ───────────────────────────────────────────────────────────

/// A suggested text replacement at a source span.
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub message: String,
    pub span: techscript_common::Span,
    pub replacement: String,
}

// ─── RichDiagnostic ──────────────────────────────────────────────────────────

/// A rich diagnostic with multiple spans, related diagnostics, and suggestions.
#[derive(Debug, Clone)]
pub struct RichDiagnostic {
    pub severity: Severity,
    pub code: Option<ErrorCode>,
    pub message: String,
    pub primary_span: Option<SpanLabel>,
    pub secondary_spans: Vec<SpanLabel>,
    pub related: Vec<RichDiagnostic>,
    pub suggestions: Vec<FixSuggestion>,
}

impl RichDiagnostic {
    /// Creates a simple error with no spans.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: None,
            message: message.into(),
            primary_span: None,
            secondary_spans: Vec::new(),
            related: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Creates a warning with no spans.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code: None,
            message: message.into(),
            primary_span: None,
            secondary_spans: Vec::new(),
            related: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Attaches a primary span with an optional label.
    pub fn with_primary(mut self, span: SpanLabel) -> Self {
        self.primary_span = Some(span);
        self
    }

    /// Adds a secondary span.
    pub fn with_secondary(mut self, span: SpanLabel) -> Self {
        self.secondary_spans.push(span);
        self
    }

    /// Adds a fix suggestion.
    pub fn with_suggestion(mut self, suggestion: FixSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Bridges a legacy `techscript_errors::Diagnostic` to a `RichDiagnostic`.
    pub fn from_legacy(diag: &Diagnostic, file_id: FileId) -> Self {
        let severity = match diag.level {
            DiagnosticLevel::Error => Severity::Error,
            DiagnosticLevel::Warning => Severity::Warning,
            DiagnosticLevel::Note => Severity::Note,
        };
        let primary = SpanLabel {
            span: diag.span,
            file_id,
            label: diag.help.clone(),
        };
        let mut rich = Self {
            severity,
            code: Some(diag.code),
            message: diag.message.clone(),
            primary_span: Some(primary),
            secondary_spans: Vec::new(),
            related: Vec::new(),
            suggestions: Vec::new(),
        };
        if let Some(help) = &diag.help {
            rich.related.push(RichDiagnostic {
                severity: Severity::Help,
                code: None,
                message: help.clone(),
                primary_span: None,
                secondary_spans: Vec::new(),
                related: Vec::new(),
                suggestions: Vec::new(),
            });
        }
        rich
    }
}

// ─── DiagnosticOutput ────────────────────────────────────────────────────────

/// Output mode for the diagnostic renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticOutput {
    /// ANSI-colored terminal output. Auto-detected via `colored`.
    Colored,
    /// Plain text (no ANSI). For CI and redirected output.
    Plain,
    /// Machine-readable JSON (one object per diagnostic).
    Json,
}

// ─── DiagnosticRenderer ──────────────────────────────────────────────────────

/// Renders `RichDiagnostic`s to the configured output format.
pub struct DiagnosticRenderer<'a> {
    output: DiagnosticOutput,
    source_manager: &'a SourceManager,
}

impl<'a> DiagnosticRenderer<'a> {
    /// Creates a new renderer.
    pub fn new(output: DiagnosticOutput, source_manager: &'a SourceManager) -> Self {
        Self {
            output,
            source_manager,
        }
    }

    /// Auto-detects terminal color support.
    pub fn auto_detect(source_manager: &'a SourceManager) -> Self {
        // colored crate respects NO_COLOR and TERM env vars automatically.
        let output = if colored::control::SHOULD_COLORIZE.should_colorize() {
            DiagnosticOutput::Colored
        } else {
            DiagnosticOutput::Plain
        };
        Self {
            output,
            source_manager,
        }
    }

    /// Renders a `RichDiagnostic` to a string.
    pub fn render(&self, diag: &RichDiagnostic) -> String {
        match self.output {
            DiagnosticOutput::Colored | DiagnosticOutput::Plain => self.render_human(diag),
            DiagnosticOutput::Json => self.render_json(diag),
        }
    }

    /// Prints a `RichDiagnostic` to stderr.
    pub fn emit(&self, diag: &RichDiagnostic) {
        eprintln!("{}", self.render(diag));
    }

    fn render_human(&self, diag: &RichDiagnostic) -> String {
        let mut out = String::new();

        // Header: "error[E0300]: message"
        let header = if let Some(code) = diag.code {
            format!("{:?}", code)
        } else {
            String::new()
        };

        if self.output == DiagnosticOutput::Colored {
            if header.is_empty() {
                out.push_str(&format!(
                    "{}: {}\n",
                    diag.severity.colored_label(),
                    diag.message.bold()
                ));
            } else {
                out.push_str(&format!(
                    "{}[{}]: {}\n",
                    diag.severity.colored_label(),
                    header.white(),
                    diag.message.bold()
                ));
            }
        } else {
            if header.is_empty() {
                out.push_str(&format!("{}: {}\n", diag.severity.label(), diag.message));
            } else {
                out.push_str(&format!(
                    "{}[{}]: {}\n",
                    diag.severity.label(),
                    header,
                    diag.message
                ));
            }
        }

        // Primary span with source snippet
        if let Some(primary) = &diag.primary_span {
            if let Some(file) = self.source_manager.get_file(primary.file_id) {
                let (line, col) = file.line_col(primary.span.start).unwrap_or((1, 1));

                let file_ref = format!("  --> {}:{}:{}", file.path().display(), line, col);
                if self.output == DiagnosticOutput::Colored {
                    out.push_str(&format!("{}\n", file_ref.cyan()));
                } else {
                    out.push_str(&format!("{}\n", file_ref));
                }

                // Source line
                if let Some(line_text) = file.line_content(line) {
                    let line_num = format!("{:>4}", line);
                    let bar = "|";
                    if self.output == DiagnosticOutput::Colored {
                        out.push_str(&format!("{} {}\n", line_num.cyan(), bar.cyan()));
                        out.push_str(&format!(
                            "{} {} {}\n",
                            line_num.cyan(),
                            bar.cyan(),
                            line_text
                        ));
                    } else {
                        out.push_str(&format!("{} {}\n", line_num, bar));
                        out.push_str(&format!("{} {} {}\n", line_num, bar, line_text));
                    }

                    // Caret underline — Unicode-safe (count chars, not bytes)
                    let caret_offset = col.saturating_sub(1);
                    let span_len = {
                        let end = primary.span.end.min(primary.span.start + line_text.len());
                        (end - primary.span.start).max(1)
                    };
                    let spaces = " ".repeat(caret_offset);
                    let carets = "^".repeat(span_len);
                    let caret_line = if let Some(lbl) = &primary.label {
                        format!("{}{}  {}", spaces, carets, lbl)
                    } else {
                        format!("{}{}", spaces, carets)
                    };
                    if self.output == DiagnosticOutput::Colored {
                        out.push_str(&format!(
                            "     {} {}\n",
                            bar.cyan(),
                            caret_line.red().bold()
                        ));
                    } else {
                        out.push_str(&format!("     {} {}\n", bar, caret_line));
                    }
                }
            }
        }

        // Related diagnostics (help, notes)
        for related in &diag.related {
            if self.output == DiagnosticOutput::Colored {
                out.push_str(&format!(
                    "  {}: {}\n",
                    related.severity.colored_label(),
                    related.message
                ));
            } else {
                out.push_str(&format!(
                    "  {}: {}\n",
                    related.severity.label(),
                    related.message
                ));
            }
        }

        // Fix suggestions
        for suggestion in &diag.suggestions {
            if self.output == DiagnosticOutput::Colored {
                out.push_str(&format!(
                    "  {}: {} → `{}`\n",
                    "suggestion".green().bold(),
                    suggestion.message,
                    suggestion.replacement.green()
                ));
            } else {
                out.push_str(&format!(
                    "  suggestion: {} → `{}`\n",
                    suggestion.message, suggestion.replacement
                ));
            }
        }

        // Documentation link if code is present
        if let Some(code) = diag.code {
            if self.output == DiagnosticOutput::Colored {
                out.push_str(&format!(
                    "  {}: For more details see: {}\n",
                    "note".cyan().bold(),
                    format!("https://github.com/Tcode-Motion/TechScript-2.0/docs/errors#{:?}", code).underline()
                ));
            } else {
                out.push_str(&format!(
                    "  note: For more details see: https://github.com/Tcode-Motion/TechScript-2.0/docs/errors#{:?}\n",
                    code
                ));
            }
        }

        out
    }

    fn render_json(&self, diag: &RichDiagnostic) -> String {
        let span_json = diag.primary_span.as_ref().map(|s| {
            serde_json::json!({
                "start": s.span.start,
                "end": s.span.end,
                "file_id": s.file_id.as_u32(),
                "label": s.label,
            })
        });

        let related_json: Vec<_> = diag
            .related
            .iter()
            .map(|r| serde_json::json!({ "severity": r.severity.label(), "message": r.message }))
            .collect();

        let obj = serde_json::json!({
            "severity": diag.severity.label(),
            "code": diag.code.map(|c| format!("{:?}", c)),
            "message": diag.message,
            "span": span_json,
            "related": related_json,
        });

        obj.to_string()
    }
}

// ─── DiagnosticStats ─────────────────────────────────────────────────────────

/// Accumulated statistics reported after a build.
#[derive(Debug, Default, Clone)]
pub struct DiagnosticStats {
    pub errors: usize,
    pub warnings: usize,
    pub notes: usize,
    pub hints: usize,
    pub files_compiled: usize,
    pub files_cached: usize,
    pub elapsed: Duration,
    pub peak_memory_bytes: usize,
}

impl DiagnosticStats {
    /// Cache hit percentage (0.0–100.0).
    pub fn cache_hit_percent(&self) -> f64 {
        let total = self.files_compiled + self.files_cached;
        if total == 0 {
            0.0
        } else {
            self.files_cached as f64 / total as f64 * 100.0
        }
    }

    /// Records a `RichDiagnostic` in the stats counters.
    pub fn record(&mut self, diag: &RichDiagnostic) {
        match diag.severity {
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
            Severity::Note => self.notes += 1,
            Severity::Help | Severity::Hint => self.hints += 1,
        }
    }

    /// Human-readable summary line.
    pub fn render_human(&self) -> String {
        let total = self.files_compiled + self.files_cached;
        let cache_str = if total > 0 {
            format!(
                " ({} cached, {:.0}% hit)",
                self.files_cached,
                self.cache_hit_percent()
            )
        } else {
            String::new()
        };

        let mem_mb = self.peak_memory_bytes as f64 / (1024.0 * 1024.0);
        let elapsed_ms = self.elapsed.as_millis();

        format!(
            "  Compiled {} file{}{}\n  Errors: {} | Warnings: {} | Notes: {}\n  Elapsed: {}ms | Peak memory: {:.1} MB",
            self.files_compiled,
            if self.files_compiled == 1 { "" } else { "s" },
            cache_str,
            self.errors,
            self.warnings,
            self.notes,
            elapsed_ms,
            mem_mb,
        )
    }

    /// JSON representation of the stats.
    pub fn render_json(&self) -> String {
        serde_json::json!({
            "errors": self.errors,
            "warnings": self.warnings,
            "notes": self.notes,
            "hints": self.hints,
            "files_compiled": self.files_compiled,
            "files_cached": self.files_cached,
            "cache_hit_percent": self.cache_hit_percent(),
            "elapsed_ms": self.elapsed.as_millis(),
            "peak_memory_bytes": self.peak_memory_bytes,
        })
        .to_string()
    }
}
