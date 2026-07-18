//! # TechScript AST Crate
//!
//! Node declarations and structures for the TechScript Abstract Syntax Tree.
//! Fully specifies node schemas for parsing, semantic analysis, and interpretation passes.

use serde::{Deserialize, Serialize};
pub use techscript_common::{Ident, NodeId, Span};

/// Root node representing a parsed TechScript program.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Program {
    pub id: NodeId,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Program {
    pub fn new(id: NodeId, statements: Vec<Statement>, span: Span) -> Self {
        Self {
            id,
            statements,
            span,
        }
    }
}

/// Statements that form the structures of block execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    // Declarations
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    FuncDecl(FuncDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    ModelDecl(ModelDecl),
    ExportDecl(ExportDecl),

    // Control Flow
    If(IfStmt),
    For(ForStmt),
    Repeat(RepeatStmt),
    While(WhileStmt),
    Try(TryStmt),

    // Simple Statements
    Say(SayStmt),
    Return(ReturnStmt),
    Throw(ThrowStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Import(ImportStmt),

    // Expression as statement
    Expression(ExpressionStmt),

    // Block
    Block(Block),
}


/// make/let/var pattern[: type] = expression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VarDecl {
    pub id: NodeId,
    pub pattern: Pattern,
    pub type_ann: Option<TypeSpec>,
    pub initializer: Expression,
    pub span: Span,
}

impl VarDecl {
    pub fn new(
        id: NodeId,
        pattern: Pattern,
        type_ann: Option<TypeSpec>,
        initializer: Expression,
        span: Span,
    ) -> Self {
        Self {
            id,
            pattern,
            type_ann,
            initializer,
            span,
        }
    }
}

/// const NAME[: type] = expression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstDecl {
    pub id: NodeId,
    pub pattern: Pattern,
    pub type_ann: Option<TypeSpec>,
    pub initializer: Expression,
    pub span: Span,
}

impl ConstDecl {
    pub fn new(
        id: NodeId,
        pattern: Pattern,
        type_ann: Option<TypeSpec>,
        initializer: Expression,
        span: Span,
    ) -> Self {
        Self {
            id,
            pattern,
            type_ann,
            initializer,
            span,
        }
    }
}

/// Destructuring Pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Pattern {
    Single(Ident),
    Tuple(Vec<Ident>),
    List(Vec<Ident>),
    Struct(Vec<Ident>),
}

/// Type Signature Specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeSpec {
    pub name: Ident,
    pub generic_args: Option<Vec<TypeSpec>>,
    pub span: Span,
}

impl TypeSpec {
    pub fn new(name: Ident, generic_args: Option<Vec<TypeSpec>>, span: Span) -> Self {
        Self {
            name,
            generic_args,
            span,
        }
    }
}

/// build `name<T>`(params) -> RetType { body }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FuncDecl {
    pub id: NodeId,
    pub async_kw: bool,
    pub name: Ident,
    pub generic_params: Option<Vec<Ident>>,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeSpec>,
    pub body: Block,
    pub span: Span,
}

impl FuncDecl {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: NodeId,
        async_kw: bool,
        name: Ident,
        generic_params: Option<Vec<Ident>>,
        params: Vec<Parameter>,
        return_type: Option<TypeSpec>,
        body: Block,
        span: Span,
    ) -> Self {
        Self {
            id,
            async_kw,
            name,
            generic_params,
            params,
            return_type,
            body,
            span,
        }
    }
}

/// A function parameter with optional type annotation and default value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: Ident,
    pub type_ann: Option<TypeSpec>,
    pub default: Option<Expression>,
    pub span: Span,
}

impl Parameter {
    pub fn new(
        name: Ident,
        type_ann: Option<TypeSpec>,
        default: Option<Expression>,
        span: Span,
    ) -> Self {
        Self {
            name,
            type_ann,
            default,
            span,
        }
    }
}

/// struct Name { fields }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructDecl {
    pub id: NodeId,
    pub name: Ident,
    pub fields: Vec<FieldSpec>,
    pub span: Span,
}

