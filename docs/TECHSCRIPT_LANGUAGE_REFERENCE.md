# TechScript 3.0 Language Reference

> **Official Authoritative Syntax & Specification**  
> Edition: TechScript 3.0 LTS | Host Target: Multi-Platform | File Extension: `.txs`  
> Master Document: [TechScript_Final_Syntax_and_Language_Master_Spec.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/docs/language/TechScript_Final_Syntax_and_Language_Master_Spec.md)

---

## 1. Introduction & Philosophy

TechScript is an expressive, memory-safe, general-purpose programming language designed for full-stack software engineering. It unifies general-purpose algorithmic programming, reactive web applications via **Webforge**, and cross-platform native mobile applications via **AppForge** under one concise, readable grammar.

---

## 2. Source Files & Lexical Grammar

### 2.1 File Extensions & Source Roles
- **`.txs`**: Canonical TechScript source code for all modules, scripts, applications, and frameworks.

### 2.2 Comments
```txs
# Single-line comment (canonical)
```

---

## 3. Variables & Declarations

### 3.1 Creating Variables
Variables are declared using `is`:
```txs
name is "TechScript"
count is 42
price is 19.99
active is true
```

### 3.2 Reassigning Variables
Variables are modified using `becomes`:
```txs
count becomes count + 1
```

### 3.3 Constants
Constants are defined with `constant`:
```txs
constant max_connections is 100
```

---

## 4. Functions & Procedures

Functions are declared with `to` and `:` with 4-space indentation:
```txs
to greet(person):
    return "Hello, {person}!"

to add(a, b):
    return a + b

say greet("Tanmoy")
say add(10, 20)
```

---

## 5. Control Flow

### 5.1 Conditionals (`if`, `else if`, `else`)
```txs
score is 85

if score >= 90:
    grade is "A"
else if score >= 80:
    grade is "B"
else:
    grade is "C"

say "Grade: {grade}"
```

### 5.2 Counted Repetition
```txs
repeat 3 times:
    say "Practise every day"
```

### 5.3 Sequence Iteration
```txs
items is ["apple", "banana", "cherry"]
for item in items:
    say item
```

### 5.4 While Loop
```txs
while count < 5:
    say count
    count becomes count + 1
```

---

## 6. Collections & Objects

```txs
person is {
    name: "Tanmoy",
    age: 20,
    skills: ["TechScript", "Rust", "AppForge"]
}

say person.name
say person.skills[0]
person.age becomes 21
```
