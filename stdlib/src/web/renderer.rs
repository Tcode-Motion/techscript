//! # WebForge Structured HTML/CSS/JS Renderer
//!
//! Provides AST lowering for declarative TechScript WebForge UI constructs:
//! layouts, navigation, routes, components, responsive styles, 3D WebGL scenes,
//! and interactive event bindings.

use crate::web::{db, runtime_js, styles};
use std::fmt::Write;
use techscript_runtime::value::{DslBlockValue, RuntimeValue};

/// Escape raw strings for secure HTML rendering, preventing XSS vulnerabilities.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Tokenize and syntax highlight TechScript / WebForge source code snippets into HTML.
pub fn highlight_techscript_code(raw: &str) -> String {
    let mut out = String::new();
    for (line_idx, line) in raw.lines().enumerate() {
        if line_idx > 0 {
            out.push('\n');
        }
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            let _ = write!(
                out,
                "<span class=\"tok-comment\">{}</span>",
                escape_html(line)
            );
            continue;
        }

        let mut in_str = false;
        let mut cur_token = String::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            if c == '"' {
                if in_str {
                    cur_token.push(c);
                    let _ = write!(
                        out,
                        "<span class=\"tok-string\">{}</span>",
                        escape_html(&cur_token)
                    );
                    cur_token.clear();
                    in_str = false;
                } else {
                    out.push_str(&escape_html(&cur_token));
                    cur_token.clear();
                    cur_token.push(c);
                    in_str = true;
                }
                i += 1;
                continue;
            }
            if in_str {
                cur_token.push(c);
                i += 1;
                continue;
            }

            if c.is_alphanumeric() || c == '_' {
                cur_token.push(c);
                i += 1;
                continue;
            }

            if !cur_token.is_empty() {
                match cur_token.as_str() {
                    "let" | "const" | "function" | "fn" | "return" | "match" | "case" | "if"
                    | "else" | "for" | "while" | "print" | "import" | "export" | "make"
                    | "website" | "page" | "show" | "heading" | "section" | "card" | "hero"
                    | "button" | "link" | "text" | "action" | "badge" | "code" | "title"
                    | "subtitle" | "url" | "scene3d" | "scene" | "add" | "end" | "is"
                    | "called" | "table" | "database" | "enable" => {
                        let _ = write!(
                            out,
                            "<span class=\"tok-keyword\">{}</span>",
                            escape_html(&cur_token)
                        );
                    }
                    "Float" | "Int" | "String" | "Boolean" | "List" | "Map" | "Scene" | "Cube" => {
                        let _ = write!(
                            out,
                            "<span class=\"tok-type\">{}</span>",
                            escape_html(&cur_token)
                        );
                    }
                    "true" | "false" | "null" | "nil" => {
                        let _ = write!(
                            out,
                            "<span class=\"tok-literal\">{}</span>",
                            escape_html(&cur_token)
                        );
                    }
                    tok if tok.chars().all(|ch| ch.is_ascii_digit()) => {
                        let _ = write!(
                            out,
                            "<span class=\"tok-number\">{}</span>",
                            escape_html(&cur_token)
                        );
                    }
                    _ => {
                        out.push_str(&escape_html(&cur_token));
                    }
                }
                cur_token.clear();
            }

            let _ = write!(out, "{}", escape_html(&c.to_string()));
            i += 1;
        }

        if in_str {
            let _ = write!(
                out,
                "<span class=\"tok-string\">{}</span>",
                escape_html(&cur_token)
            );
        } else if !cur_token.is_empty() {
            match cur_token.as_str() {
                "let" | "const" | "function" | "fn" | "return" | "match" | "case" | "if"
                | "else" | "for" | "while" | "print" | "import" | "export" | "make" | "website"
                | "page" | "show" | "heading" | "section" | "card" | "hero" | "button" | "link"
                | "text" | "action" | "badge" | "code" | "title" | "subtitle" | "url"
                | "scene3d" | "scene" | "add" | "end" | "is" | "called" | "table" | "database"
                | "enable" => {
                    let _ = write!(
                        out,
                        "<span class=\"tok-keyword\">{}</span>",
                        escape_html(&cur_token)
                    );
                }
                _ => out.push_str(&escape_html(&cur_token)),
            }
        }
    }
    out
}

pub fn render_children(html: &mut String, dsl: &DslBlockValue) {
    for child in &dsl.children {
        dsl_to_html_inner(
            &RuntimeValue::DslBlock(std::rc::Rc::new(child.clone())),
            html,
        );
    }
}

pub fn render_page(html: &mut String, dsl: &DslBlockValue) {
    let mut page_name = String::new();
    let mut page_route = String::new();

    if let Some(arg0) = dsl.args.first() {
        page_name = arg0.to_string();
    }
    if let Some(arg1) = dsl.args.get(1) {
        page_route = arg1.to_string();
    }

    for prop in &dsl.properties {
        if prop.name == "id" {
            if let Some(ref v) = prop.value {
                page_name = v.to_string();
            }
        } else if prop.name == "route" {
            if let Some(ref v) = prop.value {
                page_route = v.to_string();
            }
        }
    }

    if page_route.is_empty() {
        let n = page_name.to_lowercase().replace(' ', "-");
        page_route = if n == "home" {
            "/".to_string()
        } else {
            format!("/{}", n)
        };
    }

    let safe_route_id = if page_route == "/" {
        "page-home".to_string()
    } else {
        format!(
            "page-{}",
            page_route.trim_start_matches('/').replace('/', "-")
        )
    };

    let is_home = page_route == "/" || page_route == "/home";
    let _ = write!(
        html,
        "<div class=\"webforge-page-view{}\" id=\"{}\" data-route=\"{}\" data-title=\"{}\">",
        if is_home { " active-page" } else { "" },
        escape_html(&safe_route_id),
        escape_html(&page_route),
        escape_html(&page_name)
    );

    // Render breadcrumbs if nested route
    if page_route.starts_with("/docs/")
        || page_route.starts_with("/showcase/")
        || page_route.starts_with("/examples/")
    {
        let parts: Vec<&str> = page_route.trim_start_matches('/').split('/').collect();
        if parts.len() >= 2 {
            let _ = write!(
                html,
                "<div class=\"webforge-breadcrumbs\"><a href=\"/\" onclick=\"event.preventDefault();webforgeRouter.navigate('/')\">Home</a> <span class=\"sep\">/</span> <a href=\"/{}\" onclick=\"event.preventDefault();webforgeRouter.navigate('/{}')\">{}</a> <span class=\"sep\">/</span> <span class=\"current\">{}</span></div>",
                parts[0],
                parts[0],
                parts[0].replace('-', " ").to_uppercase(),
                escape_html(&page_name)
            );
        }
    }

    for prop in &dsl.properties {
        if prop.name == "title" {
            if let Some(RuntimeValue::Str(t)) = &prop.value {
                let _ = write!(
                    html,
                    "<h2 class=\"webforge-subheading\">{}</h2>",
                    escape_html(t)
                );
            }
        } else if prop.name == "text" {
            if let Some(RuntimeValue::Str(t)) = &prop.value {
                let _ = write!(html, "<p class=\"webforge-text\">{}</p>", escape_html(t));
            }
        }
    }
    render_children(html, dsl);
    html.push_str("</div>");
}

pub fn render_hero(html: &mut String, dsl: &DslBlockValue) {
    html.push_str("<section class=\"hero\">");
    html.push_str("<div class=\"hero-brand-pill\"><span class=\"pulse-dot\"></span> <span>TechScript v2.0 • WebForge Full-Stack Framework</span></div>");
    for prop in &dsl.properties {
        match prop.name.as_str() {
            "title" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    let _ = write!(
                        html,
                        "<h1 class=\"webforge-heading\">{}</h1>",
                        escape_html(t)
                    );
                }
            }
            "subtitle" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    let _ = write!(
                        html,
                        "<p class=\"subtitle webforge-text\">{}</p>",
                        escape_html(t)
                    );
                }
            }
            _ => {}
        }
    }
    html.push_str("<div class=\"hero-actions\">");
    render_children(html, dsl);
    html.push_str("</div>");
    html.push_str("</section>");
}

pub fn render_section(html: &mut String, dsl: &DslBlockValue) {
    let mut sec_id = String::new();
    for prop in &dsl.properties {
        match prop.name.as_str() {
            "title" => {
                if sec_id.is_empty() {
                    if let Some(RuntimeValue::Str(t)) = &prop.value {
                        sec_id = t.to_lowercase().replace(' ', "-");
                    }
                }
            }
            "id" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    sec_id = t.to_string();
                }
            }
            _ => {}
        }
    }
    if !sec_id.is_empty() {
        let _ = write!(
            html,
            "<section class=\"content-section\" id=\"{}\">",
            escape_html(&sec_id)
        );
    } else {
        html.push_str("<section class=\"content-section\">");
    }
    for prop in &dsl.properties {
        if prop.name == "title" {
            if let Some(RuntimeValue::Str(t)) = &prop.value {
                let _ = write!(
                    html,
                    "<h2 class=\"webforge-subheading\">{}</h2>",
                    escape_html(t)
                );
            }
        }
    }
    html.push_str("<div class=\"grid-layout\">");
    render_children(html, dsl);
    html.push_str("</div>");
    html.push_str("</section>");
}

pub fn render_card(html: &mut String, dsl: &DslBlockValue) {
    html.push_str("<div class=\"card\">");
    if let Some(RuntimeValue::Str(t)) = dsl.args.first() {
        let _ = write!(html, "<h3>{}</h3>", escape_html(t));
    }
    for prop in &dsl.properties {
        match prop.name.as_str() {
            "title" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    let _ = write!(html, "<h3>{}</h3>", escape_html(t));
                }
            }
            "text" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    let _ = write!(html, "<p>{}</p>", escape_html(t));
                }
            }
            "badge" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    let _ = write!(html, "<span class=\"badge\">{}</span>", escape_html(t));
                }
            }
            "code" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    let unescaped = t
                        .replace("\\n", "\n")
                        .replace("\\t", "    ")
                        .replace("\\\"", "\"");
                    let highlighted = highlight_techscript_code(&unescaped);
                    let _ = write!(
                        html,
                        "<div class=\"code-window\">\
                            <div class=\"code-window-bar\">\
                                <div class=\"code-dots\"><span class=\"dot dot-red\"></span><span class=\"dot dot-yellow\"></span><span class=\"dot dot-green\"></span></div>\
                                <span class=\"code-window-title\">snippet.txs</span>\
                                <button type=\"button\" class=\"copy-btn\" onclick=\"webforgeCopyCode(this)\">Copy</button>\
                            </div>\
                            <pre class=\"webforge-code-block\"><code>{}</code></pre>\
                        </div>",
                        highlighted
                    );
                }
            }
            "image" => {
                if let Some(RuntimeValue::Str(t)) = &prop.value {
                    let _ = write!(
                        html,
                        "<img src=\"{}\" alt=\"card image\" class=\"card-img\">",
                        escape_html(t)
                    );
                }
            }
            _ => {}
        }
    }
    render_children(html, dsl);
    html.push_str("</div>");
}

pub fn render_heading(html: &mut String, dsl: &DslBlockValue) {
    let mut id_attr = String::new();
    let text = dsl
        .args
        .first()
        .map(|a| a.to_string())
        .or_else(|| {
            dsl.properties
                .iter()
                .find(|p| p.name == "text")
                .and_then(|p| p.value.as_ref().map(|v| v.to_string()))
        })
        .unwrap_or_else(|| "Heading".to_string());
    for p in &dsl.properties {
        if p.name == "id" {
            if let Some(ref v) = p.value {
                id_attr = format!(" id=\"{}\"", escape_html(&v.to_string()));
            }
        }
    }
    let _ = write!(
        html,
        "<h1{} class=\"webforge-heading\">{}</h1>",
        id_attr,
        escape_html(&text)
    );
}

