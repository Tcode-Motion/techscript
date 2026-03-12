// ── TechScript AST Node Definitions ──────────────────────────────────
// Port of ast_nodes.py — every node is a Rust enum variant.

/// A parsed TechScript program.
#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}

/// Function / method parameter.
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: Option<Expr>,
}

// ─── Statements ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Stmt {
    /// `say <expr>, ...`
    Say { values: Vec<Expr> },
    /// `make <name> = <expr>`
    Set { name: String, value: Expr },
    /// `keep <name> = <expr>`
    Const { name: String, value: Expr },
    /// `<target> = | += | -= | *= | /= <expr>`
    Assign { target: Expr, op: String, value: Expr },
    /// A bare expression used as a statement (e.g. function call).
    Expression { expression: Expr },
    /// `when <cond> { ... } or when ... else { ... }`
    If {
        condition: Expr,
        body: Vec<Stmt>,
        elif_clauses: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
    },
    /// `each <var> in <iter> { ... }`
    For { var_name: String, iterable: Expr, body: Vec<Stmt> },
    /// `repeat <cond> { ... }`
    While { condition: Expr, body: Vec<Stmt> },
    /// `build <name>(<params>) { ... }`
    Fn { name: String, params: Vec<Param>, body: Vec<Stmt> },
    /// `model <name>(<parent>) { ... }`
    Class { name: String, parent: Option<String>, body: Vec<Stmt> },
    /// `send <expr>`
    Return { value: Option<Expr> },
    /// `stop`
    Break,
    /// `skip`
    Skip,
    /// `pass`
    Pass,
    /// `attempt { ... } rescue <var> { ... } always { ... }`
    Try {
        body: Vec<Stmt>,
        catch_var: Option<String>,
        catch_body: Vec<Stmt>,
        finally_body: Option<Vec<Stmt>>,
    },
    /// `fail <expr>`
    Throw { value: Expr },
    /// `match <subject> { case <val> { ... } }`
    Match { subject: Expr, cases: Vec<(Expr, Vec<Stmt>)> },
    /// `use <module>`
    Import { module: String, names: Option<Vec<String>>, alias: Option<String> },
    /// `take <names> from <module>`
    FromImport { module: String, names: Vec<String> },
    /// `drop <name>`
    Del { name: String },
    /// `defer <expr>`
    Defer { expression: Expr },
    /// `guard <cond> else { ... }`
    Guard { condition: Expr, else_body: Vec<Stmt> },
    /// `with <expr> as <var> { ... }`
    With { expression: Expr, var_name: String, body: Vec<Stmt> },
    /// `share <declaration>`
    Export { declaration: Box<Stmt> },
    /// `unless <cond> { ... }`
    Unless { condition: Expr, body: Vec<Stmt> },
    /// `until <cond> { ... }`
    Until { condition: Expr, body: Vec<Stmt> },
}

// ─── Expressions ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    NumberInt(i64),
    NumberFloat(f64),
    String(String),
    FString(String),
    Bool(bool),
    None,
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Identifier(String),
    BinaryOp { left: Box<Expr>, op: String, right: Box<Expr> },
    UnaryOp { op: String, operand: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Index { obj: Box<Expr>, index: Box<Expr> },
    Member { obj: Box<Expr>, member: String },
    Lambda { params: Vec<Param>, body: Box<Expr> },
    Ask { prompt: Box<Expr> },
    Ternary { true_val: Box<Expr>, condition: Box<Expr>, false_val: Box<Expr> },
    Range { start: Box<Expr>, end: Box<Expr>, inclusive: bool },
}
