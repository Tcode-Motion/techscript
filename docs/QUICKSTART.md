# TechScript v1.0.3 — Quick Start Guide

> **You don't need to be a programmer to use TechScript.**  
> Just follow the steps below for your operating system!

---

## 🪟 Windows — Easy Install (Recommended)

1. Download **`TechScript-Setup.exe`** from the [Releases](../../releases) page
2. Double-click it to run the installer
3. Follow the on-screen steps (it sets up everything automatically — PATH, file icons, and VS Code extension)
4. Open **PowerShell** or **Command Prompt** and type:

```
tech version
```

You should see: `TechScript v1.0.3` 🎉

---

## 📱 Android (Termux)

See the detailed [Termux Guide](TERMUX.md) or just run:
```bash
pkg install python -y && pip install techscript
tech version
```

---

## 🚀 New in v1.0.3 — 150+ Functions

Your language just got a massive upgrade! You can now use professional tools out of the box:

- `crypto.sha256("hello")` — Generate secure hashes
- `math.sin(x)`, `math.factorial(n)` — Advanced math
- `json.encode(my_list)`, `json.decode(str)` — Handle data
- `fs.read("file.txt")`, `fs.write("file.txt", "hi")` — File management
- `date.now`, `os.name`, `random.uuid` — And much more!

---

## ⚡ Execution Modes

### 1. Run a File
Create `hello.txs`:
```
say "Hello, World!"
```
Run it: `tech run hello.txs`

### 2. Interactive Mode (REPL)
Just type `tech repl` to open a live console where you can type code and see results instantly.

### 3. Inline Execution (New!)
Run code instantly without a file:
`tech eval "say crypto.sha256('hello')"`

---

## 🛠️ Available Commands

| Command | What it does | Example |
|---|---|---|
| `tech run file.txs` | Run a TechScript file | `tech run calc.txs` |
| `tech eval "code"` | **NEW** — Run code instantly | `tech eval "say 42"` |
| `tech check file.txs` | Check for errors without running | `tech check script.txs` |
| `tech repl` | Interactive coding mode | `tech repl` |
| `tech version` | Show version | `tech version` |