pub fn render_text(html: &mut String, dsl: &DslBlockValue) {
    let mut id_attr = String::new();
    let text = dsl
        .args
        .first()
        .map(|a| a.to_string())
        .or_else(|| {
            dsl.properties
                .iter()
                .find(|p| p.name == "text" || p.name == "content")
                .and_then(|p| p.value.as_ref().map(|v| v.to_string()))
        })
        .unwrap_or_default();
    for p in &dsl.properties {
        if p.name == "id" {
            if let Some(ref v) = p.value {
                id_attr = format!(" id=\"{}\"", escape_html(&v.to_string()));
            }
        }
    }
    let _ = write!(
        html,
        "<p{} class=\"webforge-text\">{}</p>",
        id_attr,
        escape_html(&text)
    );
}

pub fn render_button(html: &mut String, dsl: &DslBlockValue) {
    let mut id_attr = String::new();
    let mut onclick_attr = String::new();
    let mut variant = String::new();
    let mut animation = String::new();
    let mut size = String::new();
    let mut icon = String::new();
    let mut label_val = dsl.args.first().map(|a| a.to_string());
    for p in &dsl.properties {
        match p.name.as_str() {
            "label" | "text" => {
                if label_val.is_none() {
                    label_val = p.value.as_ref().map(|v| v.to_string());
                }
            }
            "id" => {
                if let Some(ref v) = p.value {
                    id_attr = format!(" id=\"{}\"", escape_html(&v.to_string()));
                }
            }
            "variant" | "style" => {
                if let Some(ref v) = p.value {
                    variant = v.to_string();
                }
            }
            "animation" | "animate" => {
                if let Some(ref v) = p.value {
                    animation = v.to_string();
                }
            }
            "size" => {
                if let Some(ref v) = p.value {
                    size = v.to_string();
                }
            }
            "icon" => {
                if let Some(ref v) = p.value {
                    icon = v.to_string();
                }
            }
            "action" => {
                if let Some(ref v) = p.value {
                    let act = v.to_string();
                    if let Some(target) = act.strip_prefix("open ") {
                        let clean = target.trim();
                        if clean.starts_with("http://")
                            || clean.starts_with("https://")
                            || clean.starts_with("mailto:")
                            || clean.starts_with("/api/")
                        {
                            onclick_attr = format!(
                                " onclick=\"window.open('{}', '_blank')\"",
                                escape_html(clean)
                            );
                        } else {
                            let route = if clean.starts_with('/') {
                                clean.to_string()
                            } else if clean.starts_with('#') {
                                clean.to_string()
                            } else {
                                format!("/{}", clean.to_lowercase().replace(' ', "-"))
                            };
                            onclick_attr = format!(
                                " onclick=\"webforgeRouter.navigate('{}')\"",
                                escape_html(&route)
                            );
                        }
                    } else if act.contains("+=") || act.contains("=") {
                        onclick_attr =
                            format!(" onclick=\"webforgeAction('{}')\"", escape_html(&act));
                    } else {
                        onclick_attr = format!(" data-action=\"{}\"", escape_html(&act));
                    }
                }
            }
            _ => {}
        }
    }
    let label = label_val.unwrap_or_else(|| "Button".to_string());
    let mut classes = vec!["webforge-btn".to_string()];
    match variant.to_lowercase().as_str() {
        "glow" => classes.push("btn-glow".to_string()),
        "shimmer" => classes.push("btn-shimmer".to_string()),
        "cyber" => classes.push("btn-cyber".to_string()),
        "outline" => classes.push("btn-outline".to_string()),
        "pill" => classes.push("btn-pill".to_string()),
        "glass" => classes.push("btn-glass".to_string()),
        _ => {}
    }
    match animation.to_lowercase().as_str() {
        "pulse" => classes.push("btn-anim-pulse".to_string()),
        "bounce" => classes.push("btn-anim-bounce".to_string()),
        "float" => classes.push("btn-anim-float".to_string()),
        "glitch" => classes.push("btn-anim-glitch".to_string()),
        _ => {}
    }
    match size.to_lowercase().as_str() {
        "sm" | "small" => classes.push("btn-sm".to_string()),
        "lg" | "large" => classes.push("btn-lg".to_string()),
        _ => {}
    }
    let icon_html = if !icon.is_empty() {
        format!("<span class=\"btn-icon\">{}</span> ", escape_html(&icon))
    } else {
        String::new()
    };
    let _ = write!(
        html,
        "<button{}{} class=\"{}\">{}{}</button>",
        id_attr,
        onclick_attr,
        classes.join(" "),
        icon_html,
        escape_html(&label)
    );
}

pub fn render_timeline(html: &mut String, dsl: &DslBlockValue) {
    let mut id_attr = String::new();
    if let Some(arg0) = dsl.args.first() {
        id_attr = format!(
            " id=\"timeline-{}\"",
            escape_html(&arg0.to_string().to_lowercase().replace(' ', "-"))
        );
    }
    for p in &dsl.properties {
        if p.name == "id" {
            if let Some(ref v) = p.value {
                id_attr = format!(" id=\"{}\"", escape_html(&v.to_string()));
            }
        }
    }
    let _ = write!(html, "<div{} class=\"webforge-timeline\">", id_attr);
    for (idx, child) in dsl.children.iter().enumerate() {
        let mut title = child
            .args
            .first()
            .map(|a| a.to_string())
            .unwrap_or_default();
        let mut desc = String::new();
        let mut date = String::new();
        let mut status = "done".to_string();
        let mut badge = String::new();
        let mut icon = "✓".to_string();
        for p in &child.properties {
            match p.name.as_str() {
                "title" => {
                    if title.is_empty() {
                        if let Some(ref v) = p.value {
                            title = v.to_string();
                        }
                    }
                }
                "description" | "text" | "content" => {
                    if let Some(ref v) = p.value {
                        desc = v.to_string();
                    }
                }
                "date" | "time" | "quarter" => {
                    if let Some(ref v) = p.value {
                        date = v.to_string();
                    }
                }
                "status" => {
                    if let Some(ref v) = p.value {
                        status = v.to_string();
                    }
                }
                "badge" => {
                    if let Some(ref v) = p.value {
                        badge = v.to_string();
                    }
                }
                "icon" => {
                    if let Some(ref v) = p.value {
                        icon = v.to_string();
                    }
                }
                _ => {}
            }
        }
        if title.is_empty() {
            title = format!("Step {}", idx + 1);
        }
        let status_class = match status.to_lowercase().as_str() {
            "completed" | "done" => "status-done",
            "active" | "current" | "in_progress" => "status-active",
            _ => "status-upcoming",
        };
        let _ = write!(
            html,
            "<div class=\"timeline-item {status_class}\">\
                <div class=\"timeline-marker\"><span class=\"timeline-icon\">{icon}</span></div>\
                <div class=\"timeline-content\">\
                    <div class=\"timeline-header\">\
                        <h4 class=\"timeline-title\">{title}</h4>\
                        {badge_html}\
                        {date_html}\
                    </div>\
                    <p class=\"timeline-desc\">{desc}</p>\
                </div>\
            </div>",
            status_class = status_class,
            icon = escape_html(&icon),
            title = escape_html(&title),
            badge_html = if !badge.is_empty() {
                format!(
                    "<span class=\"timeline-badge\">{}</span>",
                    escape_html(&badge)
                )
            } else {
                String::new()
            },
            date_html = if !date.is_empty() {
                format!(
                    "<span class=\"timeline-date\">{}</span>",
                    escape_html(&date)
                )
            } else {
                String::new()
            },
            desc = escape_html(&desc)
        );
    }
    html.push_str("</div>");
}

pub fn render_accordion(html: &mut String, dsl: &DslBlockValue) {
    let mut id_attr = String::new();
    if let Some(arg0) = dsl.args.first() {
        id_attr = format!(
            " id=\"accordion-{}\"",
            escape_html(&arg0.to_string().to_lowercase().replace(' ', "-"))
        );
    }
    let _ = write!(html, "<div{} class=\"webforge-accordion\">", id_attr);
    for (idx, child) in dsl.children.iter().enumerate() {
        let mut title = child
            .args
            .first()
            .map(|a| a.to_string())
            .unwrap_or_default();
        let mut content = String::new();
        for p in &child.properties {
            match p.name.as_str() {
                "title" => {
                    if title.is_empty() {
                        if let Some(ref v) = p.value {
                            title = v.to_string();
                        }
                    }
                }
                "content" | "text" | "description" => {
                    if let Some(ref v) = p.value {
                        content = v.to_string();
                    }
                }
                _ => {}
            }
        }
        if title.is_empty() {
            title = format!("Item {}", idx + 1);
        }
        let is_first = idx == 0;
        let _ = write!(
            html,
            "<div class=\"accordion-item{}\">\
                <button type=\"button\" class=\"accordion-trigger\" onclick=\"this.parentElement.classList.toggle('open')\">\
                    <span>{}</span>\
                    <svg class=\"accordion-chevron\" width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.5\"><polyline points=\"6 9 12 15 18 9\"/></svg>\
                </button>\
                <div class=\"accordion-body\"><div class=\"accordion-inner\">{}</div></div>\
            </div>",
            if is_first { " open" } else { "" },
            escape_html(&title),
            escape_html(&content)
        );
    }
    html.push_str("</div>");
}

pub fn render_tabs(html: &mut String, dsl: &DslBlockValue) {
    let tab_id = dsl
        .args
        .first()
        .map(|a| a.to_string())
        .unwrap_or_else(|| "tabs".to_string());
    let safe_id = escape_html(&tab_id.to_lowercase().replace(' ', "-"));
    let _ = write!(
        html,
        "<div class=\"webforge-tabs webforge-tabs-container\" id=\"tabs-{}\">",
        safe_id
    );
    html.push_str("<div class=\"tabs-nav\">");
    for (idx, child) in dsl.children.iter().enumerate() {
        let mut title = child
            .args
            .first()
            .map(|a| a.to_string())
            .unwrap_or_default();
        for p in &child.properties {
            if p.name == "title" && title.is_empty() {
                if let Some(ref v) = p.value {
                    title = v.to_string();
                }
            }
        }
        if title.is_empty() {
            title = format!("Tab {}", idx + 1);
        }
        let _ = write!(
            html,
            "<button type=\"button\" class=\"tab-btn{}\" onclick=\"webforgeSelectTab(this, '{}-panel-{}')\">{}</button>",
            if idx == 0 { " active" } else { "" },
            safe_id,
            idx,
            escape_html(&title)
        );
    }
    html.push_str("</div><div class=\"tabs-content\">");
    for (idx, child) in dsl.children.iter().enumerate() {
        let mut content = String::new();
        for p in &child.properties {
            if (p.name == "text" || p.name == "content" || p.name == "description")
                && content.is_empty()
            {
                if let Some(ref v) = p.value {
                    content = v.to_string();
                }
            }
        }
        let _ = write!(
            html,
            "<div class=\"tab-panel{}\" id=\"{}-panel-{}\">",
            if idx == 0 { " active" } else { "" },
            safe_id,
            idx
        );
        if !content.is_empty() {
            let _ = write!(
                html,
                "<p class=\"webforge-text\">{}</p>",
                escape_html(&content)
            );
        }
        render_children(html, child);
        html.push_str("</div>");
    }
    html.push_str("</div></div>");
}