impl StructDecl {
    pub fn new(id: NodeId, name: Ident, fields: Vec<FieldSpec>, span: Span) -> Self {
        Self {
            id,
            name,
            fields,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldSpec {
    pub name: Ident,
    pub type_ann: TypeSpec,
    pub span: Span,
}

impl FieldSpec {
    pub fn new(name: Ident, type_ann: TypeSpec, span: Span) -> Self {
        Self {
            name,
            type_ann,
            span,
        }
    }
}

/// enum Name { variants }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumDecl {
    pub id: NodeId,
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

impl EnumDecl {
    pub fn new(id: NodeId, name: Ident, variants: Vec<EnumVariant>, span: Span) -> Self {
        Self {
            id,
            name,
            variants,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumVariant {
    pub name: Ident,
    pub payload: Option<Vec<TypeSpec>>,
    pub span: Span,
}

impl EnumVariant {
    pub fn new(name: Ident, payload: Option<Vec<TypeSpec>>, span: Span) -> Self {
        Self {
            name,
            payload,
            span,
        }
    }
}

/// model Name extends ParentName { fields and methods }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelDecl {
    pub id: NodeId,
    pub name: Ident,
    pub parent: Option<Ident>,
    pub fields: Vec<VarDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

impl ModelDecl {
    pub fn new(
        id: NodeId,
        name: Ident,
        parent: Option<Ident>,
        fields: Vec<VarDecl>,
        methods: Vec<MethodDecl>,
        span: Span,
    ) -> Self {
        Self {
            id,
            name,
            parent,
            fields,
            methods,
            span,
        }
    }
}

/// Keyword categories for method compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodKeyword {
    Build,
    Fun,
}

/// build method_name(params) { body } (inside a model)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MethodDecl {
    pub id: NodeId,
    pub keyword: MethodKeyword,
    pub name: Ident,
    pub generic_params: Option<Vec<Ident>>,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeSpec>,
    pub body: Block,
    pub span: Span,
}

impl MethodDecl {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: NodeId,
        keyword: MethodKeyword,
        name: Ident,
        generic_params: Option<Vec<Ident>>,
        params: Vec<Parameter>,
        return_type: Option<TypeSpec>,
        body: Block,
        span: Span,
    ) -> Self {
        Self {
            id,
            keyword,
            name,
            generic_params,
            params,
            return_type,
            body,
            span,
        }
    }
}

/// export declaration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportDecl {
    pub id: NodeId,
    pub declaration: Box<Statement>,
    pub span: Span,
}

impl ExportDecl {
    pub fn new(id: NodeId, declaration: Box<Statement>, span: Span) -> Self {
        Self {
            id,
            declaration,
            span,
        }
    }
}

/// if/when condition { body } elif condition { body } else { body }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IfStmt {
    pub id: NodeId,
    pub condition: Expression,
    pub body: Block,
    pub else_ifs: Vec<(Expression, Block)>,
    pub else_body: Option<Block>,
    pub span: Span,
}

impl IfStmt {
    pub fn new(
        id: NodeId,
        condition: Expression,
        body: Block,
        else_ifs: Vec<(Expression, Block)>,
        else_body: Option<Block>,
        span: Span,
    ) -> Self {
        Self {
            id,
            condition,
            body,
            else_ifs,
            else_body,
            span,
        }
    }
}

/// for/each item in iterable { body }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForStmt {
    pub id: NodeId,
    pub item: Ident,
    pub iterable: Expression,
    pub body: Block,
    pub span: Span,
}

impl ForStmt {
    pub fn new(id: NodeId, item: Ident, iterable: Expression, body: Block, span: Span) -> Self {
        Self {
            id,
            item,
            iterable,
            body,
            span,
        }
    }
}

/// repeat N { body }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepeatStmt {
    pub id: NodeId,
    pub count: Expression,
    pub body: Block,
    pub span: Span,
}

impl RepeatStmt {
    pub fn new(id: NodeId, count: Expression, body: Block, span: Span) -> Self {
        Self {
            id,
            count,
            body,
            span,
        }
    }
}

/// while condition { body }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhileStmt {
    pub id: NodeId,
    pub condition: Expression,
    pub body: Block,
    pub span: Span,
}

impl WhileStmt {
    pub fn new(id: NodeId, condition: Expression, body: Block, span: Span) -> Self {
        Self {
            id,
            condition,
            body,
            span,
        }
    }
}

/// try/attempt { body } catch error { catch_body }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TryStmt {
    pub id: NodeId,
    pub body: Block,
    pub catch_var: Ident,
    pub catch_body: Block,
    pub span: Span,
}

