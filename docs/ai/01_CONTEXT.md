# 01 — CONTEXT

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Complete context layer for TechScript 2.0
> **Parent Link**: [PROJECT](./00_project.md)
> **Child Links**: [MEMORY](./02_memory.md) · [RULES](./03_rules.md)

---

## 1. What is TechScript?

TechScript is an English-like scripting language designed for absolute beginners, rapid prototypers, and web builders. It prioritizes readable keywords and block formatting over symbolic control structure.

---

## 2. Why does it exist?

1. **Reduce Beginner Friction**: Eliminate abstract syntax syntax errors (like missing semicolons, complex bracket combinations, or indentation spacing errors) by using explicit keywords (`when`/`else`, `each` loops) and standard braces `{ }`.
2. **Remove Python Dependencies**: The Version 1 prototype was written in Python. It suffered from slow startup overhead and runtime environment configuration issues. TechScript 2.0 replaces Python with a fast, self-contained Rust executable.
3. **Built-in Capabilities**: Include standard I/O, file, time, collections, and page-building web capabilities within the core distribution.

---

## 3. Supported Platforms

- **Linux** (x86_64, aarch64)
- **macOS** (x86_64, Apple Silicon)
- **Windows** (x86_64)

---

## 4. Feature Matrices

### 4.1 Supported in Version 2.0
- Lexical parsing of `.txs` source files.
- Unified `build` keyword for functions and methods.
- Deprecated `fun` keyword mapping inside models with warnings (`W0015`).
- Numeric digit separators (e.g. `1_000_000`).
- F-string interpolation (`f"Hello {name}!"`).
- Dynamic variable assignments (`make`, `const`).
- Sequential control statements (`when`, `each`, `repeat`, `while`).
- Custom object definitions via `model`.
- Error handling via `attempt`/`catch`/`throw`.

### 4.2 Unsupported in Version 2.0 (Planned for v2.1+)
- Static type annotations.
- Class inheritance (composition only in 2.0).
- Concurrency (`async`/`await`, threads, channels).
- Package registries (`techpm`).
- Native machine compilation.

---

## 5. Current Compiler Pipeline

```
[ .txs File ] → Lexer (logos) → Token Stream → Parser (Pratt) → AST → Sema (Validation) → Checked AST → Interpreter (Rust)
```
- Decoupled Frontend/Backend structure.
- Errors use diagnostic code mappings (`E0001`–`E1999`) displaying source location spans and suggestions.
