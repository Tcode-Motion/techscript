//! # WebForge Standard Library Web Module
//!
//! Provides the core runtime services for the TechScript WebForge ecosystem:
//! - Structured HTML/CSS/JS rendering engine (`renderer`)
//! - Embedded dev server and static asset streamer (`server`)
//! - Parameterized SQLite database persistence (`db`)
//! - Universal client-side SPA routing runtime (`runtime_js`)
//! - Enterprise CSS design system and tokens (`styles`)

pub mod db;
pub mod renderer;
pub mod runtime_js;
pub mod server;
pub mod styles;

use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use techscript_runtime::{
    context::{Capability, RuntimeContext},
    error::RuntimeError,
    function::Callable,
    value::RuntimeValue,
};

pub fn web_render_html_pub(ctx: &RuntimeContext) -> String {
    let env = ctx.global_env.borrow();
    let blocks = match env.lookup("_dsl_blocks") {
        Ok(RuntimeValue::List { items, .. }) => items.borrow().clone(),
        _ => Vec::new(),
    };
    let event_handlers = match env.lookup("_event_handlers") {
        Ok(RuntimeValue::List { items, .. }) => items.borrow().clone(),
        _ => Vec::new(),
    };
    renderer::render_complete_document(&blocks, &event_handlers)
}

fn web_start(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if !ctx.config.capabilities.contains(&Capability::Network) {
        return Err(RuntimeError::new(
            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                "Security policy violation: Network capability is denied".to_string(),
            ),
            None,
            None,
        ));
    }
    let port = args[0].try_into_int().map_err(|e| {
        RuntimeError::new(
            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()),
            None,
            None,
        )
    })? as u16;
    let content = args[1].to_string();
    *server::PAGE_CONTENT.lock().unwrap() = content;
    server::start_webforge_server(port)
}

fn web_page(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let _path = args[0].to_string();
    let content = args[1].to_string();
    *server::PAGE_CONTENT.lock().unwrap() = content;
    Ok(RuntimeValue::Null)
}

fn web_serve(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if !ctx.config.capabilities.contains(&Capability::Network) {
        return Err(RuntimeError::new(
            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                "Security policy violation: Network capability is denied".to_string(),
            ),
            None,
            None,
        ));
    }
    let port = args[0].try_into_int().map_err(|e| {
        RuntimeError::new(
            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()),
            None,
            None,
        )
    })? as u16;
    server::start_webforge_server(port)
}

fn web_stop(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    server::SERVER_RUNNING.store(false, Ordering::SeqCst);
    Ok(RuntimeValue::Null)
}

fn web_set_content(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let new_content = args[0].to_string();
    *server::PAGE_CONTENT.lock().unwrap() = new_content;
    Ok(RuntimeValue::Str("Content updated".to_string()))
}

fn web_fetch(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if !ctx.config.capabilities.contains(&Capability::Network) {
        return Err(RuntimeError::new(
            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                "Security policy violation: Network capability is denied".to_string(),
            ),
            None,
            None,
        ));
    }
    let url = args[0].to_string();

    if !server::is_safe_url(&url) {
        return Err(RuntimeError::new(
            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                format!("Access denied: the URL '{}' points to a blocked or internal destination (SSRF prevention).", url)
            ),
            None,
            None,
        ));
    }

    let agent = ureq::builder()
        .resolver(server::SafeResolver)
        .redirects(0)
        .build();
    let body = agent
        .get(&url)
        .call()
        .map_err(|e| {
            RuntimeError::new(
                techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()),
                None,
                None,
            )
        })?
        .into_string()
        .map_err(|e| {
            RuntimeError::new(
                techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()),
                None,
                None,
            )
        })?;
    Ok(RuntimeValue::Str(body))
}

fn web_render_html(
    ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let html = web_render_html_pub(ctx);
    Ok(RuntimeValue::Str(html))
}

fn web_render_dsl(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let mut html = String::new();
    renderer::dsl_to_html_inner(&args[0], &mut html);
    Ok(RuntimeValue::Str(html))
}

fn web_start_website(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let port = if let Some(arg) = args.first() {
        arg.try_into_int().unwrap_or(3000) as u16
    } else {
        3000
    };

    let blocks = {
        let env = ctx.global_env.borrow();
        match env.lookup("_dsl_blocks") {
            Ok(RuntimeValue::List { items, .. }) => items.borrow().clone(),
            _ => Vec::new(),
        }
    };
    let event_handlers = {
        let env = ctx.global_env.borrow();
        match env.lookup("_event_handlers") {
            Ok(RuntimeValue::List { items, .. }) => items.borrow().clone(),
            _ => Vec::new(),
        }
    };

    for b in &blocks {
        if let RuntimeValue::DslBlock(dsl) = b {
            if dsl.kind == "database" {
                db::init_sqlite_database(dsl);
            }
        }
    }

    let full_html = renderer::render_complete_document(&blocks, &event_handlers);
    *server::PAGE_CONTENT.lock().unwrap() = full_html;

    server::start_webforge_server(port)
}

