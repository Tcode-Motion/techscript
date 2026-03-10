# TechScript v2 — Quick Start Guide

> **You don't need to be a programmer to use TechScript.**  
> Just follow the steps below for your operating system!

---

## 🪟 Windows — Easy Install (Recommended)

1. Download **`TechScript-Setup.exe`** from the [Releases](../../releases) page
2. Double-click it to run the installer
3. Follow the on-screen steps (it sets up everything automatically — PATH, file icons, and VS Code extension)
4. Open **PowerShell** or **Command Prompt** and type:

```
tech run examples/hello.txs
```

5. Done! Your first TechScript program just ran. 🎉

---

## 🍎 macOS — Terminal Install

Open **Terminal** and paste this one command:

```bash
curl -fsSL https://raw.githubusercontent.com/your-org/techscript/main/scripts/install.sh | bash
```

After install, test it:
```bash
tech run examples/hello.txs
```

---

## 🐧 Linux — Terminal Install

Open a terminal and paste:

```bash
curl -fsSL https://raw.githubusercontent.com/your-org/techscript/main/scripts/install.sh | bash
```

Or if you have `pip`:
```bash
pip install techscript
```

Then:
```bash
tech run examples/hello.txs
```

---

## 📱 Android (Termux)

```bash
pkg install python
pip install techscript
tech run examples/hello.txs
```

---

## 🖊️ Writing Your First Program

Create a file called `hello.txs` and write:

```
say "Hello, World!"
say "I am learning TechScript!"
```

Run it with:
```
tech run hello.txs
```

---

## 🌐 Building a Website

```
use web

make page = WebPage("My First Website")

page.style("body", { "background": "#111", "color": "white", "font-family": "sans-serif" })

page.body([
    page.h1("Welcome to My Website"),
    page.p("Built 100% in TechScript — no HTML or CSS needed!")
])

page.run()
```

Run it:
```
tech run my_site.txs
```

Your browser opens automatically! 🚀

---

## 🛠️ Available Commands

| Command | What it does |
|---|---|
| `tech run file.txs` | Run a TechScript file |
| `tech check file.txs` | Check for errors without running |
| `tech repl` | Interactive coding mode (like a calculator) |
| `tech version` | Show version |
| `tech transpile file.txs` | Convert to Python |
