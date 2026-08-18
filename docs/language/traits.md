# Traits in TechScript

Traits define a set of methods that can be shared across multiple classes to enable polymorphism.

---

## 🏗️ Declaring a Trait
Use the `trait` keyword. Traits specify method signatures without providing an implementation, or they can optionally provide default implementations:

```txs
trait Speaker
    # Method signature (empty body)
    do speak()
    end
end
```

---

## 🧬 Implementing a Trait
Classes implement traits using the `with` keyword during declaration:

```txs
class Dog with Speaker
    do speak()
        say "Woof!"
    end
end

class Cat with Speaker
    do speak()
        say "Meow!"
    end
end
```

---

## 🎨 Polymorphism
You can write functions that accept any object implementing a specific trait:

```txs
do make_sound(speaker: Speaker)
    speaker.speak()
end

dog = new Dog()
cat = new Cat()

make_sound(dog) # Woof!
make_sound(cat) # Meow!
```
For contract checks without implementations, see [Interfaces](interfaces.md).
