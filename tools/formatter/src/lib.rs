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

    fn write_indent(&self, indent: usize, output: &mut String) {
        for _ in 0..(indent * self.indent_size) {
            output.push(' ');
        }
    }

    fn format_dsl_block_into(&self, block: &DSLBlock, indent: usize, output: &mut String) {
        self.write_indent(indent, output);
        output.push_str(&block.kind);
        for arg in &block.args {
            output.push(' ');
            self.format_expr_into(arg, output);
        }
        output.push('\n');

        for prop in &block.properties {
            self.write_indent(indent + 1, output);
            output.push_str(&prop.name);
            if let Some(ref val) = prop.value {
                output.push(' ');
                self.format_expr_into(val, output);
            }
            output.push('\n');
        }

        for child in &block.children {
            match child {
                DSLChild::Block(sub_block) => {
                    self.format_dsl_block_into(sub_block, indent + 1, output);
                }
                DSLChild::Code(code_block) => {
                    self.write_indent(indent + 1, output);
                    output.push_str("code\n");
                    for stmt in &code_block.statements {
                        self.format_stmt_into(stmt, indent + 2, output);
                    }
                }
                DSLChild::Property(prop) => {
                    self.write_indent(indent + 1, output);
                    output.push_str(&prop.name);
                    if let Some(ref val) = prop.value {
                        output.push(' ');
                        self.format_expr_into(val, output);
                    }
                    output.push('\n');
                }
            }
        }

        self.write_indent(indent, output);
        output.push_str("end\n");
    }

    fn format_expr_into(&self, expr: &techscript_ast::Expression, output: &mut String) {
        match expr {
            techscript_ast::Expression::Literal(lit) => self.format_lit_into(&lit.value, output),
            techscript_ast::Expression::Identifier(ident) => output.push_str(&ident.name),
            techscript_ast::Expression::FString(fs) => {
                output.push_str("f\"");
                for part in &fs.parts {
                    match part {
                        techscript_ast::FStringPart::Literal(l) => output.push_str(l),
                        techscript_ast::FStringPart::Expr(_) => output.push_str("{}"),
                    }
                }
                output.push('"');
            }
            _ => {
                let _ = write!(output, "{:?}", expr);
            }
        }
    }

    fn format_lit_into(&self, lit: &techscript_ast::LiteralVal, output: &mut String) {
        match lit {
            techscript_ast::LiteralVal::Str(s) => {
                let _ = write!(output, "\"{}\"", s);
            }
            techscript_ast::LiteralVal::Int(i) => {
                let _ = write!(output, "{}", i);
            }
            techscript_ast::LiteralVal::Float(f) => {
                let _ = write!(output, "{}", f);
            }
            techscript_ast::LiteralVal::Bool(b) => {
                let _ = write!(output, "{}", b);
            }
            techscript_ast::LiteralVal::None => output.push_str("none"),
        }
    }

    fn format_stmt_into(&self, stmt: &Statement, indent: usize, output: &mut String) {
        match stmt {
            Statement::DSL(block) => self.format_dsl_block_into(block, indent, output),
            Statement::VarDecl(decl) => {
                self.write_indent(indent, output);
                output.push_str("make ");
                let name = match &decl.pattern {
                    techscript_ast::Pattern::Single(ident) => ident.name.as_str(),
                    _ => "<pat>",
                };
                output.push_str(name);
                output.push_str(" = ");
                self.format_expr_into(&decl.initializer, output);
                output.push('\n');
            }
            Statement::ConstDecl(decl) => {
                self.write_indent(indent, output);
                output.push_str("const ");
                let name = match &decl.pattern {
                    techscript_ast::Pattern::Single(ident) => ident.name.as_str(),
                    _ => "<pat>",
                };
                output.push_str(name);
                output.push_str(" = ");
                self.format_expr_into(&decl.initializer, output);
                output.push('\n');
            }
            Statement::Say(s) => {
                self.write_indent(indent, output);
                output.push_str("say ");
                self.format_expr_into(&s.value, output);
                output.push('\n');
            }
            Statement::Return(ret) => {
                self.write_indent(indent, output);
                if let Some(ref val) = ret.value {
                    output.push_str("return ");
                    self.format_expr_into(val, output);
                    output.push('\n');
                } else {
                    output.push_str("return\n");
                }
            }
            _ => {
                self.write_indent(indent, output);
                output.push_str("<stmt>\n");
            }
        }
    }
}

impl Formatter for DocumentFormatter {
    fn format(&self, program: &Program) -> String {
        // Pre-allocate capacity, assuming roughly ~32 bytes per statement as a heuristic
        let mut output = String::with_capacity(program.statements.len() * 32);
        for stmt in &program.statements {
            self.format_stmt_into(stmt, 0, &mut output);
        }
        output
    }
}
