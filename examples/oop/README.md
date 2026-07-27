# OOP Example

This example demonstrates how class models, constructor initialization, method overriding, and inheritance work in TechScript.

## Code (`oop.txs`)
```txs
class Shape
    name = "Shape"

    do init(name)
        self.name = name
    end

    do describe()
        say $"This is a {self.name}."
    end
end

class Circle(Shape)
    radius = 0

    do init(radius)
        self.name = "Circle"
        self.radius = radius
    end

    # Override describe
    do describe()
        say $"This is a Circle with radius: {self.radius}."
    end
end

shape = new Shape("Polygon")
shape.describe()

circle = new Circle(5)
circle.describe()
```

## Running the Example
```bash
tech run oop.txs
```

## Expected Output
```
This is a Polygon.
This is a Circle with radius: 5.
```
