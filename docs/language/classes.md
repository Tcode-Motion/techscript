# Classes in TechScript

TechScript provides support for class-based object-oriented programming to structure state and behaviors.

---

## 🏗️ Declaration
Use the `class` keyword to define a class. Methods are declared inside using the `do` keyword. The constructor must be named `init`:

```txs
class Person
    # Properties with default values
    name = ""
    age = 0

    # Constructor
    do init(name, age)
        self.name = name
        self.age = age
    end

    # Instance method
    do greet()
        say $"Hello, my name is {self.name}."
    end
end
```

---

## 🧬 Instantiation
Classes are instantiated using the `new` keyword:

```txs
# Instantiate class object
bob = new Person("Bob", 25)

# Call instance method
bob.greet() # Hello, my name is Bob.
```

---

## 🔒 Member Access Control
By default, all fields and methods on a class are public. Properties can be accessed and modified directly:

```txs
bob.age = 26
say bob.age # 26
```
To implement behavior-only structures, see [Interfaces](interfaces.md) and [Traits](traits.md).
