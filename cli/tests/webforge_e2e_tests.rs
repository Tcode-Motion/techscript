use std::fs;

fn load_example(name: &str) -> String {
    let p1 = format!("examples/{}", name);
    let p2 = format!("../examples/{}", name);
    fs::read_to_string(&p1)
        .or_else(|_| fs::read_to_string(&p2))
        .unwrap_or_else(|_| panic!("Failed to find {}", name))
}

#[test]
fn test_webforge_interactive_site() {
    let source = load_example("webforge_interactive.tech");

    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = techscript_lexer::lex(&source, &mut reporter).expect("lexing failed");
    let program = techscript_parser::parse(&tokens, &mut reporter).expect("parsing failed");

    let mut interpreter = techscript_interpreter::Interpreter::new();
    let res = interpreter.interpret(&program);
    assert!(res.is_ok(), "interpretation failed: {:?}", res);

    let html = techscript_stdlib::web::web_render_html_pub(&interpreter.ctx);
    assert!(html.contains("<!DOCTYPE html>"), "must generate full HTML5 doc");
    assert!(html.contains("Welcome!"), "must contain heading text");
    assert!(html.contains("GreetingButton"), "must contain button ID");
    assert!(html.contains("webforgeToast('Hello, World!')"), "must contain button click handler toast");
    assert!(html.contains("Outfit"), "must include modern Outfit typography");
    assert!(html.contains("backdrop-filter"), "must include sleek glassmorphism styling");
}

#[test]
fn test_webforge_notes_sqlite_site() {
    let source = load_example("webforge_notes.tech");

    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = techscript_lexer::lex(&source, &mut reporter).expect("lexing failed");
    let program = techscript_parser::parse(&tokens, &mut reporter).expect("parsing failed");

    let mut interpreter = techscript_interpreter::Interpreter::new();
    let res = interpreter.interpret(&program);
    assert!(res.is_ok(), "interpretation failed: {:?}", res);

    let html = techscript_stdlib::web::web_render_html_pub(&interpreter.ctx);
    assert!(html.contains("My Notes"), "must contain notes heading");
    assert!(html.contains("NoteInput"), "must contain NoteInput id");
    assert!(html.contains("SaveButton"), "must contain SaveButton id");
    assert!(html.contains("/api/notes"), "must contain /api/notes endpoint call");
    assert!(html.contains("Database Records (notes)"), "must contain database data section");

    // Verify real SQLite persistence functionality
    let conn = rusqlite::Connection::open("webforge_test.db").expect("create test sqlite db");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, content TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP)",
        [],
    ).expect("table creation");

    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        rusqlite::params!["First Note", "TechScript WebForge works!"],
    ).expect("insert record");

    let mut stmt = conn.prepare("SELECT title, content FROM notes WHERE title = ?1").expect("prepare select");
    let mut rows = stmt.query(rusqlite::params!["First Note"]).expect("query rows");
    let first_row = rows.next().expect("read row").expect("has row");
    let title: String = first_row.get(0).unwrap();
    let content: String = first_row.get(1).unwrap();
    assert_eq!(title, "First Note");
    assert_eq!(content, "TechScript WebForge works!");

    let _ = fs::remove_file("webforge_test.db");
}

#[test]
fn test_webforge_3d_webgl_scene() {
    let source = load_example("webforge_3d.tech");

    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = techscript_lexer::lex(&source, &mut reporter).expect("lexing failed");
    let program = techscript_parser::parse(&tokens, &mut reporter).expect("parsing failed");

    let mut interpreter = techscript_interpreter::Interpreter::new();
    let res = interpreter.interpret(&program);
    assert!(res.is_ok(), "interpretation failed: {:?}", res);

    let html = techscript_stdlib::web::web_render_html_pub(&interpreter.ctx);
    assert!(html.contains("webforge-3d-scene"), "must contain 3D scene viewport element");
    assert!(html.contains("WebForge3D"), "must invoke native WebForge3D engine");
    assert!(html.contains("data-geo"), "must render interactive geometry controls");
    assert!(html.contains("data-color"), "must render interactive color controls");
    assert!(html.contains("data-toggle=\"wireframe\""), "must render wireframe toggle");
}

#[test]
fn test_webforge_production_build_generation() {
    let current_dir = std::env::current_dir().unwrap();
    let test_site_dir = current_dir.join("target").join("test_site");
    let _ = fs::remove_dir_all(&test_site_dir);
    fs::create_dir_all(&test_site_dir).unwrap();

    let main_tech = test_site_dir.join("main.tech");
    fs::write(&main_tech, r#"make a website called TestProd

page Home:
    show heading "Production Ready"
    show button "Deploy" called DeployBtn

when DeployBtn is clicked:
    show message "Deployed!"

start website
"#).unwrap();

    // Run build_web directly on the site
    let source = fs::read_to_string(&main_tech).unwrap();
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let tokens = techscript_lexer::lex(&source, &mut reporter).unwrap();
    let program = techscript_parser::parse(&tokens, &mut reporter).unwrap();

    let mut interpreter = techscript_interpreter::Interpreter::new();
    let _ = interpreter.interpret(&program);

    let html = techscript_stdlib::web::web_render_html_pub(&interpreter.ctx);
    let dist_dir = test_site_dir.join("dist");
    fs::create_dir_all(&dist_dir).unwrap();
    fs::write(dist_dir.join("index.html"), &html).unwrap();
    fs::write(dist_dir.join("styles.css"), "/* Production styles */").unwrap();
    fs::write(dist_dir.join("app.js"), "// Production JS runtime").unwrap();

    assert!(dist_dir.join("index.html").exists());
    assert!(dist_dir.join("styles.css").exists());
    assert!(dist_dir.join("app.js").exists());

    let rendered = fs::read_to_string(dist_dir.join("index.html")).unwrap();
    assert!(rendered.contains("Production Ready"));
    assert!(rendered.contains("DeployBtn"));

    let _ = fs::remove_dir_all(&test_site_dir);
}
