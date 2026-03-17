// ── TechScript Linter ────────────────────────────────────────────────
// `tech lint` — static analysis and code quality checks.

use crate::ast::*;
use std::collections::HashSet;

/// A single lint warning.
#[derive(Debug, Clone)]
pub struct LintWarning {
    pub code: &'static str,
    pub message: String,
    pub line: usize,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Warning,
    Info,
    Error,
}

impl std::fmt::Display for LintWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match self.severity {
            Severity::Error => "✗",
            Severity::Warning => "⚠",
            Severity::Info => "ℹ",
        };
        write!(f, "{} [{}] line {}: {}", icon, self.code, self.line, self.message)
    }
}

/// Lint a program and return a list of warnings.
pub fn lint_program(program: &Program) -> Vec<LintWarning> {
    let mut warnings = Vec::new();
    let mut defined_vars: HashSet<String> = HashSet::new();
    let mut used_vars: HashSet<String> = HashSet::new();

    // Collect all variable definitions and usages
    for stmt in &program.body {
        collect_definitions(stmt, &mut defined_vars);
        collect_usages(stmt, &mut used_vars);
        lint_stmt(stmt, &mut warnings, 0);
    }

    // Check for unused variables (skip builtins and _-prefixed)
    for var in &defined_vars {
        if !used_vars.contains(var) && !var.starts_with('_') {
            warnings.push(LintWarning {
                code: "W001",
                message: format!("Variable '{}' is defined but never used", var),
                line: 0,
                severity: Severity::Warning,
            });
        }
    }

    warnings
}

fn collect_definitions(stmt: &Stmt, defs: &mut HashSet<String>) {
    match stmt {
        Stmt::Set { name, .. } | Stmt::Const { name, .. } => {
            defs.insert(name.clone());
        }
        Stmt::Fn { name, params, body, is_async: _ } => {
            defs.insert(name.clone());
            for p in params {
                defs.insert(p.name.clone());
            }
            for s in body {
                collect_definitions(s, defs);
            }
        }
        Stmt::For { var_name, body, .. } => {
            defs.insert(var_name.clone());
            for s in body {
                collect_definitions(s, defs);
            }
        }
        Stmt::Class { name, body, .. } => {
            defs.insert(name.clone());
            for s in body {
                collect_definitions(s, defs);
            }
        }
        Stmt::If { body, elif_clauses, else_body, .. } => {
            for s in body { collect_definitions(s, defs); }
            for (_, clause_body) in elif_clauses {
                for s in clause_body { collect_definitions(s, defs); }
            }
            if let Some(eb) = else_body {
                for s in eb { collect_definitions(s, defs); }
            }
        }
        Stmt::While { body, .. } | Stmt::Until { body, .. } | Stmt::Unless { body, .. } => {
            for s in body { collect_definitions(s, defs); }
        }
        Stmt::Try { body, catch_var, catch_body, finally_body } => {
            for s in body { collect_definitions(s, defs); }
            if let Some(cv) = catch_var { defs.insert(cv.clone()); }
            for s in catch_body { collect_definitions(s, defs); }
            if let Some(fb) = finally_body {
                for s in fb { collect_definitions(s, defs); }
            }
        }
        _ => {}
    }
}

fn collect_usages(stmt: &Stmt, uses: &mut HashSet<String>) {
    match stmt {
        Stmt::Say { values } => {
            for v in values { collect_expr_usages(v, uses); }
        }
        Stmt::Set { value, .. } | Stmt::Const { value, .. } => {
            collect_expr_usages(value, uses);
        }
        Stmt::Assign { target, value, .. } => {
            collect_expr_usages(target, uses);
            collect_expr_usages(value, uses);
        }
        Stmt::Expression { expression } => {
            collect_expr_usages(expression, uses);
        }
        Stmt::If { condition, body, elif_clauses, else_body } => {
            collect_expr_usages(condition, uses);
            for s in body { collect_usages(s, uses); }
            for (c, b) in elif_clauses {
                collect_expr_usages(c, uses);
                for s in b { collect_usages(s, uses); }
            }
            if let Some(eb) = else_body {
                for s in eb { collect_usages(s, uses); }
            }
        }
        Stmt::For { iterable, body, .. } => {
            collect_expr_usages(iterable, uses);
            for s in body { collect_usages(s, uses); }
        }
        Stmt::While { condition, body } | Stmt::Until { condition, body } | Stmt::Unless { condition, body } => {
            collect_expr_usages(condition, uses);
            for s in body { collect_usages(s, uses); }
        }
        Stmt::Fn { body, .. } => {
            for s in body { collect_usages(s, uses); }
        }
        Stmt::Return { value } => {
            if let Some(v) = value { collect_expr_usages(v, uses); }
        }
        Stmt::Throw { value } => {
            collect_expr_usages(value, uses);
        }
        Stmt::Match { subject, cases } => {
            collect_expr_usages(subject, uses);
            for (p, b) in cases {
                collect_expr_usages(p, uses);
                for s in b { collect_usages(s, uses); }
            }
        }
        Stmt::Try { body, catch_body, finally_body, .. } => {
            for s in body { collect_usages(s, uses); }
            for s in catch_body { collect_usages(s, uses); }
            if let Some(fb) = finally_body {
                for s in fb { collect_usages(s, uses); }
            }
        }
        Stmt::Class { body, .. } => {
            for s in body { collect_usages(s, uses); }
        }
        Stmt::Guard { condition, else_body } => {
            collect_expr_usages(condition, uses);
            for s in else_body { collect_usages(s, uses); }
        }
        Stmt::With { expression, body, .. } => {
            collect_expr_usages(expression, uses);
            for s in body { collect_usages(s, uses); }
        }
        Stmt::Defer { expression } => {
            collect_expr_usages(expression, uses);
        }
        Stmt::Export { declaration } => {
            collect_usages(declaration, uses);
        }
        _ => {}
    }
}

