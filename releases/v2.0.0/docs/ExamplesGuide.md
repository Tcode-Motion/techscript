# TechScript 2.0 Examples Guide

> **Status**: Authoritative Reference — 2.0.0 Stable
> **Last Updated**: 2026-07-26

TechScript 2.0 includes structured examples in the `examples/` directory to demonstrate
the canonical syntax, modules, and language features.

---

## 1. Core Language Examples

### Hello World (`examples/01_hello_world.txs`)
Demonstrates basic implicit execution and stdout printing.
```txs
# Standard Hello World
say "Hello, TechScript 2.0!"
```

### Variables and Scope (`examples/02_variables.txs`)
Demonstrates variable declaration (first-assignment) and constant declarations.
```txs
const PI = 3.14159265

# Implicit variable declarations
radius = 10
area = PI * radius * radius

say $"Area is {area}"
```

### Conditionals & Match (`examples/03_control.txs`)
Demonstrates `when`/`else when`/`else` conditionals and `match`/`case` switching.
```txs
score = 82

when score >= 90
    say "Grade: A"
else when score >= 80
    say "Grade: B"
else
    say "Grade: C"
end

# Pattern matching
status = "success"
match status
case "success"
    say "Operation succeeded"
case "error"
    say "Operation failed"
default
    say "Operation state unknown"
end
```

### Counted Loops & Iteration (`examples/04_loops.txs`)
Demonstrates counted `loop N`, conditional `repeat cond`, and `for x in y` ranges.
```txs
# Counted loop (executes exactly 5 times)
loop 5
    say "Hello from loop!"
end

# Conditional repeat (while)
count = 0
repeat count < 3
    count += 1
    say $"Count is {count}"
end

# List iteration
fruits = ["Apple", "Banana", "Cherry"]
for fruit in fruits
    say fruit
end

# Range iteration
for i in 1..=5
    say i
end
```

---

## 2. Advanced Language Examples

### Classes and Objects (`examples/05_classes.txs`)
Demonstrates type definition with `class`, constructor declaration with `do init()`, methods, and inheritance.
```txs
class Shape
    name = "Shape"

    do init(name)
        self.name = name
    end

    do describe()
        say $"This is a {self.name}."
    end
end

class Circle(Shape)
    radius = 0

    do init(radius)
        self.name = "Circle"
        self.radius = radius
    end

    do area()
        use math
        send math.pi * self.radius * self.radius
    end
end

c = new Circle(5)
c.describe()
say $"Area: {c.area()}"
```

---

## 3. Standard Library Integration Examples

See the `examples/compat/` folder for legacy test fixtures. They verify that older
1.x dialect syntaxes (such as `make`, `model`, `fun`, `{}`) compile correctly while
emitting compiler warnings.
