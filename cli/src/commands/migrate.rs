//! # tsc migrate Command
//!
//! Automatically migrates legacy TechScript 1.0.8 code to TechScript 2.0 canonical syntax.

use crate::exit_code::ExitCode;
use std::path::PathBuf;

pub fn execute(path_str: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_path = path_str.map(PathBuf::from).unwrap_or(current_dir);

    if !target_path.exists() {
        eprintln!("Error: Path does not exist: {:?}", target_path);
        return ExitCode::IoError;
    }

    println!("Migrating TechScript files in: {:?}", target_path);

    let mut files_to_migrate = Vec::new();
    if target_path.is_dir() {
        let mut dirs = vec![target_path];
        while let Some(dir) = dirs.pop() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        if name != "build" && name != ".git" && name != "target" {
                            dirs.push(path);
                        }
                    } else {
                        let ext = path.extension().unwrap_or_default().to_string_lossy();
                        if ext == "txs" || ext == "ts" {
                            files_to_migrate.push(path);
                        }
                    }
                }
            }
        }
    } else {
        files_to_migrate.push(target_path);
    }

    let mut migrated_count = 0;

    for file in files_to_migrate {
        match std::fs::read_to_string(&file) {
            Ok(content) => {
                let migrated = migrate_source(&content);
                if migrated != content {
                    if let Err(e) = std::fs::write(&file, migrated) {
                        eprintln!("Error writing migrated file {:?}: {}", file, e);
                    } else {
                        println!("Migrated: {:?}", file);
                        migrated_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file {:?}: {}", file, e);
            }
        }
    }

    println!("Migrated {} files.", migrated_count);
    ExitCode::Success
}

fn migrate_source(source: &str) -> String {
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = techscript_lexer::lex_recovered(source, &mut reporter);
    let _program = techscript_parser::parse_recovered(&tokens, &mut reporter);

    let mut deprecation_warnings: Vec<_> = reporter.get_diagnostics()
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
        return source.to_string();
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
            techscript_errors::ErrorCode::TSW1002 => "do".to_string(),
            techscript_errors::ErrorCode::TSW1003 => "send".to_string(),
            techscript_errors::ErrorCode::TSW1004 => "try".to_string(),
            techscript_errors::ErrorCode::TSW1005 => "send".to_string(),
            techscript_errors::ErrorCode::TSW1007 => "when".to_string(),
            techscript_errors::ErrorCode::TSW1008 => "repeat".to_string(),
            techscript_errors::ErrorCode::TSW1006 => {
                let slice = &output[start..end];
                if slice == ";" {
                    "".to_string()
                } else if slice == "{" {
                    "".to_string()
                } else if slice == "}" {
                    let mut follow_idx = end;
                    while follow_idx < output.len() {
                        let b = output.as_bytes()[follow_idx];
                        if b == b' ' || b == b'\t' || b == b'\r' || b == b'\n' {
                            follow_idx += 1;
                        } else {
                            break;
                        }
                    }
                    let mut is_followed_by_block_continuation = false;
                    let continuations = ["catch", "else", "elif", "or"];
                    for cont in &continuations {
                        let len = cont.len();
                        if follow_idx + len <= output.len() && &output[follow_idx..follow_idx + len] == *cont {
                            is_followed_by_block_continuation = true;
                            break;
                        }
                    }
                    if is_followed_by_block_continuation {
                        "".to_string()
                    } else {
                        "end".to_string()
                    }
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

    // ── std.io.println / std.io.print → say ──────────────────────────────
    out = replace_call(&out, "std.io.println", "say");
    out = replace_call(&out, "std.io.print", "say");

    // ── stdlib module prefix rewrites ─────────────────────────────────────
    let stdlib_rewrites: &[(&str, &str)] = &[
        ("std.math.",     "math."),
        ("std.strings.",  "string."),
        ("std.fs.",       "file."),
        ("std.path.",     "path."),
        ("std.env.",      "env."),
        ("std.os.",       "os."),
        ("std.time.",     "time."),
        ("std.net.",      "net."),
        ("std.http.",     "http."),
        ("std.json.",     "json."),
        ("std.csv.",      "csv."),
        ("std.xml.",      "xml."),
        ("std.yaml.",     "yaml."),
        ("std.toml.",     "toml."),
        ("std.regex.",    "regex."),
        ("std.crypto.",   "crypto."),
        ("std.uuid.",     "uuid."),
        ("std.database.", "database."),
        ("std.sqlite.",   "sqlite."),
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
            None => { result.push_str(remaining); break; }
            Some(pos) => {
                result.push_str(&remaining[..pos]);
                remaining = &remaining[pos + full.len()..];
                if let Some(close) = remaining.find(')') {
                    let args = &remaining[..close];
                    result.push_str(&format!("{keyword} {args}"));
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
    if ends_newline { out.push('\n'); }
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
        let expected = "do main() \n    send 42\nend";
        assert_eq!(migrate_source(input), expected);
    }

    #[test]
    fn test_migrate_try_catch() {
        let input = "attempt {\n    throw \"error\";\n} catch e {\n    say e;\n}";
        let expected = "try \n    throw \"error\"\n catch e \n    say e\nend";
        assert_eq!(migrate_source(input), expected);
    }

    #[test]
    fn test_migrate_conditionals() {
        let input = "if x > 5 {\n    say \"yes\";\n} else {\n    say \"no\";\n}";
        let result = migrate_source(input);
        assert!(result.contains("when x > 5"));
        assert!(result.contains("else"));
        assert!(result.contains("end"));
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
    fn test_post_process_import_to_use() {
        let input = "import math\nfrom json import parse";
        let result = post_process(input);
        assert!(result.contains("use math"), "Expected 'use math', got: {result}");
        assert!(result.contains("use json"), "Expected 'use json', got: {result}");
    }

    #[test]
    fn test_post_process_model_to_class() {
        let input = "model User {\n    name = \"\"\n}";
        let result = post_process(input);
        assert!(result.contains("class User"), "Expected 'class User', got: {result}");
    }

    #[test]
    fn test_post_process_fstring_prefix() {
        let input = "say f\"Hello {name}\"";
        let result = post_process(input);
        assert!(result.contains("$\"Hello {name}\""), "Expected $-string, got: {result}");
    }

    #[test]
    fn test_post_process_each_to_for() {
        let input = "each i in items {\n    say i\n}";
        let result = post_process(input);
        assert!(result.contains("for i in items"), "Expected 'for i in items', got: {result}");
    }
}
