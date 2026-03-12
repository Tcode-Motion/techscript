# TechScript Ultimate Overhaul & Fix Guide (AI Execution Plan)

<antigravity_prompt>
## System Directive for AI Agents
You are an elite Lead Systems Engineer (Rust/Python) tasked with overhauling the TechScript language. Your objective is to achieve C-like performance in the Rust VM, fix critical memory and safety issues, ensure multi-platform compatibility (Windows, Linux, macOS, Termux), and implement a massive standard library of 300 built-in functions. The language must be "as powerful as Python, but far easier to use", featuring native web generation (`use web`, `style`, `script`), built-in tuples, sets, and instant inline code execution.
Read the extensive architectural plans below, break them down into steps, and execute them autonomously to transform TechScript into a robust, lightning-fast language.
</antigravity_prompt>

---

## Part 1: Deep Architectural Restructuring (C-Like Speed)
Currently, the Rust VM cycle is slow (~2.8s in test cases). The primary cause is the excessive use of `Rc<RefCell<Value>>` and deep cloning on the stack. We must achieve `0.00s` overhead.

### 1.1 The Memory Model: NaN Boxing & Arena Allocation
The biggest bottleneck is the current `Value` enum. Every operation invokes heap allocation and borrow-checking, which defeats the purpose of a Rust VM.
*   **Action:** Implement **NaN Boxing**. A 64-bit IEEE 754 float has 52 bits of mantissa. When the exponent is all 1s (NaN), we can use the mantissa to store pointers, booleans, or type tags.
*   **Implementation Details:**
    *   If a value is a valid `f64`, it represents a float natively.
    *   If it is a NaN, inspect the sign bit and mantissa.
    *   Tag 1: Boolean (`true` or `false` in the lowest bit).
    *   Tag 2: Null/None.
    *   Tag 3: Integer (32-bit inline).
    *   Tag 4: Object Pointer (32-bit index into the Arena).
*   **The Object Arena:** Instead of `Rc<RefCell<T>>`, create a flat `Vec<Object>` in the VM. Strings, Lists, Maps, and Instances are stored here. The `Value` on the stack is just a 64-bit integer referencing the index in this Arena.
*   **Garbage Collection:** Implement a simple Mark-and-Sweep GC. When the Arena reaches capacity, trace from the global variables and the current stack, mark reachable objects, and free the rest.

### 1.2 The Execution Stack
*   **Current Flaw:** `Vec::push` and `Vec::pop` in the main `execute()` loop force capacity checks and pointer indirection on every single instruction.
*   **Action:** Replace `stack: Vec<Value>` with a fixed-size array: `stack: [Value; 2048]`.
*   **Execution Loop:** Track a raw `stack_top: usize`. The `push` operation becomes `self.stack[self.stack_top] = val; self.stack_top += 1;`. This compiles down to raw C-like pointer arithmetic without bounds checking (if you guarantee stack depth during compilation).

### 1.3 String Interning & Constant Pool
*   **Current Flaw:** Looking up variables involves hashing dynamically allocated `String`s.
*   **Action:** Create a `StringInterner`. When the compiler encounters a string literal or variable name, it hashes it once and stores it. The compiler emits an integer ID. The VM's `globals` map changes from `HashMap<String, Value>` to `HashMap<u32, Value>`, making variable lookups near-instantaneous.

---

## Part 2: Critical Safety, Logic, & Pipeline Fixes

### 2.1 The `unsafe` Transmute Vulnerability
*   **File:** `runtime/src/vm.rs`
*   **Issue:** `let op: OpCode = unsafe { std::mem::transmute(byte) };`
*   **Risk:** If bytecode is corrupted or a jump offset points to an argument instead of an opcode, this causes Undefined Behavior (UB) and a hard crash (Segmentation Fault).
*   **Fix:** Implement `TryFrom<u8>` for `OpCode`. Use a safe `match byte` to convert it. If it fails, halt the VM gracefully with a `TechError::InvalidBytecode` message.

