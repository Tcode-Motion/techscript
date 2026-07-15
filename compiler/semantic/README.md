# techscript_semantic

Name resolver, duplicate validator, and symbol scope compiler for TechScript 2.0.

## Contents
- `SymbolTable`: Scope registry mapping identifiers to their declarations.
- `CheckedProgram`: Resolved AST tree structure.
- `SemanticAnalyzer`: Implements two-pass hoisting and declaration visibility sweeps.
