# Testing Example

This example demonstrates how to write and execute unit tests utilizing the built-in test annotation and assertions.

## Code (`unit_test.txs`)
```txs
use test

say "Registering unit tests..."

test do test_math_assertions()
    result = 5 * 5
    test.assert_equals(result, 25)
end

test do test_logical_truth()
    test.assert(10 > 5)
end

say "Executing unit tests..."
test.run_all()
```

## Running the Example
```bash
tech run unit_test.txs
# or
tech test
```

## Expected Output
```
Registering unit tests...
Executing unit tests...
Running 2 tests...
[PASS] test_math_assertions
[PASS] test_logical_truth
Test result: 2 passed, 0 failed.
```
