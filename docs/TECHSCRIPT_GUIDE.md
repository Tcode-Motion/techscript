# TechScript — Getting Started Guide

> **Learn TechScript from Zero — For Complete Beginners**
> Companion to [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md)

---

## 1. Installation

### 1.1 Prerequisites

- **Python 3.10+** must be installed on your system
- A terminal (Command Prompt, PowerShell, Terminal, or Bash)
- A text editor (VS Code recommended)

### 1.2 Install TechScript

```bash
# Option A: Install from pip (once published)
pip install techscript

# Option B: Install from source
git clone https://github.com/techscript-lang/techscript.git
cd techscript
pip install -e .
```

### 1.3 Verify Installation

```bash
tech version
# Output: TechScript v1.0.0
```

### 1.4 Editor Setup

For VS Code, install the TechScript extension for syntax highlighting:
```bash
tech editor-setup vscode
```

This creates a `.txs` syntax highlighting grammar at `~/.techscript/editors/vscode/`.

---

## 2. Your First Program

### 2.1 Hello World

Create a file called `hello.txs`:

```txs
say "Hello, World!"
```

Run it:

```bash
tech run hello.txs
```

Output:

```
Hello, World!
```

That's it. One line. No imports, no `main()`, no boilerplate.

### 2.2 Hello with Your Name

```txs
name = ask "What is your name? "
say f"Hello, {name}! Welcome to TechScript!"
```

```bash
tech run hello.txs
# What is your name? Alice
# Hello, Alice! Welcome to TechScript!
```

---

## 3. Variables

Variables are containers that hold values. No special keyword is needed (though `set` is available for beginners).

```txs
# Both of these work the same way:
set name = "Alice"
age = 25

# Multiple types
message = "hello"      # text (string)
count = 42             # whole number (integer)
price = 9.99           # decimal number (float)
active = true          # yes/no value (boolean)
nothing = none         # no value
```

### Variable Rules

- Names must start with a letter or underscore: `name`, `_secret`, `player1`
- Names are case-sensitive: `Name` and `name` are different
- Use descriptive names: `user_age` is better than `x`
- Constants use `const`: `const MAX_SCORE = 100`

---

## 4. Input & Output

### 4.1 Output with `say`

```txs
# Print text
say "Hello!"

# Print multiple values
say "Name:", name, "Age:", age

# Print with f-strings (formatted strings)
say f"I am {age} years old"
say f"2 + 2 = {2 + 2}"

# Print without newline
write("Loading...")
write("Done!")
# Output: Loading...Done!
```

### 4.2 Input with `ask` or `?`

```txs
# Full keyword
name = ask "Enter your name: "

# Short symbol
name = ? "Enter your name: "

# Input always returns a string — convert if needed
age_text = ask "Enter your age: "
age = to_int(age_text)

# One-liner with pipe
age = ask "Enter your age: " |> to_int
```

---

## 5. Operators

### 5.1 Math

```txs
say 10 + 3     # 13  (addition)
say 10 - 3     # 7   (subtraction)
say 10 * 3     # 30  (multiplication)
say 10 / 3     # 3.333...  (division)
say 10 // 3    # 3   (integer division)
say 10 % 3     # 1   (remainder)
say 2 ** 10    # 1024 (power)
```

### 5.2 Comparison

```txs
say 5 == 5    # true  (equal)
say 5 != 3    # true  (not equal)
say 5 > 3     # true  (greater than)
say 5 < 3     # false (less than)
say 5 >= 5    # true  (greater or equal)
say 5 <= 3    # false (less or equal)
```

### 5.3 Logic

```txs
say true and false   # false
say true or false    # true
say not true         # false
```

### 5.4 Shorthand Assignment

```txs
x = 10
x += 5    # x is now 15
x -= 3    # x is now 12
x *= 2    # x is now 24
x /= 4   # x is now 6.0
```

---

## 6. Conditions

### 6.1 If / Elif / Else

```txs
age = ask "How old are you? " |> to_int

if age >= 18:
    say "You can vote!"
elif age >= 16:
    say "Almost there!"
else:
    say "Too young to vote."
```

### 6.2 Inline Condition

```txs
status = "adult" if age >= 18 else "minor"
say f"You are a {status}"
```

### 6.3 Unless (Opposite of If)

```txs
unless logged_in:
    say "Please log in first!"
```

