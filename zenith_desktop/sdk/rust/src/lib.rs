//! =========================================================================
//! Σ ZENITH DEVELOPER SDK (RUST BINDINGS v0.1)
//! =========================================================================
//! Standard memory-safe interfaces and container-orchestration hooks
//! for SigmaOS Zenith applications.
//! =========================================================================

#![no_std]

use core::ffi::c_char;

// ---------------------------------------------------------
// 1. Core C FFI Import Signatures
// ---------------------------------------------------------
extern "C" {
    /// Kernel-level structured logger
    pub fn zenith_log_structured(error_code: u32, component: *const c_char, description: *const c_char, container_id: u32);
    
    /// Userland system print syscall wrapper
    pub fn sys_print(format: *const c_char, ...);
    
    /// Userland system IPC dispatch syscall wrapper
    pub fn sys_ipc_send(target_shard: u32, msg_id: u32, data: *const core::ffi::c_void, len: usize) -> i32;

    /// Launch application strictly sandboxed by the Orchestrator
    pub fn zenith_launch_app_sandboxed(name: *const c_char, inode: u32) -> u32;
}

// ---------------------------------------------------------
// 2. Safe, Idiomatic Rust Wrappers
// ---------------------------------------------------------

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct Application {
    name: &'static str,
    container_id: u32,
}

impl Application {
    /// Initializes a new Zenith application, automatically negotiating with 
    /// the Sovereign Orchestrator to allocate a strictly sandboxed container.
    pub fn new(name: &'static str) -> Self {
        // Convert static str to null-terminated C string safely
        let mut name_buf = [0; 64];
        let bytes = name.as_bytes();
        let limit = bytes.len().min(63);
        name_buf[..limit].copy_from_slice(&bytes[..limit]);
        name_buf[limit] = 0;

        let container_id = unsafe {
            zenith_launch_app_sandboxed(name_buf.as_ptr() as *const c_char, 42)
        };

        unsafe {
            let msg = "[Zenith-Rust-SDK] Secured Container Sandbox Shard: %u\n\0";
            sys_print(msg.as_ptr() as *const c_char, container_id);
        }

        Self { name, container_id }
    }

    pub fn get_container_id(&self) -> u32 {
        self.container_id
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Enters the sovereign main event loop, waiting for IPC window compositing signals.
    pub fn run(&self) {
        unsafe {
            let msg = "[Zenith-Rust-SDK] Rust App entering Sovereign Event Loop.\n\0";
            sys_print(msg.as_ptr() as *const c_char);
        }
    }
}
