// ── Native Web Module ────────────────────────────────────────────────
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use crate::value::{NativeFnObj, Value};

thread_local! {
    static WEB_CTX: RefCell<WebContext> = RefCell::new(WebContext::default());
    static PAGE_ID: RefCell<u64> = RefCell::new(0);
}

#[derive(Default)]
struct WebContext {
    pages: HashMap<u64, WebPageNative>,
    states: HashMap<String, String>,
    components: HashMap<String, String>,
    framework_pages: HashMap<String, String>,
    routes: Vec<(String, String, String)>,
    active_page: Option<String>,
}

pub struct WebPageNative {
    title: String,
    styles: Vec<String>,
    scripts: Vec<String>,
    body_elements: Vec<String>,
}

impl WebPageNative {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            styles: Vec::new(),
            scripts: Vec::new(),
            body_elements: Vec::new(),
        }
    }

    fn style(&mut self, selector: &str, rules: &HashMap<String, Value>) {
        let css: String = rules
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.display_string()))
            .collect::<Vec<_>>()
            .join("; ");
        self.styles.push(format!("{} {{ {} }}", selector, css));
    }

    fn script(&mut self, code: &str) {
        self.scripts.push(code.to_string());
    }

    fn element(&self, tag: &str, content: &str, attrs: Option<&HashMap<String, Value>>) -> String {
        let attr_str = attrs
            .map(|a| {
                " ".to_string()
                    + &a.iter()
                        .map(|(k, v)| format!("{}=\"{}\"", k, html_escape(&v.display_string())))
                        .collect::<Vec<_>>()
                        .join(" ")
            })
            .unwrap_or_default();
        format!("<{}{}>{}</{}>", tag, attr_str, content, tag)
    }

    fn body(&mut self, elements: Vec<String>) {
        self.body_elements.extend(elements);
    }

    fn render(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<style>{}</style>
</head>
<body>
{}
<script>{}</script>
</body>
</html>"#,
            html_escape(&self.title),
            self.styles.join("\n"),
            self.body_elements.join("\n"),
            self.scripts.join("\n"),
        )
    }

    fn run_with_routes(
        &self,
        port: u16,
        routes: Vec<(String, String, String)>,
    ) -> Result<(), String> {
        let server = match tiny_http::Server::http(format!("127.0.0.1:{}", port)) {
            Ok(s) => s,
            Err(_) => tiny_http::Server::http("127.0.0.1:0")
                .map_err(|e| format!("Failed to bind server: {}", e))?,
        };
        let actual_port = server.server_addr().to_ip().map(|addr| addr.port()).unwrap_or(port);
        println!("🌐 TechScript web server at http://127.0.0.1:{}", actual_port);

        let html = Arc::new(self.render());
        let routes = Arc::new(routes);
        
        thread::spawn(move || {
            for request in server.incoming_requests() {
                let html = html.clone();
                let routes = routes.clone();
                thread::spawn(move || {
                    let path = request.url().to_string();
                    let method = request.method().as_str().to_string();
                    if let Some((_, _route_path, body)) = routes
                        .iter()
                        .find(|(m, p, _)| m.eq_ignore_ascii_case(&method) && p == &path)
                    {
                        let response = tiny_http::Response::from_string(body.as_str()).with_header(
                            tiny_http::Header::from_bytes(
                                &b"Content-Type"[..],
                                &b"application/json; charset=utf-8"[..],
                            )
                            .unwrap(),
                        );
                        let _ = request.respond(response);
                        return;
                    }
                    let response = tiny_http::Response::from_string(html.as_str()).with_header(
                        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
                            .unwrap(),
                    );
                    let _ = request.respond(response);
                });
            }
        });
        
        println!("Open http://127.0.0.1:{} in your browser", actual_port);
        if std::env::var("TECHSCRIPT_WEB_TEST").is_ok() {
            return Ok(());
        }
        let url = format!("http://127.0.0.1:{}", actual_port);
        let _ = open_browser(&url);
        thread::sleep(std::time::Duration::from_secs(3600));
        Ok(())
    }

    fn run(&self, port: u16) -> Result<(), String> {
        self.run_with_routes(port, Vec::new())
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn open_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", "", url])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()?;
    }
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()?;
    }
    Ok(())
}

