# TechScript Canonical Syntax Guide

This document defines the formal syntax specifications, operator precedence levels, and grammar layouts according to [TechScript_Final_Syntax_and_Language_Master_Spec.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/docs/language/TechScript_Final_Syntax_and_Language_Master_Spec.md).

---

## 📐 Block Delimiters
Block scopes are opened with a colon (`:`) after control structures (`if`, `else if`, `else`, `repeat`, `for`, `while`, `to`, `try`, `catch`) and closed exclusively by returning indentation (4 spaces per level). Curly braces `{}` are not valid delimiters for control blocks.

---

## ⚖️ Operator Precedence Table

Operators are evaluated in the following order of precedence (highest to lowest):

| Precedence | Operators | Description |
|:---:|:---|:---|
| 1 | `()` | Function call, grouping |
| 2 | `.` | Member access |
| 3 | `**` | Exponentiation |
| 4 | `-` (unary), `not` | Negation, Logical NOT |
| 5 | `*`, `/`, `//`, `%` | Multiplicative math |
| 6 | `+`, `-` | Additive math, concatenation |
| 7 | `in` | Membership containment check |
| 8 | `<`, `<=`, `>`, `>=` | Comparison relation |
| 9 | `==`, `!=` | Equality relation |
| 10 | `and` | Logical conjunction |
| 11 | `or` | Logical disjunction |
| 12 | `is`, `becomes` | Variable declaration and reassignment |

---

## 🧬 Expressions & Statements
* **Statement Terminators**: Newlines serve as statement terminators. Standard semicolons `;` are not required.
* **String Interpolation**: Double-quoted strings automatically interpolate embedded curly braces without prefix:
  ```txs
  name is "Tanmoy"
  say "Hello, {name}!"
  ```
* **Variables & Reassignment**:
  ```txs
  count is 0
  count becomes count + 1
  ```
* **Functions**:
  ```txs
  to greet(person):
      say "Hello, {person}!"
  ```
