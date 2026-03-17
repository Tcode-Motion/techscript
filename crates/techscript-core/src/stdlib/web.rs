use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

// ── Web-based GUI Server Helper ─────────────────────────────────────────────
fn start_blocking_server(html: &str, module_name: &str, port: Option<u16>, open_browser: bool, once: bool) -> Result<(), String> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind localhost server");
    let bound_port = listener.local_addr().unwrap().port();
    let port = port.unwrap_or(bound_port);
    if port != bound_port {
        // Rebind to requested port
        drop(listener);
    }
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .map_err(|e| format!("server bind: {}", e))?;
    let url = format!("http://localhost:{}", port);

    if open_browser {
        #[cfg(windows)] {
            let _ = std::process::Command::new("cmd")
                .args(&["/C", "start", "msedge", "--app", &url])
                .spawn();
        }
        #[cfg(not(windows))] { let _ = std::process::Command::new("xdg-open").arg(&url).spawn(); }
    }
    
    println!("🚀 {} running at: {}", module_name, url);
    println!("Press Ctrl+C to stop the server.");

    let mut served = 0usize;
    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            use std::io::{Read, Write};
            let mut buffer = [0; 1024];
            let _ = s.read(&mut buffer);

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                html.len(),
                html
            );
            if s.write_all(response.as_bytes()).is_err() { continue; }
            let _ = s.flush();
        }
        served += 1;
        if once && served >= 1 {
            break;
        }
    }
    Ok(())
}

