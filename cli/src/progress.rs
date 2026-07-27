//! # TechScript CLI Progress Bar Utility
//!
//! Renders visual progress bars in-place inside terminal environments.

use colored::Colorize;
use std::io::Write;

pub struct ProgressBar {
    width: usize,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self { width: 25 }
    }

    /// Renders the progress bar at a specific percentage and label.
    pub fn update(&self, percent: usize, label: &str) {
        let filled = (percent * self.width) / 100;
        let empty = self.width - filled;
        let color_enabled = colored::control::SHOULD_COLORIZE.should_colorize();

        let bar_str = if color_enabled {
            format!("{}{}", "=".repeat(filled).cyan(), " ".repeat(empty))
        } else {
            format!("{}{}", "=".repeat(filled), " ".repeat(empty))
        };

        print!(
            "\r  [{}] {:>3}% — Compiling: {}",
            bar_str,
            percent,
            label.bold()
        );
        std::io::stdout().flush().ok();
    }

    /// Clears the progress line.
    pub fn clear(&self) {
        print!("\r{}\r", " ".repeat(75));
        std::io::stdout().flush().ok();
    }
}
