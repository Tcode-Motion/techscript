# Best Practices for TechScript Developers

This document highlights guidelines for writing clean, optimized, and secure TechScript code.

---

## 📐 1. Variable Assignments vs Constants
* Always prefer `const` over mutable variables for values that never change during program execution (e.g. settings, ports, limits).
* Constants help the semantic analyzer optimize register allocation in the stack frame.

```txs
# Good
const SERVER_PORT = 8080

# Bad (mutable variable unnecessarily)
server_port = 8080
```

---

## 🔀 2. Lexical Scoping and Variable Shadowing
Minimize shadowing of variables from outer scopes to improve code readability:

```txs
# Confusing
x = 10
do calc()
    x = 5 # Shadows outer x
    send x
end
```

Keep scope footprints narrow by declaring variables only when they are needed.

---

## 🔒 3. FFI Safety
* Use the `ffi` module only when necessary. FFI calls bypass the VM's memory manager, exposing programs to crashes.
* Wrap all raw FFI calls in clean helper functions that perform bounds checks on arguments.

---

## 🧱 4. Designing Classes and Traits
* Use **Traits** instead of inheritance when sharing behavior across disparate objects.
* Keep **Interfaces** behavior-free; they should only define contracts for classes to fulfill.