### 2.2 Control Flow Fixes (Break, Continue, Operators)
*   **Break/Continue Logic:** The `Break` statement currently compiles as `OpCode::Return`.
    *   **Fix in `compiler.rs`:** Introduce a `LoopScope` struct tracking the current loop's start instruction index and a list of jump instruction indices that need to be patched at the end of the loop.
    *   `Continue` should emit an unconditional jump backward to the loop start.
    *   `Break` should emit an unconditional jump forward, to be patched when the loop ends.
*   **Missing Operators:** Implement `is` (identity check, comparing Arena IDs, not values), `in` (collection containment via a native loop), and `typeof` (returning an interned string of the object type) in `vm.rs`.

### 2.3 Windows PATH Destruction (Installer)
*   **File:** `scripts/setup.bat`
*   **Issue:** `setx PATH "!SCRIPTS_DIR!;%PATH%"` destroys user PATHs longer than 1024 characters, potentially breaking Windows installations.
*   **Fix:** Replace the `.bat` PATH injection. Call a PowerShell one-liner that uses `.NET` APIs, which do not have the 1024 limit:
    `powershell -Command "[Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')"`

---

## Part 3: New Syntax & Core Features

### 3.1 Instant Inline Code Execution (`[[[ ... ]]]`)
*   **Goal:** Allow users to run code directly from the terminal without creating a `.txs` file. Ideal for quick math, testing features, or pipeline scripting.
*   **Command Format:** `tech "[[[ say 'hello world' ]]]"`
*   **Implementation:**
    *   In `runtime/src/main.rs`, parse `std::env::args()`.
    *   If `args[1]` starts with `[[[` and ends with `]]]`, slice the inner string.
    *   Pass this string directly to `Lexer::new(inner_code, "<inline>")`.
    *   Bypass file I/O entirely and invoke the VM immediately.

### 3.2 Tuples Data Type (Immutable Arrays)
*   **Goal:** Provide read-only arrays, identical to Python.
*   **Implementation:** Add `OpCode::BuildTuple`. Tuples use `(1, 2, 3)` syntax. They differ from lists `[1, 2, 3]` because they lack mutability methods (`append`, `pop`). Because their size is fixed, they can be allocated more tightly in the Arena, making them significantly faster and safer for passing fixed data.

---

## Part 4: Native Web Module Architecture
The `use web` functionality must be natively integrated into Rust, utilizing high-speed HTTP servers to generate apps instantly, bypassing the need for Python's `http.server`.

### 4.1 UI Builder Functions (DOM Generation)
The `WebPage` object must provide chained, easy-to-read DOM generators.
*   `page = WebPage("App")`
*   `page.style("body", {"background": "#000", "color": "#fff"})`
*   `page.script("console.log('App loaded');")`
*   `page.body([ h1("Hello"), button("Click Me") ])`
*   **Security:** All HTML content injected via text arguments must be escaped automatically (converting `<` to `&lt;`, etc.) to prevent XSS. A special `web.raw()` function can bypass this.

### 4.2 High-Speed Local Server
*   **Action:** Add the `tiny_http` crate to `Cargo.toml`.
*   **Execution:** `page.run(8080)` spawns a background thread running a TCP server. It serves the rendered HTML from memory, avoiding temporary file creation.
*   **Security:** It must bind strictly to `127.0.0.1` (localhost) to prevent local network snooping, a flaw present in the original Python implementation.

---

## Part 5: The 300 Standard Library Functions Directory
To make TechScript as powerful as Python but far easier to use, the Rust runtime must pre-register these 300 built-in functions. They must be injected into the VM's global environment upon startup.

