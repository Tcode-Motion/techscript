# TechScript (TX) — Complete Guide

**Purpose:** A single, complete reference and implementation guide for **TechScript (TX)** — a Python-like, compact, beginner-friendly scripting language using `.tx` files and short symbols. This document contains design goals, syntax, interpreter architecture, error-handling & tooling, and a reference list of **200 common commands/functions** with meaning, syntax, usage, and brief implementation notes.

> Quick note on references: TX aims to be simple like entity["programming_language","Python","high-level language"], but with a compact, readable symbol-first surface syntax and helpful error messages. Use `.tx` as the file extension.

---

## Table of Contents

1. Overview & Goals
2. Core Language Design
3. File & Execution Model
4. Lexical tokens & grammar summary
5. Error messages & user-friendly hints
6. Interpreter architecture (reference implementation notes)
7. Tooling & developer ergonomics
8. 200 Common TX functions/keywords — meaning, syntax, example, implementation hint
9. Implementation checklist & roadmap
10. Testing, performance & packaging
11. Appendix: Examples & cheatsheet

---

# 1. Overview & Goals

* **Readable:** Syntax looks like simple English expressions and compact symbols. New users who know basic English should read/write TX easily.
* **Concise:** Use short symbols/operators (e.g. `?`, `say`, `+=`, `<<`) for common tasks.
* **Actionable errors:** Clear, short error messages and "Did you mean..." suggestions.
* **Fast iteration:** Implement TX as either (A) a transpiler to entity["programming_language","Python","high-level language"] for rapid prototyping, or (B) a bytecode VM for performance.
* **Extensible:** Easy to add new builtins and a standard library.

# 2. Core Language Design

* File extension: `.tx`
* Encoding: UTF-8
* Line endings: LF (\n)
* Comments:

  * Single-line: `# this is a comment`
  * Multiline: `''' multi\nline '''` (optional)
* Blocks: Indentation *or* explicit `end` keyword. For simplicity, TX uses `end` for all blocks.

## Minimal primitives

* Types: number (float), int (optional), string, bool, list, map (dict), function, null
* Operators: `+ - * / % ^` (power), `== != > < >= <=`
* Short input: `? "prompt"` returns string (cast with `num()` or `int()` as needed)
* Output: `say "text"` or `say expr`

# 3. File & Execution Model

* A `.tx` file is parsed into an AST, optionally compiled to bytecode, then executed by a VM or transpiled to Python for immediate execution.
* Entry: top-level statements run in global frame.
* Modules: `import "mylib.tx"` (future feature); start simple with single-file execution.

# 4. Lexical tokens & grammar summary (short)

* Tokens: IDENT (letters/underscores), NUMBER, STRING, SYMBOLS (`=`, `:=`, `+=`, `-=` ...), KEYWORDS (`if`, `else`, `loop`, `fn`, `end`, `return`, `say`, `?`)
* Statement forms: `assign`, `expr`, `if`, `loop`, `fn`, `return`, `say`, `io`
* Sample grammar (informal):

  * program := stmt*
  * stmt := assign | expr_stmt | if_stmt | loop_stmt | fn_decl | return_stmt | io_stmt

# 5. Error messages & user-friendly hints

* Always print: **Error type**, **line number**, **snippet**, **short hint**.
* Examples:

  * `SyntaxError: unexpected '+' at line 3\n  say a +\n  Hint: missing right-hand operand after '+'`.
  * `NameError: unknown variable 'nmae' at line 1\n  Hint: Did you mean 'name' ?`.
* Provide `Did you mean` suggestions by checking edit distance (Levenshtein) against available keywords/identifiers.

# 6. Interpreter architecture (reference implementation notes)

A recommended, pragmatic 3-stage implementation for v1:

1. **Lexer** — tokenizes input into tokens.
2. **Parser** — builds AST. Use a simple recursive descent parser since TX grammar is small.
3. **Executor** — either evaluate AST directly (interpreter) or compile AST to a small bytecode then execute on a VM.

Implementation tips:

* Keep AST node types small: `Program`, `ExprStmt`, `Assign`, `BinaryOp`, `If`, `Loop`, `FnDef`, `Call`, `Return`, `Variable`, `Literal`.
* Represent values as a small union/dict: `{type: 'num'|'str'|'list'|..., value: ...}`
* Builtins table: map of name -> function(value list, env) for `say`, `num`, `len`, etc.
* Error handling: Parser throws `SyntaxError(line, msg)`; main loop catches and prints formatted hint.