pub fn render_stats(html: &mut String, dsl: &DslBlockValue) {
    html.push_str("<div class=\"webforge-stats-grid\">");
    for child in &dsl.children {
        let mut value = child
            .args
            .first()
            .map(|a| a.to_string())
            .unwrap_or_default();
        let mut label = String::new();
        let mut trend = String::new();
        let mut icon = String::new();
        for p in &child.properties {
            match p.name.as_str() {
                "value" | "number" => {
                    if value.is_empty() {
                        if let Some(ref v) = p.value {
                            value = v.to_string();
                        }
                    }
                }
                "label" | "title" => {
                    if let Some(ref v) = p.value {
                        label = v.to_string();
                    }
                }
                "trend" | "change" => {
                    if let Some(ref v) = p.value {
                        trend = v.to_string();
                    }
                }
                "icon" => {
                    if let Some(ref v) = p.value {
                        icon = v.to_string();
                    }
                }
                _ => {}
            }
        }
        let _ = write!(
            html,
            "<div class=\"webforge-stat-card\">\
                <div class=\"stat-top\">\
                    <span class=\"stat-label\">{}</span>\
                    {}{}\
                </div>\
                <div class=\"stat-value\">{}</div>\
            </div>",
            escape_html(&label),
            if !icon.is_empty() {
                format!("<span class=\"stat-icon\">{}</span>", escape_html(&icon))
            } else {
                String::new()
            },
            if !trend.is_empty() {
                format!("<span class=\"stat-trend\">{}</span>", escape_html(&trend))
            } else {
                String::new()
            },
            escape_html(&value)
        );
    }
    html.push_str("</div>");
}

pub fn render_alert(html: &mut String, dsl: &DslBlockValue) {
    let mut message = dsl.args.first().map(|a| a.to_string()).unwrap_or_default();
    let mut variant = "info".to_string();
    let mut title = String::new();
    for p in &dsl.properties {
        match p.name.as_str() {
            "message" | "text" | "content" => {
                if message.is_empty() {
                    if let Some(ref v) = p.value {
                        message = v.to_string();
                    }
                }
            }
            "variant" | "type" => {
                if let Some(ref v) = p.value {
                    variant = v.to_string();
                }
            }
            "title" => {
                if let Some(ref v) = p.value {
                    title = v.to_string();
                }
            }
            _ => {}
        }
    }
    let icon = match variant.to_lowercase().as_str() {
        "success" => "✅",
        "warning" => "⚠️",
        "error" | "danger" => "🛑",
        "tip" => "💡",
        _ => "ℹ️",
    };
    let _ = write!(
        html,
        "<div class=\"webforge-alert alert-{}\">\
            <div class=\"alert-icon\">{}</div>\
            <div class=\"alert-body\">\
                {}{}\
            </div>\
        </div>",
        escape_html(&variant.to_lowercase()),
        icon,
        if !title.is_empty() {
            format!("<div class=\"alert-title\">{}</div>", escape_html(&title))
        } else {
            String::new()
        },
        format!("<div class=\"alert-text\">{}</div>", escape_html(&message))
    );
}

pub fn render_input(html: &mut String, dsl: &DslBlockValue) {
    let mut id_attr = String::new();
    let mut placeholder_val = dsl.args.first().map(|a| a.to_string());
    for p in &dsl.properties {
        if p.name == "placeholder" && placeholder_val.is_none() {
            placeholder_val = p.value.as_ref().map(|v| v.to_string());
        } else if p.name == "id" {
            if let Some(ref v) = p.value {
                id_attr = format!(" id=\"{}\"", escape_html(&v.to_string()));
            }
        }
    }
    let placeholder = placeholder_val.unwrap_or_default();
    let _ = write!(
        html,
        "<input{} class=\"webforge-input\" placeholder=\"{}\">",
        id_attr,
        escape_html(&placeholder)
    );
}

