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

    fn format_indent(&self, indent: usize) -> String {
        " ".repeat(indent * self.indent_size)
    }

    fn format_dsl_block(&self, block: &DSLBlock, indent: usize) -> String {
        let mut output = String::new();
        let base = self.format_indent(indent);

        output.push_str(&base);
        output.push_str(&block.kind);
        for arg in &block.args {
            output.push(' ');
            output.push_str(&self.format_expr(arg));
        }
        output.push('\n');

        for prop in &block.properties {
            output.push_str(&self.format_indent(indent + 1));
            output.push_str(&prop.name);
            if let Some(ref val) = prop.value {
                output.push(' ');
                output.push_str(&self.format_expr(val));
            }
            output.push('\n');
        }

        for child in &block.children {
            match child {
                DSLChild::Block(sub_block) => {
                    output.push_str(&self.format_dsl_block(sub_block, indent + 1));
                }
                DSLChild::Code(code_block) => {
                    output.push_str(&self.format_indent(indent + 1));
                    output.push_str("code\n");
                    for stmt in &code_block.statements {
                        output.push_str(&self.format_stmt(stmt, indent + 2));
                    }
                }
                DSLChild::Property(prop) => {
                    output.push_str(&self.format_indent(indent + 1));
                    output.push_str(&prop.name);
                    if let Some(ref val) = prop.value {
                        output.push(' ');
                        output.push_str(&self.format_expr(val));
                    }
                    output.push('\n');
                }
            }
        }

        output.push_str(&base);
        output.push_str("end\n");
        output
    }

    fn format_expr(&self, expr: &techscript_ast::Expression) -> String {
        match expr {
            techscript_ast::Expression::Literal(lit) => self.format_lit(&lit.value),
            techscript_ast::Expression::Identifier(ident) => ident.name.clone(),
            techscript_ast::Expression::FString(fs) => {
                let mut s = "f\"".to_string();
                for part in &fs.parts {
                    match part {
                        techscript_ast::FStringPart::Literal(l) => s.push_str(l),
                        techscript_ast::FStringPart::Expr(_) => s.push_str("{}"),
                    }
                }
                s.push('"');
                s
            }
            _ => format!("{:?}", expr),
        }
    }

    fn format_lit(&self, lit: &techscript_ast::LiteralVal) -> String {
        match lit {
            techscript_ast::LiteralVal::Str(s) => format!("\"{}\"", s),
            techscript_ast::LiteralVal::Int(i) => i.to_string(),
            techscript_ast::LiteralVal::Float(f) => f.to_string(),
            techscript_ast::LiteralVal::Bool(b) => b.to_string(),
            techscript_ast::LiteralVal::None => "none".to_string(),
        }
    }

    fn format_stmt(&self, stmt: &Statement, indent: usize) -> String {
        let base = self.format_indent(indent);
        match stmt {
            Statement::DSL(block) => self.format_dsl_block(block, indent),
            Statement::VarDecl(decl) => {
                let name = match &decl.pattern {
                    techscript_ast::Pattern::Single(ident) => ident.name.clone(),
                    _ => "<pat>".to_string(),
                };
                format!("{}make {} = {}\n", base, name, self.format_expr(&decl.initializer))
            }
            Statement::ConstDecl(decl) => {
                let name = match &decl.pattern {
                    techscript_ast::Pattern::Single(ident) => ident.name.clone(),
                    _ => "<pat>".to_string(),
                };
                format!("{}const {} = {}\n", base, name, self.format_expr(&decl.initializer))
            }
            Statement::Say(s) => format!("{}say {}\n", base, self.format_expr(&s.value)),
            Statement::Return(ret) => {
                if let Some(ref val) = ret.value {
                    format!("{}return {}\n", base, self.format_expr(val))
                } else {
                    format!("{}return\n", base)
                }
            }
            _ => format!("{}<stmt>\n", base),
        }
    }
}

impl Formatter for DocumentFormatter {
    fn format(&self, program: &Program) -> String {
        let mut output = String::new();
        for stmt in &program.statements {
            output.push_str(&self.format_stmt(stmt, 0));
        }
        output
    }
}
