# Error Handling Example

This example demonstrates how to raise exceptions with the `throw` keyword and recover from them using `try` / `catch` blocks in TechScript.

## Code (`errors.txs`)
```txs
do divide(a, b)
    when b == 0
        throw "Division by zero error!"
    end
    send a / b
end

try
    say "Attempting valid division..."
    res = divide(10, 2)
    say $"Result: {res}"
    
    say "Attempting invalid division..."
    res_err = divide(10, 0)
    say $"Result (should not print): {res_err}"
catch error
    say $"Caught expected error: {error}"
end

say "Program continues running cleanly after catch block."
```

## Running the Example
```bash
tech run errors.txs
```

## Expected Output
```
Attempting valid division...
Result: 5
Attempting invalid division...
Caught expected error: Division by zero error!
Program continues running cleanly after catch block.
```
