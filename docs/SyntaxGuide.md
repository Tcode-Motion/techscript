# TechScript 2.0 Syntax Guide

This document defines the formal syntax specifications, operator precedence levels, and grammar layouts.

---

## 📐 Block Delimiters
Block scopes are opened with structure keywords (`do`, `when`, `loop`, `repeat`, `for`, `class`, `try`) and closed exclusively with the `end` keyword. Curly braces `{}` are not valid delimiters for control blocks.

---

## ⚖️ Operator Precedence Table

Operators are evaluated in the following order of precedence (highest to lowest):

| Precedence | Operators | Description |
|:---:|:---|:---|
| 1 | `()` | Function call, grouping |
| 2 | `.` | Member access |
| 3 | `typeof` | Type query operator |
| 4 | `-` (unary), `not` | Negation, Logical NOT |
| 5 | `*`, `/`, `%` | Multiplicative math |
| 6 | `+`, `-` | Additive math, concatenation |
| 7 | `in` | Membership containment check |
| 8 | `<`, `<=`, `>`, `>=` | Comparison relation |
| 9 | `==`, `!=` | Equality relation |
| 10 | `and` | Logical conjunction |
| 11 | `or` | Logical disjunction |
| 12 | `=` | Variable assignment |

---

## 🧬 Expressions & Statements
* **Statement Terminators**: Newlines serve as statement terminators. Standard semicolons `;` are syntax errors.
* **String Interpolation**: Prepend a `$` symbol before a string literal to enable interpolation:
  ```txs
  name = "Alice"
  say $"Hello, {name}!"
  ```
