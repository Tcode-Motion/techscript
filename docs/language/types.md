# Types in TechScript

**Status: Implemented & Canonical**

TechScript is dynamically typed. Variables do not have fixed types, but values do.

---

## 🟢 Primitive Types

### Int
Represents 64-bit signed integers:
```txs
count is 42
negative is -100
```

### Float
Represents 64-bit floating-point numbers:
```txs
pi is 3.14159
temp is -12.5
```

### Str
UTF-8 encoded string sequence:
```txs
message is "Welcome to TechScript"
```

### Bool
Logical values:
```txs
is_running is true
has_errors is false
```

### Null
Represents the absence of value:
```txs
data is null
```

---

## 🟡 Collection Types

### List / Array
Ordered list of values (can contain mixed types):
```txs
items is [1, 2, "three", true]
```

### Map / Dictionary
Key-value storage:
```txs
configs is {
    "host": "localhost",
    "port": 8080
}
```

---

## 🔴 Custom Types

### Class
Objects instantiated using `class` and `new`:
```txs
class Dog:
    name is ""

    to init(name):
        self.name becomes name

my_dog is new Dog("Fido")
```
For more information, see [Classes](classes.md).
