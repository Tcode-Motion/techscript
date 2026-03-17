# 🐉 TechScript — Cinema Edition (v1.0.5)

<p align="center">
  <img src="assets/logo.png" alt="TechScript Dragon Logo" width="220">
</p>

<p align="center">
  <strong>The world's easiest programming language. Now more powerful than ever.</strong>
</p>

<div align="center">

![version](https://img.shields.io/badge/version-v1.0.5-7c3aed?style=for-the-badge)
![runtime](https://img.shields.io/badge/runtime-Native_Rust_VM-f97316?style=for-the-badge&logo=rust&logoColor=white)
![platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20Android-22c55e?style=for-the-badge)
![license](https://img.shields.io/badge/license-MIT-3b82f6?style=for-the-badge)
![release](https://img.shields.io/badge/release-7th_Official_Milestone-ff0055?style=for-the-badge)

</div>

---

## ✨ What's New in v1.0.5

This is a milestone release that transitions TechScript to a **fully native Rust ecosystem**.

- 🦀 **Native Rust Engine**: A complete rewrite of the runtime for blazing performance.
- 🌟 **Enhanced Visual Identity**: New glowing, production-quality logo with neon "bloom" effects.
- 📦 **Standalone Binaries**: Zero-dependency `TechScript_TX.exe` for seamless distribution.
- ⌨️ **Pro CLI Experience**: New ASCII "TX" banner and refined interactive REPL.
- 🎨 **Visual Overhaul**: Glowing icons for VS Code and enhanced file association visuals.

---

## 📖 Learn TechScript: From 0 to 100 🚀

TechScript is designed to be **human-first**. No semicolons, no brackets, just pure logic. This guide will take you from your first "Hello" to building professional 3D simulations.

### 🏁 Step 0: Installation & Updates

| Platform | Install Method | Link / Command | Update Method |
|:---|:---|:---|:---|
| **🪟 Windows** | **Full Setup Wizard** | [**Download Setup**](https://github.com/Tcode-Motion/techscript/releases/latest) | Run new Setup.exe |
| **🪟 Windows** | Standalone Binary | [**Download EXE**](https://github.com/Tcode-Motion/techscript/releases/latest) | Replace old .exe |
| **🐧 Linux** | One-Liner Install | [**Copy Command**](#linux--macos-terminal) | Re-run command |
| **🍎 macOS** | One-Liner Install | [**Copy Command**](#linux--macos-terminal) | Re-run command |
| **📱 Android** | Termux Engine | [**Copy Command**](#android-termux) | `pip install -U techscript-lang` |

---

### 📥 Detailed Setup & Quick Commands

#### 🪟 Windows (Setup Wizard)
1. Download **`TechScript_v1.0.5_Setup.exe`** from the [Latest Release](https://github.com/Tcode-Motion/techscript/releases/latest).
2. Double-click to run. It will extract the high-performance Rust engine to your system.
3. The installer automatically adds **`tech`** to your System PATH.

#### 🐧 Linux & 🍎 macOS (Terminal)
**Copy and paste this command to install instantly:**
```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

#### 📱 Android (Termux)
**Copy and paste these commands in Termux:**
```bash
pkg update && pkg upgrade -y
pkg install python -y
pip install techscript-lang
```
*(Update: `pip install --upgrade techscript-lang`)*

---

### 🟢 Level 1: Core Basics

#### 1. Hello World
The simplest command in the language.
```techscript
say "Hello World!"
```

#### 2. Variables & Constants
Use `make` for values that change and `keep` for fixed ones.
```techscript
make score = 0
keep PI = 3.14159

score = score + 10    # Works!
PI = 3.15              # FAIL! Constants are protected.
```

#### 3. Data Types
TechScript handles numbers, text, and collections naturally.
```techscript
make name = "Dragon"       # String
make age = 5               # Number
make powers = ["Fire", "Flight"]  # List
make stats = {"hp": 100}   # Map (Dictionary)
```

---

### 🟡 Level 2: Logic & Control Flow

#### 1. Decisions (If/Else)
We use `when` instead of `if`.
```techscript
make temp = 30

when temp > 35 {
    say "It's scorching!"
} or when temp > 20 {
    say "Nice weather."
} else {
    say "Brrr... cold."
}
```

#### 2. Repeating Work (Loops)
```techscript
# Loop through a list
each skill in ["Sprinting", "Jumping"] {
    say f"Leveling up {skill}..."
}

# Counting with a range
each i in 1..5 {
    say f"Lap {i}"
}

# Loop until a condition is met
make fuel = 3
repeat fuel > 0 {
    say "Flying!"
    fuel -= 1
}
```

---

### 🟠 Level 3: Reusing Logic (Functions)

Functions are defined with `build`.
```techscript
build greet(name, time = "Morning") {
    say f"Good {time}, {name}!"
}

greet("Tanmoy")           # Good Morning, Tanmoy!
greet("Tanmoy", "Night")  # Good Night, Tanmoy!
```

---

### 🔴 Level 4: Pro Features (OOP & Error Handling)

#### 1. Models (Classes)
Create blueprints for your objects.
```techscript
model Robot {
    build init(self, name) {
        self.name = name
    }
    build wave(self) {
        say f"{self.name} waves at you! 👋"
    }
}

make bot = Robot("R2D2")
bot.wave()
```

#### 2. Handling Crashes
Protect your app with `attempt`.
```techscript
attempt {
    make result = 10 / 0
} catch err {
    say f"Caught a mistake: {err.message}"
}
```

---

### 💎 Level 5: The Ecosystem (Special Modules)

#### 🎮 use three_d — 3D Graphics
```techscript
use three_d
make s = scene.scene()
s.objects.append(scene.box("#e94560", 1.0))
scene.render(s)
```

#### 🌐 use web — Build Sites
```techscript
use web
make p = web.page("My App")
p.body.append(web.h1("Welcome!"))
web.open(p)
```

---

### 🛠️ Developer Tooling
TechScript comes with a full Rust-native toolchain:
- **`tech fmt`**: Automatically prettify your code.
- **`tech lint`**: Find errors before you run.
- **`tech build`**: Compile to fast bytecode (`.txc`).
- **`tech test`**: Run your built-in unit tests.

---

### 📂 Folder Structure
- `bin/`: Executables and installers.
- `docs/`: Full Language Spec & Reference. [Language Reference](docs/REFERENCE.md)
- `examples/`: Ready-to-run sample projects.
- `assets/`: Official icons and logos.

---

<p align="center">
  <img src="assets/logo.png" alt="TechScript Dragon" width="80">
  <br>
  <strong>Crafted with 🐉 and 🦀 by Tcode-Motion</strong>
</p>
