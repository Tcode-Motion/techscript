# API Reference

This reference documents the globally available built-in functions in TechScript 2.0.

---

## 🖨️ Global Functions

### `say(val)`
Prints the string representation of `val` to stdout, followed by a newline:
```txs
say "Hello"
say 42
```

### `ask(prompt)`
Prints the `prompt` string to stdout and blocks execution until input is read from stdin. Returns the input as a string:
```txs
name = ask "Enter your name: "
```

### `len(collection)`
Returns the size of a list, map, or string as an integer:
```txs
size = len([1, 2, 3]) # 3
char_count = len("Tech") # 4
```

### `typeof(val)`
Returns a string representing the runtime type of the value (`"int"`, `"float"`, `"str"`, `"bool"`, `"list"`, `"map"`, `"null"`, or custom class name):
```txs
say typeof(42) # "int"
```

### `sleep(ms)`
Suspends thread execution for `ms` milliseconds:
```txs
sleep(1000) # Sleep for 1 second
```

### `assert(condition)`
Triggers a runtime panic if the condition is false:
```txs
assert(1 == 1)
```

### `panic(message)`
Halts VM execution immediately and prints the error message:
```txs
panic "Unrecoverable fatal error occurred"
```

### `exit(code)`
Exits the process immediately with the specified integer status code:
```txs
exit(0)
```
