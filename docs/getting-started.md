# Getting Started with TechScript

This guide gets you up and running with your very first TechScript program.

---

## 💻 Prerequisite
Make sure you have installed the TechScript compiler and VM on your machine. If not, follow the [Installation Guide](Installation.md).

Validate your installation:
```bash
tech version
```
This should output the current version of the language (e.g., `TechScript v2.0.0`).

---

## 🐉 Write Hello World

1. Create a new text file named `hello.txs`.
2. Open it in your favorite text editor (or TechScript Studio IDE) and add the following line:
   ```txs
   say "Hello, World!"
   ```
3. Run the file using the CLI tool:
   ```bash
   tech run hello.txs
   ```
   You should see the output:
   ```
   Hello, World!
   ```

---

## 📦 A Simple Calculations Script

Let's write a slightly more complex program that accepts user input, computes a value, and uses conditions:

```txs
# Ask user for their name
name = ask "What is your name? "

say $"Welcome, {name}!"

# Prompt for age
age_input = ask "How old are you? "
age = math.parse_int(age_input)

# Logic checks
when age >= 18
    say "You are eligible to vote."
else
    years_left = 18 - age
    say $"Come back in {years_left} years!"
end
```

Run this script:
```bash
tech run age_check.txs
```

---

## 🎮 Explore Next
- Check out the full [Syntax Guide](syntax.md).
- Learn how to compile scripts to bytecode using [CLI docs](cli.md).
