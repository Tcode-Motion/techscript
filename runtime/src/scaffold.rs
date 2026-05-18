// ── Project scaffolding for `tech new` ───────────────────────────────
use std::fs;
use std::path::Path;

pub fn create_project(name: &str, path: Option<&str>) -> Result<String, String> {
    let dir = path.map(|p| p.to_string()).unwrap_or_else(|| name.to_string());
    let root = Path::new(&dir);
    if root.exists() {
        return Err(format!("Directory already exists: {}", dir));
    }
    fs::create_dir_all(root).map_err(|e| e.to_string())?;

    let main_txs = format!(
        r#"# {name} — TechScript project
say "Hello from {name}!"

make x = 42
say f"Answer: {{x}}"
"#
    );
    fs::write(root.join("main.txs"), main_txs).map_err(|e| e.to_string())?;

    let manifest = format!(
        r#"[project]
name = "{name}"
version = "0.1.0"
entry = "main.txs"

[dependencies]
"#
    );
    fs::write(root.join("techscript.toml"), manifest).map_err(|e| e.to_string())?;

    let gitignore = "target/\n.tech/\n*.txbc\n";
    fs::write(root.join(".gitignore"), gitignore).map_err(|e| e.to_string())?;

    Ok(dir)
}