fn web_api_send(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let endpoint = args
        .get(0)
        .map(|a| a.to_string())
        .unwrap_or_else(|| "/api".to_string());
    let payload = args.get(1).map(|a| a.to_string()).unwrap_or_default();
    if endpoint.starts_with("/api/") {
        let path_part = endpoint[5..].split('?').next().unwrap_or("notes");
        let clean_table = db::sanitize_identifier(path_part);
        let _ = db::insert_record(&clean_table, &payload, &payload);
    }
    Ok(RuntimeValue::Str(format!("Sent to {}", endpoint)))
}

impl StdlibRegistry {
    pub fn register_web(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "start".to_string(),
            Rc::new(StdFunction {
                name: "start".to_string(),
                arity: 2,
                callback: web_start,
            }),
        );

        exports.insert(
            "page".to_string(),
            Rc::new(StdFunction {
                name: "page".to_string(),
                arity: 2,
                callback: web_page,
            }),
        );

        exports.insert(
            "serve".to_string(),
            Rc::new(StdFunction {
                name: "serve".to_string(),
                arity: 1,
                callback: web_serve,
            }),
        );

        exports.insert(
            "stop".to_string(),
            Rc::new(StdFunction {
                name: "stop".to_string(),
                arity: 0,
                callback: web_stop,
            }),
        );

        exports.insert(
            "set_content".to_string(),
            Rc::new(StdFunction {
                name: "set_content".to_string(),
                arity: 1,
                callback: web_set_content,
            }),
        );

        exports.insert(
            "fetch".to_string(),
            Rc::new(StdFunction {
                name: "fetch".to_string(),
                arity: 1,
                callback: web_fetch,
            }),
        );

        exports.insert(
            "render_html".to_string(),
            Rc::new(StdFunction {
                name: "render_html".to_string(),
                arity: 0,
                callback: web_render_html,
            }),
        );

        exports.insert(
            "render_dsl".to_string(),
            Rc::new(StdFunction {
                name: "render_dsl".to_string(),
                arity: 1,
                callback: web_render_dsl,
            }),
        );

        exports.insert(
            "web_start_website".to_string(),
            Rc::new(StdFunction {
                name: "web_start_website".to_string(),
                arity: 0,
                callback: web_start_website,
            }),
        );

        exports.insert(
            "start_website".to_string(),
            Rc::new(StdFunction {
                name: "start_website".to_string(),
                arity: 0,
                callback: web_start_website,
            }),
        );

        exports.insert(
            "web_api_send".to_string(),
            Rc::new(StdFunction {
                name: "web_api_send".to_string(),
                arity: 2,
                callback: web_api_send,
            }),
        );

        self.register_module(
            "std.web",
            StdlibModule {
                name: "std.web".to_string(),
                version: "2.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Network],
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::server::*;
    use std::net::IpAddr;

    #[test]
    fn test_safe_ipv4() {
        let ip: IpAddr = "8.8.8.8".parse().unwrap();
        assert!(is_safe_ip(&ip));

        let ip: IpAddr = "1.1.1.1".parse().unwrap();
        assert!(is_safe_ip(&ip));
    }

    #[test]
    fn test_unsafe_ipv4() {
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "172.16.0.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "192.168.1.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "169.254.0.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "255.255.255.255".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "192.0.2.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "198.51.100.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "0.0.0.0".parse().unwrap();
        assert!(!is_safe_ip(&ip));
    }

    #[test]
    fn test_safe_ipv6() {
        let ip: IpAddr = "2001:4860:4860::8888".parse().unwrap();
        assert!(is_safe_ip(&ip));
    }

    #[test]
    fn test_unsafe_ipv6() {
        let ip: IpAddr = "::1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "::".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "fc00::1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "fd00::1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "fe80::1".parse().unwrap();
        assert!(!is_safe_ip(&ip));
    }

    #[test]
    fn test_ipv4_mapped_ipv6() {
        let ip: IpAddr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "::ffff:10.0.0.1".parse().unwrap();
        assert!(!is_safe_ip(&ip));

        let ip: IpAddr = "::ffff:8.8.8.8".parse().unwrap();
        assert!(is_safe_ip(&ip));
    }

    #[test]
    fn test_static_asset_caching_and_performance() {
        clear_asset_cache();
        let asset = get_static_asset("favicon.ico");
        assert!(asset.is_some(), "favicon.ico should be found");
        let (bytes, mime) = asset.unwrap();
        assert_eq!(mime, "image/x-icon");
        assert!(!bytes.is_empty());

        // Warm up cache
        let _ = get_static_asset("favicon.ico");

        // Benchmark cached hits vs uncached disk reads
        let iterations = 1000;

        let start_cached = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = get_static_asset("favicon.ico");
        }
        let duration_cached = start_cached.elapsed();

        let start_uncached = std::time::Instant::now();
        for _ in 0..iterations {
            clear_asset_cache();
            let _ = get_static_asset("favicon.ico");
        }
        let duration_uncached = start_uncached.elapsed();

        println!(
            "Static Asset Fetch (1,000 requests) - Cached: {:?}, Uncached: {:?}",
            duration_cached, duration_uncached
        );

        // Cached RAM lookup must be strictly faster than uncached disk I/O
        assert!(
            duration_cached <= duration_uncached,
            "In-memory asset cache ({:?}) should be at least as fast as uncached disk I/O ({:?})",
            duration_cached,
            duration_uncached
        );
    }
}
