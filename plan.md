1. **Restore `stdlib/src/canvas.rs`**
   - Use `run_in_bash_session` to execute `git restore stdlib/src/canvas.rs`.
2. **Import `DslBlockValue` in `stdlib/src/canvas.rs`**
   - Use `replace_with_git_merge_diff` to add `DslBlockValue` to the runtime value import:
```
<<<<<<< SEARCH
use techscript_runtime::{
    context::{Capability, RuntimeContext},
    error::RuntimeError,
    function::Callable,
    value::RuntimeValue,
};
=======
use techscript_runtime::{
    context::{Capability, RuntimeContext},
    error::RuntimeError,
    function::Callable,
    value::{DslBlockValue, RuntimeValue},
};
>>>>>>> REPLACE
```
3. **Refactor `render_dsl_to_svg`**
   - Use `run_in_bash_session` to write `/tmp/refactor.py` and run it:
```python
import sys

def main():
    with open('stdlib/src/canvas.rs', 'r') as f:
        lines = f.read().split('\n')

    start_func = -1
    for i, line in enumerate(lines):
        if line.startswith("fn render_dsl_to_svg"):
            start_func = i
            break

    end_func = -1
    for i in range(start_func, len(lines)):
        if lines[i] == "fn dsl_to_svg(val: &RuntimeValue, is_dragon: bool) -> String {":
            end_func = i - 2
            break

    func_lines = lines[start_func:end_func+1]

    # Extract inner content for dragon / normal
    dragon_start = func_lines.index("            if is_dragon {")
    dragon_end = func_lines.index("            } else {")
    normal_start = dragon_end
    normal_end = func_lines.index("            }")

    dragon_lines = func_lines[dragon_start+1:dragon_end]
    normal_lines = func_lines[normal_start+2:normal_end] # +2 to skip } else { and // RENDER OLD LOGO

    def extract_arms(arm_lines):
        arms = {}
        current_arm = None
        current_lines = []
        depth = 0
        for line in arm_lines:
            stripped = line.strip()
            if not current_arm:
                if stripped.startswith('"') and "=> {" in stripped:
                    current_arm = stripped.split('"')[1]
                    depth = 1
            else:
                if "{" in stripped:
                    depth += line.count("{")
                if "}" in stripped:
                    depth -= line.count("}")
                if depth == 0:
                    arms[current_arm] = current_lines
                    current_arm = None
                    current_lines = []
                else:
                    # unindent 6 levels
                    if line.startswith(" " * 24):
                        current_lines.append(line[24:])
                    else:
                        current_lines.append(line)
        return arms

    dragon_arms = extract_arms(dragon_lines)
    normal_arms = extract_arms(normal_lines)

    helpers = []
    keys = set(list(dragon_arms.keys()) + list(normal_arms.keys()))
    keys.discard("_")

    for key in keys:
        if key == "core":
            helpers.append(f"fn render_core(svg: &mut String, dsl: &DslBlockValue) {{\n" + "\n".join(normal_arms.get("core", [])) + "\n}")
            continue

        helpers.append(f"fn render_{key}(svg: &mut String, dsl: &DslBlockValue, is_dragon: bool) {{")
        helpers.append("    if is_dragon {")
        if key in dragon_arms:
            for l in dragon_arms[key]:
                if l.startswith(" "):
                    helpers.append("    " + l)
                else:
                    helpers.append("        " + l)
        helpers.append("    } else {")
        if key in normal_arms:
            for l in normal_arms[key]:
                if l.startswith(" "):
                    helpers.append("    " + l)
                else:
                    helpers.append("        " + l)
        helpers.append("    }")
        helpers.append("}")

    helpers_code = "\n\n".join(helpers)

    new_func = """fn render_dsl_to_svg(svg: &mut String, val: &RuntimeValue, is_dragon: bool) {
    if let RuntimeValue::DslBlock(dsl) = val {
        match dsl.kind.as_str() {
            "logo" => render_logo(svg, dsl, is_dragon),
            "rings" => render_rings(svg, dsl, is_dragon),
            "emblem" => render_emblem(svg, dsl, is_dragon),
            "letter" => render_letter(svg, dsl, is_dragon),
            "core" => {
                if !is_dragon {
                    render_core(svg, dsl);
                }
            }
            "circuits" => render_circuits(svg, dsl, is_dragon),
            _ => {}
        }
    }
}"""

    with open('stdlib/src/canvas.rs', 'w') as f:
        f.write("\n".join(lines[:start_func]))
        f.write("\n")
        f.write(helpers_code)
        f.write("\n\n")
        f.write(new_func)
        f.write("\n")
        f.write("\n".join(lines[end_func+1:]))

if __name__ == '__main__':
    main()
```
   - Use `run_in_bash_session` to execute `python3 /tmp/refactor.py`.
   - Use `run_in_bash_session` to execute `rm /tmp/refactor.py`.
4. **Visually verify Step 3**
   - Use `run_in_bash_session` with `cat stdlib/src/canvas.rs | grep -A 30 "fn render_dsl_to_svg"` to verify the modified `render_dsl_to_svg` function and the helpers.
5. **Format the file**
   - Use `run_in_bash_session` to execute `cargo fmt` because the indentation might be a little off.
6. **Run tests to verify functionality**
   - Use `run_in_bash_session` to execute `cargo test --workspace --all-targets` to ensure no functionality is broken.
7. **Complete pre-commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
8. **Submit the PR**
   - Use `submit` tool to create a PR:
     - Branch name: `fix-canvas-refactor`
     - Title: "🧹 [Refactor canvas SVG generation]"
     - Description: "🎯 **What:** Extracted SVG generation logic in `canvas.rs` into smaller helper functions based on `dsl.kind`.\n💡 **Why:** Refactoring deeply nested `match` statements prevents massive function sizes and improves overall code readability and maintainability.\n✅ **Verification:** Visually verified the split function logic using `cat` and successfully ran `cargo test --workspace --all-targets` to confirm no regressions.\n✨ **Result:** Cleanly segmented, modular SVG generation that is easier to maintain."
