# Strings in TechScript

Strings in TechScript are UTF-8 encoded, dynamic sequences of characters.

---

## 📝 Literals
String literals are enclosed in double quotes:

```txs
greeting = "Hello, World!"
```

---

## ⚡ String Interpolation
To insert variables or expressions directly into strings, prepend a `$` character to create an interpolated string:

```txs
name = "Boss"
age = 30
message = $"Hello {name}, you are {age} years old."
say message
```

Expressions inside the curly brackets `{}` are fully evaluated at runtime:
```txs
say $"1 + 1 is {1 + 1}"
```

---

## 🧬 String Operations

### Concatenation
Use the `+` operator to join strings:
```txs
full_name = "Tech" + "Script"
```

### Length
Retrieve character count:
```txs
say len("TechScript") # 10
```

### Containment
Check if a substring exists inside a string:
```txs
when "Script" in "TechScript"
    say "Matches!"
end
```