fn build_html_from_page_map(page: &HashMap<String, Value>) -> String {
    let title = page.get("title").map(|v| v.display_string()).unwrap_or("TechScript Page".into());
    let mut styles_css = String::new();
    if let Some(Value::List(s)) = page.get("styles").or_else(|| page.get("_styles")) {
        for st in s.borrow().iter() {
            styles_css.push_str(&st.display_string());
            styles_css.push('\n');
        }
    }
    let mut body_html = String::new();
    if let Some(Value::List(b)) = page.get("body").or_else(|| page.get("_body")) {
        for el in b.borrow().iter() {
            body_html.push_str(&el.display_string());
            body_html.push('\n');
        }
    }
    let mut scripts_js = String::new();
    if let Some(Value::List(sc)) = page.get("scripts").or_else(|| page.get("_scripts")) {
        for s in sc.borrow().iter() {
            scripts_js.push_str(&s.display_string());
            scripts_js.push('\n');
        }
    }
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<title>{}</title>\n<style>\n* {{ margin: 0; padding: 0; box-sizing: border-box; }}\nbody {{ font-family: 'Segoe UI', system-ui, sans-serif; background: #0a0a0a; color: #e0e0e0; }}\n{}\n</style>\n</head>\n<body>\n{}\n<script>\n{}\n</script>\n</body>\n</html>",
        title, styles_css, body_html, scripts_js
    )
}

fn should_open_by_default() -> bool {
    std::env::var("TECHSCRIPT_HEADLESS").is_err()
}

pub fn register_web_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("page", native!("web.page", |args| {
            let title = args.first().map(|v| v.display_string()).unwrap_or("TechScript Page".into());
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("title".into(), Value::String(Rc::new(title)));
            map.insert("body".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("styles".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("scripts".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("h1", native!("web.h1", |args| { Ok(Value::String(Rc::new(format!("<h1>{}</h1>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("h2", native!("web.h2", |args| { Ok(Value::String(Rc::new(format!("<h2>{}</h2>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("h3", native!("web.h3", |args| { Ok(Value::String(Rc::new(format!("<h3>{}</h3>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("p", native!("web.p", |args| { Ok(Value::String(Rc::new(format!("<p>{}</p>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("div", native!("web.div", |args| {
            let cls = args.get(1).map(|v| format!(" class=\"{}\"", v.display_string())).unwrap_or_default();
            let content = args.first().map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<div{}>{}</div>", cls, content))))
        })),
        ("span", native!("web.span", |args| { Ok(Value::String(Rc::new(format!("<span>{}</span>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("button", native!("web.button", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or("Click".into());
            let attrs = if let Some(Value::Map(m)) = args.get(1) { m.borrow().iter().map(|(k,v)| format!(" {}=\"{}\"", k, v.display_string())).collect::<String>() } else { String::new() };
            Ok(Value::String(Rc::new(format!("<button{}>{}</button>", attrs, text))))
        })),
        ("input", native!("web.input", |args| {
            let typ = args.first().map(|v| v.display_string()).unwrap_or("text".into());
            let placeholder = args.get(1).map(|v| format!(" placeholder=\"{}\"", v.display_string())).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<input type=\"{}\"{}/>", typ, placeholder))))
        })),
        ("img", native!("web.img", |args| {
            let src = args.first().map(|v| v.display_string()).unwrap_or_default();
            let alt = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<img src=\"{}\" alt=\"{}\"/>", src, alt))))
        })),
        ("a", native!("web.a", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or("Link".into());
            let href = args.get(1).map(|v| v.display_string()).unwrap_or("#".into());
            Ok(Value::String(Rc::new(format!("<a href=\"{}\">{}</a>", href, text))))
        })),
        ("ul", native!("web.ul", |args| {
            if let Some(Value::List(items)) = args.first() {
                let lis: String = items.borrow().iter().map(|v| format!("<li>{}</li>", v.display_string())).collect();
                Ok(Value::String(Rc::new(format!("<ul>{}</ul>", lis))))
            } else { Ok(Value::String(Rc::new("<ul></ul>".into()))) }
        })),
        ("table", native!("web.table", |args| {
            if let Some(Value::List(rows)) = args.first() {
                let mut html = String::from("<table>");
                for row in rows.borrow().iter() {
                    if let Value::List(cells) = row {
                        html.push_str("<tr>");
                        for cell in cells.borrow().iter() { html.push_str(&format!("<td>{}</td>", cell.display_string())); }
                        html.push_str("</tr>");
                    }
                }
                html.push_str("</table>");
                Ok(Value::String(Rc::new(html)))
            } else { Ok(Value::String(Rc::new("<table></table>".into()))) }
        })),
        ("form", native!("web.form", |args| {
            let action = args.first().map(|v| v.display_string()).unwrap_or("#".into());
            let content = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<form action=\"{}\" method=\"post\">{}</form>", action, content))))
        })),
        ("style", native!("web.style", |args| {
            let selector = args.first().map(|v| v.display_string()).unwrap_or_default();
            let mut css = format!("{} {{", selector);
            if let Some(Value::Map(props)) = args.get(1) {
                for (k, v) in props.borrow().iter() { css.push_str(&format!(" {}: {};", k, v.display_string())); }
            }
            css.push_str(" }");
            Ok(Value::String(Rc::new(css)))
        })),
        ("css_class", native!("web.css_class", |args| {
            let name = args.first().map(|v| v.display_string()).unwrap_or_default();
            let mut css = format!(".{} {{", name);
            if let Some(Value::Map(props)) = args.get(1) {
                for (k, v) in props.borrow().iter() { css.push_str(&format!(" {}: {};", k, v.display_string())); }
            }
            css.push_str(" }");
            Ok(Value::String(Rc::new(css)))
        })),
        ("layout", native!("web.layout", |args| {
            let typ = args.first().map(|v| v.display_string()).unwrap_or("flex".into());
            let children = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<div style=\"display: {}; gap: 1rem;\">{}</div>", typ, children))))
        })),
        ("build", native!("web.build", |args| {
            let page = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("web.build requires a page map".into()); };
            let title = page.get("title").map(|v| v.display_string()).unwrap_or("TechScript Page".into());
            let html = build_html_from_page_map(&page);
            let path = format!("{}_output.html", title.to_lowercase().replace(' ', "_"));
            std::fs::write(&path, &html).map_err(|e| format!("web.build: {}", e))?;
            Ok(Value::String(Rc::new(path)))
        })),
        ("render", native!("web.render", |args| {
            let page = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("web.render requires a page map".into()); };
            Ok(Value::String(Rc::new(build_html_from_page_map(&page))))
        })),
        ("open", native!("web.open", |args| {
            let path = args.first().map(|v| v.display_string()).unwrap_or_default();
            #[cfg(windows)] { let _ = std::process::Command::new("cmd").args(&["/C", "start", "", &path]).spawn(); }
            #[cfg(not(windows))] { let _ = std::process::Command::new("xdg-open").arg(&path).spawn(); }
            println!("🌐 Opened {} in browser", path);
            Ok(Value::None)
        })),
        ("serve", native!("web.serve", |args| {
            let port = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(8080);
            println!("🌐 TechScript web server on http://localhost:{}", port);
            println!("Press Ctrl+C to stop the server.");
            
            let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))
                .map_err(|e| format!("web.serve: {}", e))?;
            
            for stream in listener.incoming() {
                if let Ok(mut s) = stream {
                    use std::io::{Read, Write};
                    let mut buf = [0u8; 4096]; 
                    let n = s.read(&mut buf).unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]);
                    
                    let mut path = "/";
                    if let Some(line) = request.lines().next() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() > 1 { path = parts[1]; }
                    }

                    let file_path = if path == "/" || path == "/index.html" { 
                        "index.html".to_string() 
                    } else { 
                        path.trim_start_matches('/').to_string() 
                    };

                    let content = std::fs::read(&file_path).unwrap_or_else(|_| b"<h1>404 Not Found</h1>".to_vec());
                    let status = if std::path::Path::new(&file_path).exists() { "200 OK" } else { "404 Not Found" };
                    
                    let mime = if file_path.ends_with(".css") { "text/css" }
                               else if file_path.ends_with(".js") { "application/javascript" }
                               else if file_path.ends_with(".json") { "application/json" }
                               else if file_path.ends_with(".ico") { "image/x-icon" }
                               else if file_path.ends_with(".png") { "image/png" }
                               else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") { "image/jpeg" }
                               else { "text/html" };

                    let resp_header = format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", status, mime, content.len());
                    let mut full_resp = resp_header.into_bytes();
                    full_resp.extend(content);
                    let _ = s.write_all(&full_resp);
                    let _ = s.flush();
                }
            }
            Ok(Value::None)
        })),
        ("run", native!("web.run", |args| {
            let page = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("web.run requires a page map".into()); };
            let port = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i as u16) } else { None });
            let open = args.get(2).map(|v| v.is_truthy()).unwrap_or_else(should_open_by_default);
            let once = args.get(3).map(|v| v.is_truthy()).unwrap_or(false);
            let html = build_html_from_page_map(&page);
            start_blocking_server(&html, "Web App", port, open, once)?;
            Ok(Value::None)
        })),
    ]);
    globals.insert("web".into(), m);

    // Compatibility surface: WebPage("Title") -> object with methods: style/body/script/run/h1/...
    globals.insert("WebPage".into(), native!("WebPage", |args| {
        let title = args.first().map(|v| v.display_string()).unwrap_or("TechScript Page".into());

        let inner: Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));
        {
            let mut m = inner.borrow_mut();
            m.insert("title".into(), Value::String(Rc::new(title)));
            // Underscored storage keys prevent collisions with method names (body/style/script/run).
            m.insert("_body".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            m.insert("_styles".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            m.insert("_scripts".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
        }

        // Helper to add a method that can mutate `inner`.
        fn method(
            inner: Rc<RefCell<HashMap<String, Value>>>,
            name: &str,
            f: fn(Rc<RefCell<HashMap<String, Value>>>, &[Value]) -> Result<Value, String>,
        ) -> (String, Value) {
            let n = name.to_string();
            (n.clone(), Value::NativeFunction(Rc::new(crate::value::NativeFnObj {
                name: format!("WebPage.{}", n),
                func: Box::new(move |args| f(inner.clone(), args)),
            })))
        }

        let mut obj = inner.borrow().clone();
        let inner_for_methods = inner.clone();

        // style(selector, props_map)
        let (k, v) = method(inner_for_methods.clone(), "style", |inner, args| {
            let selector = args.first().map(|v| v.display_string()).unwrap_or_default();
            let props = args.get(1);
            let mut css = format!("{} {{", selector);
            if let Some(Value::Map(p)) = props {
                for (kk, vv) in p.borrow().iter() {
                    css.push_str(&format!(" {}: {};", kk, vv.display_string()));
                }
            }
            css.push_str(" }");
            if let Some(Value::List(styles)) = inner.borrow().get("_styles") {
                styles.borrow_mut().push(Value::String(Rc::new(css)));
            }
            Ok(Value::None)
        });
        obj.insert(k, v);

        // script(js)
        let (k, v) = method(inner_for_methods.clone(), "script", |inner, args| {
            let js = args.first().map(|v| v.display_string()).unwrap_or_default();
            if let Some(Value::List(scripts)) = inner.borrow().get("_scripts") {
                scripts.borrow_mut().push(Value::String(Rc::new(js)));
            }
            Ok(Value::None)
        });
        obj.insert(k, v);

        // body(list_of_elements)
        let (k, v) = method(inner_for_methods.clone(), "body", |inner, args| {
            if let Some(Value::List(new_body)) = args.first() {
                if let Some(Value::List(body)) = inner.borrow().get("_body") {
                    body.borrow_mut().clear();
                    body.borrow_mut().extend(new_body.borrow().iter().cloned());
                }
            }
            Ok(Value::None)
        });
        obj.insert(k, v);

        // run(port?, open?, once?)
        let (k, v) = method(inner_for_methods.clone(), "run", |inner, args| {
            let port = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i as u16) } else { None });
            let open = args.get(1).map(|v| v.is_truthy()).unwrap_or_else(should_open_by_default);
            let once = args.get(2).map(|v| v.is_truthy()).unwrap_or(false);
            let snapshot = inner.borrow().clone();
            let html = build_html_from_page_map(&snapshot);
            start_blocking_server(&html, "Web App", port, open, once)?;
            Ok(Value::None)
        });
        obj.insert(k, v);

        // Element helpers
        obj.insert("h1".into(), native!("WebPage.h1", |args| Ok(Value::String(Rc::new(format!("<h1>{}</h1>", args.first().map(|v| v.display_string()).unwrap_or_default()))))));
        obj.insert("h2".into(), native!("WebPage.h2", |args| Ok(Value::String(Rc::new(format!("<h2>{}</h2>", args.first().map(|v| v.display_string()).unwrap_or_default()))))));
        obj.insert("h3".into(), native!("WebPage.h3", |args| Ok(Value::String(Rc::new(format!("<h3>{}</h3>", args.first().map(|v| v.display_string()).unwrap_or_default()))))));
        obj.insert("p".into(), native!("WebPage.p", |args| Ok(Value::String(Rc::new(format!("<p>{}</p>", args.first().map(|v| v.display_string()).unwrap_or_default()))))));
        obj.insert("raw".into(), native!("WebPage.raw", |args| Ok(Value::String(Rc::new(args.first().map(|v| v.display_string()).unwrap_or_default())))));
        obj.insert("div".into(), native!("WebPage.div", |args| {
            let children = args.first().map(|v| v.display_string()).unwrap_or_default();
            let attrs = if let Some(Value::Map(m)) = args.get(1) { m.borrow().iter().map(|(k,v)| format!(" {}=\"{}\"", k, v.display_string())).collect::<String>() } else { String::new() };
            Ok(Value::String(Rc::new(format!("<div{}>{}</div>", attrs, children))))
        }));
        obj.insert("button".into(), native!("WebPage.button", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or("Click".into());
            let attrs = if let Some(Value::Map(m)) = args.get(1) { m.borrow().iter().map(|(k,v)| format!(" {}=\"{}\"", k, v.display_string())).collect::<String>() } else { String::new() };
            Ok(Value::String(Rc::new(format!("<button{}>{}</button>", attrs, text))))
        }));

        Ok(Value::Map(Rc::new(RefCell::new(obj))))
    }));
}