fn next_page_id() -> u64 {
    PAGE_ID.with(|id| {
        *id.borrow_mut() += 1;
        *id.borrow()
    })
}

fn with_page<F, T>(id: u64, f: F) -> Result<T, String>
where
    F: FnOnce(&mut WebPageNative) -> Result<T, String>,
{
    WEB_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let page = ctx.pages.get_mut(&id).ok_or_else(|| format!("Invalid WebPage id {}", id))?;
        f(page)
    })
}

fn make_bound_method(page_id: u64, method: &'static str) -> Value {
    Value::NativeFunction(Rc::new(NativeFnObj {
        name: method.to_string(),
        func: Box::new(move |args| dispatch_method(page_id, method, args)),
    }))
}

fn dispatch_method(page_id: u64, method: &str, args: &[Value]) -> Result<Value, String> {
    match method {
        "style" if args.len() >= 2 => with_page(page_id, |p| {
            if let Value::Map(m) = &args[1] {
                p.style(&args[0].display_string(), &m.borrow());
            }
            Ok(Value::None)
        }),
        "script" if !args.is_empty() => with_page(page_id, |p| {
            p.script(&args[0].display_string());
            Ok(Value::None)
        }),
        "body" if !args.is_empty() => with_page(page_id, |p| {
            if let Value::List(l) = &args[0] {
                let elems: Vec<String> = l.borrow().iter().map(|v| v.display_string()).collect();
                p.body(elems);
            }
            Ok(Value::None)
        }),
        "h1" | "h2" | "h3" | "p" | "div" | "span" | "button" => with_page(page_id, |p| {
            let content = args.first().map(|v| {
                if let Value::List(l) = v {
                    l.borrow().iter().map(|item| item.display_string()).collect::<Vec<_>>().join("")
                } else {
                    html_escape(&v.display_string())
                }
            }).unwrap_or_default();
            
            let attrs = args.get(1).and_then(|v| {
                if let Value::Map(m) = v {
                    Some(m.borrow().clone())
                } else {
                    None
                }
            });
            Ok(Value::String(Rc::new(p.element(
                method,
                &content,
                attrs.as_ref(),
            ))))
        }),
        "input" => with_page(page_id, |p| {
            let attrs = args.first().and_then(|v| {
                if let Value::Map(m) = v {
                    Some(m.borrow().clone())
                } else {
                    None
                }
            });
            let mut s = p.element("input", "", attrs.as_ref());
            s = s.replace("></input>", "/>");
            Ok(Value::String(Rc::new(s)))
        }),
        "img" => with_page(page_id, |p| {
            let attrs = args.first().and_then(|v| {
                if let Value::Map(m) = v {
                    Some(m.borrow().clone())
                } else {
                    None
                }
            });
            let mut s = p.element("img", "", attrs.as_ref());
            s = s.replace("></img>", "/>");
            Ok(Value::String(Rc::new(s)))
        }),
        "raw" if !args.is_empty() => Ok(Value::String(Rc::new(args[0].display_string()))),
        "render" => with_page(page_id, |p| Ok(Value::String(Rc::new(p.render())))),
        "run" => with_page(page_id, |p| {
            let port = args
                .first()
                .and_then(|v| if let Value::Int(n) = v { Some(*n as u16) } else { None })
                .unwrap_or(8080);
            p.run(port)?;
            Ok(Value::None)
        }),
        _ => Err(format!("Unknown WebPage method: {}", method)),
    }
}

