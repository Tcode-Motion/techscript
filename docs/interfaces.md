# Interfaces in TechScript

Interfaces define strict API contracts that implementing classes must fully satisfy.

---

## 🏗️ Declaring an Interface
Use the `interface` keyword. Interfaces only contain method signatures and cannot hold properties or default method implementations:

```txs
interface FileSystem
    do read(path)
    do write(path, data)
end
```

---

## 🧬 Implementing an Interface
Classes declare compliance to interfaces using the `with` keyword. If a class fails to implement all methods specified in the interface, a compile-time error (`TSE0320`) is thrown:

```txs
class LocalFS with FileSystem
    do read(path)
        send fs.read(path)
    end

    do write(path, data)
        fs.write(path, data)
    end
end
```

---

## ⚖️ Interfaces vs Traits
* **Interfaces** represent a strict contract: all declared methods must be implemented by the class. No state or method definitions are allowed.
* **Traits** allow shared method definitions with default implementations, serving as reusable mixins.