impl TryStmt {
    pub fn new(id: NodeId, body: Block, catch_var: Ident, catch_body: Block, span: Span) -> Self {
        Self {
            id,
            body,
            catch_var,
            catch_body,
            span,
        }
    }
}

/// say expression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SayStmt {
    pub id: NodeId,
    pub value: Expression,
    pub span: Span,
}

impl SayStmt {
    pub fn new(id: NodeId, value: Expression, span: Span) -> Self {
        Self { id, value, span }
    }
}

/// return `[expression]`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReturnStmt {
    pub id: NodeId,
    pub value: Option<Expression>,
    pub span: Span,
}

impl ReturnStmt {
    pub fn new(id: NodeId, value: Option<Expression>, span: Span) -> Self {
        Self { id, value, span }
    }
}

/// throw expression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThrowStmt {
    pub id: NodeId,
    pub value: Expression,
    pub span: Span,
}

impl ThrowStmt {
    pub fn new(id: NodeId, value: Expression, span: Span) -> Self {
        Self { id, value, span }
    }
}

/// break
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BreakStmt {
    pub id: NodeId,
    pub span: Span,
}

impl BreakStmt {
    pub fn new(id: NodeId, span: Span) -> Self {
        Self { id, span }
    }
}

/// continue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinueStmt {
    pub id: NodeId,
    pub span: Span,
}

impl ContinueStmt {
    pub fn new(id: NodeId, span: Span) -> Self {
        Self { id, span }
    }
}

/// import path `[symbols]`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportStmt {
    pub id: NodeId,
    pub path: Vec<Ident>,
    pub symbols: Option<Vec<Ident>>,
    pub span: Span,
}

impl ImportStmt {
    pub fn new(id: NodeId, path: Vec<Ident>, symbols: Option<Vec<Ident>>, span: Span) -> Self {
        Self {
            id,
            path,
            symbols,
            span,
        }
    }
}

/// expression statement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpressionStmt {
    pub id: NodeId,
    pub expression: Expression,
    pub span: Span,
}

impl ExpressionStmt {
    pub fn new(id: NodeId, expression: Expression, span: Span) -> Self {
        Self {
            id,
            expression,
            span,
        }
    }
}

/// block statement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub id: NodeId,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Block {
    pub fn new(id: NodeId, statements: Vec<Statement>, span: Span) -> Self {
        Self {
            id,
            statements,
            span,
        }
    }
}

/// Expression node variants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    Assignment(AssignmentExpr),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LiteralVal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiteralExpr {
    pub id: NodeId,
    pub value: LiteralVal,
    pub span: Span,
}

