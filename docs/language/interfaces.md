# Interfaces in TechScript

Interfaces define strict API contracts that implementing classes must satisfy.

---

## 🏗️ Declaring an Interface
Use the `interface` keyword followed by a colon `:` and indented signatures:

```txs
interface FileSystem:
    to read(path):
        pass

    to write(path, data):
        pass
```

---

## 🧬 Implementing an Interface
Classes declare compliance to interfaces using the `with` keyword:

```txs
class LocalFS with FileSystem:
    to read(path):
        return fs.read(path)

    to write(path, data):
        fs.write(path, data)
```

---

## ⚖️ Interfaces vs Traits
* **Interfaces** represent a contract: all declared methods must be implemented by the class.
* **Traits** allow shared method definitions with default implementations, serving as reusable mixins.
