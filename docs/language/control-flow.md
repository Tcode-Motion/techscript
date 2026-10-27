# Control Flow & Syntax Guide

This document defines the formal grammar, canonical constructs, and control flow in TechScript.

---

## 📐 Block Structures
TechScript blocks are defined by a colon `:` followed by 4-space indentation. No curly braces `{}` or closing keywords like `end` or `done` are used for ordinary control flow or functions.

```txs
to print_numbers(max_count):
    count is 1
    while count <= max_count:
        say "Number: {count}"
        count becomes count + 1

print_numbers(3)
```

---

## 🔀 Conditionals (`if`, `else if`, `else`)

Conditionals use standard readable expressions followed by a colon and indented body:

```txs
score is 85

if score >= 90:
    say "Grade: A"
else if score >= 80:
    say "Grade: B"
else if score >= 70:
    say "Grade: C"
else:
    say "Grade: Needs improvement"
```

---

## 🔁 Repetition Loops (`repeat N times:`)

For executing code a fixed number of times, TechScript provides the clean `repeat` construct:

```txs
repeat 3 times:
    say "Welcome to TechScript!"
```

---

## 🔄 Iteration (`for item in collection:`)

Iterate across arrays, strings, or collections:

```txs
fruits is ["Apple", "Banana", "Cherry"]

for fruit in fruits:
    say "Fruit: {fruit}"
```

---

## ⏳ While Loops (`while condition:`)

Execute continuously while a condition remains true:

```txs
active is true
attempts is 0

while active:
    attempts becomes attempts + 1
    if attempts >= 3:
        active becomes false
```

---

## 🔑 Canonical Keyword Summary

| Construct | Canonical Syntax | Prohibited Legacy |
| :--- | :--- | :--- |
| Variable Declaration | `name is "val"` | `let`, `var`, `make` |
| Variable Reassignment | `x becomes x + 1` | `:=`, `=` |
| Constant | `constant max is 100` | `const`, `val` |
| Function | `to name(param):` | `do`, `fn`, `def`, `func` |
| Return | `return val` | `send val`, `give val` |
| Output | `say "Message"` | `print()`, `println` |
| Conditional | `if`, `else if`, `else:` | `when`, `else when` |
| Counted Loop | `repeat N times:` | `loop N`, `repeat N end` |
| Delimiters | `:` + 4-space indent | `end`, `done`, `{}` |
