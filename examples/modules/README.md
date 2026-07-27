# Modules Example

This example demonstrates how to split code into logical modules across multiple files and import them using the `use` keyword in TechScript.

## Helper Module (`math_utils.txs`)
```txs
do square(x)
    send x * x
end

do cube(x)
    send x * x * x
end
```

## Main Entry Point (`main.txs`)
```txs
use math_utils

val = 5
say $"Value: {val}"
say $"Square: {math_utils.square(val)}"
say $"Cube: {math_utils.cube(val)}"
```

## Running the Example
```bash
tech run main.txs
```

## Expected Output
```
Value: 5
Square: 25
Cube: 125
```
