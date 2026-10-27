# TechScript Language Rules for Coding Agents

## Governing Directive
TechScript syntax is strictly locked to [TechScript_Final_Syntax_and_Language_Master_Spec.md](file:///c:/Users/Tanmoy/OneDrive/Documents/techscript/TechScript_Final_Syntax_and_Language_Master_Spec.md).

## Canonical Constructs
- **Variable declaration:** `name is "TechScript"`
- **Variable mutation:** `count becomes count + 1`
- **Constants:** `constant app_name is "TechScript"`
- **Function definition:** `to greet(person):` closed by dedentation
- **Return:** `return value`
- **Output:** `say "Hello, {name}!"`
- **Conditionals:** `if condition:`, `else if condition:`, `else:` closed by dedentation
- **Loops:** `repeat N times:` and `for item in items:` and `while condition:` closed by dedentation
- **No boilerplate:** Do NOT use `do`, `fn`, `let`, `var`, `end`, or `{}` for ordinary functions or blocks.
