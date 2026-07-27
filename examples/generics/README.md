# Generics Example

This example demonstrates how to implement parameterized functions and parameterized box classes using Generics in TechScript.

## Code (`generics.txs`)
```txs
# Generic identity function
do identity<T>(value: T)
    send value
end

# Generic box class
class Box<T>
    val = null

    do init(val: T)
        self.val = val
    end

    do unbox()
        send self.val
    end
end

# Inferred generic types
num_val = identity(42)
str_val = identity("TechScript")

say $"Number value: {num_val}"
say $"String value: {str_val}"

# Explicit generic class instantiation
int_box = new Box<Int>(100)
str_box = new Box<Str>("Generic Box")

say $"Int Box holds: {int_box.unbox()}"
say $"Str Box holds: {str_box.unbox()}"
```

## Running the Example
```bash
tech run generics.txs
```

## Expected Output
```
Number value: 42
String value: TechScript
Int Box holds: 100
Str Box holds: Generic Box
```
