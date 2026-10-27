# Classes in TechScript

TechScript provides support for class-based object-oriented programming to structure state and behaviors.

---

## 🏗️ Declaration
Use the `class` keyword followed by the class name, a colon `:`, and an indented block. Methods are declared using `to name(params):`. The constructor must be named `init`:

```txs
class Person:
    name is ""
    age is 0

    # Constructor
    to init(name, age):
        self.name becomes name
        self.age becomes age

    # Instance method
    to greet():
        say "Hello, my name is {self.name}."
```

---

## 🧬 Instantiation
Classes are instantiated using the `new` keyword:

```txs
# Instantiate class object
bob is new Person("Bob", 25)

# Call instance method
bob.greet() # Hello, my name is Bob.
```

---

## 🔒 Member Access Control
Properties and methods can be accessed directly using dot notation:

```txs
bob.age becomes 26
say "Age: {bob.age}" # Age: 26
```

To implement contracts and mixins, see [Interfaces](interfaces.md) and [Traits](traits.md).
