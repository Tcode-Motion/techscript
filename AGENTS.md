# TECHSCRIPT REPOSITORY RULES FOR AI CODING AGENTS

## 🚨 MANDATORY GOVERNING DIRECTIVE: SYNTAX IS LOCKED

**TECHSCRIPT SYNTAX IS LOCKED TO THE CANONICAL LANGUAGE SPECIFICATION.**  
The single source of truth for the TechScript language grammar and syntax is:  
[docs/language/TechScript_Final_Syntax_and_Language_Master_Spec.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/docs/language/TechScript_Final_Syntax_and_Language_Master_Spec.md) and [docs/language/TECHSCRIPT_LANGUAGE_SPEC.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/docs/language/TECHSCRIPT_LANGUAGE_SPEC.md).

Every coding agent working in this repository MUST strictly follow these rules:

---

### 1. Consult the Specification First
Before writing or modifying any `.txs` code, consult [docs/language/TechScript_Final_Syntax_and_Language_Master_Spec.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/docs/language/TechScript_Final_Syntax_and_Language_Master_Spec.md). Never invent syntax based on habits from Python, JavaScript, Rust, Kotlin, or Go.

### 2. Follow Canonical Constructs
- **Variable Declaration:** Use `is` for first assignment:
  ```txs
  name is "TechScript"
  count is 0
  ```
- **Reassignment:** Use `becomes` for changing an existing variable:
  ```txs
  count becomes count + 1
  ```
- **Constants:** Use `constant` with `is`:
  ```txs
  constant max_attempts is 5
  ```
- **Functions:** Use `to name(params):` with colon and indentation:
  ```txs
  to greet(person):
      say "Hello, {person}!"
      return "done"
  ```
- **Return Values:** Use `return`:
  ```txs
  return result
  ```
- **Printing:** Use `say`:
  ```txs
  say "Count: {count}"
  ```
- **String Interpolation:** Embedded curly braces automatically interpolate: `"Count: {count}"`. Do NOT require prefix characters like `$` or `f`.
- **Conditionals:** Use `if ...:`, `else if ...:`, and `else:` with colon and indentation:
  ```txs
  if count < 10:
      count becomes count + 1
  else:
      say "Done"
  ```
- **Repetition:** Use `repeat N times:` with colon and indentation:
  ```txs
  repeat 3 times:
      say "Hello!"
  ```
- **Iteration:** Use `for item in collection:` with colon and indentation.
- **While Loops:** Use `while condition:` with colon and indentation.
- **Block Delimiters:** All ordinary blocks use `:` followed by 4-space indentation. Never require `do`, `fn`, `end`, or `{}` for ordinary control flow or functions.

### 3. Maintain Complete Layer Separation
- **Layer 1 (Core Language):** Syntax, variables, functions, loops, types, memory semantics. Must NEVER contain hardcoded UI or application widgets.
- **Layer 2 (Compiler & Runtime):** Lexer, parser, semantic analyzer, bytecode VM, tree-walking interpreter.
- **Layer 3 (Standard Library):** `text`, `math`, `collections`, `json`, `files`, `time`, `http`, `system`, `random`, `log`, `test`.
- **Layer 4 (Frameworks):** AppForge (mobile), Webforge (web), graphics libraries.
- **Layer 5 (Applications):** Concrete applications written in pure `.txs`.

### 4. Implementation Rigor
- When adding or modifying language features, update the specification, grammar, compiler crates, runtime, and automated tests together.
- Never document an unsupported feature as implemented.
- Always execute and verify programs with `cargo run -p techscript_cli -- run <file>` or `tech run <file>`.
