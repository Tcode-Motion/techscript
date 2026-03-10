# 🐉 TechScript

<p align="center">
  <img src="assets/logo.png" alt="TechScript Dragon Logo" width="220">
</p>

<p align="center">
  <strong>A friendly programming language that reads like plain English — and now builds websites too!</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-v1.0.2-7c3aed?style=flat-square">
  <img src="https://img.shields.io/badge/runtime-Native_Rust_VM-orange?style=flat-square">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20Android-green?style=flat-square">
  <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square">
</p>

---

## 🤔 What is TechScript? (Explain Like I'm 10)

Imagine you want to tell a computer to do something. Normally, computers only understand confusing code like this:

```javascript
const x = document.getElementById('name');
if (x !== null && x.value.length > 0) { ... }
```

**TechScript makes that feel like writing a sentence:**

```
make name = ask "What is your name? "
say f"Hello, {name}!"
```

That's it. **No semicolons. No brackets. No confusing symbols.** Just simple words that make sense.

TechScript is:
- 🟢 **A programming language** — you can write code that runs on your computer natively
- 🌐 **A web builder** — you can build full websites with it (no HTML or CSS needed!)
- 🦀 **Powered by Native Rust** — blazing fast compilation, completely independent of Python
- 📦 **One command** — `tech run yourfile.txs` and your program runs instantly into bytecodes

---

## 🚀 What Can TechScript Do?

| What you want to do | TechScript can do it? |
|---|---|
| Print text to screen | ✅ `say "Hello!"` |
| Ask the user a question | ✅ `make answer = ask "Your name? "` |
| Do math | ✅ `say 10 + 5 * 2` |
| Make decisions | ✅ `when age > 18 { say "Adult" }` |
| Loop (repeat stuff) | ✅ `each i in 1..10 { say i }` |
| Make functions (reusable code) | ✅ `build greet(name) { say f"Hi {name}!" }` |
| Make classes/objects | ✅ `model Dog { ... }` |
| Handle errors | ✅ `attempt { ... } catch err { ... }` |
| Build a website | ✅ `use web` + `page.run()` |
| Replace HTML | ✅ `page.h1("Title")`, `page.div([...])` |
| Replace CSS | ✅ `page.style("body", { "color": "white" })` |
| Replace JavaScript | ✅ `page.script("function hello() {...}")` |
| Replace React/Vue | ✅ Built-in state management via JS in `page.script()` |

---

## 🪟 Install on Windows (Easy — No Terminal Needed!)

### Option 1: One-Click Installer (Recommended for Beginners)

1. Go to the [📥 Releases page](../../releases/latest)
2. Download **`setup.exe`**
3. Double-click it — it will install everything automatically!
4. Open **PowerShell** (press `Win + X` → "Windows PowerShell") and type:

```
tech version
```

You should see: `TechScript v1.0.2` 🎉

**What the installer does automatically:**
- ✅ Puts `tech.exe` on your computer
- ✅ Makes the `tech` command available everywhere in your terminal
- ✅ Registers `.txs` files so they know they belong to TechScript
- ✅ Installs the VS Code extension for syntax highlighting

### Option 2: Using pip (Cross-Platform Wrapper)

```powershell
pip install techscript-lang
```

---

## 🐧 Install on Linux (Ubuntu, Kali, Debian, Arch)

### Super Simple — Just paste this in your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

**Using APT (Debian/Ubuntu):**
```bash
sudo apt update
sudo apt install techscript
```

---

## 🍎 Install on macOS

```bash
# Using Homebrew:
brew install tcode-motion/techscript/techscript

# OR use the one-line installer:
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

---

## 📱 Install on Android (Termux)

Using the built-in PKG manager:
```bash
pkg install techscript
tech version
```

---

## ✏️ Your First TechScript Program

Create a new file called `hello.txs` (you can use Notepad, VS Code, or any text editor):

```
# This is a comment — the computer ignores it
# The 'say' keyword means "print this to the screen"

say "Hello, World!"
say "I am learning TechScript!"
say "It is very easy to read 😊"
```

Now run it:
```
tech run hello.txs
```

**Output:**
```
Hello, World!
I am learning TechScript!
It is very easy to read 😊
```

---

## 📖 Language Guide (With Explanations for Beginners)

### 📦 Variables — Storing Information

A variable is like a box where you put something to use later.

```
# 'make' creates a new box called 'name' and puts "Alice" inside
make name = "Alice"