### 5.1 System, Core & Reflection (25 Functions)
1. `print(*args)`: Print values to stdout without a newline.
2. `say(*args)`: Print values to stdout with a newline.
3. `input(prompt)`: Ask user for CLI input and block.
4. `type(obj)`: Return the string type name of an object.
5. `id(obj)`: Return the unique memory arena ID of an object.
6. `hash(obj)`: Return a deterministic hash integer.
7. `len(obj)`: Return length of string, list, map, or tuple.
8. `eval(str)`: Evaluate a string as TechScript code.
9. `exit(code)`: Terminate the VM cleanly.
10. `sleep(ms)`: Pause execution for N milliseconds.
11. `time()`: Return current UNIX timestamp in seconds.
12. `time_ms()`: Return current UNIX timestamp in milliseconds.
13. `clear()`: Clear the terminal screen (cross-platform).
14. `assert(cond, msg)`: Throw error if condition is false.
15. `panic(msg)`: Abort the program immediately.
16. `is_defined(name)`: Check if a global variable exists.
17. `clone(obj)`: Deep copy an object.
18. `address_of(obj)`: Get memory address (for debugging).
19. `gc_collect()`: Force a garbage collection cycle.
20. `version()`: Return the TechScript engine version string.
21. `callable(obj)`: True if object is a function or method.
22. `hasattr(obj, name)`: True if object has property.
23. `getattr(obj, name)`: Get property by string name.
24. `setattr(obj, name, val)`: Set property by string name.
25. `delattr(obj, name)`: Delete property by string name.

### 5.2 Math & Numbers (45 Functions)
26. `math.abs(n)`: Absolute value.
27. `math.round(n, places)`: Round to decimal places.
28. `math.floor(n)`: Round down to nearest int.
29. `math.ceil(n)`: Round up to nearest int.
30. `math.trunc(n)`: Truncate decimal part.
31. `math.min(a, b)`: Lowest of two numbers.
32. `math.max(a, b)`: Highest of two numbers.
33. `math.clamp(x, min, max)`: Restrict value to range.
34. `math.sum(list)`: Sum of all numbers in list.
35. `math.sqrt(n)`: Square root.
36. `math.cbrt(n)`: Cube root.
37. `math.pow(base, exp)`: Exponential power.
38. `math.exp(n)`: Euler's number raised to power.
39. `math.log(n)`: Natural logarithm.
40. `math.log10(n)`: Base-10 logarithm.
41. `math.log2(n)`: Base-2 logarithm.
42. `math.sin(rad)`: Sine.
43. `math.cos(rad)`: Cosine.
44. `math.tan(rad)`: Tangent.
45. `math.asin(n)`: Arc sine.
46. `math.acos(n)`: Arc cosine.
47. `math.atan(n)`: Arc tangent.
48. `math.atan2(y, x)`: Arc tangent with two parameters.
49. `math.sinh(rad)`: Hyperbolic sine.
50. `math.cosh(rad)`: Hyperbolic cosine.
51. `math.tanh(rad)`: Hyperbolic tangent.
52. `math.degrees(rad)`: Convert radians to degrees.
53. `math.radians(deg)`: Convert degrees to radians.
54. `math.is_nan(n)`: Check if Not-a-Number.
55. `math.is_inf(n)`: Check if Infinite.
56. `math.is_even(n)`: Check if integer is even.
57. `math.is_odd(n)`: Check if integer is odd.
58. `math.sign(n)`: Returns -1, 0, or 1.
59. `math.gcd(a, b)`: Greatest common divisor.
60. `math.lcm(a, b)`: Least common multiple.
61. `math.factorial(n)`: Factorial of integer.
62. `math.mean(list)`: Average of list.
63. `math.median(list)`: Median of list.
64. `math.mode(list)`: Most common value.
65. `math.comb(n, k)`: Number of combinations.
66. `math.perm(n, k)`: Number of permutations.
67. `math.hypot(x, y)`: Euclidean distance.
68. `math.isclose(a, b, tol)`: True if a and b are close.
69. `math.PI`: Constant 3.1415926535...
70. `math.E`: Constant 2.7182818284...

