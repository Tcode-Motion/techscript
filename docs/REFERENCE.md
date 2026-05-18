# TechScript Language Reference (v1.0.6)

Canonical syntax first; legacy aliases still parse.

| Canonical | Alias |
|-----------|-------|
| `make x = 1` | (bare `x = 1` only for reassignment) |
| `const x = 1` | `keep x = 1` |
| `do fn()` | `build fn()` |
| `class Car` | `model Car` |
| `try` / `catch` / `throw` | `attempt` / `rescue` / `fail` |
| `loop` / `while` | `each` / `repeat` |
| `return x` | `send x` |

---

## 📤 Output & Input

```techscript
say "Hello"               # Print text
say "Hi", name, "!"       # Print multiple values
say f"Hello {name}!"      # F-string (insert variable)
make name = ask "Name? "  # Read input from user
```

## 📦 Variables

```techscript
make x = 10              # Create variable
keep PI = 3.14159        # Create constant (can't change)
make name = "Alice"
make items = [1, 2, 3]   # List
make info = {"age": 25}  # Map/Dictionary
```

## 🔀 Conditions

```techscript
when x > 10 {
    say "Big"
} or when x == 10 {
    say "Exactly 10"
} else {
    say "Small"
}
```

## 🔁 Loops

```techscript
each item in [1, 2, 3] {
    say item
}

each i in 1..10 {    # Range 1 to 9
    say i
}

repeat x > 0 {       # While loop
    x -= 1
}
```

## 🔧 Functions

```techscript
build greet(name, greeting = "Hello") {
    say f"{greeting}, {name}!"
}

greet("Alice")             # Hello, Alice!
greet("Bob", "Hi")         # Hi, Bob!
```

## 🏗️ Classes

```techscript
model Dog {
    build init(self, name) {
        self.name = name
    }
    build speak(self) {
        say f"{self.name} says Woof!"
    }
}

make rex = Dog("Rex")
rex.speak()
```

## ⚠️ Error Handling

```techscript
attempt {
    # risky code here
} catch err {
    say f"Error: {err.message}"
}
```

## 🌐 Web (use web)

```techscript
use web
make page = WebPage("Title")

page.style("body", { "background": "#111" })
page.script("function hello() { alert('Hi!'); }")

page.body([
    page.h1("My Page"),
    page.p("Some text"),
    page.button("Click", { "onclick": "hello()" })
])

page.run()           # Starts browser instantly!
```

## 📐 Built-in Functions

| Function | What it does |
|---|---|
| `len(x)` | Length of string/list |
| `range(n)` | List 0 to n-1 |
| `type(x)` | Get type of value |
| `int(x)` | Convert to integer |
| `str(x)` | Convert to string |
| `float(x)` | Convert to decimal |
| `abs(x)` | Absolute value |
| `round(x, n)` | Round to n places |
| `max(a, b)` | Maximum value |
| `min(a, b)` | Minimum value |

## 🔤 String Methods

```techscript
"hello".upper()       # "HELLO"
"HELLO".lower()       # "hello"
"hello world".split() # ["hello", "world"]
"  hi  ".trim()       # "hi"
"hello".replace("l", "r") # "herro"
"hello".contains("ell")   # true
```

## 📋 List Methods

```techscript
make nums = [3, 1, 2]
nums.append(4)
nums.sort()
nums.reverse()
nums.map((x) => x * 2)
nums.filter((x) => x > 2)
nums.length
```
