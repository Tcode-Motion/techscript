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
    /// `state count = 0` (web reactive state)
    State { name: String, value: Expr },
    /// `component Name { ... }`
    Component { name: String, body: Vec<Stmt> },
    /// `page Name { ... }`
    Page { name: String, body: Vec<Stmt> },
    /// `api Name { route ... }`
    Api { name: String, routes: Vec<(String, String, Vec<Stmt>)> },
    /// `window "Title" { ... }` (gui)
    Window { title: String, body: Vec<Stmt> },
    /// `scene name { ... }` (3d)
    Scene { name: String, body: Vec<Stmt> },
    /// `timeline name { ... }` (anime)
    Timeline { name: String, body: Vec<Stmt> },
    /// `render "tag" { ... }`
    Render { tag: String, body: Vec<Stmt> },
    /// `button "text" { ... }` (gui)
    Button { label: String, body: Vec<Stmt> },
    /// `input name placeholder "text"` (gui)
    Input { name: String, placeholder: String },
    /// `label "text"` (gui)
    Label { text: String },
    /// `camera pos [0, 0, 5]`
    Camera { coords: Vec<Expr> },
    /// `light ambient`
    Light { kind: String },
    /// `mesh cube color "#fff"`
    Mesh { shape: String, color: String },
    /// `move obj to [x, y] over 1s ease "out"`
    AnimeMove {
        target: String,
        coords: Vec<Expr>,
        duration: Expr,
        ease: String,
    },
    /// `fade obj to 0 over 0.5s`
    AnimeFade {
        target: String,
        opacity: Expr,
        duration: Expr,
    },
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