pub fn render_custom_3d_studio(html: &mut String, _dsl: &DslBlockValue, id: &str) {
    let _ = write!(
        html,
        "<div id=\"{id}-workbench\" class=\"webforge-3d-workbench\">\
            <div class=\"workbench-header\">\
                <div class=\"header-left\">\
                    <div class=\"workbench-brand\">\
                        <span class=\"pulse-dot\"></span>\
                        <span class=\"brand-title\">WebForge 3D Studio</span>\
                        <span class=\"brand-tag\">v2.0 CAD &amp; Procedural Engine</span>\
                    </div>\
                    <div class=\"template-selector-group\">\
                        <span class=\"selector-label\">Presets:</span>\
                        <button type=\"button\" class=\"template-btn active\" data-template=\"house\" onclick=\"WebForgeCustom3D.loadTemplate('house')\">🏠 House</button>\
                        <button type=\"button\" class=\"template-btn\" data-template=\"logo\" onclick=\"WebForgeCustom3D.loadTemplate('logo')\">⚡ Logo (SVG)</button>\
                        <button type=\"button\" class=\"template-btn\" data-template=\"mech\" onclick=\"WebForgeCustom3D.loadTemplate('mech')\">🤖 Mech Spire</button>\
                        <button type=\"button\" class=\"template-btn\" data-template=\"csg\" onclick=\"WebForgeCustom3D.loadTemplate('csg')\">🕳️ CSG Cutter</button>\
                        <button type=\"button\" class=\"template-btn\" data-template=\"blank\" onclick=\"WebForgeCustom3D.loadTemplate('blank')\">📄 Blank Canvas</button>\
                    </div>\
                </div>\
                <div class=\"header-actions\">\
                    <input type=\"file\" id=\"{id}-svg-input\" accept=\".svg\" style=\"display:none;\" onchange=\"WebForgeCustom3D.handleSvgUpload(event)\">\
                    <input type=\"file\" id=\"{id}-txs-input\" accept=\".txs,.txt\" style=\"display:none;\" onchange=\"WebForgeCustom3D.handleTxsUpload(event)\">\
                    <button type=\"button\" class=\"wb-action-btn\" onclick=\"document.getElementById('{id}-svg-input').click()\" title=\"Import 2D SVG Outline to Extrude &amp; Bevel\">📂 Import SVG</button>\
                    <button type=\"button\" class=\"wb-action-btn\" onclick=\"document.getElementById('{id}-txs-input').click()\" title=\"Open .txs source file\">📜 Open .txs</button>\
                    <div class=\"dropdown-export\" style=\"position:relative;display:inline-block;\">\
                        <button type=\"button\" class=\"wb-action-btn export-btn\" onclick=\"this.nextElementSibling.classList.toggle('show')\">💾 Export ▼</button>\
                        <div class=\"export-menu\">\
                            <button type=\"button\" onclick=\"this.parentElement.classList.remove('show');WebForgeCustom3D.exportGLTF()\">📦 Export GLB / GLTF</button>\
                            <button type=\"button\" onclick=\"this.parentElement.classList.remove('show');WebForgeCustom3D.exportOBJ()\">🧊 Export OBJ</button>\
                            <button type=\"button\" onclick=\"this.parentElement.classList.remove('show');WebForgeCustom3D.exportSTL()\">🖨️ Export STL</button>\
                            <button type=\"button\" onclick=\"this.parentElement.classList.remove('show');WebForgeCustom3D.exportTXS()\">📜 Export .txs Source</button>\
                        </div>\
                    </div>\
                    <button type=\"button\" class=\"wb-action-btn reset-btn\" onclick=\"WebForgeCustom3D.resetScene()\" title=\"Reset scene\">↺ Reset</button>\
                </div>\
            </div>\
            <div class=\"workbench-body\">\
                <div class=\"workbench-panel panel-left\">\
                    <div class=\"panel-section-title\"><span>Geometry Primitives</span></div>\
                    <div class=\"primitives-grid\">\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('box')\">📦 Box</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('cylinder')\">🥫 Cylinder</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('sphere')\">🌐 Sphere</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('cone')\">🔺 Cone</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('wall')\">🧱 Wall</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('roof')\">⛺ Roof</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('door')\">🚪 Door</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('window')\">🪟 Window</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('chimney')\">🏭 Chimney</button>\
                        <button type=\"button\" class=\"prim-btn\" onclick=\"WebForgeCustom3D.addPrimitive('text')\">✒️ 2D Outline</button>\
                    </div>\
                    <div class=\"panel-section-title\" style=\"margin-top:1.1rem;\"><span>Boolean CSG Operations</span></div>\
                    <div class=\"csg-actions-grid\">\
                        <button type=\"button\" class=\"csg-btn subtract\" onclick=\"WebForgeCustom3D.applyCSG('subtract')\" title=\"Subtract cutter volume from base mesh\">✂️ Boolean Subtract (Hollow/Cut)</button>\
                        <button type=\"button\" class=\"csg-btn union\" onclick=\"WebForgeCustom3D.applyCSG('union')\" title=\"Merge objects into single manifold mesh\">➕ Boolean Union (Merge)</button>\
                        <button type=\"button\" class=\"csg-btn intersect\" onclick=\"WebForgeCustom3D.applyCSG('intersect')\" title=\"Keep intersection volume\">✖️ Boolean Intersect</button>\
                    </div>\
                    <div class=\"panel-section-title\" style=\"margin-top:1.1rem;display:flex;justify-content:space-between;align-items:center;\">\
                        <span>Scene Hierarchy (<span id=\"scene-part-count\">0</span>)</span>\
                    </div>\
                    <div id=\"scene-hierarchy-tree\" class=\"scene-tree-list\"></div>\
                </div>\
                <div class=\"workbench-center\">\
                    <div class=\"viewport-toolbar\">\
                        <div class=\"vp-tools-left\">\
                            <span class=\"live-dot\"></span>\
                            <span class=\"vp-stat-item\"><span id=\"vp-fps\">60</span> FPS</span>\
                            <span class=\"vp-sep\">•</span>\
                            <span class=\"vp-stat-item\"><span id=\"vp-polys\">0</span> Polys</span>\
                            <span class=\"vp-sep\">•</span>\
                            <span class=\"vp-stat-item\"><span id=\"vp-verts\">0</span> Verts</span>\
                        </div>\
                        <div class=\"vp-tools-right\">\
                            <button type=\"button\" class=\"vp-tool-btn\" id=\"btn-wireframe\" onclick=\"WebForgeCustom3D.toggleWireframe()\" title=\"Toggle Wireframe\">📐 Wireframe</button>\
                            <button type=\"button\" class=\"vp-tool-btn\" id=\"btn-grid\" onclick=\"WebForgeCustom3D.toggleGrid()\" title=\"Toggle Grid &amp; Axes\">🧭 Grid</button>\
                            <button type=\"button\" class=\"vp-tool-btn\" id=\"btn-flat\" onclick=\"WebForgeCustom3D.toggleFlatShading()\" title=\"Toggle Flat/Smooth Shading\">💡 Flat Shading</button>\
                            <button type=\"button\" class=\"vp-tool-btn\" onclick=\"WebForgeCustom3D.resetCamera()\" title=\"Reset Camera View\">📸 Center</button>\
                        </div>\
                    </div>\
                    <div class=\"viewport-canvas-container\" id=\"{id}-viewport-container\">\
                        <canvas id=\"{id}-viewport-canvas\" class=\"workbench-canvas\"></canvas>\
                    </div>\
                    <div class=\"workbench-timeline-bar\">\
                        <div class=\"timeline-playback-controls\">\
                            <button type=\"button\" class=\"tl-btn\" onclick=\"WebForgeCustom3D.rewindAnimation()\" title=\"Rewind to 0s\">⏮</button>\
                            <button type=\"button\" class=\"tl-btn play-btn\" id=\"tl-play-pause-btn\" onclick=\"WebForgeCustom3D.togglePlayPause()\">▶ Play</button>\
                            <button type=\"button\" class=\"tl-btn\" id=\"tl-loop-btn\" onclick=\"WebForgeCustom3D.toggleLoop()\" title=\"Toggle Looping\">🔁 Loop</button>\
                            <div class=\"tl-speed-select\">\
                                <span class=\"tl-speed-label\">Speed:</span>\
                                <select id=\"tl-speed-selector\" onchange=\"WebForgeCustom3D.setAnimSpeed(this.value)\">\
                                    <option value=\"0.5\">0.5x</option>\
                                    <option value=\"1.0\" selected>1.0x</option>\
                                    <option value=\"1.5\">1.5x</option>\
                                    <option value=\"2.0\">2.0x</option>\
                                </select>\
                            </div>\
                            <div class=\"tl-anim-mode\">\
                                <span class=\"tl-speed-label\">Motion:</span>\
                                <select id=\"tl-mode-selector\" onchange=\"WebForgeCustom3D.setAnimMode(this.value)\">\
                                    <option value=\"turntable\" selected>Turntable 360°</option>\
                                    <option value=\"float\">Hover &amp; Float</option>\
                                    <option value=\"pulse\">Elastic Pulse</option>\
                                    <option value=\"walk\">Cyber Bounce</option>\
                                </select>\
                            </div>\
                        </div>\
                        <div class=\"timeline-scrubber-track\">\
                            <input type=\"range\" id=\"tl-scrubber\" min=\"0\" max=\"100\" value=\"0\" step=\"0.1\" class=\"timeline-scrubber-range\" oninput=\"WebForgeCustom3D.seekScrubber(this.value)\">\
                            <div class=\"timeline-timecode\"><span id=\"tl-timecode\">00:00.0</span> / 00:05.0</div>\
                        </div>\
                    </div>\
                </div>\
                <div class=\"workbench-panel panel-right\">\
                    <div class=\"panel-tabs\">\
                        <button type=\"button\" class=\"panel-tab-btn active\" id=\"tab-btn-inspector\" onclick=\"WebForgeCustom3D.switchRightTab('inspector')\">⚙️ Inspector</button>\
                        <button type=\"button\" class=\"panel-tab-btn\" id=\"tab-btn-code\" onclick=\"WebForgeCustom3D.switchRightTab('code')\">📜 TechScript Source</button>\
                    </div>\
                    <div id=\"right-tab-inspector\" class=\"right-tab-content active\">\
                        <div id=\"inspector-no-selection\" class=\"empty-selection-msg\" style=\"display:none;\">\
                            <p>Select an object from the hierarchy to view and modify its properties.</p>\
                        </div>\
                        <div id=\"inspector-content\">\
                            <div class=\"inspector-group\">\
                                <div class=\"group-title\">Object Identity</div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Name</label>\
                                    <input type=\"text\" id=\"prop-obj-name\" class=\"field-input\" onchange=\"WebForgeCustom3D.updateSelectedName(this.value)\">\
                                </div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Type</label>\
                                    <span id=\"prop-obj-type\" class=\"field-val-badge\">box</span>\
                                </div>\
                            </div>\
                            <div class=\"inspector-group\">\
                                <div class=\"group-title\">Transform</div>\
                                <div class=\"transform-3d-grid\">\
                                    <span class=\"axis-header\">Axis</span><span class=\"axis-header\">Position</span><span class=\"axis-header\">Rotation (°)</span><span class=\"axis-header\">Scale</span>\
                                    <span class=\"axis-tag axis-x\">X</span>\
                                    <input type=\"number\" id=\"prop-pos-x\" step=\"0.1\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('posX', this.value)\">\
                                    <input type=\"number\" id=\"prop-rot-x\" step=\"5\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('rotX', this.value)\">\
                                    <input type=\"number\" id=\"prop-scl-x\" step=\"0.1\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('sclX', this.value)\">\
                                    <span class=\"axis-tag axis-y\">Y</span>\
                                    <input type=\"number\" id=\"prop-pos-y\" step=\"0.1\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('posY', this.value)\">\
                                    <input type=\"number\" id=\"prop-rot-y\" step=\"5\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('rotY', this.value)\">\
                                    <input type=\"number\" id=\"prop-scl-y\" step=\"0.1\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('sclY', this.value)\">\
                                    <span class=\"axis-tag axis-z\">Z</span>\
                                    <input type=\"number\" id=\"prop-pos-z\" step=\"0.1\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('posZ', this.value)\">\
                                    <input type=\"number\" id=\"prop-rot-z\" step=\"5\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('rotZ', this.value)\">\
                                    <input type=\"number\" id=\"prop-scl-z\" step=\"0.1\" class=\"axis-input\" oninput=\"WebForgeCustom3D.updateTransform('sclZ', this.value)\">\
                                </div>\
                            </div>\
                            <div class=\"inspector-group\" id=\"extrude-bevel-group\" style=\"display:none;\">\
                                <div class=\"group-title\">Extrusion &amp; Bevel</div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Extrude Depth</label>\
                                    <input type=\"range\" id=\"prop-extrude-depth\" min=\"0.05\" max=\"3\" step=\"0.05\" value=\"0.4\" class=\"custom-range-slider\" oninput=\"WebForgeCustom3D.updateExtrude('depth', this.value)\">\
                                    <span id=\"prop-extrude-val\" class=\"slider-num\">0.40</span>\
                                </div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Bevel Thickness</label>\
                                    <input type=\"range\" id=\"prop-bevel-thick\" min=\"0\" max=\"0.3\" step=\"0.01\" value=\"0.05\" class=\"custom-range-slider\" oninput=\"WebForgeCustom3D.updateExtrude('bevelThick', this.value)\">\
                                    <span id=\"prop-bevel-val\" class=\"slider-num\">0.05</span>\
                                </div>\
                            </div>\
                            <div class=\"inspector-group\">\
                                <div class=\"group-title\">Material &amp; Appearance</div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Preset</label>\
                                    <select id=\"prop-mat-preset\" class=\"field-input\" onchange=\"WebForgeCustom3D.updateMaterialPreset(this.value)\">\
                                        <option value=\"metal\">Polished Metal</option>\
                                        <option value=\"stone\">Textured Stone</option>\
                                        <option value=\"glass\">Clear Glass</option>\
                                        <option value=\"wood\">Hardwood</option>\
                                        <option value=\"plastic\">Smooth Polymer</option>\
                                        <option value=\"glow\">Cyber Emissive</option>\
                                    </select>\
                                </div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Color</label>\
                                    <div class=\"color-picker-combo\">\
                                        <input type=\"color\" id=\"prop-mat-color-picker\" value=\"#06b6d4\" oninput=\"WebForgeCustom3D.updateColor(this.value)\">\
                                        <input type=\"text\" id=\"prop-mat-color-hex\" value=\"#06B6D4\" class=\"field-input hex-input\" onchange=\"WebForgeCustom3D.updateColor(this.value)\">\
                                    </div>\
                                </div>\
                                <div class=\"swatches-quick-row\">\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#06b6d4;\" onclick=\"WebForgeCustom3D.updateColor('#06b6d4')\"></button>\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#14b8a6;\" onclick=\"WebForgeCustom3D.updateColor('#14b8a6')\"></button>\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#9AEA39;\" onclick=\"WebForgeCustom3D.updateColor('#9AEA39')\"></button>\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#f59e0b;\" onclick=\"WebForgeCustom3D.updateColor('#f59e0b')\"></button>\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#dc2626;\" onclick=\"WebForgeCustom3D.updateColor('#dc2626')\"></button>\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#a855f7;\" onclick=\"WebForgeCustom3D.updateColor('#a855f7')\"></button>\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#f8fafc;\" onclick=\"WebForgeCustom3D.updateColor('#f8fafc')\"></button>\
                                    <button type=\"button\" class=\"swatch-mini\" style=\"background:#1e293b;\" onclick=\"WebForgeCustom3D.updateColor('#1e293b')\"></button>\
                                </div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Roughness</label>\
                                    <input type=\"range\" id=\"prop-mat-roughness\" min=\"0\" max=\"1\" step=\"0.05\" value=\"0.3\" class=\"custom-range-slider\" oninput=\"WebForgeCustom3D.updateMaterialProp('roughness', this.value)\">\
                                    <span id=\"prop-rough-val\" class=\"slider-num\">0.30</span>\
                                </div>\
                                <div class=\"field-row\">\
                                    <label class=\"field-label\">Metalness</label>\
                                    <input type=\"range\" id=\"prop-mat-metalness\" min=\"0\" max=\"1\" step=\"0.05\" value=\"0.8\" class=\"custom-range-slider\" oninput=\"WebForgeCustom3D.updateMaterialProp('metalness', this.value)\">\
                                    <span id=\"prop-metal-val\" class=\"slider-num\">0.80</span>\
                                </div>\
                            </div>\
                        </div>\
                    </div>\
                    <div id=\"right-tab-code\" class=\"right-tab-content\">\
                        <div class=\"code-editor-header\">\
                            <span class=\"file-tag\">model.txs</span>\
                            <div class=\"code-actions\">\
                                <button type=\"button\" class=\"mini-btn\" onclick=\"WebForgeCustom3D.copyCode()\">📋 Copy</button>\
                                <button type=\"button\" class=\"mini-btn compile-btn\" onclick=\"WebForgeCustom3D.compileFromEditor()\">▶ Compile to 3D</button>\
                            </div>\
                        </div>\
                        <textarea id=\"live-txs-editor\" class=\"txs-code-textarea\" spellcheck=\"false\"></textarea>\
                        <div class=\"code-status-bar\" id=\"code-status-bar\">\
                            <span class=\"status-indicator-dot\"></span>\
                            <span id=\"code-status-msg\">TechScript AST In Sync</span>\
                        </div>\
                    </div>\
                </div>\
            </div>\
        </div>"
    );
}

