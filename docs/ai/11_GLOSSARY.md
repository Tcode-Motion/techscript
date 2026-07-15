# 11 — GLOSSARY

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Compiler and language terminology reference definitions
> **Parent Link**: [HANDOFF](./10_handoff.md)
> **Child Links**: [INDEX](./12_index.md)

---

| Term | Definition |
|---|---|
| **Lexer (Scanner)** | First compiler phase. Converts raw source code strings into discrete compiler-readable tokens. |
| **Parser** | Second compiler phase. Consumes a token stream and constructs an Abstract Syntax Tree (AST) validating grammar rules. |
| **AST (Abstract Syntax Tree)** | Tree representation of the abstract syntactic structure of source code. Nodes represent expressions, statements, and declarations. |
| **Sema (Semantic Analysis)** | Third compiler phase. Validates name resolution, scoping rules, constant definitions, keyword usages, and annotations on the AST. |
| **Checked AST** | An AST annotated by Semantic Analysis with scope mappings, symbol tables, and compiler warning markers. |
| **Symbol Table** | Associative data structure mapping node identifier references to semantic symbols (tracks scopes, bindings, declarations). |
| **Interpreter** | The execution engine walking the Checked AST, executing actions sequentially. |
| **Bytecode VM** | Future execution engine (v2.1). Compiles AST nodes into flat stack instructions executed inside a virtual machine loop. |
| **LLVM Backend** | Future native code compiler (v3.0). Compiles AST nodes to LLVM IR, generating optimized native machine binaries. |
| **Self-Hosting** | Milestone where the compiler is written in the target language (TechScript) and compiles its own source code. |
| **Span** | A byte range (start, end offsets) pointing to a token or AST node's exact position in source files. |
| **Diagnostic** | A compiler-generated warning or error message mapping a `Span` to a specific code (`E0001`–`E1999`, `W0001`–`W0099`). |
| **F-String** | An interpolated string literal (e.g. `f"x = {x}"`) containing embedded expressions parsed at runtime. |
| **Logos** | Fast Rust crate compiling regular expressions into state machine lexical tables at compile time. |
| **Pratt Parser** | An operator precedence parsing strategy that handles expressions, operators, and grouping without nesting limits. |
| **Unified keyword** | The design choice in TechScript 2.0 where both functions and methods are declared using `build`, deprecating `fun`. |
