use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use techscript_runtime::{
    context::RuntimeContext, error::RuntimeError, function::Callable, value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);
static PAGE_CONTENT: Mutex<String> = Mutex::new(String::new());

/// Convert a DslBlockValue tree to HTML string.
fn dsl_to_html(val: &RuntimeValue) -> String {
    match val {
        RuntimeValue::DslBlock(dsl) => {
            let mut html = String::new();
            match dsl.kind.as_str() {
                "website" => {
                    html.push_str("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");
                    for prop in &dsl.properties {
                        match prop.name.as_str() {
                            "title" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<title>{}</title>", t));
                                }
                            }
                            _ => {}
                        }
                    }
                    html.push_str("</head><body>");
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</body></html>");
                }
                "page" => {
                    html.push_str("<div class=\"page\">");
                    if let Some(RuntimeValue::Str(t)) = dsl.args.first() {
                        html.push_str(&format!("<h1>{}</h1>", t));
                    }
                    for prop in &dsl.properties {
                        if prop.name == "title" {
                            if let Some(RuntimeValue::Str(t)) = &prop.value {
                                html.push_str(&format!("<h2>{}</h2>", t));
                            }
                        }
                    }
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</div>");
                }
                "hero" => {
                    html.push_str("<section class=\"hero\">");
                    for prop in &dsl.properties {
                        match prop.name.as_str() {
                            "title" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<h1>{}</h1>", t));
                                }
                            }
                            "subtitle" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<p class=\"subtitle\">{}</p>", t));
                                }
                            }
                            _ => {}
                        }
                    }
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</section>");
                }
                "section" => {
                    html.push_str("<section class=\"content-section\">");
                    for prop in &dsl.properties {
                        match prop.name.as_str() {
                            "title" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<h2>{}</h2>", t));
                                }
                            }
                            "id" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<!-- id: {} -->", t));
                                }
                            }
                            _ => {}
                        }
                    }
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</section>");
                }
                "card" => {
                    html.push_str("<div class=\"card\">");
                    for prop in &dsl.properties {
                        match prop.name.as_str() {
                            "title" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<h3>{}</h3>", t));
                                }
                            }
                            "text" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<p>{}</p>", t));
                                }
                            }
                            "image" => {
                                if let Some(RuntimeValue::Str(t)) = &prop.value {
                                    html.push_str(&format!("<img src=\"{}\" alt=\"card image\">", t));
                                }
                            }
                            _ => {}
                        }
                    }
                    html.push_str("</div>");
                }
                "button" => {
                    let label = dsl.properties.iter()
                        .find(|p| p.name == "label")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "Button".to_string());
                    html.push_str(&format!("<button>{}</button>", label));
                }
                "link" => {
                    let label = dsl.properties.iter()
                        .find(|p| p.name == "label")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "Link".to_string());
                    let url = dsl.properties.iter()
                        .find(|p| p.name == "url")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string());
                    if let Some(u) = url {
                        html.push_str(&format!("<a href=\"{}\">{}</a>", u, label));
                    } else {
                        html.push_str(&format!("<a href=\"#\">{}</a>", label));
                    }
                }
                "nav" => {
                    html.push_str("<nav>");
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</nav>");
                }
                "header" => {
                    html.push_str("<header>");
                    for prop in &dsl.properties {
                        if prop.name == "title" {
                            if let Some(RuntimeValue::Str(t)) = &prop.value {
                                html.push_str(&format!("<h1>{}</h1>", t));
                            }
                        }
                    }
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</header>");
                }
                "footer" => {
                    html.push_str("<footer>");
                    for prop in &dsl.properties {
                        if prop.name == "text" {
                            if let Some(RuntimeValue::Str(t)) = &prop.value {
                                html.push_str(&format!("<p>{}</p>", t));
                            }
                        }
                    }
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</footer>");
                }
                "input" => {
                    let placeholder = dsl.properties.iter()
                        .find(|p| p.name == "placeholder")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string());
                    if let Some(p) = placeholder {
                        html.push_str(&format!("<input placeholder=\"{}\">", p));
                    } else {
                        html.push_str("<input>");
                    }
                }
                "form" => {
                    html.push_str("<form>");
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</form>");
                }
                "main" => {
                    html.push_str("<main>");
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</main>");
                }
                "aside" => {
                    html.push_str("<aside>");
                    for child in &dsl.children {
                        html.push_str(&dsl_to_html(&RuntimeValue::DslBlock(Rc::new(child.clone()))));
                    }
                    html.push_str("</aside>");
                }
                "start" => {
                    let label = dsl.properties.iter()
                        .find(|p| p.name == "label")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "Get Started".to_string());
                    html.push_str(&format!("<a class=\"start-button\" href=\"#\">{}</a>", label));
                }
                _ => {
                    html.push_str(&format!("<!-- unknown DSL block: {} -->", dsl.kind));
                }
            }
            html
        }
        _ => String::new(),
    }
}

