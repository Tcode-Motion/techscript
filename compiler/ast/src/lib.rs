//! # TechScript AST Crate
//!
//! Node declarations and visitor pattern traits for the TechScript Abstract Syntax Tree.
//! Fully specifies node schemas for parsing, semantic analysis, and interpretation passes.

use serde::{Serialize, Deserialize};
pub use techscript_common::{Span, NodeId, Ident};

/// Root node representing a parsed TechScript program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub id: NodeId,
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Statements that form the structures of block execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    FuncDecl(FuncDecl),
    ModelDecl(ModelDecl),
    ExportDecl(ExportDecl),
    When(WhenStmt),
    Each(EachStmt),
    Repeat(RepeatStmt),
    While(WhileStmt),
    Attempt(AttemptStmt),
    Say(SayStmt),
    Return(ReturnStmt),
    Throw(ThrowStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Assignment(AssignmentStmt),
    Import(ImportStmt),
    Expression(ExpressionStmt),
    Block(Block),
}

/// make name = expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarDecl {
    pub id: NodeId,
    pub name: Ident,
    pub initializer: Expression,
    pub span: Span,
}

/// const NAME = expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstDecl {
    pub id: NodeId,
    pub name: Ident,
    pub initializer: Expression,
    pub span: Span,
}

/// build name(params) { body }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuncDecl {
    pub id: NodeId,
    pub name: Ident,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub span: Span,
}

/// Parameter details with optional default values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Ident,
    pub default: Option<Expression>,
    pub span: Span,
}

/// model Name { fields and methods }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDecl {
    pub id: NodeId,
    pub name: Ident,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

/// make field_name = default_value (inside a model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDecl {
    pub id: NodeId,
    pub name: Ident,
    pub default: Expression,
    pub span: Span,
}

/// Keyword categories for class methods compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodKeyword {
    Build,
    Fun, // Deprecated v1 alias
}

/// build/fun method_name(params) { body } (inside a model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDecl {
    pub id: NodeId,
    pub keyword: MethodKeyword,
    pub name: Ident,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub span: Span,
}

/// export declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDecl {
    pub id: NodeId,
    pub declaration: Box<Statement>,
    pub span: Span,
}

/// when expression { block } else { block }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhenStmt {
    pub id: NodeId,
    pub condition: Expression,
    pub body: Block,
    pub else_ifs: Vec<(Expression, Block)>,
    pub else_body: Option<Block>,
    pub span: Span,
}

/// each identifier in expression { block }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EachStmt {
    pub id: NodeId,
    pub item: Ident,
    pub iterable: Expression,
    pub body: Block,
    pub span: Span,
}

/// repeat expression { block }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatStmt {
    pub id: NodeId,
    pub count: Expression,
    pub body: Block,
    pub span: Span,
}

/// while expression { block }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileStmt {
    pub id: NodeId,
    pub condition: Expression,
    pub body: Block,
    pub span: Span,
}

/// attempt { block } catch identifier { block }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStmt {
    pub id: NodeId,
    pub body: Block,
    pub catch_var: Ident,
    pub catch_body: Block,
    pub span: Span,
}

/// say expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SayStmt {
    pub id: NodeId,
    pub value: Expression,
    pub span: Span,
}

/// return expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnStmt {
    pub id: NodeId,
    pub value: Option<Expression>,
    pub span: Span,
}

/// throw expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowStmt {
    pub id: NodeId,
    pub value: Expression,
    pub span: Span,
}

/// break
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakStmt {
    pub id: NodeId,
    pub span: Span,
}

/// continue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueStmt {
    pub id: NodeId,
    pub span: Span,
}

/// assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentStmt {
    pub id: NodeId,
    pub target: Expression, // Ident, Member, or IndexExpr
    pub op: String,         // "=", "+=", etc.
    pub value: Expression,
    pub span: Span,
}

/// import module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStmt {
    pub id: NodeId,
    pub path: Vec<Ident>,
    pub symbols: Option<Vec<Ident>>,
    pub span: Span,
}

/// expression statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionStmt {
    pub id: NodeId,
    pub expression: Expression,
    pub span: Span,
}

/// block statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: NodeId,
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Expression node variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    Identifier(Ident),
    Range(RangeExpr),
    Ask(AskExpr),
    New(NewExpr),
    Lambda(LambdaExpr),
    List(ListExpr),
    Map(MapExpr),
    FString(FStringExpr),
    Group(Box<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiteralVal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteralExpr {
    pub id: NodeId,
    pub value: LiteralVal,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub id: NodeId,
    pub left: Box<Expression>,
    pub op: String,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub id: NodeId,
    pub op: String,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpr {
    pub id: NodeId,
    pub callee: Box<Expression>,
    pub args: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberExpr {
    pub id: NodeId,
    pub object: Box<Expression>,
    pub member: Ident,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexExpr {
    pub id: NodeId,
    pub object: Box<Expression>,
    pub index: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeExpr {
    pub id: NodeId,
    pub start: Box<Expression>,
    pub inclusive: bool,
    pub end: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskExpr {
    pub id: NodeId,
    pub prompt: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExpr {
    pub id: NodeId,
    pub class_name: Ident,
    pub args: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaExpr {
    pub id: NodeId,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExpr {
    pub id: NodeId,
    pub items: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapExpr {
    pub id: NodeId,
    pub entries: Vec<(Expression, Expression)>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FStringExpr {
    pub id: NodeId,
    pub parts: Vec<FStringPart>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FStringPart {
    Literal(String),
    Expr(Expression),
}

/// Visitor trait for AST traversal.
pub trait Visitor<T> {
    fn visit_program(&mut self, node: &Program) -> T;
    fn visit_statement(&mut self, node: &Statement) -> T;
    fn visit_expression(&mut self, node: &Expression) -> T;
}