# 7. Tooling & developer ergonomics

* REPL: interactive `tx` shell with history and multiline support.
* `tx run file.tx` — run file
* `tx check file.tx` — static check & lint (basic)
* Syntax highlighting: add TextMate grammar for VS Code (extension later). Use `.tx` extension.
* Package: publish interpreter on PyPI as `tx-interpreter` (if implemented in Python) or as a single binary via PyInstaller.

---

# 8. TX Reference — 200 common functions/keywords

Below is a compact but complete reference of **200 TX commands / builtins / keywords**. For each item: **Name / symbol**, **Meaning**, **Syntax**, **Short example**, **Implementation note** (1-line).

> Style: stick to short descriptions. If you want an expanded example for any entry, ask and I'll add a page for that feature.

1. `say` — print output. Syntax: `say expr` or `say "text"`. Example: `say "hi"`. Impl: call builtin `print`.
2. `?` — input prompt. Syntax: `name = ? "prompt"`. Example: `n = ? "name"`. Impl: call `input()`; return string.
3. `=` — assign. Syntax: `x = 5`. Example: `a = 10`. Impl: store in current env.
4. `:=` — reassign (explicit). Syntax same as `=`. Impl: same as assign; use where clarity needed.
5. `+=` — add assign. Syntax: `x += 2`. Impl: load x, add, store.
6. `-=` — subtract assign. Syntax: `x -= 2`.
7. `*=` — multiply assign. Syntax: `x *= 3`.
8. `/=` — divide assign. Syntax: `x /= 4`.
9. `%=` — mod assign. Syntax: `x %= 2`.
10. `++` — increment by 1. Syntax: `x++` or `++x`. Impl: sugar for `x += 1`.
11. `--` — decrement. Syntax: `x--`.
12. `+` — addition / string concat. Syntax: `a + b`.
13. `-` — subtraction. Syntax: `a - b`.
14. `*` — multiply. Syntax: `a * b`.
15. `/` — divide. Syntax: `a / b`.
16. `%` — remainder. Syntax: `a % b`.
17. `^` — power. Syntax: `a ^ b`.
18. `==` — equals. Syntax: `a == b`.
19. `!=` — not equals. Syntax: `a != b`.
20. `>` — greater than. Syntax: `a > b`.
21. `<` — less than. Syntax: `a < b`.
22. `>=` — greater or equal.
23. `<=` — less or equal.
24. `and` — logical and. Syntax: `a and b`.
25. `or` — logical or. Syntax: `a or b`.
26. `not` — logical not. Syntax: `not a`.
27. `if` — conditional start. Syntax:

```
if condition
  ...
else
  ...
end
```

Impl: evaluate condition, run branch.
28. `else` — alternative branch.
29. `elseif` — chained conditional. Syntax: `elseif cond`.
30. `end` — close block.
31. `loop` — repeat N times. Syntax: `loop 5 ... end`. Impl: numeric iterations.
32. `while` — loop while condition. Syntax: `while cond ... end`.
33. `break` — exit loop immediately.
34. `next` — continue to next iteration.
35. `fn` — function declaration. Syntax:

```
fn name(arg1, arg2)
  ...
end
```

36. `return` — return from function. Syntax: `return expr`.

37. `call` — explicit call (usually use `name()` form). Syntax: `greet()`.

38. `->` — arrow for quick lambda. Syntax: `x -> x + 1`.

39. `len` — length of string/list. Syntax: `len s`.

40. `type` — show type. Syntax: `type x`.

41. `num` — cast to number. Syntax: `num("12")`.

42. `int` — cast to integer. Syntax: `int(3.7)`.

43. `float` — cast to float. Syntax: `float("3.14")`.

44. `str` — cast to string. Syntax: `str(5)`.

45. `bool` — cast to boolean. Syntax: `bool(0)`.

46. `null` — null value. Syntax: `x = null`.

47. `true` — boolean true.

48. `false` — boolean false.

49. `list` — create list. Syntax: `a = [1,2,3]`.

50. `map` — create map/dict. Syntax: `m = {"k":1}`.

51. `<<` — append to list. Syntax: `a << 5`.

52. `>>` — remove from list (pop). Syntax: `a >>` or `a >> idx`.

53. `in` — membership. Syntax: `if x in a`.

54. `index` — get item. Syntax: `a[0]`.

55. `slice` — slice shorthand. Syntax: `a[1:4]`.

56. `join` — join strings/list. Syntax: `join(",", list)`.

57. `split` — split string. Syntax: `split("a b")`.