### 5.3 Strings (45 Functions)
71. `str.upper()`: Convert to uppercase.
72. `str.lower()`: Convert to lowercase.
73. `str.title()`: Capitalize first letter of each word.
74. `str.capitalize()`: Capitalize first letter of string.
75. `str.swapcase()`: Swap case of all letters.
76. `str.trim()`: Remove leading/trailing whitespace.
77. `str.trim_left()`: Remove leading whitespace.
78. `str.trim_right()`: Remove trailing whitespace.
79. `str.split(sep)`: Split string into list.
80. `str.rsplit(sep)`: Split string from right.
81. `str.split_lines()`: Split string by newline.
82. `str.join(list)`: Join list of strings.
83. `str.replace(old, new)`: Replace substring.
84. `str.replace_all(old, new)`: Replace all substrings.
85. `str.find(sub)`: Find index of substring (returns -1 if not found).
86. `str.rfind(sub)`: Find index from right.
87. `str.index(sub)`: Find index (throws error if not found).
88. `str.rindex(sub)`: Find index from right (throws error).
89. `str.count(sub)`: Count occurrences of substring.
90. `str.starts_with(sub)`: Check prefix.
91. `str.ends_with(sub)`: Check suffix.
92. `str.contains(sub)`: Check if substring exists.
93. `str.is_alpha()`: Check if only letters.
94. `str.is_digit()`: Check if only numbers.
95. `str.is_alnum()`: Check if letters and numbers.
96. `str.is_space()`: Check if only whitespace.
97. `str.is_lower()`: Check if fully lowercase.
98. `str.is_upper()`: Check if fully uppercase.
99. `str.is_ascii()`: Check if ASCII characters only.
100. `str.is_printable()`: Check if characters are printable.
101. `str.pad_left(len, char)`: Pad string on left.
102. `str.pad_right(len, char)`: Pad string on right.
103. `str.center(len, char)`: Center string with padding.
104. `str.zfill(len)`: Pad with zeros on the left.
105. `str.repeat(n)`: Repeat string N times.
106. `str.reverse()`: Reverse characters.
107. `str.chars()`: Convert to list of characters.
108. `str.bytes()`: Convert to list of ASCII bytes.
109. `str.format(*args)`: Inject variables into template.
110. `str.remove_prefix(sub)`: Strip prefix if exists.
111. `str.remove_suffix(sub)`: Strip suffix if exists.
112. `str.to_int()`: Parse to integer.
113. `str.to_float()`: Parse to float.
114. `str.is_empty()`: Check if length is 0.
115. `str.expandtabs(size)`: Replace tabs with spaces.

### 5.4 Lists & Arrays (35 Functions)
116. `list.push(val)`: Append item to end.
117. `list.pop()`: Remove and return last item.
118. `list.unshift(val)`: Insert item at beginning.
119. `list.shift()`: Remove and return first item.
120. `list.insert(idx, val)`: Insert at specific index.
121. `list.remove(val)`: Remove first occurrence of value.
122. `list.remove_at(idx)`: Remove at specific index.
123. `list.clear()`: Empty the list.
124. `list.index(val)`: Get index of value.
125. `list.count(val)`: Count occurrences.
126. `list.sort()`: Sort list ascending.
127. `list.sort_desc()`: Sort list descending.
128. `list.reverse()`: Reverse list in-place.
129. `list.copy()`: Shallow copy list.
130. `list.deep_copy()`: Deep copy list.
131. `list.slice(start, end)`: Return sub-list.
132. `list.map(fn)`: Apply function to all items.
133. `list.filter(fn)`: Keep items matching function.
134. `list.reduce(fn, init)`: Reduce list to single value.
135. `list.any(fn)`: True if any item matches.
136. `list.all(fn)`: True if all items match.
137. `list.find(fn)`: Return first matching item.
138. `list.find_index(fn)`: Return index of first match.
139. `list.flat()`: Flatten nested lists 1 level.
140. `list.flat_deep()`: Flatten nested lists completely.
141. `list.chunk(size)`: Split into lists of max size.
142. `list.unique()`: Remove duplicates.
143. `list.join(sep)`: Join to string.
144. `list.is_empty()`: Check if empty.
145. `list.first()`: Get first item (or none).
146. `list.last()`: Get last item (or none).
147. `list.extend(other_list)`: Append another list.
148. `list.fill(val, start, end)`: Fill indices with value.
149. `list.swap(i, j)`: Swap elements at indices.
150. `list.shuffle()`: Randomize order in-place.

