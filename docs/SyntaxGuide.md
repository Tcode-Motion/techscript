# TechScript 2.0 Syntax Quick Reference

This guide provides a quick reference to the canonical syntax and grammar of TechScript 2.0.

## 1. Variables & Constants

Variables are declared on first assignment. No keywords like `let`, `var`, or `make` are used. Use `const` for immutables:

```txs
# Dynamic variables
name = "Alice"
age = 30

# Constants
const PI = 3.14159
```

## 2. Control Flow

TechScript 2.0 block structures use indentation and end with the `end` keyword. No braces `{}` are used.

### Conditionals (`when` / `else when` / `else`)

```txs
when score >= 90
    say "A"
else when score >= 80
    say "B"
else
    say "C"
end
```

### Counted Loops (`loop`)

Runs a block exactly N times:

```txs
loop 5
    say "Hello"
end
```

### Condition Loops (`repeat`)

Runs while a condition remains true:

```txs
count = 0
repeat count < 5
    say count
    count += 1
end
```

### Iterator Loops (`for`)

Iterates over lists, maps, or ranges:

```txs
for x in [1, 2, 3]
    say x
end

for i in 1..5
    say i
end
```

## 3. Functions

Declared using the `do` keyword and the `send` keyword to return values:

```txs
do add(a, b)
    send a + b
end

# Anonymous single-line lambda
double = do(x) -> x * 2
```

## 4. Classes

Declared using the `class` keyword:

```txs
class Person
    name = ""
    age = 0

    do init(n, a)
        self.name = n
        self.age = a
    end

    do greet()
        say $"Hello, my name is {self.name}."
    end
end

p = new Person("Bob", 25)
p.greet()
```

## 5. Structs & Enums

```txs
struct Point
    x: Int
    y: Int
end

enum Color
    Red
    Green
    Blue
end
```
