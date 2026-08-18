# Imports in TechScript

Imports bring external modules, built-ins, and local package files into the current scope.

---

## 🏗️ The `use` Keyword
Use the `use` keyword to import modules:

```txs
use math
use json
```

Imported modules are scoped as namespaces:
```txs
result = math.sqrt(16)
say result # 4.0
```

---

## 🧬 Local vs Standard Library Imports
* **Standard Library**: Looked up automatically by the compiler. Standard modules include `math`, `json`, `crypto`, `fs`, `os`, `random`, and `date`.
* **Local Files**: Imported relative to the current file.
  ```txs
  use helper # Looks for helper.txs in the current directory
  ```

---

## 🎨 Alias Imports
You can rename imports to avoid name collisions:

```txs
use math as m
say m.abs(-5) # 5
```
For packaging structure details, see [Packages](packages.md).
