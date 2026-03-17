// ── TechScript ANSI Colors ──────────────────────────────────────────
// Zero-dependency ANSI terminal color helpers.

/// ANSI color codes for terminal output.
pub struct Color;

impl Color {
    pub fn red(s: &str) -> String { format!("\x1b[31m{}\x1b[0m", s) }
    pub fn green(s: &str) -> String { format!("\x1b[32m{}\x1b[0m", s) }
    pub fn yellow(s: &str) -> String { format!("\x1b[33m{}\x1b[0m", s) }
    pub fn blue(s: &str) -> String { format!("\x1b[34m{}\x1b[0m", s) }
    pub fn cyan(s: &str) -> String { format!("\x1b[36m{}\x1b[0m", s) }
    pub fn white(s: &str) -> String { format!("\x1b[97m{}\x1b[0m", s) }
    pub fn dim(s: &str) -> String { format!("\x1b[2m{}\x1b[0m", s) }
    pub fn bold(s: &str) -> String { format!("\x1b[1m{}\x1b[0m", s) }
    pub fn bold_red(s: &str) -> String { format!("\x1b[1;31m{}\x1b[0m", s) }
    pub fn bold_green(s: &str) -> String { format!("\x1b[1;32m{}\x1b[0m", s) }
    pub fn bold_yellow(s: &str) -> String { format!("\x1b[1;33m{}\x1b[0m", s) }
    pub fn bold_cyan(s: &str) -> String { format!("\x1b[1;36m{}\x1b[0m", s) }
    pub fn bold_white(s: &str) -> String { format!("\x1b[1;97m{}\x1b[0m", s) }
}
