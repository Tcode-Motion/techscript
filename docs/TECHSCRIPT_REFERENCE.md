# TechScript Complete Keyword & Function Reference

> **200 Most Common Keywords, Built-in Functions, and Methods**
> Companion to [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md)

---

## How to Read This Reference

Each entry includes:
- **Name** and **category**
- **Syntax** — how to use it in code
- **Example** — a small working snippet
- **Explanation** — what it does
- **Internal implementation note** — how the interpreter handles it

---

## Category Index

| # | Category | Count | Range |
|---|----------|-------|-------|
| A | [Core Keywords](#a-core-keywords-1-25) | 25 | 1–25 |
| B | [Control Flow](#b-control-flow-26-50) | 25 | 26–50 |
| C | [Functions & Classes](#c-functions--classes-51-70) | 20 | 51–70 |
| D | [Output & Input](#d-output--input-71-80) | 10 | 71–80 |
| E | [String Functions](#e-string-functions-81-105) | 25 | 81–105 |
| F | [Numeric & Math](#f-numeric--math-106-130) | 25 | 106–130 |
| G | [List & Map Operations](#g-list--map-operations-131-165) | 35 | 131–165 |
| H | [File & IO](#h-file--io-166-180) | 15 | 166–180 |
| I | [Type & Conversion](#i-type--conversion-181-195) | 15 | 181–195 |
| J | [Misc & Advanced](#j-misc--advanced-196-200) | 5 | 196–200 |

---

## A. Core Keywords (1–25)

### 1. `say`
**Category:** Output
**Syntax:** `say <value> [, <value>, ...]`
```txs
say "Hello, World!"
say "Sum:", 2 + 3
```
**Explanation:** Prints values to stdout, separated by spaces, followed by a newline.
**Internal:** Evaluates each argument expression, converts to string via `to_str()`, joins with space, calls Python's `print()`.

---

### 2. `set`
**Category:** Declaration
**Syntax:** `set <name> = <value>`
```txs
set name = "Alice"
set age = 25
```
**Explanation:** Declares and assigns a variable. `set` is optional — `name = "Alice"` also works. `set` exists for beginner clarity.
**Internal:** Creates entry in current `Environment.vars` dict.

---

### 3. `ask`
**Category:** Input
**Syntax:** `ask <prompt>` or `? <prompt>`
```txs
name = ask "What is your name? "
age = ? "How old are you? "
```
**Explanation:** Displays prompt and waits for user input. Returns a string.
**Internal:** Calls Python's `input(prompt)`.

---

### 4. `if`
**Category:** Control Flow
**Syntax:** `if <condition>: <block>`
```txs
if score >= 90:
    say "Grade: A"
```
**Explanation:** Executes block if condition is truthy.
**Internal:** Evaluates condition, checks truthiness, enters block if true.

---

### 5. `elif`
**Category:** Control Flow
**Syntax:** `elif <condition>: <block>`
```txs
if x > 0:
    say "positive"
elif x < 0:
    say "negative"
```
**Explanation:** Alternative branch checked when previous `if`/`elif` was false.
**Internal:** Part of `IfStmt` AST node's `elif_clauses` list.

---

### 6. `else`
**Category:** Control Flow
**Syntax:** `else: <block>`
```txs
if logged_in:
    say "Welcome"
else:
    say "Please log in"
```
**Explanation:** Fallback block when all preceding conditions were false.
**Internal:** Part of `IfStmt` AST node's `else_body`.

---

### 7. `for`
**Category:** Loop
**Syntax:** `for <var> in <iterable>: <block>`
```txs
for fruit in ["apple", "banana", "cherry"]:
    say fruit
```
**Explanation:** Iterates over each element in a sequence.
**Internal:** Gets iterator from iterable, loops calling `next()`, assigns to loop variable.

---

### 8. `while`
**Category:** Loop
**Syntax:** `while <condition>: <block>`
```txs
count = 5
while count > 0:
    say count
    count -= 1
```
**Explanation:** Repeats block as long as condition is truthy.
**Internal:** Loop that re-evaluates condition before each iteration.

---

### 9. `in`
**Category:** Operator / Keyword
**Syntax:** `<value> in <collection>`
```txs
if "a" in "apple":
    say "found"
```
**Explanation:** Tests membership. Works with strings, lists, maps, and ranges.
**Internal:** Calls `__contains__` method on the collection type.

---

### 10. `and`
**Category:** Logical Operator
**Syntax:** `<expr> and <expr>`
```txs
if age >= 18 and has_id:
    say "Entry allowed"
```
**Explanation:** Short-circuit logical AND. Returns first falsy value or last value.
**Internal:** Evaluates left; if falsy, returns it without evaluating right.

---

### 11. `or`
**Category:** Logical Operator
**Syntax:** `<expr> or <expr>`
```txs
name = user_name or "Guest"
```
**Explanation:** Short-circuit logical OR. Returns first truthy value or last value.
**Internal:** Evaluates left; if truthy, returns it without evaluating right.

---

### 12. `not`
**Category:** Logical Operator
**Syntax:** `not <expr>`
```txs
if not found:
    say "Item missing"
```
**Explanation:** Logical negation. Returns `true` if operand is falsy, `false` otherwise.
**Internal:** Evaluates operand, returns boolean negation.

---

### 13. `true`
**Category:** Literal
**Syntax:** `true`
```txs
active = true
```
**Explanation:** Boolean true literal.
**Internal:** Maps to Python `True`.

---

### 14. `false`
**Category:** Literal
**Syntax:** `false`
```txs
done = false
```
**Explanation:** Boolean false literal.
**Internal:** Maps to Python `False`.

---

### 15. `none`
**Category:** Literal
**Syntax:** `none`
```txs
result = none
```
**Explanation:** Represents the absence of a value (like Python's `None`, JS's `null`).
**Internal:** Maps to Python `None`.

---

### 16. `is`
**Category:** Comparison
**Syntax:** `<value> is <type>`
```txs
if x is int:
    say "x is an integer"
```
**Explanation:** Type-checking operator. Checks if a value is of a given type.
**Internal:** Compares internal type tag of the value.

---

### 17. `import`
**Category:** Module
**Syntax:** `import <module>` or `import <module> as <alias>`
```txs
import math
say math.sqrt(16)
```
**Explanation:** Loads a module and makes its exports available.
**Internal:** Searches module path, parses & executes module file, returns its environment.

---

### 18. `from`
**Category:** Module
**Syntax:** `from <module> import <names>`
```txs
from math import sqrt, pi
say sqrt(pi)
```
**Explanation:** Imports specific names from a module into current scope.
**Internal:** Loads module, extracts specified names, binds them in current environment.

---

### 19. `as`
**Category:** Module / Error handling
**Syntax:** `import X as Y` or `catch Error as e`
```txs
import http_client as http
```
**Explanation:** Provides an alias for an imported module or caught error variable.
**Internal:** Binds the object to the alias name in the environment.

---

### 20. `export`
**Category:** Module
**Syntax:** `export <declaration>`
```txs
export fn greet(name):
    say f"Hello, {name}"
```
**Explanation:** Marks a function, class, or variable as public for importers.
**Internal:** Adds the declaration to the module's export table.

---

### 21. `global`
**Category:** Scope
**Syntax:** `global <name>`
```txs
count = 0
fn increment():
    global count
    count += 1
```
**Explanation:** Declares that a variable inside a function refers to the global scope.
**Internal:** Skips local environment, reads/writes directly to global environment.

---

### 22. `const`
**Category:** Declaration
**Syntax:** `const <name> = <value>`
```txs
const PI = 3.14159
const MAX = 100
```
**Explanation:** Declares an immutable variable. Reassignment raises an error.
**Internal:** Sets a `readonly` flag on the environment entry; `update()` checks this flag.

---

### 23. `mut`
**Category:** Declaration
**Syntax:** `mut <name> = <value>`
```txs
mut counter = 0
```
**Explanation:** Explicitly marks a variable as mutable (default behavior, but for clarity in strict mode).
**Internal:** Sets a `mutable` flag on the environment entry.

---

### 24. `del`
**Category:** Memory
**Syntax:** `del <name>`
```txs
del temporary_data
```
**Explanation:** Removes a variable from the current scope.
**Internal:** Deletes the entry from `Environment.vars`.

---

### 25. `pass`
**Category:** Placeholder
**Syntax:** `pass`
```txs
fn todo():
    pass
```
**Explanation:** Does nothing. Placeholder for empty blocks during development.
**Internal:** NOP — the interpreter skips it.

---

## B. Control Flow (26–50)

### 26. `break`
**Syntax:** `break`
```txs
for i in 1..100:
    if i == 10:
        break
```
**Explanation:** Exits the nearest enclosing loop immediately.
**Internal:** Raises a `BreakSignal` exception caught by the loop handler.

---

### 27. `skip`
**Syntax:** `skip`
```txs
for i in 1..10:
    if i % 2 == 0:
        skip
    say i
```
**Explanation:** Skips to the next iteration (equivalent to `continue` in other languages).
**Internal:** Raises a `SkipSignal` exception caught by the loop handler.

---

### 28. `return`
**Syntax:** `return [<value>]`
```txs
fn double(n):
    return n * 2
```
**Explanation:** Exits a function and optionally returns a value. Returns `none` if no value given.
**Internal:** Raises a `ReturnSignal(value)` exception caught by the function call handler.

---

### 29. `unless`
**Syntax:** `unless <condition>: <block>`
```txs
unless authorized:
    say "Access denied"
```
**Explanation:** Executes block if condition is falsy. Syntactic sugar for `if not`.
**Internal:** Parsed as `IfStmt` with negated condition.

---

### 30. `until`
**Syntax:** `until <condition>: <block>`
```txs
until found:
    found = search()
```
**Explanation:** Loops while condition is falsy. Sugar for `while not`.
**Internal:** Parsed as `WhileStmt` with negated condition.

---

### 31. `match`
**Syntax:** `match <expr>: case <pattern>: <block>`
```txs
match day:
    case "Mon":
        say "Monday"
    case "Fri":
        say "Friday!"
    case _:
        say "Other day"
```
**Explanation:** Pattern matching against multiple cases with wildcard `_` default.
**Internal:** Evaluates subject, sequentially checks each case pattern for equality.

---

### 32. `case`
**Syntax:** Used inside `match` blocks.
**Explanation:** Defines a single branch in a `match` statement.

---

### 33. `try`
**Syntax:** `try: <block> catch [var]: <block> [finally: <block>]`
```txs
try:
    data = read_file("config.txcfg")
catch err:
    say f"Error: {err}"
```
**Explanation:** Begins an error-handling block.
**Internal:** Wraps block execution in Python try/except.

---

### 34. `catch`
**Syntax:** `catch [<variable>]: <block>`
**Explanation:** Catches errors from the preceding `try` block.

---

### 35. `throw`
**Syntax:** `throw <error>`
```txs
throw Error("Invalid value")
throw TypeError("Expected int")
```
**Explanation:** Throws an error that can be caught by `catch`.
**Internal:** Raises a `TechScriptError` Python exception.

---

### 36. `finally`
**Syntax:** `finally: <block>`
```txs
try:
    process()
catch err:
    log(err)
finally:
    cleanup()
```
**Explanation:** Block that always runs after try/catch, regardless of errors.

---

### 37. `guard`
**Syntax:** `guard <condition> else: <block>`
```txs
fn divide(a, b):
    guard b != 0 else:
        throw Error("Division by zero")
    return a / b
```
**Explanation:** Early exit pattern. If condition is false, executes else block (must exit scope via return/throw).
**Internal:** Parsed as `if not condition: ...` with validation that else block exits.

---

### 38. `with`
**Syntax:** `with <expr> as <var>: <block>`
```txs
with open("data.txt") as f:
    content = f.read()
```
**Explanation:** Context manager. Ensures cleanup after block completes.
**Internal:** Calls `__enter__` on resource, executes block, calls `__exit__`.

---

### 39. `do`
**Syntax:** `do: <block>`
```txs
do:
    x = compute()
    say x
```
**Explanation:** Creates an explicit scope block. Variables declared inside don't leak out.
**Internal:** Creates a child `Environment`, executes block, then discards it.

---

### 40. `end`
**Syntax:** `end`
```txs
# Optional explicit block terminator (alternative to dedent)
if true:
    say "hello"
end
```
**Explanation:** Optional explicit block closer. Can be used instead of relying on dedent.

---

### 41. `each`
**Syntax:** `<collection>.each(<fn>)`
```txs
[1, 2, 3].each(n => say n)
```
**Explanation:** Iterates over collection, calling function for each element.
**Internal:** Method on list/map types that calls the function for each element.

---

### 42. `defer`
**Syntax:** `defer <expression>`
```txs
fn process():
    handle = open("file.txt")
    defer handle.close()
    # ... work with handle ...
    # handle.close() runs automatically when function exits
```
**Explanation:** Schedules an expression to execute when the current function/scope exits.
**Internal:** Adds expression to a deferred stack; pops and executes on function return.

---

### 43. `async`
**Syntax:** `async fn <name>(...): <block>`
```txs
async fn fetch_data(url):
    response = await http.get(url)
    return response.body
```
**Explanation:** Defines an asynchronous function that can use `await`.
**Internal:** Wraps function body in a coroutine/promise structure.

---

### 44. `await`
**Syntax:** `await <async_expr>`
```txs
data = await fetch_data("http://api.example.com")
```
**Explanation:** Waits for an async operation to complete and returns its result.
**Internal:** Suspends current coroutine until the promise resolves.

---

### 45. `yield`
**Syntax:** `yield <value>`
```txs
fn count_up(n):
    for i in 1..=n:
        yield i

for num in count_up(5):
    say num
```
**Explanation:** Produces a value from a generator function, pausing execution.
**Internal:** Converts function to generator; `yield` suspends and saves state.

---

### 46. `typeof`
**Syntax:** `typeof(<value>)`
```txs
say typeof(42)       # "int"
say typeof("hello")  # "str"
```
**Explanation:** Returns the type name of a value as a string.
**Internal:** Inspects the internal type tag of the TechScript value.

---

### 47. `has`
**Syntax:** `<map> has <key>` or `has(<collection>, <item>)`
```txs
if user has "email":
    say user["email"]
```
**Explanation:** Checks if a map contains a key or a list contains an element.
**Internal:** Calls `__contains__` method on the collection type.

---

### 48. `new`
**Syntax:** `new <ClassName>([args])`
```txs
dog = new Dog("Buddy")
```
**Explanation:** Creates a new instance of a class.
**Internal:** Allocates object, calls class's `init` method with arguments.

---

### 49. `self`
**Syntax:** Used inside class methods.
```txs
class Cat:
    fn init(self, name):
        self.name = name
```
**Explanation:** Reference to the current object instance within methods.
**Internal:** Automatically passed as first argument to bound methods.

---

### 50. `super`
**Syntax:** `super.<method>([args])`
```txs
class Dog(Animal):
    fn init(self, name):
        super.init(name, "Woof")
```
**Explanation:** Calls a method from the parent class.
**Internal:** Looks up method in parent class's environment and calls it.

---

## C. Functions & Classes (51–70)

### 51. `fn`
**Syntax:** `fn <name>(<params>): <block>`
```txs
fn add(a, b):
    return a + b
```
**Explanation:** Defines a named function.
**Internal:** Creates `FnStmt` AST node; at runtime, stores a `TechScriptFunction` object.

---

### 52. `class`
**Syntax:** `class <Name> [(<Parent>)]: <block>`
```txs
class Vehicle:
    fn init(self, speed):
        self.speed = speed
```
**Explanation:** Defines a class with methods and properties.
**Internal:** Creates `ClassStmt` AST node; at runtime, stores a `TechScriptClass` object.

---

### 53. `init` (method)
**Syntax:** `fn init(self, ...): <block>`
**Explanation:** Constructor method called by `new`. Initializes instance properties.
**Internal:** Called automatically after object allocation with `new`.

---

### 54. `to_str` (method)
**Syntax:** `fn to_str(self): return <string>`
**Explanation:** Defines how an object is converted to string (for `say`, f-strings, etc.).
**Internal:** Called by `to_str()` built-in and string interpolation.

---

### 55. `=>` (arrow/lambda)
**Syntax:** `(<params>) => <expression>`
```txs
square = (x) => x ** 2
add = (a, b) => a + b
```
**Explanation:** Creates an anonymous (lambda) function. Body is a single expression.
**Internal:** Creates `LambdaExpr` AST node; at runtime, stores closure.

---

### 56. `@` (decorator)
**Syntax:** `@<decorator_fn>`
```txs
@memoize
fn fibonacci(n):
    if n <= 1: return n
    return fibonacci(n-1) + fibonacci(n-2)
```
**Explanation:** Wraps a function with another function (decorator pattern).
**Internal:** Equivalent to `fibonacci = memoize(fibonacci)` after definition.

---

### 57–70. Built-in Higher-Order Functions

| # | Name | Syntax | Example | Description |
|---|------|--------|---------|-------------|
| 57 | `map` | `list.map(fn)` | `[1,2,3].map(x => x*2)` → `[2,4,6]` | Transforms each element |
| 58 | `filter` | `list.filter(fn)` | `[1,2,3,4].filter(x => x>2)` → `[3,4]` | Keeps elements where fn returns true |
| 59 | `reduce` | `list.reduce(fn, init)` | `[1,2,3].reduce((a,b) => a+b, 0)` → `6` | Accumulates values |
| 60 | `find` | `list.find(fn)` | `[1,2,3].find(x => x>1)` → `2` | First element matching predicate |
| 61 | `some` | `list.some(fn)` | `[1,2,3].some(x => x>2)` → `true` | True if any element matches |
| 62 | `every` | `list.every(fn)` | `[2,4,6].every(x => x%2==0)` → `true` | True if all elements match |
| 63 | `flat` | `list.flat()` | `[[1,2],[3,4]].flat()` → `[1,2,3,4]` | Flattens nested lists |
| 64 | `zip` | `zip(a, b)` | `zip([1,2],[3,4])` → `[[1,3],[2,4]]` | Pairs elements from two lists |
| 65 | `enumerate` | `enumerate(list)` | `enumerate(["a","b"])` → `[[0,"a"],[1,"b"]]` | Pairs each element with index |
| 66 | `sort` | `list.sort([fn])` | `[3,1,2].sort()` → `[1,2,3]` | Sorts list (optionally by key fn) |
| 67 | `reverse` | `list.reverse()` | `[1,2,3].reverse()` → `[3,2,1]` | Reverses list |
| 68 | `unique` | `list.unique()` | `[1,2,2,3].unique()` → `[1,2,3]` | Removes duplicates |
| 69 | `group_by` | `list.group_by(fn)` | `[1,2,3,4].group_by(x => x%2)` | Groups by key function |
| 70 | `chunk` | `list.chunk(n)` | `[1,2,3,4].chunk(2)` → `[[1,2],[3,4]]` | Splits into chunks of size n |

---

## D. Output & Input (71–80)

| # | Name | Syntax | Example | Description |
|---|------|--------|---------|-------------|
| 71 | `say` | `say <vals>` | `say "Hi"` | Print to stdout with newline |
| 72 | `ask` | `ask <prompt>` | `name = ask "Name? "` | Read string from stdin |
| 73 | `?` | `? <prompt>` | `name = ? "Name? "` | Shorthand for `ask` |
| 74 | `write` | `write(<val>)` | `write("no newline")` | Print without trailing newline |
| 75 | `debug` | `debug(<val>)` | `debug(my_list)` | Print with type info (for debugging) |
| 76 | `log` | `log(<msg>)` | `log("step done")` | Print timestamped log message |
| 77 | `warn` | `warn(<msg>)` | `warn("deprecated")` | Print warning to stderr |
| 78 | `error` | `error(<msg>)` | `error("failed")` | Print error to stderr (no throw) |
| 79 | `clear` | `clear()` | `clear()` | Clears the terminal screen |
| 80 | `format` | `format(<tmpl>, ...)` | `format("{} is {}", "x", 5)` | String formatting with placeholders |

---

## E. String Functions (81–105)

| # | Name | Syntax | Example | Result | Description |
|---|------|--------|---------|--------|-------------|
| 81 | `upper` | `s.upper()` | `"hi".upper()` | `"HI"` | Uppercase |
| 82 | `lower` | `s.lower()` | `"HI".lower()` | `"hi"` | Lowercase |
| 83 | `trim` | `s.trim()` | `" hi ".trim()` | `"hi"` | Strip whitespace |
| 84 | `trim_left` | `s.trim_left()` | `" hi".trim_left()` | `"hi"` | Strip leading whitespace |
| 85 | `trim_right` | `s.trim_right()` | `"hi ".trim_right()` | `"hi"` | Strip trailing whitespace |
| 86 | `split` | `s.split(sep)` | `"a,b".split(",")` | `["a","b"]` | Split into list |
| 87 | `join` | `sep.join(list)` | `",".join(["a","b"])` | `"a,b"` | Join list into string |
| 88 | `replace` | `s.replace(old,new)` | `"hi".replace("h","H")` | `"Hi"` | Replace substring |
| 89 | `contains` | `s.contains(sub)` | `"hello".contains("ell")` | `true` | Check substring |
| 90 | `starts_with` | `s.starts_with(pre)` | `"hello".starts_with("he")` | `true` | Check prefix |
| 91 | `ends_with` | `s.ends_with(suf)` | `"hello".ends_with("lo")` | `true` | Check suffix |
| 92 | `length` | `s.length` | `"hello".length` | `5` | String length (property) |
| 93 | `at` | `s.at(i)` | `"hello".at(0)` | `"h"` | Character at index |
| 94 | `slice` | `s.slice(a,b)` | `"hello".slice(1,3)` | `"el"` | Substring |
| 95 | `repeat` | `s.repeat(n)` | `"ha".repeat(3)` | `"hahaha"` | Repeat n times |
| 96 | `pad_left` | `s.pad_left(n,ch)` | `"5".pad_left(3,"0")` | `"005"` | Left pad |
| 97 | `pad_right` | `s.pad_right(n,ch)` | `"5".pad_right(3,"0")` | `"500"` | Right pad |
| 98 | `capitalize` | `s.capitalize()` | `"hello world".capitalize()` | `"Hello world"` | Capitalize first char |
| 99 | `title` | `s.title()` | `"hello world".title()` | `"Hello World"` | Title case |
| 100 | `chars` | `s.chars()` | `"abc".chars()` | `["a","b","c"]` | List of characters |
| 101 | `index_of` | `s.index_of(sub)` | `"hello".index_of("ll")` | `2` | Index of substring (-1 if not found) |
| 102 | `count` | `s.count(sub)` | `"hello".count("l")` | `2` | Count occurrences |
| 103 | `is_digit` | `s.is_digit()` | `"123".is_digit()` | `true` | All digits? |
| 104 | `is_alpha` | `s.is_alpha()` | `"abc".is_alpha()` | `true` | All alphabetic? |
| 105 | `match_regex` | `s.match_regex(pat)` | `"abc123".match_regex("[0-9]+")` | `"123"` | Regex match |

---

## F. Numeric & Math (106–130)

| # | Name | Syntax | Example | Result | Description |
|---|------|--------|---------|--------|-------------|
| 106 | `abs` | `abs(x)` | `abs(-5)` | `5` | Absolute value |
| 107 | `round` | `round(x, n)` | `round(3.146, 2)` | `3.15` | Round to n decimals |
| 108 | `ceil` | `ceil(x)` | `ceil(3.2)` | `4` | Round up |
| 109 | `floor` | `floor(x)` | `floor(3.8)` | `3` | Round down |
| 110 | `min` | `min(a, b, ...)` | `min(3, 1, 2)` | `1` | Minimum value |
| 111 | `max` | `max(a, b, ...)` | `max(3, 1, 2)` | `3` | Maximum value |
| 112 | `sum` | `sum(list)` | `sum([1,2,3])` | `6` | Sum of elements |
| 113 | `sqrt` | `sqrt(x)` | `sqrt(16)` | `4.0` | Square root |
| 114 | `pow` | `pow(x, y)` | `pow(2, 10)` | `1024` | Exponentiation |
| 115 | `log` | `math.log(x)` | `math.log(100)` | `4.605` | Natural logarithm |
| 116 | `log10` | `math.log10(x)` | `math.log10(100)` | `2.0` | Base-10 logarithm |
| 117 | `sin` | `math.sin(x)` | `math.sin(0)` | `0.0` | Sine (radians) |
| 118 | `cos` | `math.cos(x)` | `math.cos(0)` | `1.0` | Cosine (radians) |
| 119 | `tan` | `math.tan(x)` | `math.tan(0)` | `0.0` | Tangent (radians) |
| 120 | `pi` | `math.pi` | `math.pi` | `3.14159...` | Pi constant |
| 121 | `e` | `math.e` | `math.e` | `2.71828...` | Euler's number |
| 122 | `random` | `random()` | `random()` | `0.0..1.0` | Random float [0, 1) |
| 123 | `randint` | `randint(a, b)` | `randint(1, 6)` | `1..6` | Random integer [a, b] |
| 124 | `choice` | `choice(list)` | `choice(["a","b"])` | `"a"/"b"` | Random element |
| 125 | `shuffle` | `shuffle(list)` | `shuffle([1,2,3])` | randomized | Shuffle list in-place |
| 126 | `clamp` | `clamp(x, lo, hi)` | `clamp(15, 0, 10)` | `10` | Clamp to range |
| 127 | `sign` | `sign(x)` | `sign(-7)` | `-1` | Sign of number |
| 128 | `is_even` | `is_even(x)` | `is_even(4)` | `true` | Even check |
| 129 | `is_odd` | `is_odd(x)` | `is_odd(3)` | `true` | Odd check |
| 130 | `gcd` | `gcd(a, b)` | `gcd(12, 8)` | `4` | Greatest common divisor |

---

## G. List & Map Operations (131–165)

| # | Name | Syntax | Example | Result | Description |
|---|------|--------|---------|--------|-------------|
| 131 | `push` | `list.push(val)` | `[1,2].push(3)` | `[1,2,3]` | Append to end |
| 132 | `pop` | `list.pop()` | `[1,2,3].pop()` | `3` (list→`[1,2]`) | Remove & return last |
| 133 | `shift` | `list.shift()` | `[1,2,3].shift()` | `1` (list→`[2,3]`) | Remove & return first |
| 134 | `unshift` | `list.unshift(val)` | `[2,3].unshift(1)` | `[1,2,3]` | Prepend to start |
| 135 | `insert` | `list.insert(i,val)` | `[1,3].insert(1,2)` | `[1,2,3]` | Insert at index |
| 136 | `remove` | `list.remove(val)` | `[1,2,3].remove(2)` | `[1,3]` | Remove first occurrence |
| 137 | `remove_at` | `list.remove_at(i)` | `[1,2,3].remove_at(0)` | `[2,3]` | Remove at index |
| 138 | `index_of` | `list.index_of(val)` | `[10,20,30].index_of(20)` | `1` | Find index (-1 if missing) |
| 139 | `contains` | `list.contains(val)` | `[1,2,3].contains(2)` | `true` | Check membership |
| 140 | `length` | `list.length` | `[1,2,3].length` | `3` | Number of elements |
| 141 | `is_empty` | `list.is_empty()` | `[].is_empty()` | `true` | Check if empty |
| 142 | `first` | `list.first` | `[1,2,3].first` | `1` | First element |
| 143 | `last` | `list.last` | `[1,2,3].last` | `3` | Last element |
| 144 | `slice` | `list.slice(a,b)` | `[1,2,3,4].slice(1,3)` | `[2,3]` | Sublist |
| 145 | `copy` | `list.copy()` | `[1,2].copy()` | `[1,2]` (new) | Shallow copy |
| 146 | `clear` | `list.clear()` | `list.clear()` | `[]` | Remove all elements |
| 147 | `fill` | `list.fill(val)` | `[0,0,0].fill(1)` | `[1,1,1]` | Fill with value |
| 148 | `range` | `range(a,b,step)` | `range(0,10,2)` | `[0,2,4,6,8]` | Generate range list |
| 149 | `size` | `size(x)` | `size([1,2,3])` | `3` | Alias for `.length` |
| 150 | `flatten` | `list.flat()` | `[[1],[2,3]].flat()` | `[1,2,3]` | Flatten nested |
| 151 | `keys` | `map.keys()` | `{a:1,b:2}.keys()` | `["a","b"]` | Map keys |
| 152 | `values` | `map.values()` | `{a:1,b:2}.values()` | `[1,2]` | Map values |
| 153 | `entries` | `map.entries()` | `{a:1}.entries()` | `[["a",1]]` | Key-value pairs |
| 154 | `has_key` | `map.has_key(k)` | `{a:1}.has_key("a")` | `true` | Check key exists |
| 155 | `get` | `map.get(k, def)` | `{a:1}.get("b",0)` | `0` | Get with default |
| 156 | `set_key` | `map.set_key(k,v)` | `m.set_key("c",3)` | modified | Set key-value |
| 157 | `delete_key` | `map.delete_key(k)` | `m.delete_key("a")` | modified | Remove key |
| 158 | `merge` | `map.merge(other)` | `{a:1}.merge({b:2})` | `{a:1,b:2}` | Merge maps |
| 159 | `map_values` | `map.map_values(fn)` | `{a:1}.map_values(x=>x*2)` | `{a:2}` | Transform values |
| 160 | `filter_keys` | `map.filter_keys(fn)` | filter by key predicate | subset | Filter by key |
| 161 | `to_list` | `map.to_list()` | `{a:1}.to_list()` | `[["a",1]]` | Convert to pair list |
| 162 | `from_list` | `Map.from_list(pairs)` | `Map.from_list([["a",1]])` | `{a:1}` | Create from pairs |
| 163 | `count` | `list.count(val)` | `[1,2,1].count(1)` | `2` | Count occurrences |
| 164 | `take` | `list.take(n)` | `[1,2,3,4].take(2)` | `[1,2]` | First n elements |
| 165 | `drop` | `list.drop(n)` | `[1,2,3,4].drop(2)` | `[3,4]` | Skip first n elements |

---

## H. File & IO (166–180)

| # | Name | Syntax | Example | Description |
|---|------|--------|---------|-------------|
| 166 | `read_file` | `read_file(path)` | `data = read_file("in.txt")` | Read file as string |
| 167 | `write_file` | `write_file(path, data)` | `write_file("out.txt", text)` | Write string to file |
| 168 | `append_file` | `append_file(path, data)` | `append_file("log.txt", line)` | Append to file |
| 169 | `file_exists` | `file_exists(path)` | `if file_exists("cfg.txcfg")` | Check file exists |
| 170 | `delete_file` | `delete_file(path)` | `delete_file("temp.txt")` | Delete a file |
| 171 | `list_dir` | `list_dir(path)` | `files = list_dir("./")` | List directory contents |
| 172 | `make_dir` | `make_dir(path)` | `make_dir("output")` | Create directory |
| 173 | `open` | `open(path, mode)` | `f = open("data.txt", "r")` | Open file handle |
| 174 | `read_lines` | `read_lines(path)` | `lines = read_lines("f.txt")` | Read file as list of lines |
| 175 | `write_lines` | `write_lines(path, lines)` | `write_lines("f.txt", data)` | Write list of lines to file |
| 176 | `copy_file` | `copy_file(src, dst)` | `copy_file("a.txt","b.txt")` | Copy file |
| 177 | `move_file` | `move_file(src, dst)` | `move_file("old","new")` | Move/rename file |
| 178 | `file_size` | `file_size(path)` | `size = file_size("img.png")` | File size in bytes |
| 179 | `read_json` | `read_json(path)` | `cfg = read_json("c.json")` | Read & parse JSON file |
| 180 | `write_json` | `write_json(path, data)` | `write_json("out.json", obj)` | Write data as JSON |

---

## I. Type & Conversion (181–195)

| # | Name | Syntax | Example | Result | Description |
|---|------|--------|---------|--------|-------------|
| 181 | `to_int` | `to_int(val)` | `to_int("42")` | `42` | Convert to integer |
| 182 | `to_float` | `to_float(val)` | `to_float("3.14")` | `3.14` | Convert to float |
| 183 | `to_str` | `to_str(val)` | `to_str(42)` | `"42"` | Convert to string |
| 184 | `to_bool` | `to_bool(val)` | `to_bool(0)` | `false` | Convert to boolean |
| 185 | `to_list` | `to_list(val)` | `to_list("abc")` | `["a","b","c"]` | Convert to list |
| 186 | `to_map` | `to_map(pairs)` | `to_map([["a",1]])` | `{a:1}` | Convert pairs to map |
| 187 | `typeof` | `typeof(val)` | `typeof(42)` | `"int"` | Get type name |
| 188 | `is_int` | `is_int(val)` | `is_int(42)` | `true` | Check if integer |
| 189 | `is_float` | `is_float(val)` | `is_float(3.14)` | `true` | Check if float |
| 190 | `is_str` | `is_str(val)` | `is_str("hi")` | `true` | Check if string |
| 191 | `is_bool` | `is_bool(val)` | `is_bool(true)` | `true` | Check if boolean |
| 192 | `is_list` | `is_list(val)` | `is_list([1,2])` | `true` | Check if list |
| 193 | `is_map` | `is_map(val)` | `is_map({a:1})` | `true` | Check if map |
| 194 | `is_none` | `is_none(val)` | `is_none(none)` | `true` | Check if none |
| 195 | `is_fn` | `is_fn(val)` | `is_fn(add)` | `true` | Check if function |

---

## J. Misc & Advanced (196–200)

| # | Name | Syntax | Example | Description |
|---|------|--------|---------|-------------|
| 196 | `sleep` | `sleep(ms)` | `sleep(1000)` | Pause execution for ms milliseconds |
| 197 | `exit` | `exit([code])` | `exit(0)` | Terminate program with exit code |
| 198 | `assert` | `assert(cond, msg)` | `assert(x>0, "positive!")` | Throw error if condition false |
| 199 | `hash` | `hash(val)` | `hash("hello")` | Hash value (for maps, sets) |
| 200 | `print_env` | `print_env()` | `print_env()` | Debug: print all variables in scope |

---

*End of reference. See [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md) for the full language specification.*