### 5.5 Dictionaries / Maps (25 Functions)
151. `map.keys()`: Return list of keys.
152. `map.values()`: Return list of values.
153. `map.entries()`: Return list of [key, value] pairs.
154. `map.get(key, default)`: Get value or default if missing.
155. `map.set(key, val)`: Insert or update key.
156. `map.has(key)`: Check if key exists.
157. `map.remove(key)`: Delete key and return value.
158. `map.clear()`: Empty the map.
159. `map.copy()`: Shallow copy map.
160. `map.deep_copy()`: Deep copy map.
161. `map.merge(other)`: Combine with another map.
162. `map.update(other)`: In-place merge.
163. `map.is_empty()`: Check if empty.
164. `map.invert()`: Swap keys and values.
165. `map.filter(fn)`: Filter keys/values by function.
166. `map.map_values(fn)`: Apply function to values.
167. `map.group_by(list, fn)`: Create map grouped by function result.
168. `map.from_entries(list)`: Create map from key-value lists.
169. `map.pop(key)`: Remove key and return value.
170. `map.pop_item()`: Remove and return random key-value pair.
171. `map.set_default(key, val)`: Set if missing, return value.
172. `map.keys_str()`: Ensure all keys are cast to strings.
173. `map.values_str()`: Ensure all values are cast to strings.
174. `map.pick(keys_list)`: Create new map with only selected keys.
175. `map.omit(keys_list)`: Create new map omitting selected keys.

### 5.6 Tuples & Sets (25 Functions)
176. `tuple(*args)`: Create a tuple dynamically.
177. `tuple.count(val)`: Count occurrences.
178. `tuple.index(val)`: Find index.
179. `tuple.slice(start, end)`: Get sub-tuple.
180. `tuple.first()`: Get first item.
181. `tuple.last()`: Get last item.
182. `tuple.to_list()`: Convert to mutable list.
183. `tuple.contains(val)`: Check if exists.
184. `tuple.is_empty()`: Check if empty.
185. `tuple.unpack()`: Returns items for multi-assignment.
186. `set(*args)`: Create a unique Set.
187. `set.add(val)`: Add item.
188. `set.remove(val)`: Remove item.
189. `set.has(val)`: True if exists.
190. `set.union(other)`: Combine two sets.
191. `set.intersection(other)`: Common items.
192. `set.difference(other)`: Items only in first set.
193. `set.symmetric_difference(other)`: Items in either, not both.
194. `set.is_subset(other)`: True if all items in other.
195. `set.is_superset(other)`: True if contains all of other.
196. `set.is_disjoint(other)`: True if no common items.
197. `set.clear()`: Empty set.
198. `set.copy()`: Shallow copy.
199. `set.pop()`: Remove random item.
200. `set.to_list()`: Convert to list.

### 5.7 Random & Crypto (15 Functions)
201. `random.random()`: Float between 0.0 and 1.0.
202. `random.randint(min, max)`: Random integer in range.
203. `random.randfloat(min, max)`: Random float in range.
204. `random.choice(list)`: Pick random item from list.
205. `random.shuffle(list)`: Randomize list order in-place.
206. `random.sample(list, n)`: Pick N unique random items.
207. `random.seed(n)`: Set PRNG seed.
208. `random.boolean()`: Return true or false randomly.
209. `random.bytes(n)`: Return N random bytes.
210. `random.uuid()`: Generate a random UUID string v4.
211. `crypto.md5(str)`: MD5 hash string.
212. `crypto.sha1(str)`: SHA-1 hash.
213. `crypto.sha256(str)`: SHA-256 hash.
214. `crypto.base64_encode(str)`: Encode Base64.
215. `crypto.base64_decode(str)`: Decode Base64.

