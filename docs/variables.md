# Variables in TechScript

Variables in TechScript are dynamically typed, and they are defined on their first assignment.

---

## 🏷️ Assignment
No keywords like `var`, `let`, or `make` are used. Simply assign a value to an identifier:

```txs
name = "Alice"
age = 30
is_active = true
```

If you attempt to access an identifier before assigning to it, the compiler will trigger an undefined variable error (`TSE0300`):
```txs
say undefined_var # Compile error!
```

---

## 🔄 Reassignment
Variables can be reassigned to values of any type:

```txs
x = 42
say x      # 42

x = "Now a string"
say x      # Now a string
```

---

## 🧬 Scoping Rules
TechScript uses lexical scoping. Variables defined inside a block (such as a function, loop, or conditional) are not accessible outside that block:

```txs
do process()
    local_val = 100
    say local_val
end

process()
say local_val # Compile error! (local_val is out of scope)
```

Inner blocks can shadow variables from outer scopes:
```txs
x = 5
when true
    x = 10 # shadows outer x
    say x  # 10
end
say x      # 5
```
