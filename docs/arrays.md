# Lists (Arrays) in TechScript

Lists in TechScript are dynamic, ordered arrays of values.

---

## 🏗️ Initialization
Create lists using square brackets:

```txs
scores = [90, 85, 88]
mixed = [1, "two", true, null]
empty = []
```

---

## 🧬 Elements Access & Modifications

### Indexing
Access elements using zero-based index:
```txs
fruits = ["apple", "banana", "cherry"]
say fruits[0] # "apple"

fruits[1] = "blueberry"
say fruits[1] # "blueberry"
```

### Adding Elements
Add items to the end of a list:
```txs
numbers = [1, 2]
numbers.push(3)
say numbers # [1, 2, 3]
```

### Removing Elements
Remove items:
```txs
numbers.pop() # returns 3
```

---

## 🔁 Iteration
Iterate over list elements using a `for` loop:

```txs
for fruit in ["apple", "banana", "cherry"]
    say $"I like {fruit}"
end
```

---

## 📏 Length
Get list size:
```txs
size = len([10, 20, 30]) # 3
```