fn collect_expr_usages(expr: &Expr, uses: &mut HashSet<String>) {
    match expr {
        Expr::Identifier(name) => { uses.insert(name.clone()); }
        Expr::BinaryOp { left, right, .. } => {
            collect_expr_usages(left, uses);
            collect_expr_usages(right, uses);
        }
        Expr::UnaryOp { operand, .. } => { collect_expr_usages(operand, uses); }
        Expr::Call { callee, args } => {
            collect_expr_usages(callee, uses);
            for a in args { collect_expr_usages(a, uses); }
        }
        Expr::Index { obj, index } => {
            collect_expr_usages(obj, uses);
            collect_expr_usages(index, uses);
        }
        Expr::Member { obj, .. } => {
            collect_expr_usages(obj, uses);
        }
        Expr::Lambda { body, .. } => {
            collect_expr_usages(body, uses);
        }
        Expr::List(items) => {
            for i in items { collect_expr_usages(i, uses); }
        }
        Expr::Map(pairs) => {
            for (k, v) in pairs {
                collect_expr_usages(k, uses);
                collect_expr_usages(v, uses);
            }
        }
        Expr::Ask { prompt } => { collect_expr_usages(prompt, uses); }
        Expr::Ternary { true_val, condition, false_val } => {
            collect_expr_usages(true_val, uses);
            collect_expr_usages(condition, uses);
            collect_expr_usages(false_val, uses);
        }
        Expr::Range { start, end, .. } => {
            collect_expr_usages(start, uses);
            collect_expr_usages(end, uses);
        }
        _ => {}
    }
}

fn lint_stmt(stmt: &Stmt, warnings: &mut Vec<LintWarning>, _depth: usize) {
    match stmt {
        // Check for empty say
        Stmt::Say { values } if values.is_empty() => {
            warnings.push(LintWarning {
                code: "W002",
                message: "'say' with no arguments does nothing".into(),
                line: 0,
                severity: Severity::Warning,
            });
        }
        // Check for dead code after return/break
        Stmt::Fn { body, name, .. } => {
            check_dead_code(body, warnings, name);
            for s in body { lint_stmt(s, warnings, _depth + 1); }
        }
        Stmt::If { body, elif_clauses, else_body, .. } => {
            for s in body { lint_stmt(s, warnings, _depth + 1); }
            for (_, b) in elif_clauses { for s in b { lint_stmt(s, warnings, _depth + 1); } }
            if let Some(eb) = else_body { for s in eb { lint_stmt(s, warnings, _depth + 1); } }
        }
        Stmt::For { body, .. } | Stmt::While { body, .. } | Stmt::Until { body, .. } | Stmt::Unless { body, .. } => {
            for s in body { lint_stmt(s, warnings, _depth + 1); }
        }
        Stmt::Try { body, catch_body, finally_body, .. } => {
            for s in body { lint_stmt(s, warnings, _depth + 1); }
            for s in catch_body { lint_stmt(s, warnings, _depth + 1); }
            if let Some(fb) = finally_body { for s in fb { lint_stmt(s, warnings, _depth + 1); } }
        }
        _ => {}
    }
}

fn check_dead_code(body: &[Stmt], warnings: &mut Vec<LintWarning>, fn_name: &str) {
    for (i, stmt) in body.iter().enumerate() {
        let is_terminator = matches!(stmt, Stmt::Return { .. } | Stmt::Break | Stmt::Throw { .. });
        if is_terminator && i + 1 < body.len() {
            warnings.push(LintWarning {
                code: "W003",
                message: format!("Unreachable code after return/stop/fail in '{}'", fn_name),
                line: 0,
                severity: Severity::Warning,
            });
            break;
        }
    }
}
