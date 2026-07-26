# TechScript 2.0 Language Guide

> **Status**: Authoritative User Documentation — FROZEN 2.0.0
> **Version**: 2.0.0 Stable

This guide introduces the TechScript 2.0 language from first principles.
TechScript reads like English and describes **what you want**, not how to build it.

---

## Table of Contents

1. [Philosophy](#1-philosophy)
2. [Hello World](#2-hello-world)
3. [Variables](#3-variables)
4. [Functions](#4-functions)
5. [Conditionals](#5-conditionals)
6. [Loops](#6-loops)
7. [Error Handling](#7-error-handling)
8. [Classes](#8-classes)
9. [Enums & Structs](#9-enums--structs)
10. [Pattern Matching](#10-pattern-matching)
11. [Modules](#11-modules)
12. [String Interpolation](#12-string-interpolation)
13. [Collections](#13-collections)
14. [Async & Parallel](#14-async--parallel)
15. [Built-in Functions](#15-built-in-functions)

---

## 1. Philosophy

TechScript is designed around one principle:

> **Describe what you want, not how to build it.**

```txs
# This is a complete program
say "Hello, World!"
```

Rules:
- No semicolons
- No curly braces
- Blocks end with `end`
- English words over symbols where possible

---

## 2. Hello World

```txs
say "Hello, World!"
```

---

## 3. Variables

Variables are created by assignment. No `let`, `var`, or `make` keyword needed.

```txs
name = "Boss"
age  = 30
pi   = 3.14159
active = true
nothing = null
```

Constants never change:

```txs
const MAX = 100
const APP_NAME = "TechScript"
```

Reassigning a constant is a compile error (`TSE0302`).

Compound assignment:

```txs
x = 10
x += 5     # x is now 15
x -= 3     # x is now 12
x *= 2     # x is now 24
x /= 4     # x is now 6
```

---

## 4. Functions

Functions are declared with `do` and return values with `send`:

```txs
do greet(name)
    say "Hello " + name
end

do add(a, b)
    send a + b
end

result = add(3, 7)
say result
```

Functions with default parameters:

```txs
do greet(name = "World")
    say "Hello " + name
end

greet()          # Hello World
greet("Boss")    # Hello Boss
```

Single-line lambda:

```txs
double = do(x) -> x * 2

say double(5)    # 10
```

Multi-line lambda:

```txs
square = do(x)
    send x * x
end

say square(4)    # 16
```

Async functions:

```txs
async do fetch(url)
    data = await http.get(url)
    send data
end
```

---

## 5. Conditionals

Use `when` / `else when` / `else`:

```txs
score = 85

when score >= 90
    say "A"
else when score >= 80
    say "B"
else when score >= 70
    say "C"
else
    say "F"
end
```

Inline style for single statements:

```txs
when x > 0
    say "positive"
end
```

---

## 6. Loops

### Count loop — `loop N`

```txs
loop 5
    say "Hello"
end
```

### Condition loop — `repeat`

```txs
count = 0

repeat count < 10
    count += 1
    say count
end
```

### For-each — `for x in y`

```txs
names = ["Alice", "Bob", "Carol"]

for name in names
    say name
end
```

### Range iteration

```txs
for i in 1..10
    say i
end

for i in 1..=10    # inclusive
    say i
end
```

### Loop control

```txs
for i in 1..100
    when i == 50
        break
    end
    when i % 2 == 0
        continue
    end
    say i
end
```

---

## 7. Error Handling

```txs
try
    data = file.read("config.json")
    say data
catch error
    say "Could not read file: " + error
end
```

Throwing errors:

```txs
do divide(a, b)
    when b == 0
        throw "Division by zero"
    end
    send a / b
end

try
    result = divide(10, 0)
catch error
    say error
end
```

---

## 8. Classes

```txs
class Animal

    name = ""
    sound = ""

    do init(name, sound)
        self.name  = name
        self.sound = sound
    end

    do speak()
        say self.name + " says " + self.sound
    end

end

dog = new Animal("Dog", "Woof")
dog.speak()
```

Inheritance:

```txs
class Dog(Animal)

    breed = ""

    do init(name, breed)
        self.name  = name
        self.sound = "Woof"
        self.breed = breed
    end

    do fetch()
        say self.name + " fetches the ball!"
    end

end

rex = new Dog("Rex", "Labrador")
rex.speak()
rex.fetch()
```

---

## 9. Enums & Structs

```txs
enum Direction
    North
    South
    East
    West
end

enum Status
    Ok    = 0
    Error = 1
    Wait  = 2
end
```

```txs
struct Point
    x
    y
end

struct Color
    r
    g
    b
end
```

---

## 10. Pattern Matching

```txs
status = "ok"

match status
case "ok"
    say "Success"
case "error"
    say "Failed"
case "pending"
    say "Waiting..."
default
    say "Unknown status"
end
```

---

## 11. Modules

Import modules with `use`:

```txs
use math
use json
use http
```

Call qualified module functions:

```txs
use math

x = math.abs(-42)
y = math.sqrt(25)
z = math.pow(2, 10)

say x    # 42
say y    # 5.0
say z    # 1024
```

Only built-in functions use implicit call style:

```txs
say "Hello"          # implicit — built-in
name = ask "Name? "  # implicit — built-in
path = env "PATH"    # implicit — built-in
text = file "readme" # implicit — built-in
```

---

## 12. String Interpolation

Use `$"..."` with `{expression}` placeholders:

```txs
name = "Boss"
age  = 30

say $"Hello, {name}! You are {age} years old."
```

Expressions inside `{}` are fully evaluated:

```txs
x = 6
say $"6 squared is {x * x}"
say $"PI is approximately {math.round(3.14159, 2)}"
```

---

## 13. Collections

### Lists

```txs
fruits = ["Apple", "Banana", "Cherry"]

say fruits[0]          # Apple
fruits.push("Date")
say len(fruits)        # 4

for fruit in fruits
    say fruit
end
```

### Maps (Dictionaries)

```txs
person = {
    "name": "Boss",
    "age":  30,
    "city": "TechCity"
}

say person["name"]
person["email"] = "boss@example.com"

for key in person
    say $"{key}: {person[key]}"
end
```

---

## 14. Async & Parallel

```txs
use http

async do fetch_data(url)
    response = await http.get(url)
    send response.body
end

result = await fetch_data("https://api.example.com/data")
say result
```

Parallel execution:

```txs
parallel
    task_a()
    task_b()
    task_c()
end
```

---

## 15. Built-in Functions

| Built-in | Syntax | Description |
|---|---|---|
| `say` | `say expression` | Print to stdout |
| `ask` | `ask "prompt"` | Read from stdin |
| `env` | `env "VAR_NAME"` | Read environment variable |
| `file` | `file "path"` | Read file as string |
| `len` | `len(collection)` | Length of string/list/map |
| `typeof` | `typeof(value)` | Type name as string |
| `assert` | `assert(condition)` | Assert true or panic |
| `panic` | `panic "message"` | Halt with error |
| `exit` | `exit(code)` | Exit with code |
| `sleep` | `sleep(ms)` | Sleep N milliseconds |
| `json` | `json(string)` | Parse JSON string to map |
| `time` | `time()` | Current Unix timestamp |

**Examples:**

```txs
say "Hello, World!"

name = ask "What is your name? "
say $"Hello, {name}!"

path = env "PATH"
say path

text = file "readme.txt"
say text

count = len([1, 2, 3, 4, 5])
say count    # 5

t = typeof 42
say t        # int

assert len(name) > 0
```
