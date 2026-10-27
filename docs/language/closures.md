# Closures in TechScript

Closures are functions that capture and retain variables from their enclosing lexical scope.

---

## 🏗️ Capturing State
When a function is declared inside an outer function, the inner function retains access to the outer function's variables:

```txs
to create_counter(start_val):
    count is start_val
    
    to next_val():
        count becomes count + 1
        return count
        
    return next_val

counter is create_counter(10)
say "Count 1: {counter()}" # 11
say "Count 2: {counter()}" # 12
```

In this example, the variable `count` is captured and retained by `next_val` across invocations.

---

## 🧬 Capture Dynamics
Captured variables maintain their state across executions:

```txs
to build_multiplier(factor):
    to multiply(value):
        return value * factor
    return multiply

triple is build_multiplier(3)
say "Triple 10: {triple(10)}" # 30
```
