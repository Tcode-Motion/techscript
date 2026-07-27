# Examples Directory Guide

This guide explains how to run, understand, and inspect the code examples under the `examples/` directory.

---

## 📂 Example Folder Structure
Every example is in its own folder and contains:
* **Source Script**: The `.txs` source code.
* **`README.md`**: Explanation of how the code works.
* **`expected.txt`**: The exact output printed by the program when executed.

---

## 🚀 Running the Examples

Run an example using the `tech` compiler:
```bash
# Navigate to the example folder
cd examples/hello_world
tech run hello.txs
```

You can verify that output matches expectations:
```bash
tech test
```

---

## 🗺️ Index of Core Examples

| Folder | Focus | Key Concept covered |
|:---|:---|:---|
| [hello_world](../examples/hello_world/) | Core | Simplest output prints |
| [calculator](../examples/calculator/) | Math | Functions and math operators |
| [todo_cli](../examples/todo_cli/) | State | Lists and maps manipulation |
| [guess_number](../examples/guess_number/) | Logic | Ranges, loops, conditionals |
| [http_server](../examples/http_server/) | Network | Web routing and mock testing |
| [json_parser](../examples/json_parser/) | Data | Encoding/decoding maps |
| [file_reader](../examples/file_reader/) | File System | IO file writes and reads |
| [oop](../examples/oop/) | Models | Classes, inheritance, overriding |
| [modules](../examples/modules/) | Imports | Multi-file namespaces |
| [collections](../examples/collections/) | Types | Loops over list and maps |
| [generics](../examples/generics/) | Polymorph | Parameterized types |
| [error_handling](../examples/error_handling/) | Errors | `try`/`catch` boundaries |
| [async](../examples/async/) | Concurrency | Event loops and futures |
| [threads](../examples/threads/) | Parallel | Thread spawns & mutexes |
| [web_api](../examples/web_api/) | Fetch | External GET API calls |
