# TechScript v2.0.0 — Standard Library Reference

> Complete reference for all 150+ built-in functions.

---

## 🌍 Global Functions

### I/O Functions
| Function | Description | Example |
|:---|:---|:---|
| `say(value)` | Print with newline | `say "Hello"` |
| `print(value)` | Print without newline | `print "..."` |
| `write(value)` | Alias for print | `write "text"` |
| `format(fmt, ...)` | Format string | `format("{} is {}", name, age)` |
| `debug(value)` | Print with type info | `debug myvar` |
| `warn(value)` | Print as warning (yellow) | `warn "caution"` |
| `error(value)` | Print as error (red) | `error "failed"` |
| `clear()` | Clear terminal screen | `clear()` |

### Core Functions
| Function | Description | Example |
|:---|:---|:---|
| `assert(cond, msg)` | Panic if false | `assert(x > 0, "must be positive")` |
| `panic(msg)` | Stop with error | `panic("fatal")` |
| `sleep(ms)` | Pause execution | `sleep(1000)` |
| `time()` | Unix timestamp (seconds) | `make t = time()` |
| `time_ms()` | Unix timestamp (ms) | `make t = time_ms()` |
| `exit(code)` | Exit process | `exit(0)` |
| `version()` | Return version string | `say version()` |
| `callable(val)` | Check if callable | `callable(my_fn)` |

### Type Conversion
| Function | Description | Example |
|:---|:---|:---|
| `int(val)` | Convert to integer | `int("42")` → `42` |
| `float(val)` | Convert to float | `float("3.14")` → `3.14` |
| `str(val)` | Convert to string | `str(42)` → `"42"` |
| `bool(val)` | Convert to boolean | `bool(0)` → `false` |
| `list(val)` | Convert to list | `list("abc")` → `["a","b","c"]` |
| `type(val)` | Get type name string | `type(42)` → `"int"` |
| `len(val)` | Get length | `len([1,2,3])` → `3` |

### String Functions
| Function | Description | Example |
|:---|:---|:---|
| `upper(s)` | Uppercase | `upper("hi")` → `"HI"` |
| `lower(s)` | Lowercase | `lower("HI")` → `"hi"` |
| `trim(s)` | Remove whitespace | `trim("  hi  ")` → `"hi"` |
| `trim_start(s)` | Trim leading | `trim_start("  hi")` → `"hi"` |
| `trim_end(s)` | Trim trailing | `trim_end("hi  ")` → `"hi"` |
| `split(s, d)` | Split by delimiter | `split("a,b", ",")` → `["a","b"]` |
| `join(list, d)` | Join list with delimiter | `join(["a","b"], "-")` → `"a-b"` |
| `replace(s, a, b)` | Replace occurrences | `replace("hi", "h", "H")` → `"Hi"` |
| `contains(s, sub)` | Check substring | `contains("hello", "ell")` → `true` |
| `starts_with(s, p)` | Check prefix | `starts_with("hello", "he")` → `true` |
| `ends_with(s, p)` | Check suffix | `ends_with("hello", "lo")` → `true` |
| `find(s, sub)` | Find index (-1 if none) | `find("hello", "ll")` → `2` |
| `chars(s)` | String to char list | `chars("hi")` → `["h","i"]` |
| `repeat_str(s, n)` | Repeat N times | `repeat_str("ab", 3)` → `"ababab"` |

### Math Shortcuts
| Function | Description | Example |
|:---|:---|:---|
| `abs(x)` | Absolute value | `abs(-5)` → `5` |
| `round(x)` | Round | `round(3.7)` → `4` |
| `floor(x)` | Floor | `floor(3.7)` → `3` |
| `ceil(x)` | Ceiling | `ceil(3.2)` → `4` |
| `sqrt(x)` | Square root | `sqrt(16)` → `4` |
| `pow(x, y)` | Power | `pow(2, 8)` → `256` |
| `min(a, b)` | Minimum | `min(3, 7)` → `3` |
| `max(a, b)` | Maximum | `max(3, 7)` → `7` |
| `clamp(x, lo, hi)` | Clamp to range | `clamp(15, 0, 10)` → `10` |
| `sum(list)` | Sum of list | `sum([1,2,3])` → `6` |

---

## 📐 `math.*` Module (38 functions)

### Constants
| Constant | Value |
|:---|:---|
| `math.PI` | 3.141592653589793 |
| `math.E` | 2.718281828459045 |
| `math.TAU` | 6.283185307179586 |
| `math.INF` | Infinity |

### Trigonometry
`math.sin(x)` · `math.cos(x)` · `math.tan(x)` · `math.asin(x)` · `math.acos(x)` · `math.atan(x)` · `math.atan2(y,x)` · `math.sinh(x)` · `math.cosh(x)` · `math.tanh(x)` · `math.to_radians(deg)` · `math.to_degrees(rad)`