58. `upper` — upper string. Syntax: `upper(s)`.

59. `lower` — lower string.

60. `replace` — replace substring. Syntax: `replace(s, "a", "b")`.

61. `trim` — trim whitespace.

62. `starts` — startswith. Syntax: `starts(s, "pre")`.

63. `ends` — endswith.

64. `contains` — string contains.

65. `format` — formatted string. Syntax: `format("hi {}", name)`.

66. `printf` — formatted print (like C). Syntax: `printf("%d", a)`.

67. `open` — open file. Syntax: `f = open("file.txt", "r")`.

68. `read` — read file to string. Syntax: `read("file.txt")`.

69. `write` — write to file. Syntax: `write("file.txt", "text")`.

70. `append` — append to file. Syntax: `append("file.txt", "line")`.

71. `close` — close file object.

72. `readln` — read line from file/IO.

73. `sleep` — wait seconds. Syntax: `sleep(1.5)`.

74. `time` — current time string. Syntax: `time()`.

75. `date` — current date.

76. `rand` — random float 0..1.

77. `randint` — random integer. Syntax: `randint(1,10)`.

78. `abs` — absolute value.

79. `floor` — floor.

80. `ceil` — ceil.

81. `round` — round number.

82. `sqrt` — square root.

83. `log` — natural log.

84. `sin` — sine.

85. `cos` — cosine.

86. `tan` — tangent.

87. `pi` — constant π.

88. `e` — constant e.

89. `map_each` — functional map. Syntax: `map_each(fn, list)`.

90. `filter` — filter list. Syntax: `filter(fn, list)`.

91. `reduce` — reduce list. Syntax: `reduce(fn, list, init)`.

92. `enumerate` — index+value pairs.

93. `range` — numeric range. Syntax: `range(1,5)`.

94. `sort` — sort list. Syntax: `sort(list)`.

95. `reverse` — reverse list.

96. `unique` — remove dups.

97. `keys` — map keys.

98. `values` — map values.

99. `items` — map items.

100. `clear` — clear list or map.

101. `copy` — shallow copy. Syntax: `b = copy(a)`.

102. `deepcopy` — deep copy.

103. `exec` — execute TX code from string. Syntax: `exec("say 'hi'")`. Impl: feed parser/evaluator.

104. `eval` — evaluate expression string. Syntax: `eval("1+2")`.

105. `shell` — run shell command and get output. Syntax: `shell("ls")`.

106. `env` — environment variables map. Syntax: `env["PATH"]`.

107. `exit` — terminate program.

108. `throw` — raise error. Syntax: `throw "msg"`.

109. `try` — try/catch block. Syntax:

```
try
  ...
catch e
  ...
end
```

110. `catch` — catch error.
111. `finally` — final block (optional).
112. `debug` — print debug info. Syntax: `debug var`.
113. `log` — append to log file.
114. `profile` — start profiler (dev use).
115. `assert` — assert condition.
116. `match` — pattern match (simple). Syntax:

```
match val
  case x: ...
  case _: ...
end
```

117. `case` — branch inside match.

118. `panic` — fatal error (dev).

119. `is` — identity / type check. Syntax: `if x is null`.

120. `typeof` — textual type name.

121. `bind` — bind function to context.

122. `lambda` — anonymous function. Syntax: `lambda(x) -> x*2`.

123. `memo` — memoize function.

124. `pipe` — pipe operator. Syntax: `val | fn`.

125. `tap` — inspect in chain. Syntax: `val |> tap(fn)`.

126. `json_encode` — serialize to JSON.

127. `json_decode` — parse JSON.

128. `http_get` — simple HTTP GET. Syntax: `http_get(url)`.

129. `http_post` — HTTP POST.

130. `url_encode` — encode URL string.

131. `base64_encode` — base64 encode.

132. `base64_decode` — decode.

133. `crypto_hash` — hash function (sha256).

134. `sleep_ms` — sleep in ms.

135. `mkdir` — create directory.

136. `rmdir` — remove directory.

137. `exists` — file exists check. Syntax: `exists("file")`.

138. `stat` — file metadata.

139. `pwd` — print working dir.

140. `cd` — change working dir.

141. `ls` — list directory.

142. `chmod` — change permissions.

143. `chown` — change owner (optional).

144. `uuid` — generate UUID.

145. `date_parse` — parse date string.

146. `strftime` — format time.

147. `parse_int` — parse int safely.

148. `parse_float` — parse float safely.

149. `isnum` — test numeric.

