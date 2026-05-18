use std::path::PathBuf;

use techscript::run;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn test_hello_world() {
    run::run_source(r#"say "Hello, World!""#, "<test>").expect("hello world should run");
}

#[test]
fn test_variables_and_math() {
    run::run_source(
        r#"
make x = 10
make y = 5
say x + y
"#,
        "<test>",
    )
    .expect("math should run");
}

#[test]
fn test_function() {
    run::run_source(
        r#"
build add(a, b) {
    send a + b
}
say add(2, 3)
"#,
        "<test>",
    )
    .expect("function should run");
}

#[test]
fn test_class() {
    run::run_source(
        r#"
model Point {
    build init(self, x, y) {
        self.x = x
        self.y = y
    }
}
make p = Point(1, 2)
"#,
        "<test>",
    )
    .expect("class should run");
}

#[test]
fn test_syntax_aliases() {
    run::run_source(
        r#"
const PI = 3.14
do greet(name) {
    return f"Hi {name}"
}
class Dog {
    do init(self, name) {
        self.name = name
    }
}
try {
    throw "oops"
} catch err {
    say err
}
loop 3 {
    say "tick"
}
"#,
        "<test>",
    )
    .expect("syntax aliases should run");
}

#[test]
fn test_runtime_examples_smoke() {
    std::env::set_var("TECHSCRIPT_WEB_TEST", "1");
    let root = repo_root();
    let examples_dir = root.join("runtime_examples");
    if !examples_dir.exists() {
        return;
    }
    // Skip 07_performance_test in CI — 1M iterations is too slow for debug builds
    for name in [
        "01_basics.txs",
        "02_math_and_logic.txs",
        "03_control_flow.txs",
        "04_functions.txs",
        "05_classes.txs",
        "06_advanced.txs",
    ] {
        let path = examples_dir.join(name);
        if path.exists() {
            let file = path.to_str().unwrap();
            run::run_file(file).unwrap_or_else(|e| {
                panic!("Failed to run {}: {}", file, e);
            });
        }
    }
}

#[test]
fn test_basic_examples_smoke() {
    std::env::set_var("TECHSCRIPT_WEB_TEST", "1");
    let root = repo_root();
    for name in ["hello.txs", "calc.txs", "classes.txs", "fibonacci.txs", "fizzbuzz.txs"] {
        let path = root.join("examples").join(name);
        if path.exists() {
            run::run_file(path.to_str().unwrap()).unwrap_or_else(|e| {
                panic!("Failed to run {}: {}", path.display(), e);
            });
        }
    }
}

#[test]
fn test_module_examples_with_env_skip() {
    std::env::set_var("TECHSCRIPT_WEB_TEST", "1");
    std::env::set_var("TECHSCRIPT_GUI_TEST", "1");
    std::env::set_var("TECHSCRIPT_3D_TEST", "1");
    let root = repo_root();
    for rel in [
        "examples/web_complete.txs",
        "examples/web_app.txs",
        "examples/gui_app.txs",
        "examples/3d_scene.txs",
        "examples/anime_demo.txs",
        "examples/syntax_aliases.txs",
    ] {
        let p = root.join(rel);
        if p.exists() {
            run::run_file(p.to_str().unwrap()).unwrap_or_else(|e| panic!("{}: {}", rel, e));
        }
    }
}
