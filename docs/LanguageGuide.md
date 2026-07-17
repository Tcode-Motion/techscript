# TechScript 2.0 Language Guide

This guide introduces the core design features, variables, control flows, and standard type structures of TechScript 2.0.

## 1. Syntax Overview

TechScript inherits syntaxes from JavaScript/TypeScript and Rust.

### Variables

```techscript
let x = 10;
const y = 3.14159;
```

### Control Flow

```techscript
if x > 5 {
    print("Greater than 5");
} else {
    print("5 or less");
}

for let i = 0; i < 5; i = i + 1 {
    print(i);
}
```

### Functions

```techscript
function add(a, b) {
    return a + b;
}
```