pub fn render_scene3d(html: &mut String, dsl: &DslBlockValue) {
    let raw_scene_id = dsl
        .args
        .first()
        .map(|a| a.to_string())
        .or_else(|| {
            dsl.properties
                .iter()
                .find(|p| p.name == "id")
                .and_then(|p| p.value.as_ref().map(|v| v.to_string()))
        })
        .unwrap_or_else(|| "World".to_string());
    let id = escape_html(&raw_scene_id);

    let is_custom_studio = raw_scene_id.to_lowercase().contains("custom")
        || raw_scene_id.to_lowercase().contains("playground")
        || dsl.properties.iter().any(|p| {
            p.name == "mode"
                && p.value.as_ref().map(|v| v.to_string()).unwrap_or_default() == "custom"
        })
        || dsl.children.iter().any(|c| {
            matches!(
                c.kind.as_str(),
                "box"
                    | "wall"
                    | "roof"
                    | "door"
                    | "window"
                    | "chimney"
                    | "extrude"
                    | "outline"
                    | "cut"
                    | "boolean"
                    | "part"
            )
        });

    if is_custom_studio {
        render_custom_3d_studio(html, dsl, &id);
        return;
    }

    let _ = write!(
        html,
        "<div id=\"{id}-studio\" class=\"webforge-3d-studio\">\
            <div id=\"{id}\" class=\"webforge-3d-scene\">\
                <div class=\"webforge-3d-topbar\">\
                    <div class=\"topbar-left\">\
                        <span class=\"live-status-dot\" style=\"display:inline-block;width:8px;height:8px;border-radius:50%;background:#10b981;box-shadow:0 0 10px #10b981;\"></span>\
                        <span class=\"topbar-title\">WebForge 3D</span>\
                        <span style=\"color:rgba(255,255,255,0.25);font-size:0.75rem;\">•</span>\
                        <span style=\"font-size:0.75rem;color:#cbd5e1;font-weight:600;\"><span id=\"{id}-fps\" class=\"fps-val\">60</span> FPS</span>\
                        <span style=\"color:rgba(255,255,255,0.25);font-size:0.75rem;\">•</span>\
                        <span id=\"{id}-mesh-info\" class=\"topbar-badge\">Sci-Fi Core • 1,680 Polys</span>\
                    </div>\
                    <div class=\"topbar-right\">\
                        <button type=\"button\" class=\"topbar-btn\" data-scene=\"{id}\" data-toggle=\"wireframe\" title=\"Toggle Wireframe Mode\">📐 Wireframe</button>\
                        <button type=\"button\" class=\"topbar-btn\" data-scene=\"{id}\" data-toggle=\"pause\" title=\"Pause / Resume Rotation\">⏸ Pause</button>\
                        <button type=\"button\" class=\"topbar-btn\" data-scene=\"{id}\" data-toggle=\"reset\" title=\"Reset Camera View\">↺ Reset</button>\
                    </div>\
                </div>\
                <canvas id=\"{id}-canvas\" class=\"webforge-3d-canvas\"></canvas>\
                <div class=\"webforge-3d-dock\">\
                    <div class=\"dock-header\">\
                        <span class=\"dock-title\">Procedural 3D Models (16)</span>\
                        <span class=\"dock-hint\">Drag to orbit • Scroll to zoom</span>\
                    </div>\
                    <div class=\"dock-models-grid\">\
                        <button type=\"button\" class=\"scene-model-btn active\" data-scene=\"{id}\" data-geo=\"scifi\">🛸 Sci-Fi Core</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"mech\">⚙️ Mech Spire</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"quantum_chip\">⚡ Quantum Chip</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"black_hole\">🕳️ Singularity</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"gyroscope\">🧭 Gyroscope</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"tesseract\">🧊 4D Tesseract</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"dna\">🧬 DNA Helix</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"satellite\">🛰️ Satellite</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"torus\">🍩 Ripple Torus</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"star\">⭐ Stellated Star</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"mobius\">♾️ Möbius Strip</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"dyson_swarm\">🛰️ Dyson Ring</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"crystal\">💎 Prism Gem</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"geodesic\">🌐 Geodesic</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"knot\">🌀 Trefoil Knot</button>\
                        <button type=\"button\" class=\"scene-model-btn\" data-scene=\"{id}\" data-geo=\"product\">📱 Monolith</button>\
                    </div>\
                </div>\
            </div>\
            <div class=\"webforge-3d-sidepanel\">\
                <div class=\"sidepanel-card sidepanel-code\">\
                    <div class=\"code-window-bar\">\
                        <div class=\"window-dots\">\
                            <span class=\"dot dot-red\"></span>\
                            <span class=\"dot dot-yellow\"></span>\
                            <span class=\"dot dot-green\"></span>\
                        </div>\
                        <span class=\"window-tab\">model.txs</span>\
                        <button type=\"button\" class=\"copy-code-btn\" id=\"{id}-copy-btn\" data-scene=\"{id}\" title=\"Copy TechScript Code\">\
                            <span class=\"btn-copy-text\">📋 Copy Code</span>\
                        </button>\
                    </div>\
                    <pre class=\"code-editor-pre\"><code id=\"{id}-live-code\" class=\"code-editor-content\"></code></pre>\
                    <div class=\"code-note\">\
                        <div class=\"code-note-title\">🚀 Apply to your TechScript website:</div>\
                        <div class=\"code-note-desc\">Paste this block directly into any <code>.txs</code> page (e.g. <code>src/pages/home.txs</code>). WebForge compiles and hardware-accelerates it instantly with zero config.</div>\
                    </div>\
                </div>\
                <div class=\"sidepanel-card sidepanel-controls\">\
                    <div class=\"controls-section-title\">Color &amp; Palette</div>\
                    <div class=\"custom-color-control-group\">\
                        <div class=\"color-picker-wrapper\">\
                            <input type=\"color\" id=\"{id}-custom-picker\" data-scene=\"{id}\" value=\"#06b6d4\" class=\"custom-color-native\" title=\"Click to choose ANY custom color\">\
                            <div class=\"color-picker-label\">\
                                <span class=\"color-swatch-preview\" id=\"{id}-swatch-preview\" style=\"background:#06b6d4;\"></span>\
                                <span class=\"color-picker-text\">Custom Color</span>\
                            </div>\
                        </div>\
                        <div class=\"hex-input-wrapper\">\
                            <span class=\"hex-prefix\">HEX</span>\
                            <input type=\"text\" id=\"{id}-hex-input\" data-scene=\"{id}\" value=\"#06B6D4\" class=\"hex-text-input\" spellcheck=\"false\" maxlength=\"7\">\
                        </div>\
                    </div>\
                    <div class=\"palette-swatches-grid\">\
                        <button type=\"button\" class=\"color-swatch active\" data-scene=\"{id}\" data-color=\"cyan\" data-hex=\"#06b6d4\" style=\"background:#06b6d4;\" title=\"Cyber Cyan\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"teal\" data-hex=\"#14b8a6\" style=\"background:#14b8a6;\" title=\"Neon Teal\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"purple\" data-hex=\"#a855f7\" style=\"background:#a855f7;\" title=\"Quantum Violet\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"crimson\" data-hex=\"#f43f5e\" style=\"background:#f43f5e;\" title=\"Crimson Core\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"gold\" data-hex=\"#f59e0b\" style=\"background:#f59e0b;\" title=\"Solar Gold\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"emerald\" data-hex=\"#10b981\" style=\"background:#10b981;\" title=\"Matrix Emerald\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"ice\" data-hex=\"#38bdf8\" style=\"background:#38bdf8;\" title=\"Ice Blue\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"silver\" data-hex=\"#e2e8f0\" style=\"background:#e2e8f0;\" title=\"Titanium Silver\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"amber\" data-hex=\"#fbbf24\" style=\"background:#fbbf24;\" title=\"Amber Glow\"></button>\
                        <button type=\"button\" class=\"color-swatch\" data-scene=\"{id}\" data-color=\"pink\" data-hex=\"#ec4899\" style=\"background:#ec4899;\" title=\"Galactic Pink\"></button>\
                    </div>\
                    <div class=\"controls-section-title\" style=\"margin-top:1.1rem;\">GLSL Shading Style</div>\
                    <div class=\"shading-pills-row\">\
                        <button type=\"button\" class=\"shading-pill active\" data-scene=\"{id}\" data-shading=\"0\">✨ Specular PBR</button>\
                        <button type=\"button\" class=\"shading-pill\" data-scene=\"{id}\" data-shading=\"1\">⚡ Cyber Rim</button>\
                        <button type=\"button\" class=\"shading-pill\" data-scene=\"{id}\" data-shading=\"2\">🔮 Normal Map</button>\
                        <button type=\"button\" class=\"shading-pill\" data-scene=\"{id}\" data-shading=\"3\">🌈 Dual Gradient</button>\
                        <button type=\"button\" class=\"shading-pill\" data-scene=\"{id}\" data-shading=\"4\">🛸 Scanlines</button>\
                    </div>\
                    <div class=\"sliders-container\" style=\"margin-top:1.1rem;\">\
                        <div class=\"studio-slider-row\">\
                            <div class=\"slider-meta\">\
                                <span class=\"slider-title\">Rotation Speed</span>\
                                <span id=\"{id}-speed-num\" class=\"slider-val\">1.2x</span>\
                            </div>\
                            <input type=\"range\" id=\"{id}-speed-range\" data-scene=\"{id}\" min=\"0\" max=\"40\" value=\"12\" class=\"custom-range-slider\">\
                        </div>\
                        <div class=\"studio-slider-row\">\
                            <div class=\"slider-meta\">\
                                <span class=\"slider-title\">Camera Distance</span>\
                                <span id=\"{id}-zoom-num\" class=\"slider-val\">5.4</span>\
                            </div>\
                            <input type=\"range\" id=\"{id}-zoom-range\" data-scene=\"{id}\" min=\"25\" max=\"85\" value=\"54\" class=\"custom-range-slider\">\
                        </div>\
                    </div>\
                </div>\
            </div>\
        </div>"
    );
}

pub fn render_link(html: &mut String, dsl: &DslBlockValue) {
    let mut label_val = dsl.args.first().map(|a| a.to_string());
    let mut url_val = dsl.args.get(1).map(|a| a.to_string());
    for p in &dsl.properties {
        match p.name.as_str() {
            "label" | "text" | "title" => {
                if label_val.is_none() {
                    label_val = p.value.as_ref().map(|v| v.to_string());
                }
            }
            "url" | "href" | "target" => {
                if url_val.is_none() {
                    url_val = p.value.as_ref().map(|v| v.to_string());
                }
            }
            _ => {}
        }
    }
    let label = label_val.unwrap_or_else(|| "Visit Link".to_string());
    if let Some(u) = url_val {
        let clean = u.trim();
        if clean.starts_with('/') || clean.starts_with('#') {
            let clean_route = if clean.starts_with('#') {
                let tag = clean.trim_start_matches('#');
                if tag == "home" {
                    "/".to_string()
                } else {
                    format!("/{}", tag)
                }
            } else {
                clean.to_string()
            };
            let _ = write!(
                html,
                "<a href=\"{}\" data-route=\"{}\" class=\"webforge-link\" onclick=\"event.preventDefault();webforgeRouter.navigate('{}')\"><span>{}</span></a>",
                escape_html(clean),
                escape_html(&clean_route),
                escape_html(&clean_route),
                escape_html(&label)
            );
        } else {
            let _ = write!(
                html,
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" class=\"webforge-link external-link\" title=\"Opens {}\"><span>{}</span><svg width=\"13\" height=\"13\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.5\"><path d=\"M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6M15 3h6v6M10 14L21 3\"/></svg></a>",
                escape_html(clean),
                escape_html(clean),
                escape_html(&label)
            );
        }
    } else {
        let _ = write!(
            html,
            "<a href=\"#\" class=\"webforge-link\">{}</a>",
            escape_html(&label)
        );
    }
}

pub fn render_nav(html: &mut String, dsl: &DslBlockValue) {
    html.push_str("<nav class=\"webforge-nav\" id=\"main-nav\">");
    for child in &dsl.children {
        if child.kind == "link" {
            let mut label = child.args.first().map(|a| a.to_string());
            let mut url = child.args.get(1).map(|a| a.to_string());
            for p in &child.properties {
                if p.name == "label" || p.name == "text" {
                    if label.is_none() {
                        label = p.value.as_ref().map(|v| v.to_string());
                    }
                } else if p.name == "url" || p.name == "href" {
                    if url.is_none() {
                        url = p.value.as_ref().map(|v| v.to_string());
                    }
                }
            }
            let l = label.unwrap_or_else(|| "Link".to_string());
            let u = url.unwrap_or_else(|| "#".to_string());
            if u.starts_with('/') || u.starts_with('#') {
                let _ = write!(
                    html,
                    "<a href=\"{}\" data-route=\"{}\">{}</a>",
                    escape_html(&u),
                    escape_html(&u),
                    escape_html(&l)
                );
            } else {
                let _ = write!(
                    html,
                    "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                    escape_html(&u),
                    escape_html(&l)
                );
            }
        } else {
            dsl_to_html_inner(
                &RuntimeValue::DslBlock(std::rc::Rc::new(child.clone())),
                html,
            );
        }
    }
    html.push_str("</nav>");
}

