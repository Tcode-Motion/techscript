# Appforge Native Language Interoperability

**Document Path**: `docs/appforge/native-languages.md`  

---

## 1. Overview

Appforge provides dedicated native language integration via the command:
```bash
tsc native add <language>
```
Supported languages include:
- `kotlin`: Android Kotlin extensions and Activity bridges
- `java`: Legacy Android Java libraries
- `swift`: iOS and macOS native modules
- `rust`: High-performance cross-compiled shared libraries
- `cpp`: C++ native code via CMake / NDK
- `c`: Pure C ABI modules

---

## 2. Directory Layout & Scaffolding

Running `tsc native add kotlin` sets up:
```text
native/
└── kotlin/
    ├── build.gradle.kts
    └── src/main/kotlin/com/techscript/nativeplugins/
        └── CustomPlugin.kt
```

Running `tsc native add rust` sets up:
```text
native/
└── rust/
    ├── Cargo.toml
    └── src/
        └── lib.rs (extern "C" functions)
```

Running `tsc native add cpp` sets up:
```text
native/
└── cpp/
    ├── CMakeLists.txt
    └── src/
        └── plugin.cpp
```

---

## 3. Invocation Conventions

From TechScript code:
```techscript
use native

# Call native module registered in native/
result = native.call("custom_plugin", "do_action", ["param1", 42])
say $"Native returned: {result}"
```
On Android, calls route through JNI to the Kotlin/Java plugin registry. On desktop, calls route through dynamic library symbol lookup (`libloading`).