### 6.4 Match (Switch-like)

```txs
command = ask "Enter command: "

match command:
    case "hello":
        say "Hi there!"
    case "bye":
        say "Goodbye!"
    case "help":
        say "Available: hello, bye, help"
    case _:
        say f"Unknown command: {command}"
```

---

## 7. Loops

### 7.1 For Loop

```txs
# Loop through a range of numbers
for i in 1..=5:
    say i
# Output: 1 2 3 4 5 (each on its own line)

# Loop through a list
fruits = ["apple", "banana", "cherry"]
for fruit in fruits:
    say f"I like {fruit}"
```

### 7.2 While Loop

```txs
count = 5
while count > 0:
    say f"Countdown: {count}"
    count -= 1
say "Liftoff!"
```

### 7.3 Until Loop

```txs
password = ""
until password == "secret123":
    password = ask "Enter password: "
say "Access granted!"
```

### 7.4 Loop Control

```txs
# Skip even numbers
for i in 1..=10:
    if i % 2 == 0:
        skip          # 'skip' = continue in other languages
    say i

# Stop early
for i in 1..100:
    if i > 5:
        break
    say i
```

---

## 8. Functions

### 8.1 Defining Functions

```txs
fn greet(name):
    say f"Hello, {name}!"

greet("Alice")    # Hello, Alice!
greet("Bob")      # Hello, Bob!
```

### 8.2 Returning Values

```txs
fn add(a, b):
    return a + b

result = add(3, 4)
say result    # 7
```

### 8.3 Default Parameters

```txs
fn greet(name, greeting = "Hello"):
    say f"{greeting}, {name}!"

greet("Alice")              # Hello, Alice!
greet("Alice", "Namaste")   # Namaste, Alice!
```

### 8.4 Lambda (Short Functions)

```txs
double = (x) => x * 2
say double(5)    # 10

# Use with map/filter
nums = [1, 2, 3, 4, 5]
evens = nums.filter(n => n % 2 == 0)
say evens    # [2, 4]
```

---

## 9. Lists

Lists are ordered collections of values.

```txs
# Create a list
fruits = ["apple", "banana", "cherry"]

# Access elements (0-indexed)
say fruits[0]      # apple
say fruits[-1]     # cherry (last element)

# Modify
fruits.push("date")            # Add to end
fruits[1] = "blueberry"        # Change element
removed = fruits.pop()         # Remove & return last

# List info
say fruits.length              # Number of items
say fruits.contains("apple")   # true

# Useful operations
nums = [3, 1, 4, 1, 5, 9]
say nums.sort()          # [1, 1, 3, 4, 5, 9]
say nums.reverse()       # [9, 5, 4, 1, 3, 1]
say nums.unique()        # [3, 1, 4, 5, 9]
say nums.slice(1, 3)     # [1, 4]

# List comprehension
squares = [x ** 2 for x in 1..=5]
say squares    # [1, 4, 9, 16, 25]
```

---

## 10. Maps (Dictionaries)

Maps store key-value pairs.

```txs
# Create a map
person = {
    name: "Alice",
    age: 25,
    city: "Kolkata"
}

# Access values
say person["name"]       # Alice
say person.name          # Alice (dot notation)

# Modify
person["email"] = "alice@example.com"
person.age = 26

# Check keys
if person has "email":
    say person.email

# Iterate
for key in person.keys():
    say f"{key}: {person[key]}"

# Useful operations
say person.keys()       # ["name", "age", "city", "email"]
say person.values()     # ["Alice", 26, "Kolkata", "alice@example.com"]
```

---

## 11. File Operations

### 11.1 Reading Files

```txs
# Read entire file as string
content = read_file("data.txt")
say content

# Read as list of lines
lines = read_lines("data.txt")
for line in lines:
    say line
```

### 11.2 Writing Files

```txs
# Write string to file (creates or overwrites)
write_file("output.txt", "Hello, File!")

# Append to file
append_file("log.txt", f"Log entry at {time.now()}\n")

# Write list of lines
write_lines("output.txt", ["line 1", "line 2", "line 3"])
```

### 11.3 File Checks

```txs
if file_exists("config.txcfg"):
    config = read_file("config.txcfg")
else:
    say "Config not found, using defaults"
```

### 11.4 JSON Files