# 'keep' creates a CONSTANT — a box you can NEVER change
keep PI = 3.14159

# You can store numbers, text, lists, anything!
make age = 25
make items = [1, 2, 3, 4, 5]      # A list (like a shopping list)
make info = { "city": "Delhi" }    # A dictionary (like a phone book)
```

---

### 🖨️ Output — Talking to the User

```
say "Hello!"                        # Prints: Hello!
say "Name:", name                   # Prints: Name: Alice
say f"My name is {name}!"          # f-strings insert variables: My name is Alice!
say 10 + 5                          # Prints: 15
```

---

### 💬 Input — Asking the User a Question

```
make name = ask "What is your name? "
say f"Nice to meet you, {name}!"
```

When run, it pauses and waits for the user to type something.

---

### 🔀 Conditions — Making Decisions

Think of this like: "IF this is true, do that. Otherwise, do this other thing."

```
make age = 20

when age >= 18 {
    say "You are an adult!"
} or when age >= 13 {
    say "You are a teenager!"
} else {
    say "You are a child!"
}
```

---

### 🔁 Loops — Doing Things Repeatedly

**`each` loop** — do something for every item in a list:

```
# Count from 1 to 5
each i in 1..5 {
    say f"Count: {i}"
}

# Go through each item in a list
each fruit in ["apple", "banana", "mango"] {
    say f"I like {fruit}!"
}
```

**`repeat` loop** — keep doing something while a condition is true:

```
make x = 1
repeat x <= 5 {
    say x
    x = x + 1
}
```

---

### 🔧 Functions — Reusable Code Blocks

A function is a piece of code you write once and can use many times.

```
# 'build' creates a function. 'name' is the input it receives.
build greet(name, greeting = "Hello") {
    say f"{greeting}, {name}!"
}

# Call the function:
greet("Alice")             # Prints: Hello, Alice!
greet("Bob", "Hi there")  # Prints: Hi there, Bob!
greet("Charlie", "Hey")   # Prints: Hey, Charlie!
```

---

### 🏗️ Classes — Blueprints for Objects

A class is like a blueprint for creating things (called objects).

```
# Define what a "Dog" is
model Dog {
    build init(self, name, breed) {
        self.name = name      # Every dog has a name
        self.breed = breed    # Every dog has a breed
    }
    build speak(self) {
        say f"{self.name} says: Woof! Woof!"
    }
    build info(self) {
        say f"{self.name} is a {self.breed}"
    }
}

# Create two dogs (two "objects" from the Dog blueprint)
make rex = Dog("Rex", "German Shepherd")
make buddy = Dog("Buddy", "Golden Retriever")

rex.speak()     # Rex says: Woof! Woof!
rex.info()      # Rex is a German Shepherd
buddy.speak()   # Buddy says: Woof! Woof!
```

---

### ⚠️ Error Handling — Catching Mistakes Gracefully

What if something goes wrong? Instead of crashing, you can handle it:

```
attempt {
    # Try to do this risky thing
    make result = 10 / 0      # Division by zero!
} catch err {
    # If anything goes wrong, come here instead of crashing
    say f"Oops! Something went wrong: {err.message}"
}

say "Program continues normally after the error!"
```

---

## 🌐 Building Websites with TechScript (New in v1.0.1!)

This is the most exciting feature — you can build a **complete running website** using only TechScript. No HTML. No CSS. No JavaScript files. Just one `.txs` file!

### How it works

1. Write `use web` at the top — this loads the web module
2. Create a page with `make page = WebPage("My Title")`
3. Add styling, content, and scripts
4. Call `page.run()` — your browser opens automatically! 🚀

### Simple Website Example

```
use web

# Create a page with a title
make page = WebPage("My First Website")

# Add CSS styles (like a style sheet)
page.style("body", {
    "background": "#0f0f11",     # Dark background
    "color": "#eeeeee",          # White text
    "font-family": "sans-serif", # Clean font
    "text-align": "center",
    "padding": "60px"
})

# Add JavaScript (for interactive buttons)
page.script("""
    function sayHello() {
        alert('Hello from TechScript! 🐉');
    }
""")

# Build the page layout
page.body([
    page.h1("Welcome to My Website! 🐉"),
    page.p("This website was built 100% in TechScript."),
    page.p("No HTML. No CSS files. No React. Just TechScript!"),
    page.button("Click Me!", { "onclick": "sayHello()" })
])