pub fn render_header_block(html: &mut String, dsl: &DslBlockValue) {
    let mut title = "TechScript WebForge".to_string();
    for p in &dsl.properties {
        if p.name == "title" {
            if let Some(ref v) = p.value {
                title = v.to_string();
            }
        }
    }

    html.push_str("<header class=\"webforge-header\">");
    let _ = write!(
        html,
        "<a href=\"/\" class=\"brand-logo\" onclick=\"event.preventDefault();webforgeRouter.navigate('/')\" title=\"{}\">\
            <img src=\"/assets/logo.png\" alt=\"Logo\" class=\"brand-logo-img\" onerror=\"this.style.display='none';if(this.nextElementSibling)this.nextElementSibling.style.display='flex'\">\
            <div class=\"brand-logo-icon\" style=\"display:none;\">TX</div>\
            <div class=\"brand-logo-text\">{}</div>\
            <span class=\"brand-badge\">WebForge</span>\
        </a>",
        escape_html(&title),
        escape_html(&title)
    );

    let canonical_nav = "";
    let _ = canonical_nav; // used below

    let declared_links: Vec<(String, String)> = {
        let mut v = Vec::new();
        for child in &dsl.children {
            if child.kind == "nav" {
                for link in &child.children {
                    if link.kind == "link" {
                        let mut label = link.args.first().map(|a| a.to_string());
                        let mut url = link.args.get(1).map(|a| a.to_string());
                        for p in &link.properties {
                            if p.name == "label" || p.name == "text" {
                                if label.is_none() {
                                    label = p.value.as_ref().map(|v| v.to_string());
                                }
                            } else if p.name == "url" || p.name == "href" {
                                if url.is_none() {
                                    url = p.value.as_ref().map(|v| v.to_string());
                                }
                            }
                        }
                        if let (Some(l), Some(u)) = (label, url) {
                            v.push((l, u));
                        }
                    }
                }
            }
        }
        v
    };

    // Build single-row compact nav: primary links + grouped dropdown for secondary
    html.push_str("<nav class=\"webforge-nav\" id=\"main-nav\">");
    let primary: &[(&str, &str)] = &[
        ("Home", "/"),
        ("Language", "/language"),
        ("Framework", "/framework"),
        ("Features", "/features"),
        ("Components", "/components"),
        ("3D Lab", "/showcase/3d"),
        ("Playground", "/playground"),
        ("Docs", "/docs"),
        ("Examples", "/examples"),
    ];
    if declared_links.len() > 7 {
        for (label, url) in &declared_links[..6] {
            if url.starts_with('/') {
                let _ = write!(
                    html,
                    "<a href=\"{}\" data-route=\"{}\">{}</a>",
                    escape_html(url),
                    escape_html(url),
                    escape_html(label)
                );
            } else {
                let _ = write!(html, "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" class=\"external-link\">{}</a>", escape_html(url), escape_html(label));
            }
        }
        html.push_str("<div class=\"nav-sep\"></div>");
        html.push_str("<div class=\"nav-group\">");
        html.push_str("<span class=\"nav-group-trigger\">More <svg width=\"12\" height=\"12\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.5\"><polyline points=\"6 9 12 15 18 9\"/></svg></span>");
        html.push_str("<div class=\"nav-dropdown\">");
        for (label, url) in &declared_links[6..] {
            let emoji = match label.as_str() {
                "Showcase" => "🗺️",
                "3D Lab" => "🛸",
                "Playground" => "⌨️",
                "Packages" => "📦",
                "Roadmap" => "🛤️",
                "Community" => "👥",
                "Download" => "⬇️",
                "Docs" => "📖",
                "Examples" => "💡",
                "GitHub" => "⭐",
                _ => "•",
            };
            if url.starts_with('/') {
                let _ = write!(
                    html,
                    "<a href=\"{}\" data-route=\"{}\">{} {}</a>",
                    escape_html(url),
                    escape_html(url),
                    emoji,
                    escape_html(label)
                );
            } else {
                let _ = write!(
                    html,
                    "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {} ↗</a>",
                    escape_html(url),
                    emoji,
                    escape_html(label)
                );
            }
        }
        html.push_str("</div></div>");
    } else if !declared_links.is_empty() {
        for (label, url) in &declared_links {
            if url.starts_with('/') {
                let _ = write!(
                    html,
                    "<a href=\"{}\" data-route=\"{}\">{}</a>",
                    escape_html(url),
                    escape_html(url),
                    escape_html(label)
                );
            } else {
                let _ = write!(html, "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" class=\"external-link\">{}</a>", escape_html(url), escape_html(label));
            }
        }
    } else {
        for (label, url) in primary {
            let _ = write!(
                html,
                "<a href=\"{}\" data-route=\"{}\">{}</a>",
                url, url, label
            );
        }
        html.push_str("<div class=\"nav-sep\"></div>");
        // Grouped 'More' dropdown
        html.push_str("<div class=\"nav-group\">");
        html.push_str("<span class=\"nav-group-trigger\">More <svg width=\"12\" height=\"12\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.5\"><polyline points=\"6 9 12 15 18 9\"/></svg></span>");
        html.push_str("<div class=\"nav-dropdown\">");
        let more: &[(&str, &str, &str)] = &[
            ("🗺️", "Showcase", "/showcase"),
            ("📦", "Packages", "/packages"),
            ("🛤️", "Roadmap", "/roadmap"),
            ("👥", "Community", "/community"),
            ("⬇️", "Download", "/download"),
            ("⌨️", "Playground", "/playground"),
            ("🔧", "CLI Reference", "/cli"),
        ];
        for (emoji, label, url) in more {
            let _ = write!(
                html,
                "<a href=\"{}\" data-route=\"{}\">{} {}</a>",
                url, url, emoji, label
            );
        }
        html.push_str("<a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\">⭐ GitHub</a>");
        html.push_str("</div></div>");
    }
    html.push_str("</nav>");

    html.push_str("<div class=\"header-actions\">");
    html.push_str("<a href=\"/playground\" class=\"header-action-link\" onclick=\"event.preventDefault();webforgeRouter.navigate('/playground')\" title=\"Interactive Sandbox\"><span>Playground</span></a>");
    html.push_str("<a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\" class=\"header-action-github\" title=\"GitHub Repository\">⭐ GitHub</a>");
    html.push_str("<a href=\"/docs/getting-started\" class=\"webforge-btn\" style=\"padding:0.45rem 1.1rem;font-size:0.85rem;text-decoration:none;\">Get Started</a>");
    html.push_str("<button class=\"mobile-toggle\" onclick=\"document.getElementById('mobile-drawer').classList.toggle('open')\">☰</button>");
    html.push_str("</div>");
    html.push_str("</header>");

    // Mobile drawer
    html.push_str("<div id=\"mobile-drawer\" style=\"display:none;flex-direction:column;gap:0.5rem;padding:1.5rem;background:var(--surface);border-bottom:1px solid var(--border);\">");

    let all_mobile: &[(&str, &str)] = &[
        ("Home", "/"),
        ("Language", "/language"),
        ("Framework", "/framework"),
        ("Features", "/features"),
        ("Components", "/components"),
        ("3D Lab", "/showcase/3d"),
        ("Docs", "/docs"),
        ("Examples", "/examples"),
        ("Showcase", "/showcase"),
        ("Packages", "/packages"),
        ("Roadmap", "/roadmap"),
        ("Community", "/community"),
        ("Download", "/download"),
        ("Playground", "/playground"),
        ("CLI", "/cli"),
    ];
    for (label, url) in all_mobile {
        let _ = write!(
            html,
            "<a href=\"{}\" onclick=\"event.preventDefault();webforgeRouter.navigate('{}');document.getElementById('mobile-drawer').classList.remove('open')\" style=\"color:#fff;text-decoration:none;padding:0.5rem 0;border-bottom:1px solid rgba(255,255,255,0.05);\">{}</a>",
            url, url, label
        );
    }
    html.push_str("<a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\" style=\"color:var(--primary);text-decoration:none;padding:0.5rem 0;\">⭐ GitHub ↗</a>");
    html.push_str("</div>");
}

pub fn render_footer_block(html: &mut String, dsl: &DslBlockValue) {
    html.push_str("<footer class=\"webforge-footer\"><div class=\"footer-inner\">");

    // Brand column
    html.push_str("\
<div class=\"footer-brand footer-col\">\
    <a href=\"/\" onclick=\"event.preventDefault();webforgeRouter.navigate('/')\" style=\"display:inline-flex;align-items:center;gap:0.75rem;text-decoration:none;margin-bottom:0.25rem;\">\
        <div style=\"width:34px;height:34px;border-radius:8px;background:linear-gradient(135deg,#06B6D4,#14B8A6);display:flex;align-items:center;justify-content:center;font-weight:800;font-size:1rem;color:#fff;box-shadow:0 0 16px rgba(6,182,212,0.4);\">TX</div>\
        <span style=\"font-size:1.2rem;font-weight:800;color:#fff;\">TechScript WebForge</span>\
    </a>\
    <p>The modern declarative programming ecosystem for human clarity and native performance. Write pure <code style=\"background:rgba(6,182,212,0.1);color:#22d3ee;padding:0.1rem 0.35rem;border-radius:4px;font-size:0.85rem;\">.txs</code> files that compile to high-performance applications with embedded SQLite and zero-dependency WebGL 3D.</p>\
    <div style=\"display:flex;gap:0.75rem;flex-wrap:wrap;margin-top:1rem;\">\
        <a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\" style=\"display:inline-flex;align-items:center;gap:0.4rem;color:var(--text-muted);text-decoration:none;font-size:0.85rem;padding:0.4rem 0.85rem;border:1px solid var(--border);border-radius:8px;transition:all 0.2s;\">⭐ GitHub</a>\
        <a href=\"/download\" onclick=\"event.preventDefault();webforgeRouter.navigate('/download')\" style=\"display:inline-flex;align-items:center;gap:0.4rem;color:var(--primary);text-decoration:none;font-size:0.85rem;padding:0.4rem 0.85rem;border:1px solid rgba(6,182,212,0.3);border-radius:8px;background:rgba(6,182,212,0.08);transition:all 0.2s;\">⬇ Download</a>\
    </div>\
</div>");

    // Process declared sections
    let mut section_count = 0;
    for child in &dsl.children {
        if child.kind == "section" {
            section_count += 1;
            let mut title = "Section".to_string();
            for p in &child.properties {
                if p.name == "title" {
                    if let Some(ref v) = p.value {
                        title = v.to_string();
                    }
                }
            }
            let _ = write!(
                html,
                "<div class=\"footer-col\"><h4>{}</h4>",
                escape_html(&title)
            );
            for link in &child.children {
                if link.kind == "link" {
                    let mut label = link.args.first().map(|a| a.to_string());
                    let mut url = link.args.get(1).map(|a| a.to_string());
                    for p in &link.properties {
                        if p.name == "label" || p.name == "text" {
                            if label.is_none() {
                                label = p.value.as_ref().map(|v| v.to_string());
                            }
                        } else if p.name == "url" || p.name == "href" {
                            if url.is_none() {
                                url = p.value.as_ref().map(|v| v.to_string());
                            }
                        }
                    }
                    let l = label.unwrap_or_else(|| "Link".to_string());
                    let u = url.unwrap_or_else(|| "#".to_string());
                    if u.starts_with('/') || u.starts_with('#') {
                        let _ = write!(
                            html,
                            "<a href=\"{}\" onclick=\"event.preventDefault();webforgeRouter.navigate('{}')\">{}</a>",
                            escape_html(&u), escape_html(&u), escape_html(&l)
                        );
                    } else {
                        let _ = write!(
                            html,
                            "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                            escape_html(&u),
                            escape_html(&l)
                        );
                    }
                }
            }
            html.push_str("</div>");
        }
    }

    if section_count == 0 {
        // Fallback standard columns
        html.push_str("\
<div class=\"footer-col\">\
    <h4>Framework</h4>\
    <a href=\"/language\" onclick=\"event.preventDefault();webforgeRouter.navigate('/language')\">Language Spec</a>\
    <a href=\"/framework\" onclick=\"event.preventDefault();webforgeRouter.navigate('/framework')\">Architecture</a>\
    <a href=\"/features\" onclick=\"event.preventDefault();webforgeRouter.navigate('/features')\">Features</a>\
    <a href=\"/showcase/3d\" onclick=\"event.preventDefault();webforgeRouter.navigate('/showcase/3d')\">3D Laboratory</a>\
    <a href=\"/playground\" onclick=\"event.preventDefault();webforgeRouter.navigate('/playground')\">Playground</a>\
</div>\
<div class=\"footer-col\">\
    <h4>Documentation</h4>\
    <a href=\"/docs\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs')\">Documentation</a>\
    <a href=\"/docs/getting-started\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs/getting-started')\">Getting Started</a>\
    <a href=\"/docs/syntax\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs/syntax')\">Syntax Guide</a>\
    <a href=\"/docs/api\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs/api')\">API Reference</a>\
    <a href=\"/examples/fullstack\" onclick=\"event.preventDefault();webforgeRouter.navigate('/examples/fullstack')\">Fullstack Demo</a>\
</div>\
<div class=\"footer-col\">\
    <h4>Community</h4>\
    <a href=\"/roadmap\" onclick=\"event.preventDefault();webforgeRouter.navigate('/roadmap')\">Roadmap</a>\
    <a href=\"/community\" onclick=\"event.preventDefault();webforgeRouter.navigate('/community')\">Community</a>\
    <a href=\"/download\" onclick=\"event.preventDefault();webforgeRouter.navigate('/download')\">Download</a>\
    <a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\">GitHub ↗</a>\
    <a href=\"https://github.com/Tcode-Motion/techscript/issues\" target=\"_blank\" rel=\"noopener noreferrer\">Issues ↗</a>\
</div>");
    }

    html.push_str("</div>"); // .footer-inner

    // Footer bottom
    html.push_str("\
<div class=\"footer-bottom\">\
    <div style=\"display:flex;align-items:center;gap:1.25rem;flex-wrap:wrap;\">\
        <span>© 2026 TechScript Ecosystem. Open source under MIT License.</span>\
        <a href=\"/roadmap\" onclick=\"event.preventDefault();webforgeRouter.navigate('/roadmap')\">Roadmap</a>\
        <a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\">GitHub Repo</a>\
        <a href=\"https://github.com/Tcode-Motion/techscript/blob/main/LICENSE\" target=\"_blank\" rel=\"noopener noreferrer\">License</a>\
    </div>\
    <div class=\"footer-status\">\
        <span class=\"footer-status-dot\"></span>\
        <span>All systems operational</span>\
        <span style=\"color:rgba(255,255,255,0.15);\">|</span>\
        <span>Native WebGL 2.0 • SQLite</span>\
        <span style=\"color:rgba(255,255,255,0.15);\">|</span>\
        <span>Crafted with <a href=\"/\" onclick=\"event.preventDefault();webforgeRouter.navigate('/')\" style=\"color:var(--primary);font-weight:600;\">WebForge v2.0</a></span>\
    </div>\
</div>");
    html.push_str("</footer>");
}