pub fn register_gui_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("window", native!("gui.window", |args| {
            let title = args.first().map(|v| v.display_string()).unwrap_or("TechScript App".into());
            let width = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(800);
            let height = args.get(2).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(600);
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("title".into(), Value::String(Rc::new(title)));
            map.insert("width".into(), Value::Int(width));
            map.insert("height".into(), Value::Int(height));
            map.insert("elements".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("styles".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("scripts".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("label", native!("gui.label", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<div class=\"gui-label\">{}</div>", text))))
        })),
        ("button", native!("gui.button", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or("Button".into());
            let onclick = args.get(1).map(|v| format!(" onclick=\"{}\"", v.display_string())).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<button class=\"gui-btn\"{}>{}</button>", onclick, text))))
        })),
        ("textbox", native!("gui.textbox", |args| {
            let placeholder = args.first().map(|v| v.display_string()).unwrap_or_default();
            let id = args.get(1).map(|v| v.display_string()).unwrap_or("input1".into());
            Ok(Value::String(Rc::new(format!("<input class=\"gui-input\" id=\"{}\" type=\"text\" placeholder=\"{}\"/>", id, placeholder))))
        })),
        ("checkbox", native!("gui.checkbox", |args| {
            let label = args.first().map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<label class=\"gui-check\"><input type=\"checkbox\"/> {}</label>", label))))
        })),
        ("dropdown", native!("gui.dropdown", |args| {
            if let Some(Value::List(items)) = args.first() {
                let opts: String = items.borrow().iter().map(|v| format!("<option>{}</option>", v.display_string())).collect();
                Ok(Value::String(Rc::new(format!("<select class=\"gui-select\">{}</select>", opts))))
            } else { Ok(Value::String(Rc::new("<select></select>".into()))) }
        })),
        ("vbox", native!("gui.vbox", |args| {
            let children = if let Some(Value::List(l)) = args.first() { l.borrow().iter().map(|v| v.display_string()).collect::<Vec<_>>().join("\n") } else { String::new() };
            Ok(Value::String(Rc::new(format!("<div class=\"gui-vbox\">{}</div>", children))))
        })),
        ("hbox", native!("gui.hbox", |args| {
            let children = if let Some(Value::List(l)) = args.first() { l.borrow().iter().map(|v| v.display_string()).collect::<Vec<_>>().join("\n") } else { String::new() };
            Ok(Value::String(Rc::new(format!("<div class=\"gui-hbox\">{}</div>", children))))
        })),
        ("show", native!("gui.show", |args| {
            let win = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("gui.show requires a window map".into()); };
            let title = win.get("title").map(|v| v.display_string()).unwrap_or("App".into());
            let mut body = String::new();
            if let Some(Value::List(els)) = win.get("elements") { for el in els.borrow().iter() { body.push_str(&el.display_string()); body.push('\n'); } }
            let html = format!("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>{}</title>\n<style>\n* {{ margin: 0; padding: 0; box-sizing: border-box; }}\nbody {{ font-family: 'Segoe UI', sans-serif; background: #1a1a2e; color: #eee; padding: 20px; }}\n.gui-label {{ padding: 8px 0; font-size: 14px; }}\n.gui-btn {{ background: #e94560; color: white; border: none; padding: 10px 24px; border-radius: 6px; cursor: pointer; font-size: 14px; margin: 4px; }} .gui-btn:hover {{ background: #c73852; }}\n.gui-input {{ background: #16213e; border: 1px solid #0f3460; color: #eee; padding: 10px; border-radius: 6px; width: 100%; margin: 4px 0; }}\n.gui-select {{ background: #16213e; border: 1px solid #0f3460; color: #eee; padding: 10px; border-radius: 6px; }}\n.gui-check {{ display: flex; align-items: center; gap: 8px; padding: 4px 0; }}\n.gui-vbox {{ display: flex; flex-direction: column; gap: 8px; }}\n.gui-hbox {{ display: flex; flex-direction: row; gap: 8px; align-items: center; }}\n</style></head>\n<body>\n<h2 style=\"margin-bottom:16px;color:#e94560;\">{}</h2>\n{}\n</body></html>", title, title, body);
            let path = format!("{}_gui.html", title.to_lowercase().replace(' ', "_"));
            let _ = std::fs::write(&path, &html); // Keep file for debugging
            
            let (port, open, once) = if let Some(Value::Map(opts)) = args.get(1) {
                let b = opts.borrow();
                let port = b.get("port").and_then(|v| if let Value::Int(i) = v { Some(*i as u16) } else { None });
                let open = b.get("open").map(|v| v.is_truthy()).unwrap_or_else(should_open_by_default);
                let once = b.get("once").map(|v| v.is_truthy()).unwrap_or(false);
                (port, open, once)
            } else {
                (None, should_open_by_default(), false)
            };

            start_blocking_server(&html, "GUI App", port, open, once)?;
            
            Ok(Value::String(Rc::new("blocking_server_closed".into())))
        })),
    ]);
    globals.insert("gui".into(), m);
}

