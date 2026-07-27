# Memory Model in TechScript

TechScript employs a hybrid memory model designed for speed, safety, and zero-overhead.

---

## 🏗️ Stack and Heap Division
* **Stack**: Holds primitive values (integers, floats, booleans, null) and variable bindings. Allocation and deallocation are instantaneous as function scopes push and pop.
* **Heap**: Holds dynamic structures (strings, lists, maps, custom class instances). Allocations return a reference handle stored on the stack.

---

## 🧬 Memory Safety Guarantees
TechScript inherits its memory safety guarantees from its Rust runtime core:
1. **No Data Races**: Concurrent data access is governed by the VM, preventing race conditions.
2. **Bounds Checking**: Every list indexing operation is validated against list dimensions at runtime to prevent buffer overflows.
3. **No Null Pointer Exceptions**: The language does not expose raw pointers; null values are clean enum values in the VM stack.

---

## 🔁 Garbage Collection
Objects on the heap are tracked and freed using Automatic Reference Counting (ARC) backed by a cycle-detecting mark-and-sweep Garbage Collector (GC).
* **Reference Counting**: Objects are deleted immediately when their reference count drops to zero.
* **Cycle Detector**: Periodically runs in the background to identify and clean up cyclic reference structures (e.g. object A referencing B, which references A).
See [Ownership Guide](ownership.md) for data lifecycle details.
