# TechScript Syntax Guide

This document defines the formal grammar, keywords, and structural layout of TechScript.

---

## 📐 Block Structures
Unlike C/C++ or JavaScript, TechScript does not use curly braces `{}` to define scopes. Blocks are defined by indentation or line breaks, and closed using the `end` keyword. Semicolons are not allowed.

```txs
# A typical block structure
do print_numbers(max)
    for i in 1..=max
        say i
    end
end
```

---

## 🔑 Keywords

TechScript defines a set of **canonical keywords**:

| Keyword | Purpose |
|:---|:---|
| `do` | Defines a function |
| `send` | Returns a value from a function |
| `when` | Initiates a conditional block |
| `else when` | Alternative conditional arm |
| `else` | Default conditional arm |
| `loop` | Counted loop |
| `repeat` | While loop |
| `for` | Collection iterator |
| `in` | Membership checks & iterator binding |
| `match` | Pattern matching statement |
| `case` | Pattern matching arm |
| `default` | Pattern matching fallback |
| `try` | Exception boundary |
| `catch` | Exception handler |
| `throw` | Explicit panic or error |
| `use` | Imports a module |
| `class` | Declares a model/class |
| `struct` | Declares a structured type |
| `enum` | Declares a nominal enum |
| `trait` | Shared behaviors interface |
| `interface` | Contract layout |
| `const` | Declares a constant |
| `null` | Absence of value |
| `say` | Built-in output print |
| `ask` | Built-in input read |
| `break` | Loop exit |
| `continue` | Loop skip iteration |
| `async` | Asynchronous modifier |
| `await` | Futures block wait |
| `parallel` | Multi-thread group execution |
| `end` | Closes block scopes |
| `new` | Instantiate class models |
| `self` | Instance reference |

---

## 📝 Comments

Single-line comments start with a `#` symbol:
```txs
# This is a comment
x = 10 # This is an inline comment
```

Multi-line comments are not supported by the syntax; repeat `#` on each line instead.