pub fn register_three_d_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("scene", native!("scene.create", |_| {
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("objects".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("bg".into(), Value::String(Rc::new("#000011".into())));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("box", native!("scene.box", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#e94560".into());
            let size = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0);
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("box".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("size".into(), Value::Float(size));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("sphere", native!("scene.sphere", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#4ecdc4".into());
            let radius = args.get(1).and_then(|v| v.as_f64()).unwrap_or(0.5);
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("sphere".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("radius".into(), Value::Float(radius));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("cylinder", native!("scene.cylinder", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#ff6b6b".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("cylinder".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("plane", native!("scene.plane", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#333333".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("plane".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(-1.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("torus", native!("scene.torus", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#ffd93d".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("torus".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("mobius", native!("scene.mobius", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#ff00ff".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("mobius".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("blackhole", native!("scene.blackhole", |args| {
            let radius = args.first().and_then(|v| v.as_f64()).unwrap_or(1.0);
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("blackhole".into())));
            m.insert("radius".into(), Value::Float(radius));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("light", native!("scene.light", |args| {
            let ltype = args.first().map(|v| v.display_string()).unwrap_or("point".into());
            let color = args.get(1).map(|v| v.display_string()).unwrap_or("#ffffff".into());
            let intensity = args.get(2).and_then(|v| v.as_f64()).unwrap_or(1.0);
            let mut m = HashMap::new();
            m.insert("is_light".into(), Value::Bool(true));
            m.insert("type".into(), Value::String(Rc::new(ltype)));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("intensity".into(), Value::Float(intensity));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("render", native!("scene.render", |args| {
            let scene_map = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("scene.render requires a scene map".into()); };
            let bg = scene_map.get("bg").map(|v| v.display_string()).unwrap_or("#000011".into());
            let mut objects_js = String::new();
            if let Some(Value::List(objs)) = scene_map.get("objects") {
                for (i, obj) in objs.borrow().iter().enumerate() {
                    let s = match obj {
                        Value::Map(m) => {
                            let entries: Vec<String> = m.borrow().iter()
                                .map(|(k, v)| format!("{}:{}", k, match v {
                                    Value::String(s) => format!("'{}'", s),
                                    _ => v.display_string()
                                }))
                                .collect();
                            format!("{{{}}}", entries.join(","))
                        },
                        _ => obj.display_string(),
                    };
                    objects_js.push_str(&format!("addObject({}, {});\n", s, i));
                }
            }
            let html = format!(r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>TechScript 3D Infinity</title>
<style>body{{margin:0;overflow:hidden;background:{bg};}}</style>
<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>
</head><body><script>
const scene=new THREE.Scene();scene.background=new THREE.Color('{bg}');
const camera=new THREE.PerspectiveCamera(75,innerWidth/innerHeight,0.1,1000);
camera.position.set(5,5,10);
const renderer=new THREE.WebGLRenderer({{antialias:true}});
renderer.setSize(innerWidth,innerHeight);renderer.shadowMap.enabled=true;
document.body.appendChild(renderer.domElement);
const controls=new THREE.OrbitControls(camera,renderer.domElement);
scene.add(new THREE.AmbientLight(0x404040, 1.5));
const dl=new THREE.DirectionalLight(0xffffff,1);dl.position.set(5,10,7);scene.add(dl);
const meshes=[];
function addObject(o,i){{
  if(o.is_light){{
    let l; if(o.type==='ambient')l=new THREE.AmbientLight(o.color,o.intensity);
    else l=new THREE.DirectionalLight(o.color,o.intensity);
    l.position.set(o.x||0,o.y||0,o.z||0);scene.add(l);return;
  }}
  if(o.type==='blackhole'){{
    const g=new THREE.Group();
    const eh=new THREE.Mesh(new THREE.SphereGeometry(o.radius||1,32,32),new THREE.MeshBasicMaterial({{color:0x000000}}));
    const disk=new THREE.Mesh(new THREE.TorusGeometry((o.radius||1)*2.5,(o.radius||1)*0.4,2,64),new THREE.MeshStandardMaterial({{color:0xffaa00,emissive:0xff6600,emissiveIntensity:2,transparent:true,opacity:0.9,side:THREE.DoubleSide}}));
    disk.rotation.x=Math.PI/2;g.add(eh);g.add(disk);g.position.set(o.x||0,o.y||0,o.z||0);
    scene.add(g);meshes.push(g);return;
  }}
  let g,m=new THREE.MeshStandardMaterial({{color:o.color||0x4ecdc4,metalness:0.3,roughness:0.4,side:THREE.DoubleSide}});
  if(o.type==='box')g=new THREE.BoxGeometry(o.size||1,o.size||1,o.size||1);
  else if(o.type==='sphere')g=new THREE.SphereGeometry(o.radius||0.5,32,32);
  else if(o.type==='cylinder')g=new THREE.CylinderGeometry(o.size||0.5,o.size||0.5,1.5,32);
  else if(o.type==='plane'){{g=new THREE.PlaneGeometry(10,10);m.side=THREE.DoubleSide;}}
  else if(o.type==='torus')g=new THREE.TorusGeometry(0.7,0.3,16,48);
  else if(o.type==='mobius'){{
    const f=(u,v,t)=>{{u*=Math.PI*2;v-=0.5;const a=3;t.set(Math.cos(u)*(a+v*Math.cos(u/2)),Math.sin(u)*(a+v*Math.cos(u/2)),v*Math.sin(u/2));}};
    g=new THREE.ParametricBufferGeometry(f,40,40);
  }}
  else g=new THREE.BoxGeometry(1,1,1);
  const mesh=new THREE.Mesh(g,m);mesh.position.set(o.x||0,o.y||0,o.z||0);
  if(o.type==='plane')mesh.rotation.x=-Math.PI/2;
  scene.add(mesh);meshes.push(mesh);
}}
{objects_js}
let t=0;function animate(){{requestAnimationFrame(animate);t+=0.01;controls.update();
  meshes.forEach((m,i)=>{{
      if(m.type==='Group'){{m.children[1].rotation.z+=0.02;}}
      else if(m.geometry && m.geometry.type!=='PlaneGeometry'){{m.rotation.x=t*(i+1)*0.2;m.rotation.y=t*(i+1)*0.1;}}
  }});
  renderer.render(scene,camera);}}
animate();
window.addEventListener('resize',()=>{{camera.aspect=innerWidth/innerHeight;camera.updateProjectionMatrix();renderer.setSize(innerWidth,innerHeight);}});
</script></body></html>"#);
            let path = "techscript_3d.html";
            let _ = std::fs::write(path, &html);
            
            let (port, open, once) = if let Some(Value::Map(opts)) = args.get(1) {
                let b = opts.borrow();
                let port = b.get("port").and_then(|v| if let Value::Int(i) = v { Some(*i as u16) } else { None });
                let open = b.get("open").map(|v| v.is_truthy()).unwrap_or_else(should_open_by_default);
                let once = b.get("once").map(|v| v.is_truthy()).unwrap_or(false);
                (port, open, once)
            } else {
                (None, should_open_by_default(), false)
            };

            start_blocking_server(&html, "3D Infinity Scene", port, open, once)?;
            
            Ok(Value::String(Rc::new("blocking_server_closed".into())))
        })),
    ]);
    globals.insert("scene".into(), m);
}

pub fn register_anime_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("create", native!("anime.create", |_| {
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("elements".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("animations".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("styles".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("scripts".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("target", native!("anime.target", |args| {
            let tag = args.first().map(|v| v.display_string()).unwrap_or("div".into());
            let text = args.get(1).map(|v| v.display_string()).unwrap_or("●".into());
            let cls = args.get(2).map(|v| v.display_string()).unwrap_or("anim-el".into());
            Ok(Value::String(Rc::new(format!("<{} class=\"{}\">{}</{}>", tag, cls, text, tag))))
        })),
        ("animate", native!("anime.animate", |args| {
            let selector = args.first().map(|v| v.display_string()).unwrap_or(".anim-el".into());
            let mut config = format!("anime({{targets:'{}',", selector);
            if let Some(Value::Map(props)) = args.get(1) {
                for (k, v) in props.borrow().iter() {
                    let val = v.display_string();
                    let is_expression = val.parse::<f64>().is_ok() 
                        || val == "true" 
                        || val == "false"
                        || val.starts_with("anime.");
                    
                    if is_expression { 
                        config.push_str(&format!("{}:{},", k, val)); 
                    } else { 
                        config.push_str(&format!("{}:'{}',", k, val)); 
                    }
                }
            }
            config.push_str("});");
            Ok(Value::String(Rc::new(config)))
        })),
        ("timeline", native!("anime.timeline", |_| {
            Ok(Value::String(Rc::new("const tl = anime.timeline({easing:'easeOutExpo',duration:750});".into())))
        })),
        ("stagger", native!("anime.stagger", |args| {
            let delay = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100);
            Ok(Value::String(Rc::new(format!("anime.stagger({})", delay))))
        })),
        ("render", native!("anime.render", |args| {
            let scene = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("anime.render requires a scene map".into()); };
            let mut elements = String::new();
            if let Some(Value::List(els)) = scene.get("elements") { for el in els.borrow().iter() { elements.push_str(&el.display_string()); } }
            let mut anims = String::new();
            if let Some(Value::List(a)) = scene.get("animations") { for an in a.borrow().iter() { anims.push_str(&an.display_string()); anims.push('\n'); } }
            let mut styles = String::new();
            if let Some(Value::List(s)) = scene.get("styles") { for st in s.borrow().iter() { styles.push_str(&st.display_string()); styles.push('\n'); } }
            let mut scripts = String::new();
            if let Some(Value::List(sc)) = scene.get("scripts") { for s in sc.borrow().iter() { scripts.push_str(&s.display_string()); scripts.push('\n'); } }
            let html = format!("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>TechScript Animation</title>\n<style>body{{margin:0;background:#0a0a0a;color:#eee;font-family:sans-serif;display:flex;justify-content:center;align-items:center;min-height:100vh;}}\n.anim-el{{width:50px;height:50px;background:#e94560;border-radius:8px;margin:8px;}}\n{}</style>\n<script src=\"https://cdnjs.cloudflare.com/ajax/libs/animejs/3.2.2/anime.min.js\"></script>\n</head><body>\n{}<script>\n{}\n{}\n</script></body></html>", styles, elements, scripts, anims);
            let path = "techscript_animation.html";
            let _ = std::fs::write(path, &html);
            
            let (port, open, once) = if let Some(Value::Map(opts)) = args.get(1) {
                let b = opts.borrow();
                let port = b.get("port").and_then(|v| if let Value::Int(i) = v { Some(*i as u16) } else { None });
                let open = b.get("open").map(|v| v.is_truthy()).unwrap_or_else(should_open_by_default);
                let once = b.get("once").map(|v| v.is_truthy()).unwrap_or(false);
                (port, open, once)
            } else {
                (None, should_open_by_default(), false)
            };

            start_blocking_server(&html, "Animation Viewer", port, open, once)?;

            Ok(Value::String(Rc::new("blocking_server_closed".into())))
        })),
    ]);
    globals.insert("anime".into(), m);
}