### 5.8 File System & IO (25 Functions)
216. `fs.read(path)`: Read whole file as string.
217. `fs.write(path, content)`: Write string to file (overwrite).
218. `fs.append(path, content)`: Append string to file.
219. `fs.read_lines(path)`: Read file into list of lines.
220. `fs.write_lines(path, list)`: Write list of lines to file.
221. `fs.read_bytes(path)`: Read file into byte array.
222. `fs.write_bytes(path, bytes)`: Write byte array to file.
223. `fs.exists(path)`: Check if path exists.
224. `fs.is_file(path)`: Check if path is a file.
225. `fs.is_dir(path)`: Check if path is a directory.
226. `fs.list_dir(path)`: Get list of files in directory.
227. `fs.make_dir(path)`: Create directory.
228. `fs.make_dirs(path)`: Create directory and parents.
229. `fs.remove_file(path)`: Delete a file.
230. `fs.remove_dir(path)`: Delete an empty directory.
231. `fs.remove_all(path)`: Delete directory and contents.
232. `fs.rename(old, new)`: Rename file or directory.
233. `fs.copy(src, dest)`: Copy file.
234. `fs.size(path)`: Get file size in bytes.
235. `fs.cwd()`: Get current working directory.
236. `fs.chdir(path)`: Change current working directory.
237. `fs.abspath(path)`: Get absolute path.
238. `fs.extname(path)`: Get file extension.
239. `fs.basename(path)`: Get file name.
240. `fs.dirname(path)`: Get directory name from path.

### 5.9 System, OS & Date/Time (20 Functions)
241. `os.env_get(key)`: Get environment variable.
242. `os.env_set(key, val)`: Set environment variable.
243. `os.env_del(key)`: Delete environment variable.
244. `os.system(cmd)`: Execute shell command.
245. `os.popen(cmd)`: Execute shell command and return stdout.
246. `os.name()`: Return 'windows', 'linux', 'macos', or 'android'.
247. `os.arch()`: Return 'x86_64', 'arm64', etc.
248. `os.cpu_count()`: Get number of logical CPU cores.
249. `os.pid()`: Get process ID.
250. `os.args()`: Return list of CLI arguments passed to script.
251. `date.now()`: Current Date object.
252. `date.year(d)`: Year integer.
253. `date.month(d)`: Month integer.
254. `date.day(d)`: Day integer.
255. `date.hour(d)`: Hour integer.
256. `date.minute(d)`: Minute integer.
257. `date.second(d)`: Second integer.
258. `date.format(d, fmt)`: Format date to string.
259. `date.parse(str, fmt)`: Parse string to Date object.
260. `date.unix(d)`: Convert Date to UNIX timestamp.

### 5.10 Web, HTTP & JSON Native Framework (40 Functions)
261. `web.WebPage(title)`: Initialize a new WebPage object.
262. `web.serve(port)`: Start local web server.
263. `web.style(selector, rules_map)`: Add CSS rules.
264. `web.script(js_code)`: Add raw JS.
265. `web.body(elements_list)`: Inject HTML into body.
266. `web.h1(text, attrs)`: Create `<h1>` element.
267. `web.h2(text, attrs)`: Create `<h2>` element.
268. `web.h3(text, attrs)`: Create `<h3>` element.
269. `web.h4(text, attrs)`: Create `<h4>` element.
270. `web.h5(text, attrs)`: Create `<h5>` element.
271. `web.h6(text, attrs)`: Create `<h6>` element.
272. `web.p(text, attrs)`: Create `<p>` element.
273. `web.div(children, attrs)`: Create `<div>` element.
274. `web.span(text, attrs)`: Create `<span>` element.
275. `web.button(text, attrs)`: Create `<button>` element.
276. `web.input(attrs)`: Create `<input>` element.
277. `web.img(attrs)`: Create `<img>` element.
278. `web.a(text, attrs)`: Create `<a>` element.
279. `web.ul(items, attrs)`: Create `<ul>` element.
280. `web.ol(items, attrs)`: Create `<ol>` element.
281. `web.li(text, attrs)`: Create `<li>` element.
282. `web.table(rows, attrs)`: Create `<table>` element.
283. `web.tr(cells, attrs)`: Create `<tr>` element.
284. `web.td(text, attrs)`: Create `<td>` element.
285. `web.th(text, attrs)`: Create `<th>` element.
286. `web.form(children, attrs)`: Create `<form>` element.
287. `web.br()`: Create `<br>` element.
288. `web.hr()`: Create `<hr>` element.
289. `web.raw(html)`: Inject raw, unescaped HTML.
290. `web.iframe(attrs)`: Create `<iframe>` element.
291. `http.get(url, headers)`: Send HTTP GET request.
292. `http.post(url, body, headers)`: Send HTTP POST request.
293. `http.put(url, body, headers)`: Send HTTP PUT request.
294. `http.delete(url, headers)`: Send HTTP DELETE request.
295. `http.patch(url, body, headers)`: Send HTTP PATCH request.
296. `http.serve_api(port, routes_map)`: Start a JSON REST API server.
297. `json.encode(obj)`: Convert map/list to JSON string.
298. `json.encode_pretty(obj)`: Convert to formatted JSON string.
299. `json.decode(str)`: Convert JSON string to map/list.
300. `url.encode(str)`: URL encode a string.

