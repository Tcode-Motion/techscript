# Closures in TechScript

Closures are functions that capture variables from their enclosing lexical scope.

---

## 🏗️ Capturing State
When you declare a function inside another function, the inner function has access to the outer function's parameters and local variables:

```txs
do create_counter(start_val)
    count = start_val
    
    # Return a closure
    send do()
        count += 1
        send count
    end
end

counter = create_counter(10)
say counter() # 11
say counter() # 12
```

In this example, the variable `count` is captured and bound to the returned anonymous function even after `create_counter` has finished executing.

---

## 🧬 Capture by Reference
Captured variables are captured by reference, meaning modifications are visible across multiple invocations:

```txs
do build_multiplier(factor)
    send do(value) -> value * factor
end

triple = build_multiplier(3)
say triple(10) # 30
```
