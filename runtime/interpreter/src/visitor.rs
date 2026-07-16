use crate::control_flow::{EvalResult, ExecResult};
use techscript_ast::{Expression, Statement};

/// Visitor interface traversing TechScript 2.0 AST nodes.
pub trait AstVisitor {
    /// Executes an AST statement node.
    fn visit_statement(&mut self, stmt: &Statement) -> ExecResult;

    /// Evaluates an AST expression node.
    fn visit_expression(&mut self, expr: &Expression) -> EvalResult;
}