---

## Part 6: Multi-Platform Support & Installation Matrix

### 6.1 Termux (Android) Build Process
When users attempt `pip install techscript` on Android Termux, the installation often fails. This is because Termux requires C/Rust compilers natively to build dependencies that lack pre-compiled Android wheels.
*   **Documentation Requirement:** Create a specific `TERMUX.md` guide.
*   **Required Commands for Users:**
    ```bash
    pkg update && pkg upgrade
    pkg install python rust binutils clang
    pip install techscript
    ```

### 6.2 Linux & macOS Distribution Generation
To support platforms beyond Windows without requiring users to install Python, cross-compilation is necessary.
*   **Action:** Update `build_release_repo.py` to utilize Rust's `cargo build --release` targeting specific architectures.
*   **Targets:**
    *   `x86_64-unknown-linux-gnu` (Standard Linux)
    *   `aarch64-apple-darwin` (Apple Silicon Mac)
    *   `x86_64-apple-darwin` (Intel Mac)
*   **Distribution:** Zip the resulting binaries and offer them in the GitHub release page alongside the existing `.exe`.

---

<antigravity_plan>
## AI Execution Checklist (Step-by-Step)

### Phase 1: High-Speed Refactor (C-Like Speed)
- [ ] In `runtime/src/value.rs`, completely strip out `Rc<RefCell<HashMap>>` and `Rc<RefCell<Vec>>`.
- [ ] Implement an `Arena` struct in `vm.rs` containing `Vec<Object>`.
- [ ] Convert the `Value` enum to use NaN Boxing or Tagged Unions for primitive types.
- [ ] Convert the `stack` in `VM` from a dynamically sized `Vec<Value>` to a statically sized array `[Value; 2048]`.

### Phase 2: Bug Fixes & Security Enhancements
- [ ] In `runtime/src/opcode.rs`, implement `TryFrom<u8>` for the `OpCode` enum.
- [ ] In `runtime/src/vm.rs`, remove `unsafe { std::mem::transmute(byte) }` and use `OpCode::try_from(byte)`.
- [ ] In `runtime/src/compiler.rs`, implement loop tracking to ensure `Break` and `Continue` compile to appropriate jumps, not returns.
- [ ] Update `scripts/setup.bat` to replace `setx PATH` with safe PowerShell registry manipulation to avoid truncating Windows PATH variables.

### Phase 3: Implement the 300 Standard Functions
- [ ] Set up a robust macro in `builtins.rs` to map the 300 functions to VM globals.
- [ ] Implement the `math` and `crypto` modules natively in Rust using `f64` operations and crates like `md5`, `sha2`.
- [ ] Implement all 45 `str` methods natively in Rust, mapping back to interned string IDs.
- [ ] Implement all functional methods (`map`, `filter`, `reduce`) for lists, maps, tuples, and sets.
- [ ] Build the `fs` module ensuring `std::path::Path` is utilized for OS-agnostic path parsing.

### Phase 4: Web UI Native Implementation
- [ ] Add `tiny_http` and `serde_json` to `runtime/Cargo.toml`.
- [ ] Port the `web.py` functionality into a new `runtime/src/web.rs` file.
- [ ] Implement all 30 HTML DOM generator methods in Rust (`web.h1`, `web.div`, etc.), ensuring automatic HTML escaping.

### Phase 5: Inline Execution
- [ ] In `runtime/src/main.rs`, capture CLI arguments matching the exact pattern `[[[` ... `]]]`.
- [ ] Slice the string, feed it to `Lexer::new()`, compile, and execute it directly, bypassing file reads.
</antigravity_plan>