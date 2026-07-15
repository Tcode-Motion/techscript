# 07 — TechScript 2.0 Parser Design

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [03 Grammar](./03_grammar_ebnf.md) · [05 AST Design](./05_ast_design.md) · [06 Lexer Design](./06_lexer_design.md) · [14 Error Codes](./14_error_codes.md)

---

## 1. Parsing Strategy

### 1.1 Decision: Recursive Descent + Pratt Parser

Statements and declarations are parsed using recursive descent because of their clear, keyword-led structure. Expressions are parsed using a Pratt parser to handle operator precedence and associativity.

---

## 2. Parser State

```rust
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    errors: Vec<Diagnostic>,
    node_id_counter: u32,
}
```

---

## 3. Statement & Method Parsing

Methods inside a model can be declared with either `build` or the deprecated `fun` keyword:

```
parse_model_decl():
    expect(Model)
    name = expect(Identifier)
    expect(LeftBrace)
    fields = []
    methods = []
    while not check(RightBrace) and not is_at_end():
        skip_newlines()
        if check(Make):
            fields.push(parse_field_decl())
        else if check(Build) or check(Fun):
            methods.push(parse_method_decl())
        else:
            error("Expected field or method declaration")
            synchronize()
    expect(RightBrace)
    return ModelDecl { name, fields, methods }

parse_method_decl():
    keyword_token = advance() // consume Build or Fun
    keyword = if keyword_token.kind == Build: MethodKeyword::Build else MethodKeyword::Fun
    name = expect(Identifier)
    expect(LeftParen)
    params = parse_parameter_list()
    expect(RightParen)
    body = parse_block()
    return MethodDecl { keyword, name, params, body }
```

---

## 4. Expression Parsing — Pratt Parser

### 4.1 Binding Power Table

| Operator | LBP | RBP | Associativity |
|---|---|---|---|
| `or` | 10 | 11 | Left |
| `and` | 20 | 21 | Left |
| `==`, `!=`, `is` | 30 | 31 | Left |
| `<`, `>`, `<=` | 40 | 41 | Left |
| `..`, `..=` | 50 | 50 | Non-associative |
| `+`, `-` | 60 | 61 | Left |
| `*`, `/`, `//`, `%` | 70 | 71 | Left |
| `**` | 81 | 80 | Right |
| Unary `-`, `not` | — | 90 | Right |
| `()`, `[]`, `.` | 100 | — | Postfix |

---

## 5. Error Recovery

Panic-mode synchronization skips tokens until a statement boundary or synchronization point (like `;`, `}`, `make`, `build`, `when`) is met, allowing the parser to detect multiple errors.

---

## 6. Compatibility & Evolution Analysis

### 6.1 Compatibility Notes
- **Parsing `fun` Methods**: The parser treats the `fun` token exactly like the `build` token inside `model` declarations. Both map to a `MethodDecl` AST node.
- **Strict Extensions**: Files with non-`.txs` extensions are rejected prior to parsing by the CLI dispatch.

### 6.2 Migration Notes
- When `fun` is parsed:
  - The AST stores `MethodKeyword::Fun`.
  - The parser registers a compilation note.
  - The semantic check phase maps this flag to diagnostic `W0015` (warning).
- The parser itself does not halt on `fun`; it executes recovery logic normally.

### 6.3 Rationale
- **Identical Node structure**: Using `MethodDecl` for both keywords avoids duplicate parsing branches, simplifying code and minimizing maintenance overhead.
- **Synchronized Error limits**: Aborting after 20 errors prevents cascading loop crashes on severely truncated `.txs` source files.

### 6.4 Future Roadmap
- **v2.2**: The parser will be expanded to support optional type signatures:
  ```rust
  fn parse_parameter(&mut self) -> Result<Parameter, Diagnostic> {
      let name = self.expect(TokenKind::Identifier)?;
      let mut param_type = None;
      if self.match_token(TokenKind::Colon) {
          param_type = Some(self.parse_type_annotation()?);
      }
      // ...
  }
  ```
