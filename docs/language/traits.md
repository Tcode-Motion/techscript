# Traits in TechScript

Traits define a set of methods that can be shared across multiple classes to enable polymorphism.

---

## 🏗️ Declaring a Trait
Use the `trait` keyword followed by a colon `:` and an indented block:

```txs
trait Speaker:
    # Method signature
    to speak():
        pass
```

---

## 🧬 Implementing a Trait
Classes implement traits using the `with` keyword during declaration:

```txs
class Dog with Speaker:
    to speak():
        say "Woof!"

class Cat with Speaker:
    to speak():
        say "Meow!"
```

---

## 🎨 Polymorphism
You can write functions that accept any object implementing a specific trait:

```txs
to make_sound(speaker):
    speaker.speak()

dog is new Dog()
cat is new Cat()

make_sound(dog) # Woof!
make_sound(cat) # Meow!
```

For contract checks without implementations, see [Interfaces](interfaces.md).
