//! # tsc migrate Command
//!
//! Automatically migrates legacy TechScript 1.0.8 code to TechScript 2.0 canonical syntax.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::fmt::Write;
use std::path::{Path, PathBuf};

pub fn execute(path_str: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_path = path_str.map(PathBuf::from).unwrap_or(current_dir);

    if !target_path.exists() {
        eprintln!("Error: Path does not exist: {:?}", target_path);
        return ExitCode::IoError;
    }

    println!("Migrating TechScript files in: {:?}", target_path);

    let mut discovered_files = Vec::new();
    if target_path.is_dir() {
        let mut dirs = vec![target_path.clone()];
        while let Some(dir) = dirs.pop() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        if name != "build"
                            && name != ".git"
                            && name != "target"
                            && name != "node_modules"
                        {
                            dirs.push(path);
                        }
                    } else if techscript_common::is_supported_source_file(&path) {
                        discovered_files.push(path);
                    }
                }
            }
        }
    } else {
        discovered_files.push(target_path.clone());
    }

    let mut renamed_count = 0;
    let mut files_to_migrate = Vec::new();

    // 1. Rename legacy .tech files to canonical .txs
    for file in discovered_files {
        let is_legacy = file
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("tech"))
            .unwrap_or(false);

        if is_legacy {
            let target_txs = file.with_extension("txs");
            if target_txs.exists() {
                eprintln!(
                    "{}",
                    format!(
                        "Warning: Cannot rename '{}' to '{}' because destination already exists.",
                        file.display(),
                        target_txs.display()
                    )
                    .yellow()
                );
                files_to_migrate.push(file);
            } else {
                match std::fs::rename(&file, &target_txs) {
                    Ok(_) => {
                        println!(
                            "{} Renamed legacy '{}' -> '{}'",
                            "✓".green(),
                            file.display(),
                            target_txs.display()
                        );
                        renamed_count += 1;
                        files_to_migrate.push(target_txs);
                    }
                    Err(e) => {
                        eprintln!("Error renaming file {:?}: {}", file, e);
                        files_to_migrate.push(file);
                    }
                }
            }
        } else {
            files_to_migrate.push(file);
        }
    }

    // 2. Update tech.toml entry if it still points to .tech
    let toml_candidate = if target_path.is_dir() {
        target_path.join("tech.toml")
    } else {
        target_path
            .parent()
            .unwrap_or(&target_path)
            .join("tech.toml")
    };
    if toml_candidate.exists() {
        if let Ok(toml_content) = std::fs::read_to_string(&toml_candidate) {
            if toml_content.contains(".tech\"") || toml_content.contains(".tech'") {
                let updated_toml = toml_content
                    .replace(".tech\"", ".txs\"")
                    .replace(".tech'", ".txs'");
                if let Err(e) = std::fs::write(&toml_candidate, updated_toml) {
                    eprintln!("Error updating {}: {}", toml_candidate.display(), e);
                } else {
                    println!(
                        "{} Updated manifest '{}' entry to canonical .txs",
                        "✓".green(),
                        toml_candidate.display()
                    );
                }
            }
        }
    }

    // 3. Migrate source code syntax and explicit import extensions
    let mut syntax_migrated_count = 0;
    for file in files_to_migrate {
        match std::fs::read_to_string(&file) {
            Ok(content) => {
                let mut migrated = migrate_source(&content);

                // Modernize explicit .tech import extensions in code (e.g. use "./foo.tech" -> "./foo.txs")
                if migrated.contains(".tech\"") || migrated.contains(".tech'") {
                    migrated = migrated
                        .replace(".tech\"", ".txs\"")
                        .replace(".tech'", ".txs'");
                }

                if migrated != content {
                    if let Err(e) = std::fs::write(&file, migrated) {
                        eprintln!("Error writing migrated file {:?}: {}", file, e);
                    } else {
                        println!("{} Modernized syntax & imports: {:?}", "✓".green(), file);
                        syntax_migrated_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file {:?}: {}", file, e);
            }
        }
    }

    println!();
    println!(
        "{} Migration complete: {} files renamed to .txs, {} files modernized.",
        "✓".green().bold(),
        renamed_count,
        syntax_migrated_count
    );
    ExitCode::Success
}

pub fn migrate_source(source: &str) -> String {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = techscript_lexer::lex_recovered(source, &mut reporter);
    let _program = techscript_parser::parse_recovered(&tokens, &mut reporter);

    let mut deprecation_warnings: Vec<_> = reporter
        .get_diagnostics()
        .iter()
        .filter(|d| {
            matches!(
                d.code,
                techscript_errors::ErrorCode::TSW1001
                    | techscript_errors::ErrorCode::TSW1002
                    | techscript_errors::ErrorCode::TSW1003
                    | techscript_errors::ErrorCode::TSW1004
                    | techscript_errors::ErrorCode::TSW1005
                    | techscript_errors::ErrorCode::TSW1006
                    | techscript_errors::ErrorCode::TSW1007
                    | techscript_errors::ErrorCode::TSW1008
            )
        })
        .collect();

    if deprecation_warnings.is_empty() {
        return post_process(source);
    }

    // Sort by span start in reverse order
    deprecation_warnings.sort_by(|a, b| b.span.start.cmp(&a.span.start));

    let mut output = source.to_string();
    for diag in deprecation_warnings {
        let start = diag.span.start;
        let end = diag.span.end;
        if start > output.len() || end > output.len() || start > end {
            continue;
        }

        let replacement = match diag.code {
            techscript_errors::ErrorCode::TSW1001 => "".to_string(),
            techscript_errors::ErrorCode::TSW1002 => "to".to_string(),
            techscript_errors::ErrorCode::TSW1003 => "return".to_string(),
            techscript_errors::ErrorCode::TSW1004 => "try".to_string(),
            techscript_errors::ErrorCode::TSW1005 => "return".to_string(),
            techscript_errors::ErrorCode::TSW1007 => "if".to_string(),
            techscript_errors::ErrorCode::TSW1008 => "repeat".to_string(),
            techscript_errors::ErrorCode::TSW1006 => {
                let slice = &output[start..end];
                if slice == ";" {
                    "".to_string()
                } else {
                    slice.to_string()
                }
            }
            _ => continue,
        };

        let mut actual_end = end;
        if diag.code == techscript_errors::ErrorCode::TSW1001 {
            while actual_end < output.len() && output.as_bytes()[actual_end] == b' ' {
                actual_end += 1;
            }
        }

        output.replace_range(start..actual_end, &replacement);
    }

    // Apply post-processing text transforms (stdlib calls, import→use, etc.)
    post_process(&output)
}

/// Post-processing text transforms applied after span-based migration.
/// Handles patterns not tracked by the diagnostic reporter.
fn post_process(source: &str) -> String {
    let mut out = source.to_string();

    // ── let / var / make keyword declarations → direct assignment ────────
    out = line_transform(&out, |line| {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        if let Some(rest) = trimmed.strip_prefix("let ") {
            return format!("{indent}{}", rest.trim_start());
        }
        if let Some(rest) = trimmed.strip_prefix("var ") {
            return format!("{indent}{}", rest.trim_start());
        }
        if let Some(rest) = trimmed.strip_prefix("make ") {
            return format!("{indent}{}", rest.trim_start());
        }
        line.to_string()
    });

    // ── build → to (canonical function declaration) ──────────────────────
    out = line_transform(&out, |line| {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        if let Some(rest) = trimmed.strip_prefix("build ") {
            return format!("{indent}to {}", rest.trim_start());
        }
        line.to_string()
    });

    // ── give → return (canonical function return) ────────────────────────
    out = line_transform(&out, |line| {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        if let Some(rest) = trimmed.strip_prefix("give ") {
            return format!("{indent}return {}", rest.trim_start());
        }
        if trimmed == "give" {
            return format!("{indent}return");
        }
        line.to_string()
    });

    // ── std.io.println / std.io.print → say ──────────────────────────────
    out = replace_call(&out, "std.io.println", "say");
    out = replace_call(&out, "std.io.print", "say");

    // ── stdlib module prefix rewrites ─────────────────────────────────────
    let stdlib_rewrites: &[(&str, &str)] = &[
        ("std.math.", "math."),
        ("std.strings.", "string."),
        ("std.string.", "string."),
        ("std.fs.", "file."),
        ("std.file.", "file."),
        ("std.path.", "path."),
        ("std.env.", "env."),
        ("std.os.", "os."),
        ("std.time.", "time."),
        ("std.net.", "net."),
        ("std.http.", "http."),
        ("std.json.", "json."),
        ("std.csv.", "csv."),
        ("std.xml.", "xml."),
        ("std.yaml.", "yaml."),
        ("std.toml.", "toml."),
        ("std.regex.", "regex."),
        ("std.crypto.", "crypto."),
        ("std.uuid.", "uuid."),
        ("std.database.", "database."),
        ("std.sqlite.", "sqlite."),
        ("std.collections.", "collections."),
        ("std.datetime.", "datetime."),
        ("std.hash.", "hash."),
        ("std.random.", "random."),
        ("std.thread.", "thread."),
        ("std.sync.", "sync."),
        ("std.async.", "async."),
        ("std.future.", "future."),
        ("std.channel.", "channel."),
        ("std.testing.", "testing."),
        ("std.logging.", "logging."),
        ("std.compress.", "compress."),
        ("std.encoding.", "encoding."),
        ("std.base64.", "base64."),
        ("std.hex.", "hex."),
        ("std.url.", "url."),
        ("std.system.", "system."),
        ("std.sys.", "sys."),
        ("std.graphics.", "graphics."),
        ("std.ai.", "ai."),
        ("std.process.", "process."),
        ("std.terminal.", "terminal."),
        ("std.socket.", "socket."),
        ("std.websocket.", "websocket."),
        ("std.dns.", "dns."),
        ("std.email.", "email."),
        ("std.ftp.", "ftp."),
        ("std.mysql.", "mysql."),
        ("std.postgres.", "postgres."),
        ("std.mongodb.", "mongodb."),
        ("std.redis.", "redis."),
        ("std.jwt.", "jwt."),
        ("std.oauth.", "oauth."),
        ("std.image.", "image."),
        ("std.gui.", "gui."),
        ("std.web.", "web."),
        ("std.audio.", "audio."),
        ("std.video.", "video."),
        ("std.scheduler.", "scheduler."),
        ("std.cache.", "cache."),
        ("std.binary.", "binary."),
        ("std.ini.", "ini."),
        ("std.docs.", "docs."),
        ("std.task.", "task."),
        ("std.config.", "config."),
        ("std.parallel.", "parallel."),
        ("std.benchmark.", "benchmark."),
        ("std.profiler.", "profiler."),
        ("std.svg.", "svg."),
        ("std.pdf.", "pdf."),
        ("std.barcode.", "barcode."),
        ("std.qrcode.", "qrcode."),
        ("std.notification.", "notification."),
        ("std.security.", "security."),
        ("std.settings.", "settings."),
        ("std.localization.", "localization."),
        ("std.theme.", "theme."),
        ("std.charts.", "charts."),
        ("std.report.", "report."),
        ("std.excel.", "excel."),
        ("std.word.", "word."),
        ("std.powerpoint.", "powerpoint."),
        ("std.canvas.", "canvas."),
        ("std.debug.", "debug."),
        ("std.mock.", "mock."),
    ];
    for (old, new) in stdlib_rewrites {
        out = out.replace(old, new);
    }

    // ── import / from...import → use ─────────────────────────────────────
    out = line_transform(&out, |line| {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(mod_end) = rest.find(" import ") {
                let module = &rest[..mod_end];
                return format!("{indent}use {module}");
            }
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            return format!("{indent}use {}", rest.trim());
        }
        line.to_string()
    });

    // ── model Name / model Name { → class Name ───────────────────────────
    out = line_transform(&out, |line| {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        if let Some(rest) = trimmed.strip_prefix("model ") {
            let name = rest.trim_end_matches('{').trim();
            return format!("{indent}class {name}");
        }
        line.to_string()
    });

    // ── each x in y → for x in y ─────────────────────────────────────────
    out = line_transform(&out, |line| {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        if let Some(rest) = trimmed.strip_prefix("each ") {
            let rest = rest.trim_end_matches('{').trim();
            return format!("{indent}for {rest}");
        }
        line.to_string()
    });

    // ── f"..." → $"..." ──────────────────────────────────────────────────
    for prefix in [" f\"", "\tf\"", "(f\"", "=f\"", "= f\""] {
        let replacement = prefix.replace("f\"", "$\"");
        out = out.replace(prefix, &replacement);
    }

    out
}