### Power / Roots / Logs
`math.sqrt(x)` · `math.cbrt(x)` · `math.pow(x,y)` · `math.exp(x)` · `math.log(x)` · `math.log2(x)` · `math.log10(x)`

### Rounding
`math.floor(x)` · `math.ceil(x)` · `math.round(x)` · `math.abs(x)` · `math.sign(x)`

### Integer Math
`math.factorial(n)` · `math.gcd(a,b)` · `math.lcm(a,b)` · `math.fibonacci(n)` · `math.is_prime(n)`

### Statistics
`math.min(list)` · `math.max(list)` · `math.sum(list)` · `math.mean(list)` · `math.clamp(x,lo,hi)`

---

## 📁 `fs.*` Module (20 functions)

| Function | Description |
|:---|:---|
| `fs.read(path)` | Read file to string |
| `fs.write(path, data)` | Write string to file |
| `fs.append(path, data)` | Append to file |
| `fs.exists(path)` | Check if path exists |
| `fs.delete(path)` | Delete file |
| `fs.rename(old, new)` | Rename file |
| `fs.copy(src, dst)` | Copy file |
| `fs.is_file(path)` | Check if file |
| `fs.is_dir(path)` | Check if directory |
| `fs.size(path)` | File size in bytes |
| `fs.list_dir(path)` | List directory contents |
| `fs.make_dir(path)` | Create directory |
| `fs.cwd` | Current working directory |
| `fs.read_bytes(path)` | Read as byte list |
| `fs.write_bytes(path, bytes)` | Write byte list |
| `fs.extension(path)` | Get file extension |
| `fs.basename(path)` | Get filename |
| `fs.parent(path)` | Get parent directory |
| `fs.join(a, b)` | Join path components |
| `fs.abs_path(path)` | Get absolute path |

---

## 🖥️ `os.*` Module (10 functions)

| Function | Description |
|:---|:---|
| `os.name` | OS name (e.g. "windows") |
| `os.arch` | CPU architecture |
| `os.pid` | Current process ID |
| `os.env_get(key)` | Get environment variable |
| `os.env_set(key, val)` | Set environment variable |
| `os.system(cmd)` | Run command (exit code) |
| `os.popen(cmd)` | Run command (get output) |
| `os.args` | Command line arguments |
| `os.home` | User home directory |
| `os.exit(code)` | Exit with code |

---

## 📊 `json.*` Module (3 functions)

| Function | Description | Example |
|:---|:---|:---|
| `json.encode(val)` | Value → JSON string | `json.encode({"a":1})` → `'{"a":1}'` |
| `json.encode_pretty(val)` | Value → formatted JSON | Multi-line, indented |
| `json.decode(str)` | JSON string → value | `json.decode('{"a":1}')` → `{"a":1}` |

---

## 🔐 `crypto.*` Module (4 functions)

| Function | Description | Example |
|:---|:---|:---|
| `crypto.sha256(str)` | SHA-256 hash (real FIPS 180-4) | `crypto.sha256("hi")` → `"8f43..."` |
| `crypto.md5(str)` | MD5-style hash | `crypto.md5("hi")` |
| `crypto.base64_encode(str)` | Encode to Base64 | `crypto.base64_encode("hi")` → `"aGk="` |
| `crypto.base64_decode(str)` | Decode from Base64 | `crypto.base64_decode("aGk=")` → `"hi"` |

---

## 📅 `date.*` Module (9 functions)

| Function | Description |
|:---|:---|
| `date.now` | Current date/time string (UTC) |
| `date.year` | Current year |
| `date.month` | Current month (1-12) |
| `date.day` | Current day (1-31) |
| `date.hour` | Current hour (0-23, UTC) |
| `date.minute` | Current minute (0-59) |
| `date.second` | Current second (0-59) |
| `date.unix` | Unix timestamp (seconds) |
| `date.unix_ms` | Unix timestamp (milliseconds) |

---

## 🎲 `random.*` Module (6 functions)

| Function | Description |
|:---|:---|
| `random.random` | Random float 0.0–1.0 |
| `random.randint(min, max)` | Random integer in range |
| `random.choice(list)` | Random element |
| `random.sample(list, n)` | N random elements |
| `random.uuid` | Generate UUID v4 string |
| `random.shuffle(list)` | Shuffle list in place |

---

## 🎬 `anime.*` Module (New v2.0!)

The `anime` module provides a powerful high-level interface for creating complex motion designs and cinematic animations directly within TechScript.

| Function | Description |
|:---|:---|
| `anime.create()` | Create a new animation context |
| `anime.render(anim)` | Render the animation to a window/browser |
| `anime.animate(target, props)` | Standard animation call |
| `anime.stagger(ms)` | Staggered timing for multiple elements |
| `anime.remove(target)` | Stop and remove animation |
| `anime.set(target, props)` | Instantly set properties |
| `anime.path(path)` | Create a motion path |
| `anime.timeline(config)` | Create complex sequenced animations |
