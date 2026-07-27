# LLVM Backend

TechScript supports compiling programs directly to native machine code via the LLVM compiler framework.

---

## 🏗️ Direct Compilation Pipeline
When you run `tech build --native main.txs`, the compiler bypasses the VM bytecode generation and instead runs the native compilation flow:

1. **AST to LLVM IR**: The compiler translates the verified AST into LLVM IR (Intermediate Representation) using `inkwell` (safe Rust bindings for LLVM).
2. **Optimization Passes**: LLVM runs standard compilation optimization passes (e.g. constant propagation, loop unrolling, vectorization).
3. **Machine Code Generation**: LLVM compiles the IR into target-specific assembly or machine code, linking it with the TechScript native runtime to generate a standalone executable.

---

## 🧬 LLVM Backend Features
* **Zero VM Overhead**: Executable binary runs on raw CPU instructions instead of stack emulation.
* **Link-Time Optimization (LTO)**: Combines runtime libraries directly into the output executable for optimization.
* **Native ABI**: Supports compiling functions with native C ABI calling conventions, enabling seamless FFI integration.

---

## 🛠️ Requirements
To use the LLVM backend, you must have an active LLVM development toolchain installed on your machine and configure the compiler with target options:
```bash
tech build --native --target x86_64-pc-windows-msvc main.txs
```