150. `ismap` — test map.

151. `islist` — test list.

152. `istrue` — truthiness.

153. `isnull` — test null.

154. `sleep_until` — sleep until timestamp.

155. `watch` — watch file changes (dev).

156. `on_event` — event handler register.

157. `emit` — emit event.

158. `websocket_open` — open ws connection.

159. `ws_send` — send ws message.

160. `ws_close` — close ws.

161. `xml_parse` — parse XML.

162. `yaml_parse` — parse YAML.

163. `csv_read` — read CSV file.

164. `csv_write` — write CSV.

165. `db_open` — open simple DB (sqlite).

166. `db_query` — run query.

167. `db_close` — close DB.

168. `encrypt` — symmetric encrypt.

169. `decrypt` — symmetric decrypt.

170. `compress` — compress data.

171. `decompress` — decompress.

172. `signal` — send OS signal.

173. `fork` — fork process (posix only).

174. `thread` — spawn new thread.

175. `mutex` — create mutex.

176. `lock` — lock mutex.

177. `unlock` — unlock.

178. `atomic_add` — atomic ops.

179. `cache_get` — get cached value.

180. `cache_set` — set cached value.

181. `metrics` — expose metrics counter.

182. `prometheus_push` — push metrics.

183. `observe` — observe histogram.

184. `ssl_connect` — TLS client connect.

185. `cert_load` — load certificate.

186. `service_start` — start background service.

187. `service_stop` — stop service.

188. `health_check` — basic health check.

189. `spawn` — spawn child process.

190. `join` — wait for spawned process.

191. `watchdog` — restart on failure.

192. `hot_reload` — reload module at runtime.

193. `version` — show interpreter version.

194. `help` — show help text.

195. `alias` — alias command/name.

196. `config_load` — load config file.

197. `config_get` — get config value.

198. `config_set` — set config value.

199. `license` — show license text.

200. `todo` — mark TODO at runtime (dev log).

---

# 9. Implementation notes for the common list

* **Builtins table:** For items 1..200, implement as entries in a builtin dictionary mapping token to host function. Keep I/O, filesystem, and networking behind safe flags for security.
* **Casting & type coercion:** Implement `num()`, `int()`, `str()` to control coercion. For arithmetic, attempt numeric coercion, else raise `TypeError` with helpful hint.
* **Parser & precedence:** Standard precedence: `^` highest, then `* / %`, then `+ -`, then comparison, then `and/or`, then assignment.
* **Short-circuiting:** `and`/`or` must short-circuit.
* **Syntactic sugar:** Implement `x++` and `x--` as transforms during parsing.
* **List ops:** `a << v` transforms into `a.append(v)` in the interpreter.
* **File IO:** Provide safe wrappers that open/close; recommend using context objects.
* **Networking:** Provide simple wrappers (`http_get`) backed by builtin library calls; keep async versions optional.

# 10. Implementation checklist & roadmap

**Phase 0 — design & prototype (days)**

* Language spec (this doc)
* Minimal lexer + parser (recursive descent)
* Implement 30 core builtins: `say, ?, =, +, -, *, /, if, else, loop, fn, return, list ops, len, type, num, str, int, json`, `read`, `write`, `sleep`.
* REPL with history

**Phase 1 — standard library & tooling (weeks)**

* Expand to 100 builtins (add file, math, date, random)
* Transpiler to entity["programming_language","Python","high-level language"] for fast iteration
* Basic linter and `tx check`
* Unit tests & fixtures

**Phase 2 — performance & packaging (months)**

* Bytecode compiler and VM
* JIT or native code backend (LLVM) as optional
* Release packages, VS Code syntax extension

# 11. Testing, performance & packaging

* Unit test each builtin separately.
* Use fuzzing for parser resilience (feed random tokens).
* Benchmark interpreter vs transpiled Python for common tasks.
* Package as single-file executable using PyInstaller or Rust binary for portability.

# Appendix A — Examples & Cheatsheet

## Minimal REPL example (pseudo-code)

```
> name = ? "name"
> say "Hello" + " " + name
```

## Calculator (calc.tx)

```
a = ? "a"
b = ? "b"
say "sum:"
say num(a) + num(b)
```

## Quick function

```
fn greet(name)
  say "Hello " + name
end

greet("Boss")
```

---

If you want, I can now:

* expand any specific section into a dedicated chapter (e.g., full parser code, AST definitions, or builtin implementations), or
* produce a runnable reference interpreter in Python that supports the first 60 builtins.

*End of report.*
