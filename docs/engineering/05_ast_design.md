# 05 — TechScript 2.0 AST Design

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [03 Grammar](./03_grammar_ebnf.md) · [07 Parser](./07_parser_design.md) · [10 Semantic Analysis](./10_semantic_analysis.md) · [11 Interpreter](./11_interpreter_design.md)

---

## 1. AST Node Hierarchy

```mermaid
graph TD
    PROGRAM["Program"] --> STMT["Statement"]
    STMT --> DECL["Declaration"]
    STMT --> EXPR_STMT["ExpressionStatement"]
    STMT --> CTRL["Control Flow"]
    STMT --> ASSIGN["Assignment"]
    STMT --> IO["I/O Statement"]
    STMT --> ERR_STMT["Error Statement"]
    STMT --> JUMP["Jump Statement"]
    STMT --> IMP["Import Statement"]

    DECL --> VAR["VarDecl"]
    DECL --> CONST["ConstDecl"]
    DECL --> FUNC["FuncDecl"]
    DECL --> MODEL["ModelDecl"]
    DECL --> EXPORT["ExportDecl"]

    CTRL --> WHEN["WhenStmt"]
    CTRL --> EACH["EachStmt"]
    CTRL --> REPEAT["RepeatStmt"]
    CTRL --> WHILE["WhileStmt"]
    CTRL --> ATTEMPT["AttemptStmt"]

    IO --> SAY["SayStmt"]
    ERR_STMT --> THROW["ThrowStmt"]
    JUMP --> RET["ReturnStmt"]
    JUMP --> BRK["BreakStmt"]
    JUMP --> CONT["ContinueStmt"]

    PROGRAM --> EXPR["Expression"]
    EXPR --> LIT["Literal"]
    EXPR --> BINOP["BinaryExpr"]
    EXPR --> UNOP["UnaryExpr"]
    EXPR --> CALL["CallExpr"]
    EXPR --> MEMBER["MemberExpr"]
    EXPR --> INDEX["IndexExpr"]
    EXPR --> IDENT["IdentExpr"]
    EXPR --> RANGE["RangeExpr"]
    EXPR --> ASK_EXPR["AskExpr"]
    EXPR --> NEW["NewExpr"]
    EXPR --> LAMBDA["LambdaExpr"]
    EXPR --> LIST["ListExpr"]
    EXPR --> MAP_EXPR["MapExpr"]
    EXPR --> FSTR["FStringExpr"]
    EXPR --> GROUP["GroupExpr"]

    style PROGRAM fill:#4a9eff,color:#fff
    style STMT fill:#ff6b6b,color:#fff
    style EXPR fill:#6bcb77,color:#fff
    style DECL fill:#ffd93d,color:#333
```

---

## 2. Core Types

### 2.1 Span (Source Location)

```rust
/// Byte-offset range in source code. Used by every AST node for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,  // Inclusive byte offset
    pub end: usize,    // Exclusive byte offset
}
```

### 2.2 Node ID

```rust
/// Unique identifier for each AST node. Used by the semantic analyzer
/// to attach resolved information without mutating the AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);
```

### 2.3 Identifier

```rust
/// A named identifier with source location.
#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}
```

---

## 3. Program (Root Node)

```rust
/// The root of every TechScript AST. Represents a single .txs file.
#[derive(Debug, Clone)]
pub struct Program {
    pub id: NodeId,
    pub statements: Vec<Statement>,
    pub span: Span,
}
```

---

## 4. Statements

```rust
#[derive(Debug, Clone)]
pub enum Statement {
    // Declarations
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    FuncDecl(FuncDecl),
    ModelDecl(ModelDecl),
    ExportDecl(ExportDecl),

    // Control flow
    When(WhenStmt),
    Each(EachStmt),
    Repeat(RepeatStmt),
    While(WhileStmt),
    Attempt(AttemptStmt),

    // Simple statements
    Say(SayStmt),
    Return(ReturnStmt),
    Throw(ThrowStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Assignment(AssignmentStmt),
    Import(ImportStmt),

    // Expression as statement (result discarded)
    Expression(ExpressionStmt),

    // Block
    Block(Block),
}
```

### 4.1 Declarations

```rust
/// make name = expression
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub id: NodeId,
    pub name: Ident,
    pub initializer: Expression,
    pub span: Span,
}

/// const NAME = expression
#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub id: NodeId,
    pub name: Ident,
    pub initializer: Expression,
    pub span: Span,
}

/// build name(params) { body }
#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub id: NodeId,
    pub name: Ident,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub span: Span,
}

/// A function parameter with optional default value
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Ident,
    pub default: Option<Expression>,
    pub span: Span,
}

/// model Name { fields and methods }
#[derive(Debug, Clone)]
pub struct ModelDecl {
    pub id: NodeId,
    pub name: Ident,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

/// make field_name = default_value (inside a model)
#[derive(Debug, Clone)]
pub struct FieldDecl {
    pub id: NodeId,
    pub name: Ident,
    pub default: Expression,
    pub span: Span,
}

/// The keyword used to declare the method (for compatibility tracking)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKeyword {
    Build, // build
    Fun,   // fun (deprecated)
}

/// build method_name(params) { body } (inside a model)
#[derive(Debug, Clone)]
pub struct MethodDecl {
    pub id: NodeId,
    pub keyword: MethodKeyword, // Tracks if declared with build or fun
    pub name: Ident,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub span: Span,
}

/// export <declaration>
#[derive(Debug, Clone)]
pub struct ExportDecl {
    pub id: NodeId,
    pub declaration: Box<Statement>,  // Must be FuncDecl, ConstDecl, or ModelDecl
    pub span: Span,
}
```

---

## 5. Compatibility & Evolution Analysis

### 5.1 Compatibility Notes
- **AST Node Retention**: Retaining the `MethodKeyword` variant in `MethodDecl` ensures that the AST preserves the exact syntax used in the source `.txs` file.
- **Unified AST Execution**: For downstream stages (Interpreter, VM), the `MethodKeyword` is ignored, executing methods identically regardless of whether they were declared using `build` or `fun`.

### 5.2 Migration Notes
- Tooling or formatting passes (`tech fmt`) inspect the `keyword` field of `MethodDecl`. If it is `MethodKeyword::Fun`, formatting replaces it with `build` when writing back to the `.txs` file:
  ```rust
  // formatter/src/lib.rs snippet
  fn format_method_decl(&mut self, node: &MethodDecl) {
      // Version 1 uses "fun", rewrite to "build" under v2.0
      self.write("build ");
      self.format_ident(&node.name);
      // ...
  }
  ```

### 5.3 Rationale
- **Preserving Keyword Info**: Storing `MethodKeyword` in the AST node rather than discarding it during parsing allows the Semantic Analyzer to emit precise diagnostics (`W0015` warnings pointing directly to the `fun` token span) while keeping compiler logic clean.
- **NodeId tracking**: Assigning a unique `NodeId` to every node (including expressions inside f-strings) allows the compilation pipeline to map errors back to the source spans.

### 5.4 Future Roadmap
- **v2.2**: The `Parameter` struct will be updated to include an optional `param_type` AST field supporting static type annotations:
  ```rust
  pub struct Parameter {
      pub name: Ident,
      pub param_type: Option<TypeAnnotation>, // Future type annotations
      pub default: Option<Expression>,
      pub span: Span,
  }
  ```
