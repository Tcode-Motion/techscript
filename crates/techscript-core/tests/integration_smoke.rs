use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use techscript_core::compiler::Compiler;
use techscript_core::lexer::Lexer;
use techscript_core::parser::Parser;
use techscript_core::vm::VM;

fn compile(code: &str) -> techscript_core::value::Function {
    let tokens = Lexer::new(code, "<test>").tokenize().unwrap();
    let program = Parser::new(tokens, "<test>").parse().unwrap();
    Compiler::new().compile(&program).unwrap()
}

fn free_port() -> u16 {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let p = l.local_addr().unwrap().port();
    drop(l);
    p
}

#[test]
fn string_methods_work() {
    let code = r#"
make a = "hello"
assert(a.upper() == "HELLO", "upper")
assert(a.replace("l", "r") == "herlo", "replace once")
assert(a.replace_all("l", "r") == "herro", "replace all")
assert(a.contains("ell") == true, "contains")
"#;
    let mut vm = VM::new();
    vm.run(compile(code)).unwrap();
}

#[test]
fn webpage_compat_object_mutates_lists() {
    // This validates the OO-style surface: WebPage(...), page.style(), page.script(), page.body([...]).
    // We avoid page.run() here to keep the test non-blocking.
    let code = r#"
use web
make page = WebPage("Smoke")
page.style("h1", {"color":"red"})
page.script("console.log('hi')")
page.body([ page.h1("Hello"), page.p("World") ])

assert(len(page["styles"]) == 1, "styles appended")
assert(len(page["scripts"]) == 1, "scripts appended")
assert(len(page["body"]) == 2, "body set")
"#;
    let mut vm = VM::new();
    vm.run(compile(code)).unwrap();
}

#[test]
fn api_listen_serves_json_once_and_net_get_fetches() {
    let port = free_port();

    // Start the API server in a background VM thread (serve exactly one request).
    let server_code = format!(
        r#"
use api
api.listen({}, true)
"#,
        port
    );

    let h = thread::spawn(move || {
        let mut vm = VM::new();
        vm.run(compile(&server_code)).unwrap();
    });

    // Give the listener a moment to bind.
    thread::sleep(Duration::from_millis(120));

    // Validate the net module can fetch the endpoint (this is also the one request that makes the server exit).
    let client_code = format!(
        r#"
use net
make body = net.get("http://127.0.0.1:{}/hello")
assert(body.contains("\"ok\":true"), "net.get returns JSON")
assert(body.contains("\"path\":\"/hello\""), "server includes path")
"#,
        port
    );
    let mut vm = VM::new();
    vm.run(compile(&client_code)).unwrap();

    // Ensure server thread exits (once=true).
    h.join().unwrap();
}

