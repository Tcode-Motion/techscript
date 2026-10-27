# TECHSCRIPT CANONICAL LANGUAGE SPECIFICATION
**Edition:** TechScript 3.0 Canonical LTS  
**Status:** Authoritative Single Source of Truth  
**Target Runtimes:** Multi-Platform (Native VM, Tree-Walking Interpreter, LLVM Native, Android NDK)  
**Primary File Extension:** `.txs`  
**Master Document:** [TechScript_Final_Syntax_and_Language_Master_Spec.md](./TechScript_Final_Syntax_and_Language_Master_Spec.md)

---

## 1. Vision & Core Principles

TechScript is an independent, general-purpose programming language designed to be **ultra-short, intuitive, and readable by anyone who understands basic English**, while retaining the mathematical rigor, static guarantees, and execution power required for serious software engineering.

### 1.1 Non-Negotiable Tenets
1. **Plain-English Simplicity:** Eliminates artificial boilerplate (`let`, `var`, `fn`, `function`, `do`, `end`, semicolons, and curly braces for ordinary blocks).
2. **Indentation and Colons:** Code blocks are introduced with a colon (`:`) and delineated by 4-space indentation.
3. **Natural Assignment:** 
   - `name is "TechScript"` creates a variable.
   - `count becomes count + 1` mutates an existing variable.
   - `constant max is 5` creates an immutable constant.
4. **Natural Functions:** Declared with `to name(params):` and closed by indentation return.
5. **Direct Output:** Uses `say` for clean terminal output.
6. **Automatic String Interpolation:** Double-quoted strings automatically evaluate `{expression}` without prefixes like `$` or `f`.
7. **Clean Loops:** `repeat 3 times:`, `for item in list:`, `while condition:`.
8. **General-Purpose Architecture:** UI frameworks (AppForge, Webforge) and graphics are external libraries, never hardcoded compiler keywords.

---

## 2. Canonical Lexical Grammar & Tokens

### 2.1 Comments
```txs
# Standard line comment (canonical)
```

### 2.2 Reserved Words
`is`, `becomes`, `constant`, `to`, `return`, `if`, `else`, `repeat`, `times`, `for`, `in`, `while`, `stop`, `skip`, `and`, `or`, `not`, `true`, `false`, `nothing`, `use`, `as`, `try`, `catch`, `finally`, `raise`, `async`, `await`, `type`, `test`, `expect`, `say`, `ask`.

### 2.3 Normative Grammar Skeleton (EBNF)
```ebnf
program          = { statement, newline } ;
statement        = variable_decl | constant_decl | assignment
                 | function_decl | type_decl
                 | if_stmt | repeat_stmt | for_stmt | while_stmt
                 | return_stmt | use_stmt | try_stmt | raise_stmt
                 | say_stmt | ask_expr | expression_stmt | test_decl ;

variable_decl    = name, "is", expression ;
constant_decl    = "constant", name, [ ":", type_expr ], "is", expression ;
assignment       = assignable, "becomes", expression ;
assignable       = name, { ".", name | "[", expression, "]" } ;

function_decl    = "to", name, "(", [ parameters ], ")", [ "->", type_expr ], block ;
parameters       = parameter, { ",", parameter } ;
parameter        = name, [ ":", type_expr ], [ "=", expression ] ;
call             = expression, "(", [ arguments ], ")" ;
arguments        = argument, { ",", argument } ;
argument         = [ name, ":" ], expression ;
block_call       = call, block ;
block            = ":", newline, indent, { statement, newline }, dedent ;

if_stmt          = "if", expression, block,
                   { "else if", expression, block },
                   [ "else", block ] ;
repeat_stmt      = "repeat", expression, "times", block
                 | "repeat", name, "from", expression, "to", expression, block ;
for_stmt         = "for", name, "in", expression, block ;
while_stmt       = "while", expression, block ;
say_stmt         = "say", expression ;
return_stmt      = "return", [ expression ] ;

list_literal     = "[", [ expression, { ",", expression } ], "]" ;
object_literal   = "{", [ pair, { ",", pair } ], "}" ;
pair             = ( name | string ), ":", expression ;
interpolation    = "{", expression, [ ":", format_spec ], "}" ;
expression       = literal | name | list_literal | object_literal
                 | call | property_access | index_access
                 | unary_expr | binary_expr | lambda_expr | grouped_expr ;
```

---

## 3. Variables & Assignment

### 3.1 Creating Variables
```txs
name is "TechScript"
count is 0
active is true
nothing_here is nothing
```

### 3.2 Mutating Variables
```txs
count becomes count + 1
name becomes "AppForge"
active becomes false
```

### 3.3 Constants
```txs
constant app_name is "TechScript"
constant max_attempts is 5
```

---

## 4. Functions & Returns

### 4.1 Canonical Function Declaration
```txs
to greet(person):
    return "Hello, {person}!"

to add(first, second):
    return first + second

say greet("Tanmoy")
say add(2, 3)
```

### 4.2 Optional Type Annotations
```txs
to add(first: number, second: number) -> number:
    return first + second
```

---

## 5. Control Flow

### 5.1 Conditionals
```txs
if score >= 90:
    say "Excellent"
else if score >= 70:
    say "Good"
else:
    say "Keep practising"
```

### 5.2 Counted Loops
```txs
repeat 3 times:
    say "Hello again!"
```

### 5.3 Range Loops
```txs
repeat index from 1 to 5:
    say "Item {index}"
```

### 5.4 Collections Iteration
```txs
colors is ["cyan", "indigo", "violet"]
for color in colors:
    say color
```

### 5.5 While Loops
```txs
while count < 5:
    say count
    count becomes count + 1
```

---

## 6. Collections & Structured Data

### 6.1 Lists
```txs
items is ["first", "second", "third"]
say items[0]
items.add("fourth")
```

### 6.2 Objects / Maps
```txs
person is {
    name: "Tanmoy",
    age: 20,
    skills: ["TechScript", "Rust"]
}

say person.name
person.age becomes 21
```

---

## 7. Canonical Example Program
```txs
# Path: examples/hello_world/modern_syntax.txs

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
