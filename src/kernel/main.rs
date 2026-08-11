// SigmaOS Kernel Main Entry Point
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

use alloc::collections::BTreeMap;

/// Ubuntu/Linux-style Kernel Command Line Parser
#[derive(Debug, Clone)]
pub struct KernelCmdLineParser {
    pub params: BTreeMap<String, String>,
    pub flags: alloc::vec::Vec<String>,
}

impl KernelCmdLineParser {
    /// Parse a raw command-line string (e.g. "init=/bin/sh console=ttyS0 quiet boot=uefi")
    pub fn new(cmdline: &str) -> Self {
        let mut params = BTreeMap::new();
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

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel entry point
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {
    let parser = KernelCmdLineParser::new("init=/bin/sh console=ttyS0 quiet boot=uefi");
    println!("Init Path: {}", parser.init_path());
    println!("Console: {}", parser.console());
    println!("Quiet flag: {}", parser.has_flag("quiet"));
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_cmdline_parsing() {
        let parser = KernelCmdLineParser::new("init=/bin/sh console=ttyS0 quiet boot=uefi debug");
        assert_eq!(parser.init_path(), "/bin/sh");
        assert_eq!(parser.console(), "ttyS0");
        assert_eq!(parser.get("boot").unwrap(), "uefi");
        assert!(parser.has_flag("quiet"));
        assert!(parser.has_flag("debug"));
        assert!(!parser.has_flag("verbose"));
    }

    #[test]
    fn test_kernel_cmdline_defaults() {
        let parser = KernelCmdLineParser::new("quiet");
        assert_eq!(parser.init_path(), "/sbin/init");
        assert_eq!(parser.console(), "tty0");
        assert!(parser.has_flag("quiet"));
    }
}