pub fn dsl_to_html_inner(val: &RuntimeValue, html: &mut String) {
    if let RuntimeValue::DslBlock(dsl) = val {
        match dsl.kind.as_str() {
            "page" => render_page(html, dsl),
            "hero" => render_hero(html, dsl),
            "section" => render_section(html, dsl),
            "card" => render_card(html, dsl),
            "heading" => render_heading(html, dsl),
            "text" => render_text(html, dsl),
            "button" => render_button(html, dsl),
            "input" => render_input(html, dsl),
            "timeline" => render_timeline(html, dsl),
            "accordion" => render_accordion(html, dsl),
            "tabs" => render_tabs(html, dsl),
            "stats" => render_stats(html, dsl),
            "alert" | "callout" | "banner" => render_alert(html, dsl),
            "scene" | "scene3d" | "model3d" | "model3D" => render_scene3d(html, dsl),
            "link" => render_link(html, dsl),
            "nav" => render_nav(html, dsl),
            "header" => render_header_block(html, dsl),
            "footer" => render_footer_block(html, dsl),
            "component" => {
                html.push_str("<div class=\"webforge-component card\">");
                render_children(html, dsl);
                html.push_str("</div>");
            }
            "website" => {
                render_children(html, dsl);
            }
            "database" | "table" | "cube" => {}
            _ => {
                render_children(html, dsl);
            }
        }
    }
}

pub fn find_3d_scenes(blocks: &[RuntimeValue]) -> Vec<DslBlockValue> {
    let mut scenes = Vec::new();
    fn walk(dsl: &DslBlockValue, out: &mut Vec<DslBlockValue>) {
        if dsl.kind == "scene"
            || dsl.kind == "scene3d"
            || dsl.kind == "model3d"
            || dsl.kind == "model3D"
        {
            out.push(dsl.clone());
        }
        for child in &dsl.children {
            walk(child, out);
        }
    }
    for b in blocks {
        if let RuntimeValue::DslBlock(dsl) = b {
            walk(dsl, &mut scenes);
        }
    }
    scenes
}

pub fn find_database_tables(blocks: &[RuntimeValue]) -> Vec<String> {
    let mut tables = Vec::new();
    for b in blocks {
        if let RuntimeValue::DslBlock(dsl) = b {
            if dsl.kind == "database" {
                for child in &dsl.children {
                    if child.kind == "table" {
                        let name = child
                            .args
                            .first()
                            .map(|a| a.to_string())
                            .or_else(|| {
                                child
                                    .properties
                                    .iter()
                                    .find(|p| p.name == "name" || p.name == "id")
                                    .and_then(|p| p.value.as_ref().map(|v| v.to_string()))
                            })
                            .unwrap_or_else(|| "records".to_string());
                        let clean = db::sanitize_identifier(&name);
                        if !tables.contains(&clean) {
                            tables.push(clean);
                        }
                    }
                }
            }
        }
    }
    tables
}

