# Class Inheritance in TechScript

Inheritance allows a class to inherit properties and methods from a parent class.

---

## 🏗️ Subclassing
To inherit from another class, specify the parent class name in parentheses after the subclass name:

```txs
class Animal:
    name is ""

    to init(name):
        self.name becomes name

    to sleep():
        say "{self.name} is sleeping..."

# Dog inherits from Animal
class Dog(Animal):
    breed is ""

    to init(name, breed):
        self.name becomes name
        self.breed becomes breed

    to bark():
        say "Woof!"
```

---

## 🧬 Method Overriding
Subclasses can override methods defined on the parent class:

```txs
class Cat(Animal):
    to sleep():
        say "Cat is sleeping on the couch."

my_cat is new Cat("Whiskers")
my_cat.sleep() # "Cat is sleeping on the couch."
```

---

## 🚫 Multiple Inheritance Prevention
TechScript supports single inheritance. To share behaviors across multiple distinct types, implement [Traits](traits.md) or [Interfaces](interfaces.md).
