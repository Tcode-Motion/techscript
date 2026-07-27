# Types in TechScript

TechScript is dynamically typed. Variables do not have fixed types, but values do.

---

## 🟢 Primitive Types

### Int
Represents 64-bit signed integers:
```txs
count = 42
negative = -100
```

### Float
Represents 64-bit floating-point numbers:
```txs
pi = 3.14159
temp = -12.5
```

### Str
UTF-8 encoded string sequence:
```txs
message = "Welcome to TechScript"
```

### Bool
Logical values:
```txs
is_running = true
has_errors = false
```

### Null
Represents the absence of value:
```txs
data = null
```

---

## 🟡 Collection Types

### List
Ordered list of values (can contain mixed types):
```txs
items = [1, 2, "three", true]
```

### Map
Key-value storage (dictionary):
```txs
configs = {
    "host": "localhost",
    "port": 8080
}
```

---

## 🔴 Custom Types

### Class / Model
Objects instantiated using `class` and `new`:
```txs
class Dog
    do init(name)
        self.name = name
    end
end

my_dog = new Dog("Fido")
```
For more information, see [Classes](classes.md).
