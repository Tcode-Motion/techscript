# Ownership and Value Semantics

This document describes how TechScript variables store, copy, and pass data.

---

## 🏗️ Pass-by-Value vs Pass-by-Reference

### Primitives (Pass-by-Value)
Integers, floats, booleans, and null are copied directly when assigned or passed to a function:

```txs
x = 10
y = x # y gets a copy of 10
y = 20
say x # prints 10 (unchanged)
```

### Collections & Classes (Pass-by-Reference)
Lists, maps, and class instances are passed by reference. Assigning them to a new variable creates a alias:

```txs
list_a = [1, 2]
list_b = list_a # list_b references the same heap array

list_b.push(3)
say list_a # prints [1, 2, 3]!
```

---

## 🧬 Cloning Objects
If you need a distinct copy of a collection, use the `.clone()` method:

```txs
list_a = [1, 2]
list_b = list_a.clone() # creates a deep copy in the heap

list_b.push(3)
say list_a # prints [1, 2] (remains unaffected)
```

---

## 🔁 Variable Lifecycles
When a scope exits, all variable references inside that scope are dropped. If an object's total reference count drops to zero, its heap memory is immediately reclaimed by the VM allocator.
See [Memory Model](memory-model.md) for Garbage Collector mechanics.
