# TECHSCRIPT PERMANENT SYNTAX MEMORY

**Document Status:** Locked & Persistent  
**Authoritative Target Specification:** [TechScript_Final_Syntax_and_Language_Master_Spec.md](./TechScript_Final_Syntax_and_Language_Master_Spec.md)  
**Specification:** [docs/language/TECHSCRIPT_LANGUAGE_SPEC.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/docs/language/TECHSCRIPT_LANGUAGE_SPEC.md)

---

## 1. Locked Syntax Rules

| Operation | Canonical Syntax | Prohibited Legacy / Foreign Syntax |
| :--- | :--- | :--- |
| Variable Declaration | `name is "value"` | `let`, `var`, `make`, `val`, `def` |
| Variable Reassignment | `count becomes count + 1` | `:=`, `let count =` |
| Constant Declaration | `constant max is 100` | `const`, `val`, `final` |
| Function Declaration | `to name(param):` | `fn`, `func`, `function`, `def`, `do` |
| Return Statement | `return val` | `send val` |
| Output / Print | `say "Message"` | `print()`, `console.log`, `println!` |
| Conditionals | `if cond:`, `else if cond:`, `else:` | `when`, `elif`, `elsif`, braces `{}` |
| Counted Loops | `repeat N times:` | `repeat N end`, `loop`, `while (true)` |
| Iteration | `for item in items:` | `foreach`, `for (int i=0;...)` |
| Block Open / Close | Colon `:` + 4-space indent / dedent | `end`, `done`, `{ ... }` |
| String Interpolation | `"Hello, {name}!"` | `$"..."`, `f"..."`, `\"Hello, \" + name` |

---

## 2. Canonical Complete Demonstration

```txs
name is "TechScript"
count is 0

to greet(person):
    say "Hello, {person}!"

if count < 10:
    count becomes count + 1

greet(name)
say "Count: {count}"

repeat 3 times:
    say "Hello again from TechScript!"
```

---

## 3. Strict Rules for All Future Agent Sessions
1. Never re-introduce `do`, `fn`, `send`, `when`, `end`, or braces `{}` for ordinary control blocks.
2. Every `.txs` file must follow the canonical syntax above.
3. Every change to compiler or runtime must preserve tests for this syntax.
