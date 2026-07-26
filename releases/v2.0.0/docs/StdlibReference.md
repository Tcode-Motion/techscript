# TechScript 2.0 Standard Library Reference

> **Status**: Frozen Specification — 2.0.0 Stable
> **Last Updated**: 2026-07-26

TechScript 2.0 standard library functions are grouped into qualified modules. To
use a module, import it using the `use` keyword. All calls to standard library
functions must be fully qualified. True language built-ins (`say`, `ask`, `env`,
`file`) are available implicitly without any imports.

---

## 1. Built-in Operations (Implicit Calls)

Available in all files without imports. Never use parentheses when calling these.

| Built-in | Signature | Description | Example |
|---|---|---|---|
| `say` | `say expression` | Prints the expression followed by a newline to stdout. | `say "Hello"` |
| `ask` | `ask "prompt"` | Prints the prompt and reads a line of input from stdin. | `name = ask "Name?"` |
| `env` | `env "VAR"` | Reads the environment variable value. Returns `null` if not set. | `path = env "PATH"` |
| `file` | `file "path"` | Reads the file at `path` as a UTF-8 string. Panics if file not found. | `data = file "config.json"` |
| `len` | `len(collection)` | Returns length of list, map, or string. (Uses parentheses) | `n = len(items)` |
| `typeof` | `typeof(expr)` | Returns string name of expression's type. | `t = typeof 42` |
| `assert` | `assert(cond)` | Panics if the condition evaluates to `false`. | `assert x > 0` |
| `panic` | `panic "msg"` | Immediately terminates the runtime with the error message. | `panic "unreachable"` |
| `exit` | `exit(code)` | Terminates execution with the given status code. | `exit 0` |
| `sleep` | `sleep(ms)` | Blocks execution for `ms` milliseconds. | `sleep 1000` |
| `json` | `json "..."` | Fast-parses a JSON string into a Map or List. | `obj = json raw_str` |
| `time` | `time()` | Returns the current Unix timestamp in seconds. | `now = time()` |

---

## 2. Standard Modules (Qualified Calls)

### math

Mathematical operations and constants.

```txs
use math

say math.pi                # 3.14159265...
val = math.abs(-42)        # 42
root = math.sqrt(25)       # 5.0
rounded = math.round(3.6)  # 4
power = math.pow(2, 10)    # 1024.0
```

Functions:
- `math.abs(x)`: Absolute value of `x`.
- `math.sqrt(x)`: Square root of `x`.
- `math.round(x, decimals = 0)`: Rounds `x` to specified decimal places.
- `math.pow(base, exp)`: Raises `base` to `exp` power.
- `math.floor(x)`: Returns largest integer less than or equal to `x`.
- `math.ceil(x)`: Returns smallest integer greater than or equal to `x`.
- `math.sin(x)`, `math.cos(x)`, `math.tan(x)`: Trigonometric functions.

---

### json

Structured JSON serialization and deserialization.

```txs
use json

data = {"name": "Alice", "age": 30}
encoded = json.stringify(data)
say encoded    # {"name":"Alice","age":30}

decoded = json.parse(encoded)
say decoded["name"]    # Alice
```

Functions:
- `json.parse(json_str)`: Parses JSON string into Map or List.
- `json.stringify(val)`: Serializes value to JSON string.

---

### http

Network capability to perform HTTP client operations.

```txs
use http

# GET request
response = http.get("https://api.example.com/status")
say response.status    # 200
say response.body      # JSON string response

# POST request
payload = json.stringify({"id": 123})
response = http.post("https://api.example.com/submit", payload)
say response.status    # 201
```

Functions:
- `http.get(url)`: Performs GET request. Returns Response object.
- `http.post(url, body)`: Performs POST request with body. Returns Response.
- Response fields: `status` (Int), `body` (Str), `headers` (Map).

---

### crypto

Hashing and encryption primitives.

```txs
use crypto

hash = crypto.sha256("password123")
say hash    # hex string representation
```

Functions:
- `crypto.sha256(data)`: Returns SHA-256 hash as a hex string.
- `crypto.md5(data)`: Returns MD5 hash as a hex string.

---

### string

Advanced string manipulation utilities.

```txs
use string

upper = string.to_upper("hello")      # "HELLO"
parts = string.split("a,b,c", ",")    # ["a", "b", "c"]
joined = string.join(parts, "-")      # "a-b-c"
replaced = string.replace("abc", "a", "x") # "xbc"
trimmed = string.trim("  hello  ")    # "hello"
```

Functions:
- `string.to_upper(s)`, `string.to_lower(s)`: Convert case.
- `string.split(s, delimiter)`: Split string into List of substrings.
- `string.join(list, delimiter)`: Join List of strings.
- `string.replace(s, target, replacement)`: Replace occurrences of substring.
- `string.trim(s)`: Remove leading/trailing whitespace.
- `string.contains(s, substring)`: Check if substring exists.
- `string.starts_with(s, prefix)`, `string.ends_with(s, suffix)`: Checks prefix/suffix.
