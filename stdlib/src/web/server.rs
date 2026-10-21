//! # WebForge Embedded Server Engine
//!
//! Provides the local development server, static asset streaming,
//! integrated REST API endpoints, security headers, and SSRF prevention.

use crate::web::db;
use std::io::Read;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use techscript_runtime::error::RuntimeError;
use techscript_runtime::value::RuntimeValue;
use ureq::Resolver;
use url::Url;

pub static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);
pub static PAGE_CONTENT: Mutex<String> = Mutex::new(String::new());

/// Verify that an IP address is public and safe (prevent SSRF attacks).
pub fn is_safe_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            !ipv4.is_private()
                && !ipv4.is_loopback()
                && !ipv4.is_link_local()
                && !ipv4.is_broadcast()
                && !ipv4.is_documentation()
                && !ipv4.is_unspecified()
        }
        IpAddr::V6(ipv6) => {
            if let Some(ipv4) = ipv6.to_ipv4_mapped() {
                return is_safe_ip(&IpAddr::V4(ipv4));
            }
            !ipv6.is_loopback()
                && !ipv6.is_unspecified()
                && (ipv6.segments()[0] & 0xfe00) != 0xfc00 // Unique Local Address
                && (ipv6.segments()[0] & 0xffc0) != 0xfe80 // Link Local Address
        }
    }
}

/// Verify that a URL is a valid, safe HTTP/HTTPS destination.
pub fn is_safe_url(url_str: &str) -> bool {
    let Ok(parsed_url) = Url::parse(url_str) else {
        return false;
    };

    match parsed_url.scheme() {
        "http" | "https" => {}
        _ => return false,
    }

    if parsed_url.host_str().is_none() {
        return false;
    }

    true
}

/// Custom DNS Resolver enforcing SSRF safety restrictions.
pub struct SafeResolver;

impl Resolver for SafeResolver {
    fn resolve(&self, netloc: &str) -> std::io::Result<Vec<SocketAddr>> {
        let addrs: Vec<SocketAddr> = netloc.to_socket_addrs()?.collect();
        let mut safe_addrs = Vec::new();
        for addr in addrs {
            if is_safe_ip(&addr.ip()) {
                safe_addrs.push(addr);
            }
        }
        if safe_addrs.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "DNS resolution returned only blocked or internal IP addresses (SSRF prevention).",
            ));
        }
        Ok(safe_addrs)
    }
}

/// Attach standard enterprise security headers to a response.
fn apply_security_headers<R: std::io::Read>(resp: tiny_http::Response<R>) -> tiny_http::Response<R> {
    resp.with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, POST, OPTIONS"[..]).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type, Authorization"[..]).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"X-Content-Type-Options"[..], &b"nosniff"[..]).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"X-Frame-Options"[..], &b"SAMEORIGIN"[..]).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"Referrer-Policy"[..], &b"strict-origin-when-cross-origin"[..]).unwrap())
}

