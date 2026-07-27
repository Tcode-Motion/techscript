# Modules in TechScript

Modules provide namespaces to divide code across multiple files and scopes.

---

## 🏗️ Structure
Any TechScript file acts as a module. For example, if you have a file `utils.txs`:

```txs
# utils.txs
do add_tax(amount)
    send amount * 1.18
end
```

You can import and use it in another file in the same directory using `use`:

```txs
# main.txs
use utils

price = utils.add_tax(100)
say price # 118.0
```

---

## 🧬 Module Directory Structure
To organize complex modules, you can create a folder with the module's name containing an `index.txs` file:

```
project/
├── main.txs
└── math_helpers/
    ├── index.txs
    ├── algebra.txs
    └── trig.txs
```

In `math_helpers/index.txs`:
```txs
# Re-export submodules
use algebra
use trig
```

Now, `main.txs` can access everything inside via `use math_helpers`.
For imports details, see [Imports Guide](imports.md).
