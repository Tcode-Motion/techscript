# Virtual Machine (VM) Specification

The TechScript VM is a stack-based virtual machine written in Rust that executes optimized bytecode files (`.txc`).

---

## 🏗️ Core Architecture Components

### 1. Execution Stack
Stores runtime values. Binary math operators pop the top two values, execute calculations, and push results back.

### 2. Stack Frames
Each active function call pushes a new `StackFrame` holding:
* **Instruction Pointer (IP)**: Points to the next bytecode offset.
* **Locals Registers**: Array storing local variables and parameters.
* **Return Address**: Frame to jump back to after the function returns.

---

## 🧬 NaN-Boxed Value Representation
To maximize cache locality, TechScript values are represented using 64-bit floating-point numbers:
* **Float**: Encoded directly.
* **Int**: Packed in special non-standard NaN payload space.
* **Heap Pointer**: Pointer addresses for strings, lists, maps, and classes are boxed into the unused bits of NaN double-precision floats.

---

## 🔁 Garbage Collector (GC)
* **Automatic Reference Counting (ARC)**: Heap objects have an associated reference count. The VM immediately deallocates an object when its count falls to zero.
* **Cycle Detector**: Runs a periodic mark-and-sweep phase to resolve cyclic references (e.g. mutual self-references between objects).
