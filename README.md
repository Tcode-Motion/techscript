# TechScript

![TechScript Logo](assets/icons/icon-128.png)

TechScript (`.txs`) is a modern, statically-analyzable scripting language that aims to be simple, friendly, and powerful. It is built to offer a clean syntax that compiles directly to Python or evaluates on the fly.

## Why TechScript? What makes it better?

- **Zero Setup Complexity**: No heavy runtime environments to install. Download, run `setup.bat`, and you are writing code in seconds.
- **Python Ecosystem Power**: TechScript can transpile your `.txs` files directly into valid Python code, allowing you to gradually adopt it or use it as a powerful front-end for Python.
- **Friendly Syntax**: Designed to be intuitive for beginners while offering the robustness needed for serious scripting.
- **Built-in Tooling**: The single `tech` binary includes an interpreter, transpiler, syntax checker, and an interactive REPL out of the box.

## Installation

### Windows (Recommended)
1. Download this repository.
2. Double-click `setup.bat` to install. This will:
   - Copy `tech.exe` to a safe `.techscript/bin` user directory.
   - Automatically add the command to your `PATH`.
   - Install the official **VS Code extension**, providing syntax highlighting and file icons for `.txs` files.
3. Restart your terminal or command prompt!

### Manual Installation
You can manually place the `bin/tech.exe` file anywhere in your system's `PATH`. *(Note: You will need to install the VS Code extension manually from the `vscode-extension` folder if you choose this route).*

## Usage Guide

TechScript is designed to be straightforward from the command line. Use the `tech` command to interact with your code.

### 1. Run a script
Execute a script directly using the built-in interpreter:
```bash
tech run file.txs
```

### 2. Transpile to Python
Convert your TechScript code into valid Python code. You can run it immediately or save the output:
```bash
tech transpile file.txs
tech transpile file.txs -o output.py
```

### 3. Syntax Checking
Validate your code for errors without actually running it:
```bash
tech check file.txs
```

### 4. Interactive Console (REPL)
Start the interactive TechScript prompt to test out commands line-by-line:
```bash
tech repl
```

## Language Overview & Examples

TechScript (`.txs`) reads like English and supports modern features out of the box.

```python
# Variables
name = "Alice"
age = 30
const PI = 3.14159

# Output & Input
say "Hello!"
say f"Name: {name}, Age: {age}"
answer = ask "What's your name? "

# Control flow
if age >= 18:
    say "Adult"
elif age >= 13:
    say "Teen"
else:
    say "Child"

# Loops
for i in 1..=10:
    say i

while age > 0:
    age -= 1

# Functions
fn greet(name, greeting = "Hello"):
    say f"{greeting}, {name}!"

greet("Bob")

# Classes
class Dog:
    fn init(self, name):
        self.name = name
    fn speak(self):
        say f"{self.name} says Woof!"

rex = Dog("Rex")
rex.speak()

# Pipe operator (functional style)
"hello world" |> upper |> say

# List methods
[1, 2, 3].map((x) => x * 2).filter((x) => x > 3)

# Error handling
try:
    throw "something went wrong"
catch err:
    say err
```

## Documentation & Learning

- **[Language Specification](docs/TECHSCRIPT_SPEC.md)**: Complete language spec with EBNF grammar.
- **[200 Keyword Reference](docs/TECHSCRIPT_REFERENCE.md)**: All keywords, functions, and methods.
- **[User Guide](docs/TECHSCRIPT_GUIDE.md)**: Beginner-friendly getting started guide.
- **[Example Programs](docs/TECHSCRIPT_EXAMPLES.md)**: 15 complete example programs.
- **[Build Guide (Lexer/Parser)](docs/TECHSCRIPT_BUILD.md)**: How the interpreter works internally.
- **[Build Guide (Interpreter)](docs/TECHSCRIPT_BUILD_2.md)**: Evaluator, CLI, REPL, and packaging details.

You can find ready-to-use `.txs` files in the **[examples/](examples/)** directory!

## License

TechScript is released under the MIT License. See [LICENSE](LICENSE) for details.

---

<p align="center">
  <strong>Made with 🐉 by the TechScript Team</strong>
</p>
