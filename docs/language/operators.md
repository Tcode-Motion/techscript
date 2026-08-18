# Operators in TechScript

TechScript features a standard set of mathematical, logical, and containment operators.

---

## 🧮 Arithmetic Operators

| Operator | Action | Example |
|:---:|:---|:---|
| `+` | Addition / Concatenation | `5 + 10` or `"a" + "b"` |
| `-` | Subtraction | `20 - 4` |
| `*` | Multiplication | `4 * 5` |
| `/` | Division | `10 / 2` |
| `%` | Modulo | `10 % 3` |

---

## ⚖️ Comparison Operators

All comparison operators evaluate to a boolean (`true` or `false`):

| Operator | Action | Example |
|:---:|:---|:---|
| `==` | Equals | `x == 10` |
| `!=` | Not Equals | `x != 10` |
| `<` | Less Than | `x < 20` |
| `<=` | Less Than or Equals | `x <= 20` |
| `>` | Greater Than | `x > 5` |
| `>=` | Greater Than or Equals | `x >= 5` |

---

## 🧠 Logical Operators

Used to combine or invert boolean values:

| Operator | Action | Example |
|:---:|:---|:---|
| `and` | Logical AND | `x > 0 and x < 10` |
| `or` | Logical OR | `x == 0 or x == 5` |
| `not` | Logical NOT | `not true` (evaluates to `false`) |

---

## 🐉 Membership & Type Operators

### `in`
Checks if a value is contained within a string, list, or map:
```txs
fruits = ["apple", "banana"]
when "apple" in fruits
    say "Found it!"
end

# Check substring
when "hello" in "hello world"
    say "Matches substring!"
end
```

### `typeof`
Queries the type name of an expression as a string:
```txs
say typeof 42          # "int"
say typeof "Hello"     # "str"
say typeof [1, 2, 3]   # "list"
```
