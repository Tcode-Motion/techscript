with open("runtime/vm/src/debugger.rs", "r") as f:
    content = f.read()

content = content.replace("operands_str\n                        .push_str(&format!(", "let _ = write!(operands_str, ")

with open("runtime/vm/src/debugger.rs", "w") as f:
    f.write(content)
