# Official TechScript Examples & Applications

This directory contains the verified, first-party examples and official showcase applications for **TechScript 2.0**.

Every example here is written in pure **TechScript (`.txs`)** and continuously verified against the compiler (`tsc.exe`) in CI.

---

## 📱 Official Showcase Applications

### 1. TechScript Showcase App (`/showcase-app`)
The flagship cross-platform showcase application authored **100% in TechScript (`.txs`)** and compiled with AppForge.
- **Source Code**: [`showcase-app/src/`](../showcase-app/src/)
- **Features Demonstrated**: Multi-tab navigation, iOS-inspired spring dock, live Android hardware bridge, reactive state management, syntax-highlighted code runner, and local persistence.
- **Targets**: Android APK (API 26-34+), Android AAB (Google Play), Windows desktop executable.
- **Run Locally**:
  ```bash
  cd showcase-app
  tsc app run android
  ```

---

## 💻 Language & Standard Library Examples

All 20 language examples are located in their respective subdirectories and can be executed directly using `tsc run`:

| Category | Subdirectory | Entry Point | Description |
| :--- | :--- | :--- | :--- |
| **Basics** | [`hello_world/`](hello_world/) | `hello.txs` | Minimal TechScript program & standard output |
| **Logic** | [`calculator/`](calculator/) | `calculator.txs` | Arithmetic expressions, conditionals & operators |
| **Games** | [`guess_number/`](guess_number/) | `guess.txs` | Interactive console game with user input & loops |
| **Data Structures** | [`collections/`](collections/) | `collections.txs` | Lists, maps, slicing, filtering, and mutation |
| **Types** | [`enums/`](enums/) | `enums.txs` | Sum types, pattern matching, and state tags |
| **Generics** | [`generics/`](generics/) | `generics.txs` | Type parameterization and polymorphic functions |
| **OOP** | [`oop/`](oop/) | `oop.txs` | Class models, methods, encapsulation, and records |
| **Modules** | [`modules/`](modules/) | `main.txs` | Multi-file imports, exports, and encapsulation |
| **I/O** | [`file_reader/`](file_reader/) | `reader.txs` | File system reading, parsing, and stream handling |
| **Data** | [`json_parser/`](json_parser/) | `parser.txs` | JSON serialization, deserialization, and maps |
| **Database** | [`database/`](database/) | `db.txs` | Relational table queries, records, and schemas |
| **Networking** | [`http_server/`](http_server/) | `server.txs` | HTTP request routing, headers, and responses |
| **Web API** | [`web_api/`](web_api/) | `web_api.txs` | REST API integration and remote JSON endpoints |
| **Async** | [`async/`](async/) | `async.txs` | Non-blocking execution, promises, and tasks |
| **Concurrency** | [`threads/`](threads/) | `threads.txs` | Worker threads, parallel jobs, and sync channels |
| **Graphics** | [`canvas/`](canvas/) | `draw.txs` | 2D drawing primitives, paths, and vector graphics |
| **AI Integration** | [`ai/`](ai/) | `prompt.txs` | AI model prompting, completions, and token streams |
| **Testing** | [`testing/`](testing/) | `unit_test.txs` | Built-in test assertions, suites, and runners |
| **Robustness** | [`error_handling/`](error_handling/) | `errors.txs` | Try/catch, structured errors, and exit codes |
| **CLI Tools** | [`todo_cli/`](todo_cli/) | `todo.txs` | Complete task management console application |

---

## 🧪 Automated Verification

To verify that all 20 examples execute without errors:
```bash
python verify_all_examples.py
```
Expected output:
```text
Verification Results: 20/20 passed
```
