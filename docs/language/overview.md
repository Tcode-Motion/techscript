# TechScript 2.0 Language Guide

This guide introduces the core concepts and syntax of TechScript 2.0.

---

## 📐 Philosophy
TechScript is designed around one core rule: **Write like a human, execute at native speeds.** Semicolons and curly braces are replaced by clean English-like keywords, and scopes are closed with the `end` keyword.

---

## 📦 Variables & Constants
Variables are dynamically typed and initialized on first assignment:
```txs
name = "Boss"
age = 30
is_active = true
```

Constants cannot be changed after declaration:
```txs
const PI = 3.14159
```

---

## 🔀 Conditionals
Use the `when` keyword for conditional branching:
```txs
score = 85

when score >= 90
    say "A"
else when score >= 80
    say "B"
else
    say "F"
end
```

---

## 🔁 Loops
TechScript features three types of loop structures:

### Counted Loop (`loop`)
Executes a block exactly N times:
```txs
loop 5
    say "Hello!"
end
```

### Condition Loop (`repeat`)
Executes while a condition remains true:
```txs
count = 0
repeat count < 5
    count += 1
    say count
end
```

### Iterator Loop (`for`)
Iterates over lists, maps, or ranges:
```txs
# Range iteration (inclusive)
for i in 1..=5
    say i
end

# List iteration
names = ["Alice", "Bob"]
for name in names
    say name
end
```

---

## 🏗️ Functions
Functions are declared using the `do` keyword and return values using the `send` keyword:
```txs
do calculate_square(x)
    send x * x
end

result = calculate_square(4)
say result # 16
```

---

## 🧱 Objects & Models
Classes are declared using the `class` keyword. Use the constructor method named `init` to initialize values:
```txs
class Dog
    name = ""

    do init(name)
        self.name = name
    end

    do speak()
        say $"{self.name} says Woof!"
    end
end

my_dog = new Dog("Fido")
my_dog.speak()
```
