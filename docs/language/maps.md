# Maps in TechScript

Maps (also known as dictionaries or hash maps) are collections of key-value pairs.

---

## 🏗️ Initialization
Declare maps using curly braces:

```txs
user = {
    "name": "Alice",
    "age": 30,
    "is_active": true
}
```

---

## 🧬 Elements Access & Modifications

### Retrieval & Assignment
Access values using keys:
```txs
say user["name"] # "Alice"

user["age"] = 31 # Update value
user["role"] = "admin" # Insert new key
```

---

## 🔁 Iteration
Iterate over all keys:

```txs
for key in user
    say $"{key} is {user[key]}"
end
```

---

## 📏 Properties

### Length
Retrieve the number of key-value pairs in a map:
```txs
say len(user) # 4
```

### Containment
Check if a key exists in a map:
```txs
when "role" in user
    say "Role is defined!"
end
```
