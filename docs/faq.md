# Frequently Asked Questions (FAQ)

Answers to common questions about the TechScript programming language.

---

## 🐲 Design & Philosophy

### Why was TechScript created?
TechScript was designed to make programming readable and accessible. By removing brackets, semicolons, and symbol noise, it lets developers focus on system logic.

### Does it require a Python runtime?
No. Since version 1.0.2, the compiler and VM have been rewritten entirely in **Rust**. It compiles down to standalone native binaries with no external runtime dependencies.

---

## ⚡ Execution & Speed

### How performant is TechScript?
TechScript executes code extremely fast. The virtual machine uses **NaN-boxed value registers**, meaning numbers, references, and booleans are stored directly on a 64-bit stack. It evaluates 1 million loop iterations in approximately 2.9 seconds.

### Does it support native compilation?
Yes! The compiler features an LLVM code generation backend, compiling `.txs` scripts directly into native standalone machine-code executables.

---

## 🤝 Editor Integrations

### How do I configure VS Code?
Install the TechScript extension (`editors/vscode` folder). This configures syntax highlighting, auto-completions, formatting, and triggers the built-in LSP language server.
