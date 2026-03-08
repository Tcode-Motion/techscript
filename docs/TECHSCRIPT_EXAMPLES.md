# TechScript — Example Programs

> A collection of practical `.txs` programs demonstrating TechScript features.
> Companion to [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md)

---

## Table of Contents

1. [Hello World](#1-hello-world)
2. [Temperature Converter](#2-temperature-converter)
3. [FizzBuzz](#3-fizzbuzz)
4. [Fibonacci Sequence](#4-fibonacci-sequence)
5. [File Word Counter](#5-file-word-counter)
6. [Simple HTTP Server](#6-simple-http-server-concept)
7. [Student Grade Manager](#7-student-grade-manager)
8. [Password Generator](#8-password-generator)
9. [Mini Database (JSON-backed)](#9-mini-database-json-backed)
10. [Text Adventure Game](#10-text-adventure-game)
11. [List Comprehensions & Functional](#11-list-comprehensions--functional)
12. [Class & OOP Demo](#12-class--oop-demo)
13. [Async & Await Demo](#13-async--await-demo)
14. [Module System Demo](#14-module-system-demo)
15. [Decorator Pattern](#15-decorator-pattern)

---

## 1. Hello World

```txs
# hello.txs — The simplest TechScript program
say "Hello, World!"
```

---

## 2. Temperature Converter

```txs
# temp.txs — Celsius ↔ Fahrenheit converter

say "=== Temperature Converter ==="
say "1) Celsius → Fahrenheit"
say "2) Fahrenheit → Celsius"

choice = ask "Choose (1 or 2): "

if choice == "1":
    c = ask "Enter °C: " |> to_float
    f = c * 9 / 5 + 32
    say f"{c}°C = {round(f, 2)}°F"
elif choice == "2":
    f = ask "Enter °F: " |> to_float
    c = (f - 32) * 5 / 9
    say f"{f}°F = {round(c, 2)}°C"
else:
    say "Invalid choice!"
```

---

## 3. FizzBuzz

```txs
# fizzbuzz.txs — Classic interview problem

for i in 1..=100:
    match [i % 3 == 0, i % 5 == 0]:
        case [true, true]:
            say "FizzBuzz"
        case [true, false]:
            say "Fizz"
        case [false, true]:
            say "Buzz"
        case _:
            say i
```

---

## 4. Fibonacci Sequence

```txs
# fibonacci.txs — Generate Fibonacci numbers

# Iterative approach
fn fibonacci(n):
    guard n >= 0 else:
        throw ValueError("n must be non-negative")

    if n <= 1:
        return n

    a = 0
    b = 1
    for _ in 2..=n:
        a, b = b, a + b
    return b

# Generator approach
fn fib_gen(limit):
    a = 0
    b = 1
    for _ in 0..limit:
        yield a
        a, b = b, a + b

# Usage
say "First 15 Fibonacci numbers:"
for num in fib_gen(15):
    write(f"{num} ")
say ""

say f"F(50) = {fibonacci(50)}"
```

---

## 5. File Word Counter

```txs
# wordcount.txs — Count words in a text file

fn count_words(filepath):
    guard file_exists(filepath) else:
        throw FileError(f"File not found: {filepath}")

    content = read_file(filepath)
    words = content.split(" ")
    word_map = {}

    for word in words:
        w = word.lower().trim()
        if w.length > 0:
            word_map[w] = word_map.get(w, 0) + 1

    return word_map

# Main
filepath = ask "Enter file path: "
counts = count_words(filepath)

say f"\nTotal unique words: {counts.keys().length}"
say "\nTop 10 words:"

sorted_words = counts.entries().sort((a, b) => b[1] - a[1])
for i, entry in enumerate(sorted_words.take(10)):
    say f"  {i+1}. '{entry[0]}' — {entry[1]} times"
```

---

## 6. Simple HTTP Server (Concept)

```txs
# server.txs — Basic HTTP server (requires http module)

import http

fn handle_request(req, res):
    match req.path:
        case "/":
            res.send("<h1>Welcome to TechScript Web!</h1>")
        case "/api/hello":
            res.json({message: "Hello from TechScript!", status: "ok"})
        case "/api/time":
            res.json({time: time.now()})
        case _:
            res.status(404).send("Not Found")

server = http.create_server(handle_request)
server.listen(3000)
say "Server running at http://localhost:3000"
```

---

## 7. Student Grade Manager

```txs
# grades.txs — Student grade management system

class Student:
    fn init(self, name):
        self.name = name
        self.grades = []

    fn add_grade(self, subject, score):
        self.grades.push({subject: subject, score: score})

    fn average(self):
        if self.grades.is_empty():
            return 0
        total = self.grades.reduce((sum, g) => sum + g.score, 0)
        return round(total / self.grades.length, 2)

    fn grade_letter(self):
        avg = self.average()
        if avg >= 90: return "A"
        if avg >= 80: return "B"
        if avg >= 70: return "C"
        if avg >= 60: return "D"
        return "F"

    fn report(self):
        say f"\n--- Report Card: {self.name} ---"
        for g in self.grades:
            say f"  {g.subject}: {g.score}"
        say f"  Average: {self.average()} ({self.grade_letter()})"

# Usage
alice = new Student("Alice")
alice.add_grade("Math", 92)
alice.add_grade("Science", 88)
alice.add_grade("English", 95)
alice.add_grade("History", 78)
alice.report()
```

---

## 8. Password Generator

```txs
# passgen.txs — Random password generator

const LOWER = "abcdefghijklmnopqrstuvwxyz"
const UPPER = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
const DIGITS = "0123456789"
const SYMBOLS = "!@#$%^&*()-_=+[]{}|;:,.<>?"

fn generate_password(length = 16, use_symbols = true):
    chars = LOWER + UPPER + DIGITS
    if use_symbols:
        chars += SYMBOLS

    char_list = chars.chars()
    password = ""
    for _ in 0..length:
        password += choice(char_list)

    return password

fn check_strength(password):
    score = 0
    if password.length >= 8: score += 1
    if password.length >= 12: score += 1
    if password.match_regex("[a-z]"): score += 1
    if password.match_regex("[A-Z]"): score += 1
    if password.match_regex("[0-9]"): score += 1
    if password.match_regex("[^a-zA-Z0-9]"): score += 1

    match score:
        case 1..2: return "Weak"
        case 3..4: return "Medium"
        case 5..6: return "Strong"

# Main
length = ask "Password length (default 16): "
length = to_int(length) if length.length > 0 else 16

password = generate_password(length)
strength = check_strength(password)

say f"\nGenerated: {password}"
say f"Strength: {strength}"
```

---

## 9. Mini Database (JSON-backed)

```txs
# minidb.txs — Simple JSON-file database

class Database:
    fn init(self, filepath):
        self.filepath = filepath
        if file_exists(filepath):
            self.data = read_json(filepath)
        else:
            self.data = []

    fn save(self):
        write_json(self.filepath, self.data)

    fn insert(self, record):
        record["id"] = self.data.length + 1
        self.data.push(record)
        self.save()
        return record

    fn find_all(self):
        return self.data.copy()

    fn find_by(self, key, value):
        return self.data.filter(r => r[key] == value)

    fn update(self, id, updates):
        for record in self.data:
            if record["id"] == id:
                for key in updates.keys():
                    record[key] = updates[key]
                self.save()
                return record
        return none

    fn delete(self, id):
        self.data = self.data.filter(r => r["id"] != id)
        self.save()

# Usage
db = new Database("contacts.json")

db.insert({name: "Alice", email: "alice@example.com"})
db.insert({name: "Bob", email: "bob@example.com"})

say "All contacts:"
for contact in db.find_all():
    say f"  [{contact.id}] {contact.name} — {contact.email}"

results = db.find_by("name", "Alice")
say f"\nFound {results.length} result(s) for 'Alice'"
```

---

## 10. Text Adventure Game

```txs
# adventure.txs — Simple text adventure

rooms = {
    start: {
        desc: "You are in a dark room. There is a door to the NORTH and EAST.",
        north: "hallway",
        east: "garden"
    },
    hallway: {
        desc: "A long hallway. Doors to the SOUTH and EAST.",
        south: "start",
        east: "treasure"
    },
    garden: {
        desc: "A beautiful garden with flowers. Door to the WEST.",
        west: "start"
    },
    treasure: {
        desc: "You found a treasure chest! 🎉 YOU WIN!",
        is_end: true
    }
}

current = "start"

say "=== TechScript Adventure ==="
say "Commands: north, south, east, west, look, quit\n"

while true:
    room = rooms[current]
    say room.desc
    say ""

    if room.get("is_end", false):
        say "Congratulations! Game over."
        break

    cmd = ask "> " |> lower |> trim

    match cmd:
        case "look":
            say room.desc
        case "quit":
            say "Goodbye!"
            break
        case "north" or "south" or "east" or "west":
            if room has cmd:
                current = room[cmd]
            else:
                say "You can't go that way."
        case _:
            say "Unknown command. Try: north, south, east, west, look, quit"
    say ""
```

---

## 11. List Comprehensions & Functional

```txs
# functional.txs — Functional programming patterns

# List comprehension
squares = [x ** 2 for x in 1..=10]
say f"Squares: {squares}"

# Filtered comprehension
evens = [x for x in 1..=20 if x % 2 == 0]
say f"Evens: {evens}"

# Chained operations with pipe
result = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    |> filter(x => x % 2 == 0)
    |> map(x => x ** 2)
    |> reduce((a, b) => a + b, 0)
say f"Sum of squares of evens: {result}"  # 220

# Zip and enumerate
names = ["Alice", "Bob", "Charlie"]
scores = [95, 87, 92]

for name, score in zip(names, scores):
    say f"{name}: {score}"
```

---

## 12. Class & OOP Demo

```txs
# oop.txs — Object-Oriented Programming

class Shape:
    fn init(self, color = "black"):
        self.color = color

    fn area(self):
        throw Error("area() must be implemented by subclass")

    fn describe(self):
        say f"A {self.color} {typeof(self)} with area {self.area()}"

class Circle(Shape):
    fn init(self, radius, color = "red"):
        super.init(color)
        self.radius = radius

    fn area(self):
        return 3.14159 * self.radius ** 2

class Rectangle(Shape):
    fn init(self, width, height, color = "blue"):
        super.init(color)
        self.width = width
        self.height = height

    fn area(self):
        return self.width * self.height

# Polymorphism
shapes = [
    new Circle(5, "red"),
    new Rectangle(4, 6, "blue"),
    new Circle(3),
    new Rectangle(10, 2, "green")
]

total_area = 0
for shape in shapes:
    shape.describe()
    total_area += shape.area()

say f"\nTotal area of all shapes: {round(total_area, 2)}"
```

---

## 13. Async & Await Demo

```txs
# async_demo.txs — Asynchronous operations

import http
import time

async fn fetch_user(id):
    response = await http.get(f"https://api.example.com/users/{id}")
    return response.json()

async fn fetch_all_users(ids):
    tasks = ids.map(id => fetch_user(id))
    return await Promise.all(tasks)

# Usage
async fn main():
    say "Fetching users..."
    start = time.now()

    users = await fetch_all_users([1, 2, 3, 4, 5])

    elapsed = time.now() - start
    say f"Fetched {users.length} users in {elapsed}ms"

    for user in users:
        say f"  {user.name} ({user.email})"

await main()
```

---

## 14. Module System Demo

### `math_utils.txs` (Module File)

```txs
# math_utils.txs — Reusable math utilities

## Calculate the factorial of n
export fn factorial(n):
    guard n >= 0 else:
        throw ValueError("n must be non-negative")
    if n <= 1: return 1
    return n * factorial(n - 1)

## Check if n is prime
export fn is_prime(n):
    if n < 2: return false
    for i in 2..=sqrt(n) |> to_int:
        if n % i == 0:
            return false
    return true

## Generate list of primes up to max
export fn primes_up_to(max):
    return [n for n in 2..=max if is_prime(n)]

export const PHI = 1.6180339887  # Golden ratio
```

### `main.txs` (Uses the Module)

```txs
# main.txs — Using math_utils module

from math_utils import factorial, is_prime, primes_up_to, PHI

say f"10! = {factorial(10)}"
say f"Is 97 prime? {is_prime(97)}"
say f"Primes up to 30: {primes_up_to(30)}"
say f"Golden ratio: {PHI}"
```

---

## 15. Decorator Pattern

```txs
# decorators.txs — Decorator pattern examples

## Timing decorator — measures function execution time
fn timer(func):
    fn wrapper(...args):
        start = time.now()
        result = func(...args)
        elapsed = time.now() - start
        say f"[timer] {func.name} took {elapsed}ms"
        return result
    return wrapper

## Memoization decorator — caches results
fn memoize(func):
    cache = {}
    fn wrapper(...args):
        key = to_str(args)
        unless cache has key:
            cache[key] = func(...args)
        return cache[key]
    return wrapper

## Retry decorator — retries on failure
fn retry(max_attempts = 3):
    fn decorator(func):
        fn wrapper(...args):
            for attempt in 1..=max_attempts:
                try:
                    return func(...args)
                catch err:
                    say f"[retry] Attempt {attempt} failed: {err.message}"
                    if attempt == max_attempts:
                        throw err
        return wrapper
    return decorator

# Usage
@timer
fn slow_sum(n):
    total = 0
    for i in 1..=n:
        total += i
    return total

@memoize
fn fibonacci(n):
    if n <= 1: return n
    return fibonacci(n - 1) + fibonacci(n - 2)

say slow_sum(1_000_000)
say fibonacci(50)        # Instant due to memoization
```

---

*For the full specification, see [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md).*
*For keyword reference, see [TECHSCRIPT_REFERENCE.md](./TECHSCRIPT_REFERENCE.md).*
*For beginner guide, see [TECHSCRIPT_GUIDE.md](./TECHSCRIPT_GUIDE.md).*
