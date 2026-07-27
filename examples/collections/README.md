# Collections Example

This example demonstrates how to initialize, modify, and iterate over list and map collections in TechScript.

## Code (`collections.txs`)
```txs
# List modifications
fruits = ["apple", "banana"]
fruits.push("cherry")

say "--- List Iteration ---"
for fruit in fruits
    say fruit
end

# Map modifications
prices = {
    "apple": 50,
    "banana": 20
}
prices["cherry"] = 80

say "--- Map Iteration ---"
for item in prices
    say $"{item}: {prices[item]}"
end
```

## Running the Example
```bash
tech run collections.txs
```

## Expected Output
```
--- List Iteration ---
apple
banana
cherry
--- Map Iteration ---
apple: 50
banana: 20
cherry: 80
```
