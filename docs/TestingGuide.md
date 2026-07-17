# TechScript 2.0 Testing Guide

Unit tests are written using `std.testing` and run via the `tsc test` command.

## Writing Tests

```techscript
import std.testing;

function test_addition() {
    let result = 1 + 1;
    std.testing.assert_eq(result, 2, "1 + 1 should equal 2");
}

test_addition();
```

## Assertions

- `std.testing.assert(condition, message)`
- `std.testing.assert_eq(actual, expected, message)`
- `std.testing.assert_ne(actual, expected, message)`

## Benchmarks

```techscript
import std.testing;

function my_heavy_loop() {
    let sum = 0;
    for let i = 0; i < 1000; i = i + 1 {
        sum = sum + i;
    }
}

std.testing.benchmark(my_heavy_loop, 5000);
```
