# TechScript 2.0 Compiler Performance Report

Generated on standard test environment (Windows target execution).

## Compilation Phase Speeds
| Phase | Duration |
| :--- | :--- |
| **Lexing & Tokenization** | 0.092 ms |
| **Pratt Parsing & AST Building** | 0.116 ms |
| **Semantic Check & Name Binding** | 0.199 ms |
| **SSA IR Lowering & Optimization** | 0.194 ms |
| **Bytecode Generation** | 0.110 ms |
| **VM Execution (Fibonacci 25)** | 38629.399 ms |

## Benchmark Details
- **Test File**: Recursive Fibonacci 25 calculation (`fib(25)`)
- **Optimization Level**: SSA optimizations enabled (Constant Folding, Dead Code Elimination)
- **Garbage Collector**: Mark-sweep tracing enabled