```txs
# Read JSON
data = read_json("users.json")
say data[0]["name"]

# Write JSON
users = [{name: "Alice", age: 25}, {name: "Bob", age: 30}]
write_json("users.json", users)
```

---

## 12. Error Handling

```txs
# Basic try-catch
try:
    result = 10 / 0
catch err:
    say f"Error: {err.message}"

# With specific error types
try:
    data = read_file("missing.txt")
catch FileError as err:
    say "File not found!"
catch err:
    say f"Unexpected error: {err}"

# Assert for quick checks
assert(age > 0, "Age must be positive")
```

---

## 13. Simple Applications

### 13.1 Calculator

```txs
## Simple Calculator in TechScript

say "=== TechScript Calculator ==="
say "Operations: + - * / ** %"
say "Type 'quit' to exit"
say ""

while true:
    input = ask "Enter expression (e.g. 5 + 3): "

    if input == "quit":
        say "Goodbye!"
        break

    parts = input.split(" ")
    guard parts.length == 3 else:
        say "Please enter: number operator number"
        skip

    a = to_float(parts[0])
    op = parts[1]
    b = to_float(parts[2])

    match op:
        case "+":
            say f"= {a + b}"
        case "-":
            say f"= {a - b}"
        case "*":
            say f"= {a * b}"
        case "/":
            if b == 0:
                say "Error: Division by zero!"
            else:
                say f"= {a / b}"
        case "**":
            say f"= {a ** b}"
        case "%":
            say f"= {a % b}"
        case _:
            say f"Unknown operator: {op}"
```

### 13.2 To-Do List

```txs
## Simple To-Do Application

todos = []

say "=== To-Do List ==="
say "Commands: add, list, done, quit"
say ""

while true:
    cmd = ask "> "

    match cmd:
        case "add":
            task = ask "Task: "
            todos.push({task: task, done: false})
            say f"Added: {task}"

        case "list":
            if todos.is_empty():
                say "No tasks yet!"
            else:
                for i, todo in enumerate(todos):
                    status = "✓" if todo.done else "○"
                    say f"  {i+1}. [{status}] {todo.task}"

        case "done":
            num = ask "Task number: " |> to_int
            if num >= 1 and num <= todos.length:
                todos[num - 1].done = true
                say f"Completed: {todos[num-1].task}"
            else:
                say "Invalid task number"

        case "quit":
            say "Goodbye!"
            break

        case _:
            say "Unknown command. Try: add, list, done, quit"
```

### 13.3 Number Guessing Game

```txs
## Number Guessing Game

target = randint(1, 100)
attempts = 0

say "=== Guess the Number ==="
say "I'm thinking of a number between 1 and 100."
say ""

until false:
    guess = ask "Your guess: " |> to_int
    attempts += 1

    if guess < target:
        say "Too low! Try higher."
    elif guess > target:
        say "Too high! Try lower."
    else:
        say f"Correct! You got it in {attempts} attempts!"
        break
```

---

## 14. Tips & Best Practices

### Naming Conventions

```txs
# Variables: snake_case
user_name = "Alice"
max_retries = 3

# Functions: snake_case
fn calculate_total(items):
    pass

# Classes: PascalCase
class ShoppingCart:
    pass

# Constants: UPPER_SNAKE_CASE
const MAX_USERS = 100
const API_URL = "https://api.example.com"
```

### Common Mistakes

```txs
# ❌ Wrong: Missing colon
if x > 5
    say "big"

# ✅ Right: Add colon after condition
if x > 5:
    say "big"

# ❌ Wrong: Using = instead of ==
if x = 5:
    say "five"

# ✅ Right: Use == for comparison
if x == 5:
    say "five"

# ❌ Wrong: Forgetting to convert input
age = ask "Age: "
if age > 18:        # Error: comparing string to int!

# ✅ Right: Convert input to number
age = ask "Age: " |> to_int
if age > 18:
    say "adult"
```

### The Pipe Operator `|>`

The pipe operator chains operations left-to-right, making code read naturally:

```txs
# Without pipe (nested, hard to read)
result = reverse(upper(trim("  hello  ")))

# With pipe (clean, left-to-right)
result = "  hello  " |> trim |> upper |> reverse
# Result: "OLLEH"
```

---

*For the full language specification, see [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md).*
*For complete keyword reference, see [TECHSCRIPT_REFERENCE.md](./TECHSCRIPT_REFERENCE.md).*