# Run the server — browser opens automatically!
page.run()
```

**Run it:**
```
tech run my_website.txs
```

That's it! Your browser opens and shows your website. Press `Ctrl+C` to stop the server.

---

## 🛠️ All CLI Commands

| Command | What it does | Example |
|---|---|---|
| `tech run file.txs` | Run a TechScript file | `tech run hello.txs` |
| `tech run file.txs --debug` | Run with debug info | `tech run calc.txs --debug` |
| `tech check file.txs` | Check for errors without running | `tech check myapp.txs` |
| `tech repl` | Open interactive mode (type and run live) | `tech repl` |
| `tech transpile file.txs` | Convert your code to Python | `tech transpile hello.txs` |
| `tech version` or `tech -V` | Show installed version | `tech -V` |

---

## 📋 All Example Programs

Run any of these to see TechScript in action:

| File | What it does | Run it |
|---|---|---|
| `examples/hello.txs` | Prints Hello World | `tech run examples/hello.txs` |
| `examples/fibonacci.txs` | Calculates Fibonacci numbers | `tech run examples/fibonacci.txs` |
| `examples/fizzbuzz.txs` | The classic FizzBuzz challenge | `tech run examples/fizzbuzz.txs` |
| `examples/classes.txs` | Dogs and cats using OOP | `tech run examples/classes.txs` |
| `examples/calculator.txs` | Simple calculator | `tech run examples/calculator.txs` |
| `examples/guessing_game.txs` | Guess the number game | `tech run examples/guessing_game.txs` |
| `examples/web_app_simple.txs` | Simple dark-theme website | `tech run examples/web_app_simple.txs` |
| `examples/web_complete.txs` | Full showcase: counter, API, form | `tech run examples/web_complete.txs` |

---

## 🎨 VS Code / Cursor Editor Extension

Get **syntax highlighting**, **code snippets**, and the **🐉 dragon file icon** for `.txs` files:

**Method 1 — Command line (fastest):**
```bash
code --install-extension vscode-extension/techscript-1.0.2.vsix
```

**Method 2 — Using the GUI:**
1. Open VS Code
2. Press `Ctrl+Shift+X` to open Extensions
3. Click the `···` menu (top right of the Extensions panel)
4. Select **"Install from VSIX..."**
5. Choose `vscode-extension/techscript-1.0.2.vsix`

After installing, all your `.txs` files will have the dragon icon and coloured syntax! 🎨

---

## ✨ v1.0.2 Changelog

| Feature | v1.0.1 | v1.0.2 |
|---|---|---|
| Runtime Engine | Python Interpreted | ✅ **Native Rust VM** |
| Execution Speed | Standard | ✅ **Blazing Fast Bytecode** |
| Try/Rescue Blocks | ❌ | ✅ **NEW Stack Unwinding** |
| Division by Zero Errors | Immediate Crash | ✅ **Safely Trapped** |
| VS Code extension | ✅ | ✅ Updated |
| Windows Executable | `tech.exe` | ✅ `techscriptv1.0.2.exe` |
| Native `setup.exe` Bundle | ❌ | ✅ **NEW** |

---

## 📚 Documentation

| Document | Description |
|---|---|
| [Quick Start Guide](docs/QUICKSTART.md) | Simple step-by-step install for all platforms |
| [Language Cheat Sheet](docs/REFERENCE.md) | All keywords, functions, and syntax at a glance |
| [Web Module Guide](docs/WEB_MODULE.md) | How to build websites with TechScript |

---

## 🌍 Platform Support

| Platform | Status | Install Method |
|---|---|---|
| **Windows 10/11** | ✅ Fully supported | `setup.exe` or `pip` |
| **macOS** | ✅ Fully supported | `install.sh` or `brew` |
| **Linux** (Ubuntu, Kali, Arch) | ✅ Fully supported | `install.sh` or `apt` |
| **Android (Termux)** | ✅ Works | `pkg install techscript` |

---

## 📄 License

MIT License — free to use, share, and modify. See [LICENSE](LICENSE).

---

<p align="center">
  <img src="assets/logo.png" alt="TechScript Dragon" width="80">
  <br>
  <strong>Made with 🐉 by the TechScript Team</strong>
</p>