/// Renders the complete, self-contained HTML5 document for a TechScript WebForge application.
pub fn render_complete_document(
    blocks: &[RuntimeValue],
    event_handlers: &[RuntimeValue],
) -> String {
    let mut doc_title = "TechScript WebForge Application".to_string();
    let mut header_block: Option<DslBlockValue> = None;
    let mut footer_block: Option<DslBlockValue> = None;

    // Scan for declared website title, header, and footer blocks
    for b in blocks {
        if let RuntimeValue::DslBlock(dsl) = b {
            if dsl.kind == "website" {
                if let Some(t) = dsl.args.first() {
                    doc_title = t.to_string();
                }
                for p in &dsl.properties {
                    if p.name == "title" {
                        if let Some(ref v) = p.value {
                            doc_title = v.to_string();
                        }
                    }
                }
            } else if dsl.kind == "header" && header_block.is_none() {
                header_block = Some((**dsl).clone());
            } else if dsl.kind == "footer" && footer_block.is_none() {
                footer_block = Some((**dsl).clone());
            }
        }
    }

    let scenes_3d = find_3d_scenes(blocks);
    let tables = find_database_tables(blocks);

    let has_pages = blocks.iter().any(|b| {
        if let RuntimeValue::DslBlock(dsl) = b {
            dsl.kind == "page"
                || (dsl.kind == "website" && dsl.children.iter().any(|c| c.kind == "page"))
        } else {
            false
        }
    });

    let mut body_html = String::new();
    for b in blocks {
        if let RuntimeValue::DslBlock(dsl) = b {
            if dsl.kind == "database" || dsl.kind == "header" || dsl.kind == "footer" {
                continue;
            }
            if has_pages && (dsl.kind == "scene" || dsl.kind == "scene3d") {
                // Standalone 3D scene must not leak into the global body outside pages when pages exist
                continue;
            }
            if dsl.kind == "website" {
                for child in &dsl.children {
                    if child.kind != "header" && child.kind != "footer" {
                        dsl_to_html_inner(
                            &RuntimeValue::DslBlock(std::rc::Rc::new(child.clone())),
                            &mut body_html,
                        );
                    }
                }
                continue;
            }
        }
        dsl_to_html_inner(b, &mut body_html);
    }

    // Append database table record containers
    for t in &tables {
        let sec = format!(
            "<div class=\"webforge-data-section\" style=\"margin-top:2.5rem;border-top:1px solid rgba(255,255,255,0.08);padding-top:2rem;\">\
                <h3 style=\"color:var(--primary);font-size:1.4rem;margin-bottom:1rem;font-weight:700;\">Database Records ({})</h3>\
                <div id=\"{}-container\" class=\"grid-layout\"></div>\
            </div>",
            escape_html(t),
            escape_html(t)
        );
        if let Some(pos) = body_html.find("id=\"page-examples-fullstack\"") {
            if let Some(end_div) = body_html[pos..].find("</div>") {
                body_html.insert_str(pos + end_div, &sec);
            } else {
                body_html.push_str(&sec);
            }
        } else {
            body_html.push_str(&sec);
        }
    }

    // Build the outer HTML5 document
    let mut doc = String::new();
    let _ = write!(
        doc,
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>{}</title>",
        escape_html(&doc_title)
    );

    // Metadata, icons, and modern typography tokens
    doc.push_str("<link rel=\"icon\" type=\"image/png\" href=\"/assets/logo.png\">\
    <link rel=\"icon\" type=\"image/x-icon\" href=\"/favicon.ico\">\
    <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\
    <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\
    <link href=\"https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap\" rel=\"stylesheet\">\
    <style>");
    doc.push_str(styles::get_webforge_stylesheet());
    doc.push_str("</style>");

    doc.push_str("<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\
    <script src=\"https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js\"></script>\
    <script src=\"https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/SVGLoader.js\"></script>\
    <script src=\"https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/exporters/GLTFExporter.js\"></script>\
    <script src=\"https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/exporters/OBJExporter.js\"></script>\
    <script src=\"https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/exporters/STLExporter.js\"></script>");

    doc.push_str("</head><body>");

    // 1. Header (either developer-declared or minimal default)
    if let Some(ref hb) = header_block {
        render_header_block(&mut doc, hb);
    } else {
        let dummy = DslBlockValue {
            kind: "header".to_string(),
            args: vec![],
            properties: vec![],
            children: vec![],
        };
        render_header_block(&mut doc, &dummy);
    }

    // 2. Main content container
    doc.push_str("<main class=\"webforge-main\">");
    doc.push_str(&body_html);
    doc.push_str("</main>");

    // 3. Footer (either developer-declared or minimal default)
    if let Some(ref fb) = footer_block {
        render_footer_block(&mut doc, fb);
    } else {
        // Rich multi-column default footer
        doc.push_str("<footer class=\"webforge-footer\"><div class=\"footer-inner\">");
        // Brand column
        doc.push_str("\
<div class=\"footer-brand footer-col\">\
    <a href=\"/\" onclick=\"event.preventDefault();webforgeRouter.navigate('/')\" style=\"display:inline-flex;align-items:center;gap:0.75rem;text-decoration:none;margin-bottom:0.25rem;\">\
        <div style=\"width:32px;height:32px;border-radius:8px;background:linear-gradient(135deg,#06B6D4,#14B8A6);display:flex;align-items:center;justify-content:center;font-weight:800;font-size:1rem;color:#fff;box-shadow:0 0 16px rgba(6,182,212,0.4);\">TX</div>\
        <span style=\"font-size:1.1rem;font-weight:800;color:#fff;\">TechScript WebForge</span>\
    </a>\
    <p>The modern declarative programming ecosystem. Build full-stack web applications with embedded SQLite, hardware-accelerated WebGL 3D, and client-side SPA routing — from a single <code style=\"background:rgba(6,182,212,0.1);color:#22d3ee;padding:0.1rem 0.35rem;border-radius:4px;font-size:0.85rem;\">.txs</code> source file.</p>\
    <div style=\"display:flex;gap:0.75rem;flex-wrap:wrap;\">\
        <a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\" style=\"display:inline-flex;align-items:center;gap:0.4rem;color:var(--text-muted);text-decoration:none;font-size:0.85rem;padding:0.4rem 0.85rem;border:1px solid var(--border);border-radius:8px;transition:all 0.2s;\" onmouseover=\"this.style.color='#fff';this.style.borderColor='rgba(6,182,212,0.4)'\" onmouseout=\"this.style.color='';this.style.borderColor=''\">⭐ GitHub</a>\
        <a href=\"/download\" onclick=\"event.preventDefault();webforgeRouter.navigate('/download')\" style=\"display:inline-flex;align-items:center;gap:0.4rem;color:var(--primary);text-decoration:none;font-size:0.85rem;padding:0.4rem 0.85rem;border:1px solid rgba(6,182,212,0.3);border-radius:8px;background:rgba(6,182,212,0.08);transition:all 0.2s;\">⬇ Download</a>\
    </div>\
</div>");
        // Framework column
        doc.push_str("\
<div class=\"footer-col\">\
    <h4>Framework</h4>\
    <a href=\"/language\" onclick=\"event.preventDefault();webforgeRouter.navigate('/language')\">Language Spec</a>\
    <a href=\"/framework\" onclick=\"event.preventDefault();webforgeRouter.navigate('/framework')\">Architecture</a>\
    <a href=\"/features\" onclick=\"event.preventDefault();webforgeRouter.navigate('/features')\">Features</a>\
    <a href=\"/showcase/3d\" onclick=\"event.preventDefault();webforgeRouter.navigate('/showcase/3d')\">3D Laboratory</a>\
    <a href=\"/playground\" onclick=\"event.preventDefault();webforgeRouter.navigate('/playground')\">Playground</a>\
    <a href=\"/packages\" onclick=\"event.preventDefault();webforgeRouter.navigate('/packages')\">Packages</a>\
</div>");
        // Resources column
        doc.push_str("\
<div class=\"footer-col\">\
    <h4>Resources</h4>\
    <a href=\"/docs\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs')\">Documentation</a>\
    <a href=\"/docs/getting-started\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs/getting-started')\">Getting Started</a>\
    <a href=\"/docs/syntax\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs/syntax')\">Syntax Reference</a>\
    <a href=\"/docs/api\" onclick=\"event.preventDefault();webforgeRouter.navigate('/docs/api')\">API Reference</a>\
    <a href=\"/examples\" onclick=\"event.preventDefault();webforgeRouter.navigate('/examples')\">Examples</a>\
    <a href=\"/examples/fullstack\" onclick=\"event.preventDefault();webforgeRouter.navigate('/examples/fullstack')\">Fullstack Demo</a>\
</div>");
        // Community column
        doc.push_str("\
<div class=\"footer-col\">\
    <h4>Community</h4>\
    <a href=\"/roadmap\" onclick=\"event.preventDefault();webforgeRouter.navigate('/roadmap')\">Roadmap</a>\
    <a href=\"/community\" onclick=\"event.preventDefault();webforgeRouter.navigate('/community')\">Community</a>\
    <a href=\"/cli\" onclick=\"event.preventDefault();webforgeRouter.navigate('/cli')\">CLI Reference</a>\
    <a href=\"/download\" onclick=\"event.preventDefault();webforgeRouter.navigate('/download')\">Download</a>\
    <a href=\"https://github.com/Tcode-Motion/techscript\" target=\"_blank\" rel=\"noopener noreferrer\">GitHub ↗</a>\
    <a href=\"https://github.com/Tcode-Motion/techscript/issues\" target=\"_blank\" rel=\"noopener noreferrer\">Report Issue ↗</a>\
</div>");
        doc.push_str("</div>"); // footer-inner
        doc.push_str("\
<div class=\"footer-bottom\">\
    <div style=\"display:flex;align-items:center;gap:1rem;\">\
        <span>© 2026 TechScript Ecosystem. Open-source under MIT License.</span>\
        <a href=\"/roadmap\" onclick=\"event.preventDefault();webforgeRouter.navigate('/roadmap')\">Roadmap</a>\
        <a href=\"https://github.com/Tcode-Motion/techscript/blob/main/LICENSE\" target=\"_blank\" rel=\"noopener noreferrer\">License</a>\
    </div>\
    <div class=\"footer-status\">\
        <span class=\"footer-status-dot\"></span>\
        <span>All systems operational</span>\
        <span style=\"color:rgba(255,255,255,0.15);\">|</span>\
        <span>Crafted with <a href=\"/\" onclick=\"event.preventDefault();webforgeRouter.navigate('/')\" style=\"color:var(--primary);\">WebForge v2.0</a></span>\
    </div>\
</div>");
        doc.push_str("</footer>");
    }

    doc.push_str("<div id=\"webforge-toast-container\"></div>");

    // 4. Client JavaScript runtime
    doc.push_str("<script>");
    doc.push_str(runtime_js::get_runtime_javascript());
    doc.push_str("</script>");

    // 5. Client script for declared 3D scenes
    if !scenes_3d.is_empty() {
        doc.push_str("<script>document.addEventListener('DOMContentLoaded', () => {");
        for sc in &scenes_3d {
            let sc_id = sc
                .args
                .first()
                .map(|a| a.to_string())
                .or_else(|| {
                    sc.properties
                        .iter()
                        .find(|p| p.name == "id")
                        .and_then(|p| p.value.as_ref().map(|v| v.to_string()))
                })
                .unwrap_or_else(|| "World".to_string());

            let mut initial_geo = "scifi".to_string();
            let mut initial_color = "cyan".to_string();
            let mut rot_speed: f64 = 1.2;

            // 1. Direct arguments (e.g. `show 3D "mech"` or `model3D "quantum_chip"`)
            if let Some(arg) = sc.args.first() {
                let s = arg.to_string().trim_matches('"').to_lowercase();
                if [
                    "scifi",
                    "mech",
                    "knot",
                    "gyroscope",
                    "tesseract",
                    "dna",
                    "satellite",
                    "torus",
                    "star",
                    "crystal",
                    "geodesic",
                    "product",
                    "quantum_chip",
                    "black_hole",
                    "mobius",
                    "dyson_swarm",
                ]
                .contains(&s.as_str())
                {
                    initial_geo = s;
                }
            }

            // 2. Direct block properties (e.g. `color is "#F59E0B"`, `speed is 1.4`)
            for p in &sc.properties {
                if p.name == "geo" || p.name == "model" || p.name == "mesh" {
                    if let Some(ref v) = p.value {
                        initial_geo = v.to_string().trim_matches('"').to_lowercase();
                    }
                } else if p.name == "color" {
                    if let Some(ref v) = p.value {
                        initial_color = v.to_string().trim_matches('"').to_string();
                    }
                } else if p.name == "speed"
                    || p.name == "rotation"
                    || p.name == "rotation speed"
                    || p.name == "rotation_speed"
                {
                    if let Some(ref v) = p.value {
                        rot_speed = v.try_into_float().unwrap_or(1.2);
                    }
                }
            }

            // 3. Child blocks (e.g. `add mech: ... end` for backwards compatibility)
            for child in &sc.children {
                match child.kind.as_str() {
                    "scifi" | "core" => initial_geo = "scifi".to_string(),
                    "mech" | "tower" => initial_geo = "mech".to_string(),
                    "knot" | "torus_knot" | "sculpture" => initial_geo = "knot".to_string(),
                    "product" | "phone" => initial_geo = "product".to_string(),
                    "crystal" | "octahedron" | "gem" => initial_geo = "crystal".to_string(),
                    "geodesic" | "sphere" => initial_geo = "geodesic".to_string(),
                    "gyroscope" | "gyro" => initial_geo = "gyroscope".to_string(),
                    "tesseract" => initial_geo = "tesseract".to_string(),
                    "dna" => initial_geo = "dna".to_string(),
                    "satellite" => initial_geo = "satellite".to_string(),
                    "torus" => initial_geo = "torus".to_string(),
                    "star" => initial_geo = "star".to_string(),
                    "quantum_chip" => initial_geo = "quantum_chip".to_string(),
                    "black_hole" => initial_geo = "black_hole".to_string(),
                    "mobius" => initial_geo = "mobius".to_string(),
                    "dyson_swarm" => initial_geo = "dyson_swarm".to_string(),
                    _ => {}
                }
                for p in &child.properties {
                    if p.name == "color" {
                        if let Some(ref v) = p.value {
                            initial_color = v.to_string().trim_matches('"').to_string();
                        }
                    } else if p.name == "speed"
                        || p.name == "rotation"
                        || p.name == "rotation speed"
                        || p.name == "rotation_speed"
                    {
                        if let Some(ref v) = p.value {
                            rot_speed = v.try_into_float().unwrap_or(1.2);
                        }
                    }
                }
            }

            let is_custom = sc_id.to_lowercase().contains("custom")
                || sc_id.to_lowercase().contains("playground")
                || sc.properties.iter().any(|p| {
                    p.name == "mode"
                        && p.value.as_ref().map(|v| v.to_string()).unwrap_or_default() == "custom"
                })
                || sc.children.iter().any(|c| {
                    matches!(
                        c.kind.as_str(),
                        "box"
                            | "wall"
                            | "roof"
                            | "door"
                            | "window"
                            | "chimney"
                            | "extrude"
                            | "outline"
                            | "cut"
                            | "boolean"
                            | "part"
                    )
                });

            if is_custom {
                let _ = write!(
                    doc,
                    "if (window.WebForgeCustom3D) window.WebForgeCustom3D.init('{id}');",
                    id = sc_id
                );
            } else {
                let _ = write!(
                    doc,
                    "if (window.WebForge3D) window.WebForge3D.mount('{id}', {{ geo: '{geo}', color: '{col}', speed: {spd} }});",
                    id = sc_id,
                    col = initial_color,
                    geo = initial_geo,
                    spd = rot_speed
                );
            }
        }
        doc.push_str("});</script>");
    }

    // 6. Client script for declared event handlers
    if !event_handlers.is_empty() {
        doc.push_str("<script>document.addEventListener('DOMContentLoaded', () => {");
        for eh in event_handlers {
            if let RuntimeValue::DslBlock(dsl) = eh {
                let target = dsl.args.first().map(|a| a.to_string()).unwrap_or_default();
                let mut action = String::new();
                let mut arg1 = String::new();
                let mut arg2 = String::new();
                for p in &dsl.properties {
                    match p.name.as_str() {
                        "action" => {
                            action = p.value.as_ref().map(|v| v.to_string()).unwrap_or_default()
                        }
                        "arg1" => {
                            arg1 = p.value.as_ref().map(|v| v.to_string()).unwrap_or_default()
                        }
                        "arg2" => {
                            arg2 = p.value.as_ref().map(|v| v.to_string()).unwrap_or_default()
                        }
                        _ => {}
                    }
                }
                if !target.is_empty() {
                    let clean_target = target.replace('-', "_");
                    if action == "message" {
                        let _ = write!(
                            doc,
                            "{{ const el = document.getElementById('{}');\
                            if (el) el.addEventListener('click', () => {{ webforgeToast('{}'); }}); }}",
                            target, arg1.replace('\'', "\\'")
                        );
                    } else if action == "api_send" {
                        let _ = write!(
                            doc,
                            "{{ const el = document.getElementById('{}');\
                            if (el) el.addEventListener('click', () => {{\
                                const inputEl = document.getElementById('{}');\
                                const payload = inputEl ? inputEl.value : '';\
                                const base = (typeof webforgeRouter !== 'undefined' && webforgeRouter.getBasePath) ? webforgeRouter.getBasePath() : '';\
                                fetch(base + '{}', {{\
                                    method: 'POST',\
                                    headers: {{ 'Content-Type': 'application/json' }},\
                                    body: JSON.stringify({{ content: payload, title: payload }})\
                                }}).then(() => {{\
                                    webforgeToast('Saved to {}');\
                                    if (typeof webforgeLoadDatabase === 'function') {{ webforgeLoadDatabase('{}'); }}\
                                }}).catch(() => {{\
                                    webforgeToast('Saved to {} (local cache)');\
                                }});\
                            }}); }}",
                            target, arg2, arg1, arg1, arg1, arg1
                        );
                    }
                }
            }
        }
        doc.push_str("});</script>");
    }

    doc.push_str("</body></html>");
    doc
}
