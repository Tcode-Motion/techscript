// ── TechScript Formatter ─────────────────────────────────────────────
// `tech fmt` — pretty-prints TechScript source with consistent style.

use crate::ast::*;

/// Configuration for the formatter.
pub struct FmtConfig {
    pub indent_size: usize,
    pub max_line_width: usize,
}

impl Default for FmtConfig {
    fn default() -> Self {
        FmtConfig {
            indent_size: 4,
            max_line_width: 100,
        }
    }
}

/// Format a parsed program back to source code.
pub fn format_program(program: &Program, config: &FmtConfig) -> String {
    let mut out = String::new();
    for (i, stmt) in program.body.iter().enumerate() {
        format_stmt(&mut out, stmt, 0, config);
        if i + 1 < program.body.len() {
            out.push('\n');
        }
    }
    // Ensure trailing newline
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn indent(out: &mut String, depth: usize, config: &FmtConfig) {
    for _ in 0..depth * config.indent_size {
        out.push(' ');
    }
}

fn format_stmt(out: &mut String, stmt: &Stmt, depth: usize, config: &FmtConfig) {
    indent(out, depth, config);
    match stmt {
        Stmt::Say { values } => {
            out.push_str("say ");
            for (i, v) in values.iter().enumerate() {
                format_expr(out, v);
                if i + 1 < values.len() {
                    out.push_str(", ");
                }
            }
            out.push('\n');
        }
        Stmt::Set { name, value, type_ann } => {
            out.push_str("make ");
            out.push_str(name);
            if let Some(t) = type_ann {
                out.push_str(": ");
                out.push_str(t);
            }
            out.push_str(" = ");
            format_expr(out, value);
            out.push('\n');
        }
        Stmt::Const { name, value, type_ann } => {
            out.push_str("keep ");
            out.push_str(name);
            if let Some(t) = type_ann {
                out.push_str(": ");
                out.push_str(t);
            }
            out.push_str(" = ");
            format_expr(out, value);
            out.push('\n');
        }
        Stmt::Assign { target, op, value } => {
            format_expr(out, target);
            out.push(' ');
            out.push_str(op);
            out.push(' ');
            format_expr(out, value);
            out.push('\n');
        }
        Stmt::Expression { expression } => {
            format_expr(out, expression);
            out.push('\n');
        }
        Stmt::If { condition, body, elif_clauses, else_body } => {
            out.push_str("when ");
            format_expr(out, condition);
            out.push_str(" {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push('}');
            for (cond, clause_body) in elif_clauses {
                out.push_str(" or when ");
                format_expr(out, cond);
                out.push_str(" {\n");
                for s in clause_body {
                    format_stmt(out, s, depth + 1, config);
                }
                indent(out, depth, config);
                out.push('}');
            }
            if let Some(else_stmts) = else_body {
                out.push_str(" else {\n");
                for s in else_stmts {
                    format_stmt(out, s, depth + 1, config);
                }
                indent(out, depth, config);
                out.push('}');
            }
            out.push('\n');
        }
        Stmt::For { var_name, iterable, body } => {
            out.push_str("each ");
            out.push_str(var_name);
            out.push_str(" in ");
            format_expr(out, iterable);
            out.push_str(" {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::While { condition, body } => {
            out.push_str("repeat ");
            format_expr(out, condition);
            out.push_str(" {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::Until { condition, body } => {
            out.push_str("until ");
            format_expr(out, condition);
            out.push_str(" {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::Unless { condition, body } => {
            out.push_str("unless ");
            format_expr(out, condition);
            out.push_str(" {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::Fn { name, params, body, is_async } => {
            if *is_async {
                out.push_str("async ");
            }
            out.push_str("build ");
            out.push_str(name);
            out.push('(');
            for (i, p) in params.iter().enumerate() {
                out.push_str(&p.name);
                if let Some(t) = &p.type_ann {
                    out.push_str(": ");
                    out.push_str(t);
                }
                if let Some(default) = &p.default {
                    out.push_str(" = ");
                    format_expr(out, default);
                }
                if i + 1 < params.len() {
                    out.push_str(", ");
                }
            }
            out.push_str(") {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::Class { name, parent, body } => {
            out.push_str("model ");
            out.push_str(name);
            if let Some(p) = parent {
                out.push('(');
                out.push_str(p);
                out.push(')');
            }
            out.push_str(" {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::Return { value } => {
            out.push_str("send");
            if let Some(v) = value {
                out.push(' ');
                format_expr(out, v);
            }
            out.push('\n');
        }
        Stmt::Break => { out.push_str("stop\n"); }
        Stmt::Skip => { out.push_str("skip\n"); }
        Stmt::Pass => { out.push_str("pass\n"); }
        Stmt::Try { body, catch_var, catch_body, finally_body } => {
            out.push_str("attempt {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("} rescue");
            if let Some(var) = catch_var {
                out.push(' ');
                out.push_str(var);
            }
            out.push_str(" {\n");
            for s in catch_body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push('}');
            if let Some(fin) = finally_body {
                out.push_str(" always {\n");
                for s in fin {
                    format_stmt(out, s, depth + 1, config);
                }
                indent(out, depth, config);
                out.push('}');
            }
            out.push('\n');
        }
        Stmt::Throw { value } => {
            out.push_str("fail ");
            format_expr(out, value);
            out.push('\n');
        }
        Stmt::Match { subject, cases } => {
            out.push_str("match ");
            format_expr(out, subject);
            out.push_str(" {\n");
            for (pattern, case_body) in cases {
                indent(out, depth + 1, config);
                out.push_str("case ");
                format_expr(out, pattern);
                out.push_str(" {\n");
                for s in case_body {
                    format_stmt(out, s, depth + 2, config);
                }
                indent(out, depth + 1, config);
                out.push_str("}\n");
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::Import { module, names: _, alias } => {
            out.push_str("use ");
            out.push_str(module);
            if let Some(a) = alias {
                out.push_str(" as ");
                out.push_str(a);
            }
            out.push('\n');
        }
        Stmt::FromImport { module, names } => {
            out.push_str("take ");
            out.push_str(&names.join(", "));
            out.push_str(" from ");
            out.push_str(module);
            out.push('\n');
        }
        Stmt::Del { name } => {
            out.push_str("drop ");
            out.push_str(name);
            out.push('\n');
        }
        Stmt::Defer { expression } => {
            out.push_str("defer ");
            format_expr(out, expression);
            out.push('\n');
        }
        Stmt::Guard { condition, else_body } => {
            out.push_str("guard ");
            format_expr(out, condition);
            out.push_str(" else {\n");
            for s in else_body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::With { expression, var_name, body } => {
            out.push_str("with ");
            format_expr(out, expression);
            out.push_str(" as ");
            out.push_str(var_name);
            out.push_str(" {\n");
            for s in body {
                format_stmt(out, s, depth + 1, config);
            }
            indent(out, depth, config);
            out.push_str("}\n");
        }
        Stmt::Export { declaration } => {
            out.push_str("share ");
            // Remove the leading indent since format_stmt adds it
            let mut inner = String::new();
            format_stmt(&mut inner, declaration, 0, config);
            out.push_str(inner.trim_start());
        }
    }
}

fn format_expr(out: &mut String, expr: &Expr) {
    match expr {
        Expr::NumberInt(n) => out.push_str(&n.to_string()),
        Expr::NumberFloat(n) => out.push_str(&format!("{}", n)),
        Expr::String(s) => {
            out.push('"');
            out.push_str(&s.replace('\\', "\\\\").replace('"', "\\\""));
            out.push('"');
        }
        Expr::FString(s) => {
            out.push_str("f\"");
            out.push_str(s);
            out.push('"');
        }
        Expr::Bool(true) => out.push_str("true"),
        Expr::Bool(false) => out.push_str("false"),
        Expr::None => out.push_str("none"),
        Expr::Identifier(name) => out.push_str(name),
        Expr::List(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                format_expr(out, item);
                if i + 1 < items.len() {
                    out.push_str(", ");
                }
            }
            out.push(']');
        }
        Expr::Map(pairs) => {
            out.push('{');
            for (i, (k, v)) in pairs.iter().enumerate() {
                format_expr(out, k);
                out.push_str(": ");
                format_expr(out, v);
                if i + 1 < pairs.len() {
                    out.push_str(", ");
                }
            }
            out.push('}');
        }
        Expr::BinaryOp { left, op, right } => {
            format_expr(out, left);
            out.push(' ');
            out.push_str(op);
            out.push(' ');
            format_expr(out, right);
        }
        Expr::UnaryOp { op, operand } => {
            out.push_str(op);
            if op == "not" {
                out.push(' ');
            }
            format_expr(out, operand);
        }
        Expr::Call { callee, args } => {
            format_expr(out, callee);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                format_expr(out, arg);
                if i + 1 < args.len() {
                    out.push_str(", ");
                }
            }
            out.push(')');
        }
        Expr::Index { obj, index } => {
            format_expr(out, obj);
            out.push('[');
            format_expr(out, index);
            out.push(']');
        }
        Expr::Member { obj, member } => {
            format_expr(out, obj);
            out.push('.');
            out.push_str(member);
        }
        Expr::Lambda { params, body } => {
            out.push('(');
            for (i, p) in params.iter().enumerate() {
                out.push_str(&p.name);
                if let Some(t) = &p.type_ann {
                    out.push_str(": ");
                    out.push_str(t);
                }
                if i + 1 < params.len() {
                    out.push_str(", ");
                }
            }
            out.push_str(") => ");
            format_expr(out, body);
        }
        Expr::Ask { prompt } => {
            out.push_str("ask ");
            format_expr(out, prompt);
        }
        Expr::Ternary { true_val, condition, false_val } => {
            format_expr(out, true_val);
            out.push_str(" when ");
            format_expr(out, condition);
            out.push_str(" else ");
            format_expr(out, false_val);
        }
        Expr::Range { start, end, inclusive } => {
            format_expr(out, start);
            if *inclusive {
                out.push_str("..=");
            } else {
                out.push_str("..");
            }
            format_expr(out, end);
        }
        Expr::Await { expression } => {
            out.push_str("await ");
            format_expr(out, expression);
        }
        Expr::Spawn { expression } => {
            out.push_str("spawn ");
            format_expr(out, expression);
        }
    }
}
