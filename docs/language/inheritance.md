# Class Inheritance in TechScript

Inheritance allows a class to inherit properties and methods from a parent class.

---

## 🏗️ Subclassing
To inherit from another class, specify the parent class name in parentheses after the subclass name:

```txs
class Animal
    name = ""

    do init(name)
        self.name = name
    end

    do sleep()
        say $"Self.name is sleeping..."
    end
end

# Dog inherits from Animal
class Dog(Animal)
    breed = ""

    do init(name, breed)
        # Call parent constructor
        self.name = name
        self.breed = breed
    end

    do bark()
        say "Woof!"
    end
end
```

---

## 🧬 Method Overriding
Subclasses can override methods defined on the parent class:

```txs
class Cat(Animal)
    do sleep()
        say "Cat is sleeping on the couch."
    end
end

my_cat = new Cat("Whiskers")
my_cat.sleep() # "Cat is sleeping on the couch."
```

---

## 🚫 Multiple Inheritance Prevention
TechScript supports only single inheritance (a class can inherit from only one parent class). To share behaviors across multiple distinct domains, implement multiple [Traits](traits.md) instead:

```txs
class Dog(Animal) with Speaker, Runner
```
