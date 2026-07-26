# TechScript 2.0 Testing Guide

> **Status**: Frozen Specification — 2.0.0 Stable
> **Last Updated**: 2026-07-26

TechScript 2.0 provides testing utilities under the `testing` module. Unit tests
and benchmarks can be run directly using the `tsc test` command line runner.

---

## 1. Writing Unit Tests

Unit tests are standard functions. Use the language built-in `assert` or qualified
utilities from the `testing` module to verify correctness.

```txs
use testing

do test_addition()
    result = 1 + 1
    testing.assert_eq(result, 2)
end

do test_string_format()
    name = "Boss"
    greeting = $"Hello {name}"
    testing.assert_eq(greeting, "Hello Boss")
end

# Run the test functions directly if executed as scripts
test_addition()
test_string_format()
```

### Assertions

The following assertion functions are exported by the `testing` module:

- `testing.assert(condition, message = "")`: Asserts that the condition is true.
- `testing.assert_eq(actual, expected, message = "")`: Asserts that two values are equal.
- `testing.assert_ne(actual, expected, message = "")`: Asserts that two values are not equal.

---

## 2. Writing Benchmarks

Benchmarks measure execution duration over repeated iterations. Use `testing.benchmark`
to execute a test function a specified number of times:

```txs
use testing

do heavy_operation()
    sum = 0
    for i in 1..1000
        sum += i
    end
end

# Benchmark: execute heavy_operation 5000 times
duration = testing.benchmark(heavy_operation, 5000)
say $"5000 iterations took {duration}ms"
```

---

## 3. Running Tests via CLI

Run all tests in the current project workspace:

```bash
tsc test .
```

The runner automatically scans for functions prefixed with `test_`, executes them,
and reports pass/fail diagnostics.
