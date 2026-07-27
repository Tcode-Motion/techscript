# Standard Library Reference

This reference documents the standard library modules available in TechScript 2.0. Standard modules must be imported using `use`.

---

## 🔢 `math` Module
Mathematical constants and functions.
```txs
use math
```
* **`math.sqrt(x)`**: Returns the square root of `x`.
* **`math.abs(x)`**: Returns the absolute value of `x`.
* **`math.pow(base, exp)`**: Returns `base` raised to the power of `exp`.
* **`math.parse_int(str)`**: Parses a string to an integer.

---

## 📁 `fs` Module
File system utilities.
```txs
use fs
```
* **`fs.read(path)`**: Reads file contents as a string.
* **`fs.write(path, data)`**: Writes string data to a file.
* **`fs.exists(path)`**: Returns `true` if the path exists.
* **`fs.delete(path)`**: Deletes the specified file.

---

## 💻 `os` Module
Operating system integration.
```txs
use os
```
* **`os.name()`**: Returns OS family name (`"windows"`, `"linux"`, `"macos"`).
* **`os.env_get(key)`**: Reads environment variable `key`.
* **`os.system(cmd)`**: Runs a system command shell.

---

## 📄 `json` Module
JSON parsing and serialization.
```txs
use json
```
* **`json.encode(val)`**: Serializes values/maps into a JSON string.
* **`json.decode(str)`**: Deserializes a JSON string into a map or list.

---

## 🔐 `crypto` Module
Cryptography utilities.
```txs
use crypto
```
* **`crypto.sha256(str)`**: Returns the SHA-256 hash of a string.
* **`crypto.md5(str)`**: Returns the MD5 hash of a string.
* **`crypto.base64_encode(str)`**: Encodes string to Base64.
* **`crypto.base64_decode(str)`**: Decodes Base64 to string.

---

## 🎲 `random` Module
Pseudo-random generator.
```txs
use random
```
* **`random.randint(min, max)`**: Returns a random integer in range `[min, max]`.
* **`random.random()`**: Returns a float in range `[0.0, 1.0)`.
* **`random.uuid()`**: Generates a random UUIDv4 string.
