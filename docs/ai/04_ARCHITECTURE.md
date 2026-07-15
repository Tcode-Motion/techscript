# 04 — ARCHITECTURE

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Technical architecture overview of the compiler and runtime
> **Parent Link**: [RULES](./03_rules.md)
> **Child Links**: [LANGUAGE](./05_language.md) · [REPOSITORY](./08_repository.md)

---

## 1. System Pipeline

```mermaid
flowchart TD
    SRC["Source File (.txs)"]
    LEX["1. Lexer (techscript_lexer)\n• logos library\n• Mode-stack for f-strings"]
    TOK["Token Stream"]
    PAR["2. Parser (techscript_parser)\n• Recursive Descent (Statements)\n• Pratt Parser (Expressions)"]
    RAST["Raw AST (techscript_ast)"]
    SEM["3. Semantic Analyzer (techscript_sema)\n• Pass 1: Declaration hoarding\n• Pass 2: Scope checks & resolve\n• Emits W0015 on 'fun'"]
    CAST["Checked AST + Symbol Table"]
    INT["4a. Interpreter (v2.0)\n(techscript_interpreter)\n• Walk AST nodes\n• Environment scope chain\n• Native stdlib dispatch"]
    OUT["Process Output / Exit Code"]

    BC["4b. Bytecode Compiler (v2.1)\n(techscript_bytecode)"]
    BYTE["Bytecode"]
    VM["5b. VM & GC (v2.1)\n(techscript_vm / gc)\n• Stack-based VM\n• Tracing Mark-and-Sweep"]

    LLVM["4c. LLVM Backend (v3.0)\n(techscript_llvm)\n• inkwell FFI\n• Native optimized binary"]

    SRC --> LEX
    LEX --> TOK
    TOK --> PAR
    PAR --> RAST
    RAST --> SEM
    SEM --> CAST
    CAST --> INT
    INT --> OUT

    CAST -.-> BC
    BC -.-> BYTE
    BYTE -.-> VM
    VM -.-> OUT
    CAST -.-> LLVM
    LLVM -.-> OUT
```

---

## 2. Shared Data Structures

### 2.1 AST (techscript_ast)
- Represents syntactic constructs. Contains `Statement`, `Expression`, and `Declaration` enum nodes.
- Each node carries a `Span` (source coordinates) and a unique auto-incremented `NodeId`.
- Includes the `Visitor` trait for traversal.

### 2.2 Symbol Table
- Constructed by the Semantic Analyzer.
- Maps `NodeId` references to `Symbol` structures (tracks variables, functions, models, scopes).

---

## 3. Execution Engines

### 3.1 AST Interpreter (v2.0)
- Directly executes Checked AST nodes.
- Environment structures manage variable storage using a vector of scope HashMaps.
- Scopes are pushed on blocks and popped on exit.
- Simple call stack keeps frames for tracing runtime errors.

### 3.2 Bytecode VM (v2.1 - Planned)
- Compiles AST to stack instructions (e.g. `PUSH_INT`, `ADD`, `STORE_LOCAL`).
- Replaces Environment HashMaps with fast index-based slot tables.
- A tracing GC manages allocations.

### 3.3 LLVM Native Compiler (v3.0 - Planned)
- Lowers AST nodes directly to LLVM IR.
- Applies LLVM optimization passes to output native executables.

---

## 4. Subsystem Dependency Graph

```mermaid
graph BT
    ast["techscript_ast"]
    lexer["techscript_lexer"]
    parser["techscript_parser"]
    sema["techscript_sema"]
    runtime["techscript_runtime"]
    builtins["techscript_builtins"]
    interp["techscript_interpreter"]
    stdlib["techscript_stdlib"]
    cli["techscript_cli"]
    errors["techscript_errors"]

    lexer --> errors
    lexer --> ast
    parser --> lexer
    parser --> ast
    parser --> errors
    sema --> ast
    sema --> errors
    runtime --> ast
    runtime --> errors
    builtins --> runtime
    interp --> sema
    interp --> runtime
    interp --> builtins
    stdlib --> runtime
    stdlib --> builtins
    cli --> lexer
    cli --> parser
    cli --> sema
    cli --> interp
    cli --> stdlib
```
- `techscript_errors` is the foundational dependency.
- `techscript_ast` is shared across all front-end and back-end modules.
- `techscript_cli` forms the final executable integrating all crates.
