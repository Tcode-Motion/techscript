# 05 — LANGUAGE

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Complete TechScript 2.0 language surface cheat sheet
> **Parent Link**: [ARCHITECTURE](./04_architecture.md)
> **Child Links**: [DECISIONS](./07_decisions.md) · [REPOSITORY](./08_repository.md)

---

## 1. Syntax Overview

TechScript 2.0 has an English-like scripting syntax:
- Uses lowercase keywords (`make`, `when`, `build`, `model`) instead of symbols.
- Statement separation uses newlines or semicolons.
- Lexically scoped variables.
- Dynamic typing.

---

## 2. Primitive Types

| Type | Literal Example | Description |
|---|---|---|
| `Int` | `42`, `1_000_000` | 64-bit signed int. Supports underscore separators. |
| `Float` | `3.14`, `-0.5` | 64-bit IEEE double float. |
| `Str` | `"Hello"`, `f"x = {x}"` | UTF-8 heap string. F-strings interpolate expressions. |
| `Bool` | `true`, `false` | Boolean states. |
| `None` | `none` | Unit state. Default function return. |
| `List` | `[1, "two", true]` | Heterogeneous dynamic array. |
| `Map` | `{"k": 1, "v": 2}` | Insertion-order preserved map. |

---

## 3. Variables & Assignment

Variables are declared with `make` (mutable) or `const` (constant):
```
make score = 10
const PI = 3.14159

score = 20        // OK
PI = 3.0          // Error E0302
```

---

## 4. Control Flow

### 4.1 Conditionals
```
when score > 90 {
    say "A"
} else when score > 80 {
    say "B"
} else {
    say "F"
}
```

### 4.2 Loops
```
each i in 1..10 { say i } // prints 1 to 9
repeat 5 { say "Hi" }     // executes 5 times
while x < 10 { x += 1 }   // while loop
```

---

## 5. Functions & Lambdas

Declared using `build`:
```
build multiply(a, b = 1) {
    return a * b
}

// Anonymous / Lambda
make double = build(x) { return x * 2 }
```

---

## 6. Objects (Models)

Classes are defined with `model`. Default field assignments use `make`. Methods are declared with `build`. `fun` is supported but deprecated:
```
model Point {
    make x = 0
    make y = 0
    
    build init(x, y) {
        self.x = x
        self.y = y
    }
    
    build distance() {
        return (self.x ** 2 + self.y ** 2) ** 0.5
    }
}
make pt = new Point(3, 4)
say pt.distance()
```

---

## 7. Error Handling

```
attempt {
    throw "Oops"
} catch err {
    say f"Error caught: {err}"
}
```

---

## 8. Built-in Core Functions

`say(val)`, `ask(prompt)`, `len(collection)`, `type_of(val)`, `to_int(val)`, `to_float(val)`, `to_str(val)`, `to_bool(val)`, `range(start, end)`, `exit(code)`, `assert(cond)`.
