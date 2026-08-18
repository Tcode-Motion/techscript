# Bytecode Specification

TechScript compiles source files into a binary bytecode format (`.txc`) optimized for fast loading and low-overhead VM execution.

---

## 📁 The `.txc` Binary Layout
Every compiled `.txc` file has a structured header followed by data blocks:

```
+------------------------------------+
| Magic Header: "TXS\x00" (4 bytes)  |
+------------------------------------+
| Major & Minor Version (4 bytes)    |
+------------------------------------+
| Constant Pool Size (4 bytes)       |
+------------------------------------+
| Constant Pool Data                 |
+------------------------------------+
| Instruction Count (4 bytes)        |
+------------------------------------+
| Bytecode Instructions              |
+------------------------------------+
```

---

## ⚙️ Instruction Set Architecture (ISA)

The TechScript VM uses an 8-bit instruction set. Here are the primary instructions:

| Opcode | Mnemonic | Arguments | Description |
|:---:|:---|:---|:---|
| `0x01` | `LOAD_CONST` | `const_idx (u32)` | Pushes a value from the constant pool onto the stack. |
| `0x02` | `LOAD_LOCAL` | `local_idx (u32)` | Pushes a local variable value onto the stack. |
| `0x03` | `STORE_LOCAL`| `local_idx (u32)` | Pops the top of the stack and stores it in local memory. |
| `0x04` | `ADD` | None | Pops two values, adds them, pushes the result. |
| `0x05` | `SUB` | None | Pops two values, subtracts them, pushes the result. |
| `0x06` | `MUL` | None | Pops two values, multiplies them, pushes the result. |
| `0x07` | `DIV` | None | Pops two values, divides them, pushes the result. |
| `0x08` | `JUMP` | `offset (i32)` | Unconditionally jumps instruction pointer. |
| `0x09` | `JUMP_IF_FALSE`| `offset (i32)` | Pops value; if false, jumps to offset. |
| `0x0A` | `CALL` | `arg_count (u8)` | Calls a function with N arguments on stack. |
| `0x0B` | `RETURN` | None | Returns from the current stack frame. |
| `0x0C` | `HALT` | None | Stops VM execution immediately. |
