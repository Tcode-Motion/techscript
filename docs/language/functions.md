# Functions in TechScript

Functions are the core units of logic in TechScript.

---

## 🏗️ Declaration (`to name(params):`)
Functions are declared using the `to` keyword followed by the function name, parameters, a colon `:`, and an indented block:

```txs
to greet(name):
    say "Hello, {name}!"

to add(a, b):
    return a + b

result is add(10, 20)
say "Sum: {result}" # Sum: 30
```

---

## 🎨 Return Values (`return`)
Use `return` to send a value back to the caller:

```txs
to calculate_area(width, height):
    return width * height

area is calculate_area(5, 8)
say "Area: {area}"
```

---

## 🎨 Parameter Default Values
You can define parameters with fallback values:

```txs
to welcome(name, greeting = "Hello"):
    say "{greeting}, {name}!"

welcome("Alice")            # Hello, Alice!
welcome("Bob", "Hi there")  # Hi there, Bob!
```

---

## 🔁 Recursion
Functions can call themselves recursively:

```txs
to factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

say "Factorial(5): {factorial(5)}" # 120
```
