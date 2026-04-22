/// gui/backend.rs — Zero-dependency Rust HTTP backend
/// Uses only std::net::TcpListener — NO actix-web, NO hyper, NO tokio.
/// Runs on port 8080, mirrors every sigmactl command as a REST endpoint.
///
/// GET  /status          → manager.status()
/// POST /build           → manager.build_all()
/// POST /build/{shard}   → manager.build_shard(shard)
/// POST /sync            → manager.sync_github()
/// POST /shard/add/{n}   → manager.add_shard(n)
/// POST /shard/remove/{n}→ manager.remove_shard(n)
/// POST /profile/set/{n} → manager.apply_profile(n)
/// GET  /shards          → JSON list of shards
/// GET  /health          → {"ok":true}

mod orchestrator { include!("../core/orchestrator.rs"); }
mod config        { include!("../core/config.rs"); }

use orchestrator::ShardManager;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// ── Minimal HTTP parser ────────────────────────────────────────────────────────
struct Request {
    method: String,
    path:   String,
}

fn parse_request(stream: &mut TcpStream) -> Option<Request> {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).ok()?;
    let text = std::str::from_utf8(&buf[..n]).ok()?;
    let first_line = text.lines().next()?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next()?.to_string();
    let path   = parts.next()?.to_string();
    Some(Request { method, path })
}

fn respond(stream: &mut TcpStream, status: u16, body: &str) {
    let status_text = match status { 200 => "OK", 404 => "Not Found", _ => "Error" };
    let resp = format!(
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: text/plain\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(resp.as_bytes());
}

fn respond_json(stream: &mut TcpStream, body: &str) {
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(resp.as_bytes());
}

fn handle_connection(mut stream: TcpStream, mgr: Arc<Mutex<ShardManager>>) {
    let req = match parse_request(&mut stream) {
        Some(r) => r, None => return,
    };

    // Path segment extraction helper
    let segs: Vec<&str> = req.path.trim_start_matches('/').split('/').collect();

    let mut mgr = mgr.lock().unwrap();

    match (req.method.as_str(), segs.as_slice()) {

        ("GET", ["health"]) => respond(&mut stream, 200, r#"{"ok":true}"#),

        ("GET", ["status"]) => {
            let s = mgr.status();
            respond(&mut stream, 200, &s);
        }

        ("GET", ["shards"]) => {
            let list = mgr.list_shards();
            let mut json = String::from("[");
            for (i, s) in list.iter().enumerate() {
                if i > 0 { json.push(','); }
                json.push_str(&format!(r#"{{"name":"{}","lang":"{}"}}"#, s.name, s.lang));
            }
            json.push(']');
            respond_json(&mut stream, &json);
        }

        ("POST", ["build"]) => {
            let result = mgr.build_all();
            match result {
                Ok(()) => respond(&mut stream, 200, "Build complete"),
                Err(e) => respond(&mut stream, 500, &e),
            }
        }

        ("POST", ["build", shard]) => {
            match mgr.build_shard(shard) {
                Ok(()) => respond(&mut stream, 200, &format!("Built: {shard}")),
                Err(e) => respond(&mut stream, 500, &e),
            }
        }

        ("POST", ["sync"]) => {
            match mgr.sync_github() {
                Ok(msg) => respond(&mut stream, 200, &msg),
                Err(e)  => respond(&mut stream, 500, &e),
            }
        }

        ("POST", ["shard", "add", name]) => {
            match mgr.add_shard(name) {
                Ok(()) => respond(&mut stream, 200, &format!("Shard added: {name}")),
                Err(e) => respond(&mut stream, 500, &e),
            }
        }

        ("DELETE" | "POST", ["shard", "remove", name]) => {
            let _ = mgr.remove_shard(name);
            respond(&mut stream, 200, &format!("Shard removed: {name}"));
        }

        ("POST", ["profile", "set", name]) => {
            match mgr.apply_profile(name) {
                Ok(()) => respond(&mut stream, 200, &format!("Profile applied: {name}")),
                Err(e) => respond(&mut stream, 500, &e),
            }
        }

        _ => respond(&mut stream, 404, "Not found"),
    }
}

fn main() {
    let root = std::env::var("SIGMA_ROOT").unwrap_or_else(|_| ".".to_string());
    let mgr  = Arc::new(Mutex::new(ShardManager::with_root(&root)));
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr)
        .unwrap_or_else(|e| { eprintln!("Σ [ERR] Cannot bind {addr}: {e}"); std::process::exit(1); });

    eprintln!("Σ [SERVER] Zenith backend listening on http://{addr}");
    eprintln!("Σ [SERVER] SIGMA_ROOT = {root}");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let mgr_clone = Arc::clone(&mgr);
                thread::spawn(move || handle_connection(s, mgr_clone));
            }
            Err(e) => eprintln!("Σ [WARN] Connection error: {e}"),
        }
    }
}