pub fn register(globals: &mut HashMap<String, Value>) {
    globals.insert(
        "WebPage".into(),
        Value::NativeFunction(Rc::new(NativeFnObj {
            name: "WebPage".into(),
            func: Box::new(|args| {
                let title = args
                    .first()
                    .map(|v| v.display_string())
                    .unwrap_or_else(|| "TechScript App".into());
                let id = next_page_id();
                WEB_CTX.with(|ctx| {
                    ctx.borrow_mut().pages.insert(id, WebPageNative::new(&title));
                });
                let mut map = HashMap::new();
                map.insert("_page_id".into(), Value::Int(id as i64));
                for m in [
                    "style", "script", "body", "h1", "h2", "h3", "p", "div", "span", "button", "input", "img", "raw",
                    "render", "run",
                ] {
                    map.insert(m.into(), make_bound_method(id, m));
                }
                Ok(Value::Map(Rc::new(RefCell::new(map))))
            }),
        })),
    );

    for (name, func) in [
        ("__web_state", web_state as fn(&[Value]) -> Result<Value, String>),
        ("__web_component", web_component),
        ("__web_page", web_page),
        ("__web_route", web_route),
        ("__web_render", web_render),
        ("__web_run_framework", web_run_framework),
    ] {
        globals.insert(
            name.into(),
            Value::NativeFunction(Rc::new(NativeFnObj {
                name: name.to_string(),
                func: Box::new(func),
            })),
        );
    }
}

fn web_state(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("web_state needs name and value".into());
    }
    WEB_CTX.with(|ctx| {
        ctx.borrow_mut()
            .states
            .insert(args[0].display_string(), args[1].display_string());
    });
    Ok(Value::None)
}

fn web_component(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("web_component needs name".into());
    }
    WEB_CTX.with(|ctx| {
        ctx.borrow_mut().components.insert(
            args[0].display_string(),
            args.get(1).map(|v| v.display_string()).unwrap_or_default(),
        );
    });
    Ok(Value::None)
}

fn web_page(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("web_page needs name".into());
    }
    let name = args[0].display_string();
    let html = args.get(1).map(|v| v.display_string()).unwrap_or_default();
    WEB_CTX.with(|ctx| {
        let mut c = ctx.borrow_mut();
        c.framework_pages.insert(name.clone(), html);
        c.active_page = Some(name);
    });
    Ok(Value::None)
}

fn web_route(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("web_route needs method, path, and optional response body".into());
    }
    let response = args.get(2).map(|v| v.display_string()).unwrap_or_else(|| "{\"ok\":true}".into());
    WEB_CTX.with(|ctx| {
        ctx.borrow_mut().routes.push((
            args[0].display_string(),
            args[1].display_string(),
            response,
        ));
    });
    Ok(Value::None)
}

fn web_render(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("web_render needs tag".into());
    }
    let tag = args[0].display_string();
    let inner = args.get(1).map(|v| v.display_string()).unwrap_or_default();
    Ok(Value::String(Rc::new(format!("<{}>{}</{}>", tag, inner, tag))))
}

fn web_run_framework(_args: &[Value]) -> Result<Value, String> {
    WEB_CTX.with(|ctx| {
        let ctx = ctx.borrow();
        let page_name = ctx.active_page.clone().unwrap_or_else(|| "Home".into());
        let mut body = ctx
            .framework_pages
            .get(&page_name)
            .cloned()
            .unwrap_or_else(|| "<h1>TechScript Web</h1>".into());
            
        // Expand components recursively (up to 10 levels)
        let mut expanded = true;
        let mut iterations = 0;
        while expanded && iterations < 10 {
            expanded = false;
            for (comp_name, comp_html) in &ctx.components {
                let tag_open_close = format!("<{0}></{0}>", comp_name);
                let tag_self_closing = format!("<{0} />", comp_name);
                let tag_self_closing_no_space = format!("<{0}/>", comp_name);
                let tag_open = format!("<{0}>", comp_name);

                if body.contains(&tag_open_close) {
                    body = body.replace(&tag_open_close, comp_html);
                    expanded = true;
                }
                if body.contains(&tag_self_closing) {
                    body = body.replace(&tag_self_closing, comp_html);
                    expanded = true;
                }
                if body.contains(&tag_self_closing_no_space) {
                    body = body.replace(&tag_self_closing_no_space, comp_html);
                    expanded = true;
                }
                if body.contains(&tag_open) {
                    body = body.replace(&tag_open, comp_html);
                    expanded = true;
                }
            }
            iterations += 1;
        }

        let mut page = WebPageNative::new(&page_name);
        page.body(vec![body]);
        let routes = ctx.routes.clone();
        for (method, path, _) in &routes {
            page.script(&format!(
                "console.log('API route registered: {} {}');",
                method, path
            ));
        }
        page.run_with_routes(8080, routes)
    })?;
    Ok(Value::None)
}
