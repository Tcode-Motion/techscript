# TechScript Canonical Syntax Reference

**Specification Status:** Locked & Persistent (TechScript 2.0.2)  
**Governing Source:** `docs/language/TechScript_Final_Syntax_and_Language_Master_Spec.md`

---

## 1. Variable Binding & Mutation

### Variable Declaration (`is`)
TechScript uses `is` for declaring and binding a new identifier to a value:
```txs
count is 0
name is "TechScript"
is_running is true
rates is [0.05, 0.10, 0.15]
```
*Note: `let`, `var`, `make`, `val`, `def`, and `=` are strictly prohibited.*

### Variable Reassignment (`becomes`)
To mutate an existing variable in the current scope, use `becomes`:
```txs
count becomes count + 1
name becomes "TechScript 2.0"
```
*Note: `:=` and `=` for reassignment are syntax errors.*

### Constant Declaration (`constant ... is`)
Immutable identifiers must be declared with `constant`:
```txs
constant max_connections is 100
constant pi is 3.14159
```

---

## 2. Functions & Subroutines

Functions are declared with `to name(params):` followed by a colon and 4-space indentation:
```txs
to calculate_area(width, height):
    if width <= 0 or height <= 0:
        return 0
    return width * height

to greet(person):
    say "Hello, {person}!"
    return "done"
```

### Return Values
Use `return` to pass values back to callers:
```txs
return result
```

---

## 3. String Interpolation

TechScript strings use double quotes `"..."`. Embedded expressions inside curly braces `{...}` automatically interpolate without requiring prefix markers like `$` or `f`:
```txs
user is "Alice"
score is 95
say "Player {user} scored {score} points!"
```

To include literal curly braces in strings (e.g. JSON strings), escape them with `\{` and `\}`:
```txs
json_str is "\{\"status\": \"ok\"\}"
```

---

## 4. Control Flow

### Conditionals (`if`, `else if`, `else:`)
```txs
if score >= 90:
    say "Grade: A"
else if score >= 80:
    say "Grade: B"
else:
    say "Grade: Retake"
```

### Counted Loops (`repeat N times:`)
```txs
repeat 5 times:
    say "Executing task..."
```

### Iteration Loops (`for item in collection:`)
```txs
languages is ["TechScript", "Rust", "TypeScript"]
for lang in languages:
    say "Language: {lang}"
```

### While Loops (`while condition:`)
```txs
active is true
attempts is 0

while active:
    attempts becomes attempts + 1
    if attempts >= 3:
        active becomes false
```

---

## 5. Classes & Models

Classes are declared with `class Name:` and constructor `to init(...):`:
```txs
class Vector2:
    x is 0.0
    y is 0.0

    to init(x, y):
        self.x becomes x
        self.y becomes y

    to magnitude():
        return math.sqrt(self.x * self.x + self.y * self.y)

vec is new Vector2(3.0, 4.0)
say "Magnitude: {vec.magnitude()}" # 5.0
```

---

## 6. Error Handling

```txs
try:
    content is files.read("config.json")
catch err:
    say "Failed to load config: {err}"
```
