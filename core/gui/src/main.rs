/// gui/src/main.rs — Freestanding Silicon Backend
/// Modular, zero-dependency networking.

use sigma_core::ffi;
use sigma_core::orchestrator::ShardManager;

static mut GLOBAL_MGR: *mut ShardManager = core::ptr::null_mut();

use core::sync::atomic::{AtomicUsize, Ordering};

static CONNECTION_COUNT: AtomicUsize = AtomicUsize::new(0);

extern "C" fn client_handler(s: usize) -> u32 {
    unsafe {
        let count = CONNECTION_COUNT.fetch_add(1, Ordering::SeqCst);
        if count > 100 { 
            CONNECTION_COUNT.fetch_sub(1, Ordering::SeqCst);
            ffi::closesocket(s); 
            return 0; 
        }
        
        let mut buf = [0u8; 1024];
        let n = ffi::recv(s, buf.as_mut_ptr(), 1024, 0);
        if n > 0 && n < 1024 {
            let mut text = String::new();
            for &b in &buf[..n as usize] {
                if b == 0 { break; } 
                text.push(b as char);
            }
            if text.contains("/status") {
                let status = (*GLOBAL_MGR).status();
                respond(s, &status);
            } else {
                respond(s, "Σ SigmaOS Freestanding Modular Active");
            }
        }
        ffi::closesocket(s);
        CONNECTION_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
    0
}

fn respond(s: usize, body: &str) {
    let head = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n";
    unsafe {
        ffi::send(s, head.as_ptr(), head.len() as i32, 0);
        ffi::send(s, body.as_ptr(), body.len() as i32, 0);
    }
}

fn main() {
    unsafe {
        let mut d = [0u8; 512];
        ffi::WSAStartup(0x0202, d.as_mut_ptr());
        
        let mgr = Box::new(ShardManager::with_root("."));
        GLOBAL_MGR = Box::into_raw(mgr);
        
        let s = ffi::socket(2, 1, 6);
        let addr: [u8; 16] = [2, 0, 0x1F, 0x90, 127, 0, 0, 1, 0,0,0,0,0,0,0,0];
        
        if ffi::bind(s, addr.as_ptr(), 16) == 0 && ffi::listen(s, 5) == 0 {
            eprintln!("Σ [MODULAR] Backend active via raw FFI on http://127.0.0.1:8080");
            loop {
                let mut l = 16i32;
                let c = ffi::accept(s, core::ptr::null_mut(), &mut l);
                if c != usize::MAX {
                    ffi::CreateThread(core::ptr::null_mut(), 0, client_handler as usize, c, 0, core::ptr::null_mut());
                }
            }
        }
    }
}
