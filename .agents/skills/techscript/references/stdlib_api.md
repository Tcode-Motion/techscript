# TechScript Standard Library Reference

The standard library modules provide native, zero-dependency capabilities across common domains.

---

## 1. `math`
High-performance arithmetic and trigonometric operations:
- `math.abs(x)` — Absolute value
- `math.sqrt(x)` — Square root
- `math.pow(x, exp)` — Power
- `math.sin(rad)`, `math.cos(rad)`, `math.tan(rad)` — Trigonometry
- `math.floor(x)`, `math.ceil(x)`, `math.round(x)` — Rounding
- `math.min(a, b)`, `math.max(a, b)` — Extrema

```txs
dist is math.sqrt(math.pow(x2 - x1, 2) + math.pow(y2 - y1, 2))
```

---

## 2. `text`
Unicode-aware string manipulations:
- `text.len(s)` — Character length
- `text.upper(s)` — Convert to uppercase
- `text.lower(s)` — Convert to lowercase
- `text.trim(s)` — Strip leading and trailing whitespace
- `text.split(s, delimiter)` — Split into array of strings
- `text.join(array, separator)` — Join array of strings
- `text.contains(s, substring)` — Substring check
- `text.replace(s, target, replacement)` — Substring replacement

---

## 3. `collections`
Dynamic arrays and dictionaries:
- `collections.len(col)` — Count elements
- `collections.push(arr, item)` — Append item to array
- `collections.pop(arr)` — Remove and return last element
- `collections.insert(arr, index, item)` — Insert at index
- `collections.remove(arr, index)` — Remove at index
- `collections.contains(col, item)` — Membership check
- `collections.keys(map)` — List of dictionary keys
- `collections.values(map)` — List of dictionary values

---

## 4. `json`
Serialization and deserialization:
- `json.parse(str)` — Parse JSON string into TechScript data structure
- `json.stringify(val)` — Encode TechScript data structure into JSON string

```txs
raw_config is files.read("app.json")
config is json.parse(raw_config)
say "Port: {config['port']}"
```

---

## 5. `files`
Safe filesystem operations:
- `files.read(path)` — Read entire file to string
- `files.write(path, content)` — Overwrite file with content
- `files.append(path, content)` — Append content to file
- `files.exists(path)` — Check path existence
- `files.remove(path)` — Delete file

---

## 6. `http`
Synchronous and asynchronous network requests:
- `http.get(url)` — Send HTTP GET request, returns response dictionary with `status`, `body`, and `headers`
- `http.post(url, body)` — Send HTTP POST request with payload
- `http.put(url, body)` — Send HTTP PUT request
- `http.delete(url)` — Send HTTP DELETE request

---

## 7. `time`
Time, timestamps, and delays:
- `time.now()` — Current UNIX timestamp in seconds
- `time.now_ms()` — Current UNIX timestamp in milliseconds
- `time.sleep_ms(ms)` — Sleep current execution thread for milliseconds
