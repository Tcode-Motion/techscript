# Generics in TechScript

Generics allow functions, classes, and structs to be parameterized over types to maximize code reuse.

---

## 🏗️ Generic Functions
Generic parameters are placed inside angle brackets `<T>`:

```txs
do identity<T>(value: T)
    send value
end

say identity<Int>(42)
say identity<Str>("Hello")
```

The type parameter can also be inferred automatically by the compiler:
```txs
say identity(42) # type Int is inferred
```

---

## 🧬 Generic Classes
Classes can take generic parameters to store type-safe internal states:

```txs
class Box<T>
    value = null

    do init(val: T)
        self.value = val
    end

    do get()
        send self.value
    end
end

int_box = new Box<Int>(100)
say int_box.get() # 100
```

---

## 🔒 Generic Constraints
You can limit what types can be passed to generics by using Traits:

```txs
do print_sound<T: Speaker>(obj: T)
    obj.speak()
end
```
See [Traits](traits.md) for more details.