impl StdlibRegistry {
    pub fn register_web(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert("start".to_string(), Rc::new(StdFunction {
            name: "start".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let port = args[0].try_into_int().map_err(|e| RuntimeError::new(
                    techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))? as u16;
                let content = args[1].to_string();
                *PAGE_CONTENT.lock().unwrap() = content;
                if SERVER_RUNNING.load(Ordering::SeqCst) {
                    return Ok(RuntimeValue::Str("Server already running".to_string()));
                }
                SERVER_RUNNING.store(true, Ordering::SeqCst);
                let server = Mutex::new(tiny_http::Server::http(format!("0.0.0.0:{}", port)).unwrap());
                thread::spawn(move || {
                    while SERVER_RUNNING.load(Ordering::SeqCst) {
                        let page = PAGE_CONTENT.lock().unwrap().clone();
                        if let Ok(mut req) = server.lock().unwrap().recv() {
                            let r = tiny_http::Response::from_string(&page)
                                .with_header(
                                    tiny_http::Header::from_bytes(
                                        &b"Content-Type"[..], &b"text/html; charset=utf-8"[..],
                                    ).unwrap()
                                );
                            let _ = req.respond(r);
                        }
                    }
                });
                Ok(RuntimeValue::Str(format!("Server started on port {}", port)))
            },
        }));

        exports.insert("page".to_string(), Rc::new(StdFunction {
            name: "page".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let _path = args[0].to_string();
                let content = args[1].to_string();
                *PAGE_CONTENT.lock().unwrap() = content;
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("serve".to_string(), Rc::new(StdFunction {
            name: "serve".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let port = args[0].try_into_int().map_err(|e| RuntimeError::new(
                    techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))? as u16;
                if SERVER_RUNNING.load(Ordering::SeqCst) {
                    return Ok(RuntimeValue::Str("Server already running".to_string()));
                }
                SERVER_RUNNING.store(true, Ordering::SeqCst);
                let server = Mutex::new(tiny_http::Server::http(format!("0.0.0.0:{}", port)).unwrap());
                thread::spawn(move || {
                    while SERVER_RUNNING.load(Ordering::SeqCst) {
                        let page = PAGE_CONTENT.lock().unwrap().clone();
                        if let Ok(mut req) = server.lock().unwrap().recv() {
                            let r = tiny_http::Response::from_string(&page)
                                .with_header(
                                    tiny_http::Header::from_bytes(
                                        &b"Content-Type"[..], &b"text/html; charset=utf-8"[..],
                                    ).unwrap()
                                );
                            let _ = req.respond(r);
                        }
                    }
                });
                Ok(RuntimeValue::Str(format!("Server started on port {}", port)))
            },
        }));

        exports.insert("stop".to_string(), Rc::new(StdFunction {
            name: "stop".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                SERVER_RUNNING.store(false, Ordering::SeqCst);
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("set_content".to_string(), Rc::new(StdFunction {
            name: "set_content".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let new_content = args[0].to_string();
                *PAGE_CONTENT.lock().unwrap() = new_content;
                Ok(RuntimeValue::Str("Content updated".to_string()))
            },
        }));

        exports.insert("fetch".to_string(), Rc::new(StdFunction {
            name: "fetch".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let url = args[0].to_string();
                let body = ureq::get(&url)
                    .call()
                    .map_err(|e| RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?
                    .into_string()
                    .map_err(|e| RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                Ok(RuntimeValue::Str(body))
            },
        }));

        exports.insert("render_html".to_string(), Rc::new(StdFunction {
            name: "render_html".to_string(),
            arity: 0,
            callback: |ctx, _args| {
                let env = ctx.global_env.borrow();
                let blocks = match env.lookup("_dsl_blocks") {
                    Ok(RuntimeValue::List { items, .. }) => items.borrow().clone(),
                    _ => return Ok(RuntimeValue::Str(String::new())),
                };
                let mut html = String::new();
                for block in &blocks {
                    html.push_str(&dsl_to_html(block));
                }
                Ok(RuntimeValue::Str(html))
            },
        }));

        exports.insert("render_dsl".to_string(), Rc::new(StdFunction {
            name: "render_dsl".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let html = dsl_to_html(&args[0]);
                Ok(RuntimeValue::Str(html))
            },
        }));

        self.register_module("std.web", StdlibModule {
            name: "std.web".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
