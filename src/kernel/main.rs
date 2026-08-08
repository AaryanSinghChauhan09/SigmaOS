#![allow(warnings)]
#![allow(clippy::all)]
||||||| 23ef22a4a
// SigmaOS Kernel Main Entry Point
#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Kernel Main Entry Point
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

use std::collections::HashMap;

/// Ubuntu/Linux-style Kernel Command Line Parser
#[derive(Debug, Clone)]
pub struct KernelCmdLineParser {
    pub params: HashMap<String, String>,
    pub flags: alloc::vec::Vec<String>,
}

impl KernelCmdLineParser {
    /// Parse a raw command-line string (e.g. "init=/bin/sh console=ttyS0 quiet boot=uefi")
    pub fn new(cmdline: &str) -> Self {
        let mut params = HashMap::new();
        let mut flags = alloc::vec::Vec::new();

        for arg in cmdline.split_whitespace() {
            if let Some(pos) = arg.find('=') {
                let key = arg[..pos].to_string();
                let val = arg[pos + 1..].to_string();
                params.insert(key, val);
            } else {
                flags.push(arg.to_string());
            }
        }

        Self { params, flags }
    }

    /// Read parameter value by key
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Check standalone flag existence (e.g. "quiet", "debug")
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }

    /// Retrieve "init" binary path parameter (defaults to "/sbin/init")
    pub fn init_path(&self) -> &str {
        self.get("init").unwrap_or("/sbin/init")
    }

    /// Retrieve "console" output parameter (defaults to "tty0")
    pub fn console(&self) -> &str {
        self.get("console").unwrap_or("tty0")
    }
}

extern crate alloc;

// SigmaOS Kernel Main Entry Point

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel entry point
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
