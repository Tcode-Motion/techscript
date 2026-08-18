# Testing Framework

TechScript has a built-in unit testing framework to write and execute assertions directly in the language.

---

## 🏗️ Writing Tests
Test blocks are declared using the `test` modifier before functions:

```txs
# math_test.txs
use test

test do test_addition()
    result = 5 + 5
    test.assert_equals(result, 10)
end

test do test_failing_case()
    test.assert(true)
end
```

---

## 🧬 Assertion Functions
The standard `test` module provides several assertion utilities:
* `test.assert(condition)`: Panics if the condition is false.
* `test.assert_equals(val1, val2)`: Panics if the values are not equal.
* `test.assert_throws(func)`: Passes if executing `func` throws an exception.

---

## 🚀 Running Tests
Execute the tests using the CLI runner:
```bash
tech test
```

This searches the directory structure for all files matching `*_test.txs` or `tests/` directories, runs the tests, and reports output:
```
Running 2 tests...
[PASS] test_addition
[PASS] test_failing_case

Test result: 2 passed, 0 failed.
```
