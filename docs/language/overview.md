# TechScript 2.0 Language Guide

This guide introduces the core concepts and canonical syntax of TechScript 2.0.

---

## 📐 Philosophy
TechScript is designed around one core rule: **Write like a human, execute at native speeds.** Blocks are defined using a colon `:` followed by 4-space indentation.

---

## 📦 Variables & Constants
Variables are dynamically typed and initialized on first assignment using `is`:
```txs
name is "Alice"
age is 30
is_active is true
```

Variables are updated using `becomes`:
```txs
age becomes age + 1
```

Constants cannot be changed after declaration:
```txs
constant PI is 3.14159
```

---

## 🔀 Conditionals
Use standard `if`, `else if`, and `else` blocks:
```txs
score is 85

if score >= 90:
    say "A"
else if score >= 80:
    say "B"
else:
    say "F"
```

---

## 🔁 Loops
TechScript features clean loop structures:

### Counted Loop (`repeat N times:`)
Executes a block exactly N times:
```txs
repeat 5 times:
    say "Hello!"
```

### Condition Loop (`while`)
Executes while a condition remains true:
```txs
count is 0
while count < 5:
    count becomes count + 1
    say "Count: {count}"
```

### Iterator Loop (`for`)
Iterates over lists, maps, or collections:
```txs
names is ["Alice", "Bob"]
for name in names:
    say "Name: {name}"
```

---

## 🏗️ Functions
Functions are declared using `to name(params):` and return values using `return`:
```txs
to calculate_square(x):
    return x * x

result is calculate_square(4)
say "Result: {result}" # 16
```

---

## 🧱 Objects & Models
Classes are declared using the `class` keyword with colon and indentation:
```txs
class Dog:
    name is ""

    to init(name):
        self.name becomes name

    to speak():
        say "{self.name} says Woof!"

my_dog is new Dog("Fido")
my_dog.speak()
```
