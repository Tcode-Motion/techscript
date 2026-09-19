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
| [hello_world](https://github.com/Tcode-Motion/techscript/tree/main/examples/hello_world/) | Core | Simplest output prints |
| [calculator](https://github.com/Tcode-Motion/techscript/tree/main/examples/calculator/) | Math | Functions and math operators |
| [todo_cli](https://github.com/Tcode-Motion/techscript/tree/main/examples/todo_cli/) | State | Lists and maps manipulation |
| [guess_number](https://github.com/Tcode-Motion/techscript/tree/main/examples/guess_number/) | Logic | Ranges, loops, conditionals |
| [http_server](https://github.com/Tcode-Motion/techscript/tree/main/examples/http_server/) | Network | Web routing and mock testing |
| [json_parser](https://github.com/Tcode-Motion/techscript/tree/main/examples/json_parser/) | Data | Encoding/decoding maps |
| [file_reader](https://github.com/Tcode-Motion/techscript/tree/main/examples/file_reader/) | File System | IO file writes and reads |
| [oop](https://github.com/Tcode-Motion/techscript/tree/main/examples/oop/) | Models | Classes, inheritance, overriding |
| [modules](https://github.com/Tcode-Motion/techscript/tree/main/examples/modules/) | Imports | Multi-file namespaces |
| [collections](https://github.com/Tcode-Motion/techscript/tree/main/examples/collections/) | Types | Loops over list and maps |
| [generics](https://github.com/Tcode-Motion/techscript/tree/main/examples/generics/) | Polymorph | Parameterized types |
| [error_handling](https://github.com/Tcode-Motion/techscript/tree/main/examples/error_handling/) | Errors | `try`/`catch` boundaries |
| [async](https://github.com/Tcode-Motion/techscript/tree/main/examples/async/) | Concurrency | Event loops and futures |
| [threads](https://github.com/Tcode-Motion/techscript/tree/main/examples/threads/) | Parallel | Thread spawns & mutexes |
| [web_api](https://github.com/Tcode-Motion/techscript/tree/main/examples/web_api/) | Fetch | External GET API calls |