/// Replace `prefix(args)` calls with `keyword args` (implicit call style).
fn replace_call(source: &str, prefix: &str, keyword: &str) -> String {
    let full = format!("{prefix}(");
    let mut result = String::with_capacity(source.len());
    let mut remaining = source;
    loop {
        match remaining.find(full.as_str()) {
            None => {
                result.push_str(remaining);
                break;
            }
            Some(pos) => {
                result.push_str(&remaining[..pos]);
                remaining = &remaining[pos + full.len()..];
                if let Some(close) = remaining.find(')') {
                    let args = &remaining[..close];
                    let _ = write!(result, "{keyword} {args}");
                    remaining = &remaining[close + 1..];
                } else {
                    result.push_str(remaining);
                    break;
                }
            }
        }
    }
    result
}

/// Apply a per-line transformation.
fn line_transform<F: Fn(&str) -> String>(source: &str, f: F) -> String {
    let ends_newline = source.ends_with('\n');
    let transformed: Vec<String> = source.lines().map(|l| f(l)).collect();
    let mut out = transformed.join("\n");
    if ends_newline {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_variable_declarations() {
        let input = "make x = 5;\nlet y = 10;\nvar z = 15;";
        let expected = "x = 5\ny = 10\nz = 15";
        assert_eq!(migrate_source(input), expected);
    }

    #[test]
    fn test_migrate_function_and_return() {
        let input = "build main() {\n    give 42;\n}";
        let result = migrate_source(input);
        assert!(
            result.contains("to main()"),
            "Expected to main(), got:\n{result}"
        );
        assert!(
            result.contains("return 42"),
            "Expected return 42, got:\n{result}"
        );
    }

    #[test]
    fn test_migrate_try_catch() {
        let input = "attempt {\n    throw \"error\";\n} catch e {\n    say e;\n}";
        let result = migrate_source(input);
        assert!(result.contains("try"), "Expected 'try' in:\n{result}");
        assert!(
            !result.contains("attempt"),
            "Did not expect 'attempt' in:\n{result}"
        );
        assert!(
            result.contains("catch e"),
            "Expected 'catch e' in:\n{result}"
        );
        assert!(
            !result.contains(";"),
            "Expected semicolons stripped in:\n{result}"
        );
    }

    #[test]
    fn test_migrate_conditionals() {
        let input = "if x > 5 {\n    say \"yes\";\n} else {\n    say \"no\";\n}";
        let result = migrate_source(input);
        assert!(
            result.contains("if x > 5"),
            "Expected 'if x > 5' in:\n{result}"
        );
        assert!(result.contains("else"), "Expected 'else' in:\n{result}");
        assert!(
            !result.contains(";"),
            "Expected semicolons stripped in:\n{result}"
        );
    }

    // ── post_process tests ──────────────────────────────────────────────────

    #[test]
    fn test_post_process_println_to_say() {
        let input = "std.io.println(\"Hello, World!\")";
        let result = post_process(input);
        assert_eq!(result, "say \"Hello, World!\"");
    }

    #[test]
    fn test_post_process_stdlib_prefix() {
        let input = "x = std.math.abs(-42)";
        let result = post_process(input);
        assert_eq!(result, "x = math.abs(-42)");
    }

    #[test]
    fn test_post_process_std_module_calls() {
        let input = "x = std.collections.new()\nt = std.datetime.now()\nparsed = std.json.parse(s)\nlen = std.string.length(str)\nf = std.file.open(path)\nh = std.hash.sha256(data)";
        let result = post_process(input);
        assert!(result.contains("x = collections.new()"));
        assert!(result.contains("t = datetime.now()"));
        assert!(result.contains("parsed = json.parse(s)"));
        assert!(result.contains("len = string.length(str)"));
        assert!(result.contains("f = file.open(path)"));
        assert!(result.contains("h = hash.sha256(data)"));
    }

    #[test]
    fn test_migrate_source_std_calls() {
        let input = "data = std.json.parse(s)";
        let result = migrate_source(input);
        assert_eq!(result, "data = json.parse(s)");
    }

    #[test]
    fn test_post_process_import_to_use() {
        let input = "import math\nfrom json import parse";
        let result = post_process(input);
        assert!(
            result.contains("use math"),
            "Expected 'use math', got: {result}"
        );
        assert!(
            result.contains("use json"),
            "Expected 'use json', got: {result}"
        );
    }

    #[test]
    fn test_post_process_model_to_class() {
        let input = "model User {\n    name = \"\"\n}";
        let result = post_process(input);
        assert!(
            result.contains("class User"),
            "Expected 'class User', got: {result}"
        );
    }

    #[test]
    fn test_post_process_fstring_prefix() {
        let input = "say f\"Hello {name}\"";
        let result = post_process(input);
        assert!(
            result.contains("$\"Hello {name}\""),
            "Expected $-string, got: {result}"
        );
    }

    #[test]
    fn test_post_process_each_to_for() {
        let input = "each i in items {\n    say i\n}";
        let result = post_process(input);
        assert!(
            result.contains("for i in items"),
            "Expected 'for i in items', got: {result}"
        );
    }
}