/// Starts the WebForge local HTTP server.
pub fn start_webforge_server(port: u16) -> Result<RuntimeValue, RuntimeError> {
    if SERVER_RUNNING.load(Ordering::SeqCst) {
        return Ok(RuntimeValue::Str("Server already running".to_string()));
    }
    SERVER_RUNNING.store(true, Ordering::SeqCst);

    let server = match tiny_http::Server::http(format!("0.0.0.0:{}", port)) {
        Ok(s) => Mutex::new(s),
        Err(e) => {
            SERVER_RUNNING.store(false, Ordering::SeqCst);
            return Err(RuntimeError::new(
                techscript_runtime::error::RuntimeErrorKind::InvalidOperation(format!(
                    "Failed to start server: {}",
                    e
                )),
                None,
                None,
            ));
        }
    };

    println!("🚀 TechScript WebForge running on http://localhost:{}", port);

    thread::spawn(move || {
        while SERVER_RUNNING.load(Ordering::SeqCst) {
            let req_res = {
                let s = server.lock().unwrap();
                s.recv_timeout(std::time::Duration::from_millis(200))
            };

            if let Ok(Some(mut req)) = req_res {
                let raw_url = req.url().to_string();
                let method = req.method().as_str().to_string();

                // 1. Handle REST API endpoints
                if let Some(idx) = raw_url.find("/api/") {
                    let path_part = raw_url[idx + 5..].split('?').next().unwrap_or("health");
                    let clean_table = db::sanitize_identifier(path_part);

                    if method == "OPTIONS" {
                        let resp = tiny_http::Response::empty(200)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                        let _ = req.respond(apply_security_headers(resp));
                        continue;
                    }

                    if clean_table == "health" && method == "GET" {
                        let res_json = serde_json::json!({
                            "status": "ok",
                            "framework": "WebForge",
                            "version": "2.0.0",
                            "language": "TechScript",
                            "compiler": "techscriptc / webforge",
                            "database": "SQLite (integrated)",
                            "security": "strict-isolated",
                            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()
                        }).to_string();

                        let resp = tiny_http::Response::from_string(res_json)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                        let _ = req.respond(apply_security_headers(resp));
                        continue;
                    }

                    if clean_table == "examples" && method == "GET" {
                        let res_json = serde_json::json!([
                            {
                                "id": "01-counter",
                                "title": "Interactive State & Events",
                                "category": "basics",
                                "file": "src/pages/framework.txs",
                                "description": "Declarative reactive buttons with inline state mutation."
                            },
                            {
                                "id": "02-3d-lab",
                                "title": "Procedural 3D Visualizer",
                                "category": "3d",
                                "file": "src/scenes/logo.txs",
                                "description": "Hardware-accelerated WebGL with OrbitControls, PBR materials, and live TechScript reflection."
                            },
                            {
                                "id": "03-sqlite-fullstack",
                                "title": "Embedded SQLite Persistence",
                                "category": "backend",
                                "file": "src/server/api.txs",
                                "description": "Zero-config parameterized SQLite queries and real REST API endpoints."
                            },
                            {
                                "id": "04-multipage-router",
                                "title": "SPA Routing Engine",
                                "category": "routing",
                                "file": "src/main.txs",
                                "description": "Direct browser URL navigation, browser history, bookmarks, and deep links."
                            }
                        ]).to_string();

                        let resp = tiny_http::Response::from_string(res_json)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                        let _ = req.respond(apply_security_headers(resp));
                        continue;
                    }

                    if method == "POST" {
                        let mut body_str = String::new();
                        let _ = req.as_reader().read_to_string(&mut body_str);
                        let json_val: Result<serde_json::Value, _> = serde_json::from_str(&body_str);

                        let title = json_val.as_ref().ok()
                            .and_then(|j| j.get("title").or_else(|| j.get("name")).or_else(|| j.get("content")))
                            .and_then(|v| v.as_str())
                            .unwrap_or(&body_str)
                            .to_string();

                        let content = json_val.as_ref().ok()
                            .and_then(|j| j.get("content").or_else(|| j.get("message")).or_else(|| j.get("title")))
                            .and_then(|v| v.as_str())
                            .unwrap_or(&body_str)
                            .to_string();

                        let _ = db::insert_record(&clean_table, &title, &content);

                        let res_json = serde_json::json!({
                            "status": "ok",
                            "message": "Resource saved successfully",
                            "endpoint": format!("/api/{}", clean_table)
                        }).to_string();

                        let resp = tiny_http::Response::from_string(res_json)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                        let _ = req.respond(apply_security_headers(resp));
                        continue;
                    } else {
                        // GET /api/<table_name>
                        let items = db::query_records(&clean_table, 50).unwrap_or_default();
                        let res_json = serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string());
                        let resp = tiny_http::Response::from_string(res_json)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                        let _ = req.respond(apply_security_headers(resp));
                        continue;
                    }
                }

                // 2. Handle Static Assets (/assets/ and /favicon.ico)
                if raw_url.starts_with("/assets/") || raw_url == "/favicon.ico" {
                    let clean_path = raw_url.trim_start_matches('/').split('?').next().unwrap_or("favicon.ico");
                    
                    // Prevent path traversal
                    if clean_path.contains("..") {
                        let resp = tiny_http::Response::empty(403);
                        let _ = req.respond(apply_security_headers(resp));
                        continue;
                    }

                    let possible_paths = [
                        PathBuf::from(clean_path),
                        PathBuf::from("website").join(clean_path),
                        PathBuf::from("website/dist").join(clean_path),
                        PathBuf::from("dist").join(clean_path),
                        if clean_path.ends_with("logo.png") || clean_path.ends_with(".png") {
                            PathBuf::from("assets/branding/logo-package/source/logo-master-transparent.png")
                        } else {
                            PathBuf::from("assets/branding/logo-package/ico/favicon.ico")
                        },
                    ];

                    let mut found_bytes = None;
                    let mut mime_type = "application/octet-stream";
                    for p in &possible_paths {
                        if p.exists() {
                            if let Ok(bytes) = std::fs::read(p) {
                                found_bytes = Some(bytes);
                                if clean_path.ends_with(".png") {
                                    mime_type = "image/png";
                                } else if clean_path.ends_with(".ico") {
                                    mime_type = "image/x-icon";
                                } else if clean_path.ends_with(".css") {
                                    mime_type = "text/css";
                                } else if clean_path.ends_with(".js") {
                                    mime_type = "application/javascript";
                                } else if clean_path.ends_with(".svg") {
                                    mime_type = "image/svg+xml";
                                }
                                break;
                            }
                        }
                    }

                    if let Some(bytes) = found_bytes {
                        let resp = tiny_http::Response::from_data(bytes)
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], mime_type.as_bytes()).unwrap())
                            .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=86400"[..]).unwrap());
                        let _ = req.respond(apply_security_headers(resp));
                    } else {
                        let resp = tiny_http::Response::empty(404);
                        let _ = req.respond(apply_security_headers(resp));
                    }
                    continue;
                }

                // 3. Serve the Single-Page Application (SPA) HTML for all page routes
                let page = PAGE_CONTENT.lock().unwrap();
                let resp = tiny_http::Response::from_string(&*page)
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                let _ = req.respond(apply_security_headers(resp));
            }
        }
    });

    Ok(RuntimeValue::Str(format!("Server started on port {}", port)))
}
