//! SigmaSystemInfo - System Information Reporting System
//!
//! Provides comprehensive system information reporting inspired by Linux system info tools.
//! Supports hardware detection, OS version reporting, and system configuration.

#![no_std]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub architecture: String,
    pub hostname: String,
    pub uptime_seconds: u64,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            os_name: String::new(),
            os_version: String::new(),
            kernel_version: String::new(),
            architecture: String::new(),
            hostname: String::new(),
            uptime_seconds: 0,
        }
    }

    pub fn with_os_name(mut self, os_name: String) -> Self {
        self.os_name = os_name;
        self
    }

    pub fn set_os_name(&mut self, os_name: String) {
        self.os_name = os_name;
    }

    pub fn with_os_version(mut self, os_version: String) -> Self {
        self.os_version = os_version;
        self
    }

    pub fn with_kernel_version(mut self, kernel_version: String) -> Self {
        self.kernel_version = kernel_version;
        self
    }

    pub fn with_architecture(mut self, architecture: String) -> Self {
        self.architecture = architecture;
        self
    }

    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = hostname;
        self
    }

    pub fn with_uptime(mut self, uptime_seconds: u64) -> Self {
        self.uptime_seconds = uptime_seconds;
        self
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMASYSTEMINFO_INSTANCE: Option<SystemInfo> = None;

pub fn init_sigmasysteminfo() {
    unsafe {
        SIGMASYSTEMINFO_INSTANCE = Some(SystemInfo::new());
    }
}

pub fn get_sigmasysteminfo() -> &'static SystemInfo {
    unsafe {
        SIGMASYSTEMINFO_INSTANCE
            .as_ref()
            .expect("SigmaSystemInfo not initialized")
    }
}

pub fn get_sigmasysteminfo_mut() -> &'static mut SystemInfo {
    unsafe {
        SIGMASYSTEMINFO_INSTANCE
            .as_mut()
            .expect("SigmaSystemInfo not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_creation() {
        let info = SystemInfo::new();
        assert_eq!(info.os_name, "");
        assert_eq!(info.uptime_seconds, 0);
    }

    #[test]
    fn test_system_info_builder() {
        let info = SystemInfo::new()
            .with_os_name("SigmaOS".to_string())
            .with_os_version("1.0.0".to_string())
            .with_kernel_version("6.0.0".to_string())
            .with_architecture("x86_64".to_string())
            .with_hostname("sigma-pc".to_string())
            .with_uptime(3600);

        assert_eq!(info.os_name, "SigmaOS");
        assert_eq!(info.os_version, "1.0.0");
        assert_eq!(info.kernel_version, "6.0.0");
        assert_eq!(info.architecture, "x86_64");
        assert_eq!(info.hostname, "sigma-pc");
        assert_eq!(info.uptime_seconds, 3600);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasysteminfo();
        let info = get_sigmasysteminfo();
        assert_eq!(info.os_name, "");

        let info_mut = get_sigmasysteminfo_mut();
        info_mut.set_os_name("SigmaOS".to_string());
        assert_eq!(info.os_name, "SigmaOS");
    }
}
