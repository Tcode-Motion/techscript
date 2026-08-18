# Functions in TechScript

Functions are the core block units of logic inside TechScript programs.

---

## 🏗️ Declaration
Functions are declared using the `do` keyword and return values using the `send` keyword:

```txs
do greet(name)
    say $"Hello, {name}!"
end

do add(a, b)
    send a + b
end

result = add(10, 20)
say result # 30
```

---

## 🎨 Parameter Default Values
You can define parameters with fallback values:

```txs
do welcome(name, greeting = "Hello")
    say $"{greeting}, {name}!"
end

welcome("Alice")            # Hello, Alice!
welcome("Bob", "Hi there") # Hi there, Bob!
```

---

## 🧬 Anonymous & Lambda Functions
TechScript supports single-line lambda expressions:

```txs
double = do(x) -> x * 2
say double(5) # 10
```

And multi-line anonymous functions:
```txs
square = do(x)
    send x * x
end
say square(4) # 16
```
