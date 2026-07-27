# Foreign Function Interface (FFI) in TechScript

The Foreign Function Interface allows TechScript programs to call external native libraries written in C, C++, or Rust.

---

## 🏗️ Declaring External Functions
Use the `ffi` module to load a dynamic library (`.dll` on Windows, `.so` on Linux, `.dylib` on macOS) and bind its functions:

```txs
use ffi

# Load standard C library or a custom DLL
libc = ffi.load("libc.so.6") # or "msvcrt.dll" on Windows

# Bind function signatures: ffi.bind(lib, function_name, return_type, [arg_types])
print_func = ffi.bind(libc, "puts", "int", ["string"])

# Execute the native C function
print_func("Hello from the native C library!")
```

---

## 🧬 Supported Type Conversions

When calling FFI functions, TechScript automatically converts values back and forth:

| TechScript Type | FFI Type Target |
|:---|:---|
| `Int` | `int`, `int32`, `int64`, `size_t` |
| `Float` | `double`, `float` |
| `Str` | `const char*` (null-terminated string) |
| `Bool` | `bool` / `char` |
| `Null` | `void*` (NULL pointer) |

---

## ⚠️ Security & Panics
FFI is inherently **unsafe**. If a bound C function triggers a segmentation fault or memory leak, the TechScript VM cannot catch it and the application will crash. Use FFI only when highly optimized native hooks are required.
