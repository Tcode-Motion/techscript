# 14 — TechScript 2.0 Error Code Specification

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [06 Lexer](./06_lexer_design.md) · [07 Parser](./07_parser_design.md) · [10 Semantic Analysis](./10_semantic_analysis.md) · [11 Interpreter](./11_interpreter_design.md)

---

## Error Code Ranges

| Range | Category | Phase |
|---|---|---|
| `E0001 – E0099` | Lexer errors | Lexical analysis |
| `E0100 – E0299` | Parser errors | Syntactic analysis |
| `E0300 – E0499` | Semantic errors | Semantic analysis |
| `E1000 – E1999` | Runtime errors | Interpretation / execution |
| `W0001 – W0099` | Warnings | Any phase |

---

## Lexer Errors (E0001 – E0099)
- `E0001`: Unexpected character (e.g. `@`).
- `E0010`: Trailing underscore in number (e.g. `42_`).
- `E0011`: Empty numeric literal after prefix (e.g. `0x`).
- `E0012`: Invalid digit for base (e.g. `0b102`).
- `E0021`: Unterminated string literal.

---

## Parser Errors (E0100 – E0299)
- `E0100`: Expected expression.
- `E0101`: Expected identifier.
- `E0104`: Expected `{` to begin block.
- `E0105`: Expected `}` to close block.
- `E0107`: Expected statement terminator (missing newline or `;`).
- `E0113`: Invalid assignment target (e.g. `42 = x`).

---

## Semantic Errors (E0300 – E0499)
- `E0300`: Undefined variable (variable used before declaration).
- `E0301`: Duplicate variable declaration in same scope.
- `E0302`: Cannot reassign constant.
- `E0310`: Wrong number of arguments (too few).
- `E0311`: Wrong number of arguments (too many).
- `E0312`: Return statement outside function.
- `E0320`: `self` used outside method declaration.
- `E0340`: Module not found (cannot resolve `.txs` import).
- `E0350`: Cannot export non-exportable statement.

---

## Runtime Errors (E1000 – E1999)
- `E1010`: Division by zero.
- `E1011`: Type mismatch in operation (e.g. `"hello" - 5`).
- `E1020`: Stack overflow (recursion limit exceeded).
- `E1030`: Value not iterable in `each` loop.
- `E1041`: Field or method not found on object.
- `E1050`: Index out of bounds.

---

## Warnings (W0001 – W0099)

### W0015 — Deprecated Keyword 'fun'

| Field | Value |
|---|---|
| **Description** | Use of the deprecated `fun` keyword inside a model definition |
| **Cause** | Declaring a class method using `fun` instead of the unified `build` keyword |
| **Suggested Fix** | Replace `fun` with `build` |
| **Example** | `model Dog { fun bark() {} }` → `Warning [W0015]: Use of deprecated keyword 'fun'. Replace 'fun' with 'build'` |

- `W0001`: Identifier starts with `__` (reserved).
- `W0010`: Variable shadows an outer scope variable.
- `W0011`: Variable declared but never used.

---

## Compatibility & Evolution Analysis

### 16.1 Compatibility Notes
- **Warnings vs Errors**: `W0015` is a warning, not an error. Compilation and execution continue successfully.
- **Import Diagnostics**: Non-`.txs` module imports trigger `E0340` (Module not found).

### 16.2 Migration Notes
- Running the linter fix command resolves all `W0015` warnings:
  ```bash
  tech lint src/ --fix
  ```
- Example diagnostic report:
  ```
  Warning [W0015]: Use of deprecated keyword 'fun'
    --> main.txs:12:5
     |
  12 |     fun bark() {}
     |     ^^^ help: replace 'fun' with 'build'
  ```

### 16.3 Rationale
- **Distinct Ranges**: Categorizing errors into distinct ranges (Lexer, Parser, Sema, Runtime, Warnings) allows developers and IDE extensions to quickly locate failures.
- **Warning Tolerances**: Emitting warnings instead of hard errors for deprecated keywords preserves backward compatibility with Version 1 scripts while encouraging clean migration.

### 16.4 Future Roadmap
- **v2.2**: Introduce new static semantic type error codes:
  - `E0500`: Type mismatch in assignment.
  - `E0501`: Incompatible return type.
