# TechScript 2.0 Compiler Performance Report

Generated on standard test environment (Windows target execution).

## Compilation Phase Speeds
| Phase | Duration |
| :--- | :--- |
| **Lexing & Tokenization** | 0.090 ms |
| **Pratt Parsing & AST Building** | 0.245 ms |
| **Semantic Check & Name Binding** | 0.454 ms |
| **SSA IR Lowering & Optimization** | 0.103 ms |
| **Bytecode Generation** | 0.138 ms |
| **VM Execution (Fibonacci 25)** | 1279.449 ms |

## Benchmark Details
- **Test File**: Recursive Fibonacci 25 calculation (`fib(25)`)
- **Optimization Level**: SSA optimizations enabled (Constant Folding, Dead Code Elimination)
- **Garbage Collector**: Mark-sweep tracing enabled
