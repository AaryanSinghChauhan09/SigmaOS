/// core/src/orchestrator.rs — Sovereign ShardManager
/// Freestanding silicon orchestrator.

use crate::ffi;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct ShardInfo {
    pub name: String,
    pub path: String,
}

pub struct ShardManager {
    pub shards: Vec<ShardInfo>,
    pub root:   String,
}

impl ShardManager {
    pub fn with_root(root: &str) -> Self {
        Self { shards: Vec::new(), root: root.to_string() }
    }

    pub fn status(&self) -> String {
        let mut s = String::from("Σ SIGMAOS MODULAR FREESTANDING\n");
        s.push_str("Mode: Silicon-Native\n");
        s
    }

    pub fn spawn_raw(&self, cmd: &str) {
        if cmd.len() > 2048 { return; } // Loophole: Buffer overflow protection
        #[cfg(windows)]
        unsafe {
            let mut si = [0u8; 128]; // Manual zeroing implicitly handled by array init
            let mut pi = [0u8; 32];
            let mut cmd_buf = Vec::with_capacity(cmd.len() + 1);
            for c in cmd.chars() {
                if c == '\0' { break; } // Loophole: Null injection protection
                cmd_buf.push(c as u8);
            }
            cmd_buf.push(0);
            
            ffi::CreateProcessA(
                core::ptr::null(),
                cmd_buf.as_mut_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                1, 0,
                core::ptr::null_mut(),
                core::ptr::null(),
                si.as_mut_ptr(),
                pi.as_mut_ptr(),
            );
        }
    }
}
