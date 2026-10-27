# Variables in TechScript

**Status: Implemented & Canonical**

Variables in TechScript are dynamically typed, strictly scoped, and follow the canonical declaration and reassignment grammar.

---

## 🏷️ Declaration (`is`)
TechScript uses `is` for declaring and binding a new variable:

```txs
name is "Alice"
age is 30
is_active is true
```

If you attempt to access an identifier before defining it, the compiler triggers an undefined variable diagnostic (`TSE0300`):
```txs
say undefined_var # Compile error!
```

---

## 🔒 Constants (`constant ... is`)
Immutable values are declared using `constant`:

```txs
constant max_attempts is 5
constant app_name is "TechScript"
```

---

## 🔄 Reassignment (`becomes`)
To change the value of an existing variable, use `becomes`:

```txs
count is 0
say "Initial: {count}"     # Initial: 0

count becomes count + 1
say "Updated: {count}"     # Updated: 1

x is 42
say "Number: {x}"

x becomes "Now a string"
say "Text: {x}"
```

---

## 🧬 Scoping Rules
TechScript uses lexical scoping with indentation blocks. Variables defined inside a function, loop, or conditional are not accessible outside that block:

```txs
to process():
    local_val is 100
    say "Inside: {local_val}"

process()
# say local_val  # Compile error: local_val is out of scope
```

Inner blocks can shadow variables from outer scopes:
```txs
x is 5
if true:
    x is 10 # Shadows outer x within block
    say "Block x: {x}" # 10

say "Outer x: {x}"     # 5
```