impl LiteralExpr {
    pub fn new(id: NodeId, value: LiteralVal, span: Span) -> Self {
        Self { id, value, span }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryExpr {
    pub id: NodeId,
    pub left: Box<Expression>,
    pub op: String,
    pub right: Box<Expression>,
    pub span: Span,
}

impl BinaryExpr {
    pub fn new(
        id: NodeId,
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
        span: Span,
    ) -> Self {
        Self {
            id,
            left,
            op,
            right,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnaryExpr {
    pub id: NodeId,
    pub op: String,
    pub right: Box<Expression>,
    pub span: Span,
}

impl UnaryExpr {
    pub fn new(id: NodeId, op: String, right: Box<Expression>, span: Span) -> Self {
        Self {
            id,
            op,
            right,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallExpr {
    pub id: NodeId,
    pub callee: Box<Expression>,
    pub args: Vec<Expression>,
    pub span: Span,
}

impl CallExpr {
    pub fn new(id: NodeId, callee: Box<Expression>, args: Vec<Expression>, span: Span) -> Self {
        Self {
            id,
            callee,
            args,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemberExpr {
    pub id: NodeId,
    pub object: Box<Expression>,
    pub member: Ident,
    pub span: Span,
}

impl MemberExpr {
    pub fn new(id: NodeId, object: Box<Expression>, member: Ident, span: Span) -> Self {
        Self {
            id,
            object,
            member,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexExpr {
    pub id: NodeId,
    pub object: Box<Expression>,
    pub index: Box<Expression>,
    pub span: Span,
}

impl IndexExpr {
    pub fn new(id: NodeId, object: Box<Expression>, index: Box<Expression>, span: Span) -> Self {
        Self {
            id,
            object,
            index,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RangeExpr {
    pub id: NodeId,
    pub start: Box<Expression>,
    pub inclusive: bool,
    pub end: Box<Expression>,
    pub span: Span,
}

impl RangeExpr {
    pub fn new(
        id: NodeId,
        start: Box<Expression>,
        inclusive: bool,
        end: Box<Expression>,
        span: Span,
    ) -> Self {
        Self {
            id,
            start,
            inclusive,
            end,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AskExpr {
    pub id: NodeId,
    pub prompt: Box<Expression>,
    pub span: Span,
}

impl AskExpr {
    pub fn new(id: NodeId, prompt: Box<Expression>, span: Span) -> Self {
        Self { id, prompt, span }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewExpr {
    pub id: NodeId,
    pub class_name: Ident,
    pub args: Vec<Expression>,
    pub span: Span,
}

impl NewExpr {
    pub fn new(id: NodeId, class_name: Ident, args: Vec<Expression>, span: Span) -> Self {
        Self {
            id,
            class_name,
            args,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LambdaExpr {
    pub id: NodeId,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub span: Span,
}

impl LambdaExpr {
    pub fn new(id: NodeId, params: Vec<Parameter>, body: Block, span: Span) -> Self {
        Self {
            id,
            params,
            body,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListExpr {
    pub id: NodeId,
    pub items: Vec<Expression>,
    pub span: Span,
}

impl ListExpr {
    pub fn new(id: NodeId, items: Vec<Expression>, span: Span) -> Self {
        Self { id, items, span }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MapExpr {
    pub id: NodeId,
    pub entries: Vec<(Expression, Expression)>,
    pub span: Span,
}

impl MapExpr {
    pub fn new(id: NodeId, entries: Vec<(Expression, Expression)>, span: Span) -> Self {
        Self { id, entries, span }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FStringExpr {
    pub id: NodeId,
    pub parts: Vec<FStringPart>,
    pub span: Span,
}

impl FStringExpr {
    pub fn new(id: NodeId, parts: Vec<FStringPart>, span: Span) -> Self {
        Self { id, parts, span }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FStringPart {
    Literal(String),
    Expr(Expression),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignmentExpr {
    pub id: NodeId,
    pub target: Box<Expression>,
    pub op: String,
    pub value: Box<Expression>,
    pub span: Span,
}

impl AssignmentExpr {
    pub fn new(
        id: NodeId,
        target: Box<Expression>,
        op: String,
        value: Box<Expression>,
        span: Span,
    ) -> Self {
        Self {
            id,
            target,
            op,
            value,
            span,
        }
    }
}

impl Statement {
    /// Returns the source location span of this statement.
    pub fn span(&self) -> Span {
        match self {
            Statement::VarDecl(decl) => decl.span,
            Statement::ConstDecl(decl) => decl.span,
            Statement::FuncDecl(decl) => decl.span,
            Statement::StructDecl(decl) => decl.span,
            Statement::EnumDecl(decl) => decl.span,
            Statement::ModelDecl(decl) => decl.span,
            Statement::ExportDecl(decl) => decl.span,
            Statement::If(stmt) => stmt.span,
            Statement::For(stmt) => stmt.span,
            Statement::Repeat(stmt) => stmt.span,
            Statement::While(stmt) => stmt.span,
            Statement::Try(stmt) => stmt.span,
            Statement::Say(stmt) => stmt.span,
            Statement::Return(stmt) => stmt.span,
            Statement::Throw(stmt) => stmt.span,
            Statement::Break(stmt) => stmt.span,
            Statement::Continue(stmt) => stmt.span,
            Statement::Import(stmt) => stmt.span,
            Statement::Expression(stmt) => stmt.span,
            Statement::Block(stmt) => stmt.span,
        }
    }
}

impl Expression {
    /// Returns the source location span of this expression.
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal(expr) => expr.span,
            Expression::Binary(expr) => expr.span,
            Expression::Unary(expr) => expr.span,
            Expression::Call(expr) => expr.span,
            Expression::Member(expr) => expr.span,
            Expression::Index(expr) => expr.span,
            Expression::Identifier(ident) => ident.span,
            Expression::Range(expr) => expr.span,
            Expression::Ask(expr) => expr.span,
            Expression::New(expr) => expr.span,
            Expression::Lambda(expr) => expr.span,
            Expression::List(expr) => expr.span,
            Expression::Map(expr) => expr.span,
            Expression::FString(expr) => expr.span,
            Expression::Group(expr) => expr.span(),
            Expression::Assignment(expr) => expr.span,
        }
    }
}
