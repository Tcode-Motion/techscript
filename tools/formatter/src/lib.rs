use std::fmt::Write;
use techscript_ast::{DSLBlock, DSLChild, Program, Statement};

pub trait Formatter {
    fn format(&self, program: &Program) -> String;
}

pub struct DocumentFormatter {
    indent_size: usize,
}

impl DocumentFormatter {
    pub fn new(indent_size: usize) -> Self {
        Self { indent_size }
    }

    pub fn format_source(&self, source: &str) -> String {
        let mut reporter = techscript_errors::DiagnosticReporter::new();
        let tokens = techscript_lexer::lex_recovered(source, &mut reporter);
        let program = techscript_parser::parse_recovered(&tokens, &mut reporter);
        self.format(&program)
    }

    fn format_indent(&self, indent: usize, buf: &mut String) {
        for _ in 0..(indent * self.indent_size) {
            buf.push(' ');
        }
    }

    fn format_dsl_block(&self, block: &DSLBlock, indent: usize, buf: &mut String) {
        self.format_indent(indent, buf);
        buf.push_str(&block.kind);
        for arg in &block.args {
            buf.push(' ');
            self.format_expr(arg, buf);
        }
        buf.push('\n');

        for prop in &block.properties {
            self.format_indent(indent + 1, buf);
            buf.push_str(&prop.name);
            if let Some(ref val) = prop.value {
                buf.push(' ');
                self.format_expr(val, buf);
            }
            buf.push('\n');
        }

        for child in &block.children {
            match child {
                DSLChild::Block(sub_block) => {
                    self.format_dsl_block(sub_block, indent + 1, buf);
                }
                DSLChild::Code(code_block) => {
                    self.format_indent(indent + 1, buf);
                    buf.push_str("code\n");
                    for stmt in &code_block.statements {
                        self.format_stmt(stmt, indent + 2, buf);
                    }
                }
                DSLChild::Property(prop) => {
                    self.format_indent(indent + 1, buf);
                    buf.push_str(&prop.name);
                    if let Some(ref val) = prop.value {
                        buf.push(' ');
                        self.format_expr(val, buf);
                    }
                    buf.push('\n');
                }
            }
        }

        self.format_indent(indent, buf);
        buf.push_str("end\n");
    }

    fn format_expr(&self, expr: &techscript_ast::Expression, buf: &mut String) {
        match expr {
            techscript_ast::Expression::Literal(lit) => self.format_lit(&lit.value, buf),
            techscript_ast::Expression::Identifier(ident) => buf.push_str(&ident.name),
            techscript_ast::Expression::FString(fs) => {
                buf.push_str("f\"");
                for part in &fs.parts {
                    match part {
                        techscript_ast::FStringPart::Literal(l) => buf.push_str(l),
                        techscript_ast::FStringPart::Expr(_) => buf.push_str("{}"),
                    }
                }
                buf.push('"');
            }
            _ => {
                let _ = write!(buf, "{:?}", expr);
            }
        }
    }

    fn format_lit(&self, lit: &techscript_ast::LiteralVal, buf: &mut String) {
        match lit {
            techscript_ast::LiteralVal::Str(s) => {
                buf.push('"');
                buf.push_str(s);
                buf.push('"');
            }
            techscript_ast::LiteralVal::Int(i) => {
                let _ = write!(buf, "{}", i);
            }
            techscript_ast::LiteralVal::Float(f) => {
                let _ = write!(buf, "{}", f);
            }
            techscript_ast::LiteralVal::Bool(b) => {
                if *b {
                    buf.push_str("true");
                } else {
                    buf.push_str("false");
                }
            }
            techscript_ast::LiteralVal::None => buf.push_str("none"),
        }
    }

    fn format_stmt(&self, stmt: &Statement, indent: usize, buf: &mut String) {
        match stmt {
            Statement::DSL(block) => self.format_dsl_block(block, indent, buf),
            Statement::VarDecl(decl) => {
                self.format_indent(indent, buf);
                buf.push_str("make ");
                let name = match &decl.pattern {
                    techscript_ast::Pattern::Single(ident) => ident.name.clone(),
                    _ => "<pat>".to_string(),
                };
                buf.push_str(&name);
                buf.push_str(" = ");
                self.format_expr(&decl.initializer, buf);
                buf.push('\n');
            }
            Statement::ConstDecl(decl) => {
                self.format_indent(indent, buf);
                buf.push_str("const ");
                let name = match &decl.pattern {
                    techscript_ast::Pattern::Single(ident) => ident.name.clone(),
                    _ => "<pat>".to_string(),
                };
                buf.push_str(&name);
                buf.push_str(" = ");
                self.format_expr(&decl.initializer, buf);
                buf.push('\n');
            }
            Statement::Say(s) => {
                self.format_indent(indent, buf);
                buf.push_str("say ");
                self.format_expr(&s.value, buf);
                buf.push('\n');
            }
            Statement::Return(ret) => {
                self.format_indent(indent, buf);
                if let Some(ref val) = ret.value {
                    buf.push_str("return ");
                    self.format_expr(val, buf);
                    buf.push('\n');
                } else {
                    buf.push_str("return\n");
                }
            }
            _ => {
                self.format_indent(indent, buf);
                buf.push_str("<stmt>\n");
            }
        }
    }
}

impl Formatter for DocumentFormatter {
    fn format(&self, program: &Program) -> String {
        // Estimate 64 bytes per statement as a reasonable baseline capacity
        let mut output = String::with_capacity(program.statements.len() * 64);
        for stmt in &program.statements {
            self.format_stmt(stmt, 0, &mut output);
        }
        output
    }
}
