# 10 — TechScript 2.0 Semantic Analysis Specification

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
> **Related Documents**: [05 AST Design](./05_ast_design.md) · [09 Runtime Design](./09_runtime_design.md) · [11 Interpreter Design](./11_interpreter_design.md) · [14 Error Codes](./14_error_codes.md)

---

## 1. Overview

The semantic analyzer processes the AST, validating scopes, resolving identifiers, constructing the symbol table, and issuing deprecation warnings.

---

## 2. Scope & Name Resolution

A lexical Scope Frame contains symbol references. Name resolution maps every identifier node to its matching declaration. Hoisting registers top-level `build` functions and `model` templates in Pass 1.

---

## 3. Semantic Validation Rules

### 3.1 Variable & Variable Lifetime Validation
- Detect duplicate variables in same scope (`E0301`).
- Detect undefined variables (`E0300`).
- Prevent reassignment to `const` variables (`E0302`).

### 3.2 Function & Method Validation
- Verify call arguments match parameters.
- Verify `return` statements are nested inside functions.
- Verify `self` is referenced only inside model methods.
- **Unified Keyword Warn**: If a `MethodDecl` node contains `keyword: MethodKeyword::Fun`, semantic analysis raises warning `W0015` pointing directly to the method declaration span.

### 3.3 Control Flow & Module Validation
- Verify `break`/`continue` appear only inside loops.
- Verify imported module files exist.
- Verify exported symbols are declared with `export`.

---

## 4. Diagnostics & Suggestions

If warnings or errors are found, they are gathered in the diagnostics vector. For name typos, Levenshtein distance generates suggestions.

---

## 5. Compatibility & Evolution Analysis

### 5.1 Compatibility Notes
- **`fun` Deprecation**: The semantic analyzer flags `fun` methods as deprecated (`W0015`), but does not block code execution in the interpreter stage (warnings do not set `has_errors` to true).
- **Name Resolution Parity**: Scoping behaviors (including shadowing warnings) align with Version 1 name visibility rules.

### 5.2 Migration Notes
- To locate all deprecated `fun` occurrences, compile in check mode:
  ```bash
  tech check src/
  ```
  Each deprecated method produces a warning.
- Legacy files using `.tech` imports fail semantic analysis with `E0340` (Module not found).
- Automatic fix operations use the symbol table metadata to swap keywords:
  ```
  // Before
  model User {
      fun save() {}
  }
  
  // After tech lint --fix
  model User {
      build save() {}
  }
  ```

### 5.3 Rationale
- **In-place Warnings**: Raising `W0015` at the semantic analysis stage rather than during lexing or parsing ensures that the parser completes execution and constructs a clean AST, allowing formatting tools to run successfully.
- **Two-Pass Hoisting**: Pre-registering functions and models in Pass 1 allows forward-references to functions declared later in the `.txs` file.

### 5.4 Future Roadmap
- **v2.2**: The semantic analyzer will perform static type checking, comparing annotated parameters against calling arguments.
- **v3.0**: Optimizations will leverage type analysis records to emit optimized native instruction sequences.
