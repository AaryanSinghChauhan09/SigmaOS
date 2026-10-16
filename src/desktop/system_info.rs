// SPDX-License-Identifier: MIT
// SigmaOS - System Information
// Inspired by neofetch and fastfetch
// Provides system information display

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// CPU information
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub threads: u32,
    pub frequency: u32,
    pub architecture: String,
}

/// Memory information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

/// Disk information
#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub device: String,
    pub mount_point: String,
    pub file_system: String,
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

/// GPU information
#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub model: String,
    pub driver: String,
    pub memory: u64,
}

/// Network information
#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub ip_address: String,
    pub mac_address: String,
    pub status: String,
}

/// System information
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub uptime: u64,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub gpus: Vec<GpuInfo>,
    pub networks: Vec<NetworkInfo>,
    pub shell: String,
    pub terminal: String,
    pub resolution: String,
    pub theme: String,
    pub icons: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            os_name: "SigmaOS".to_string(),
            os_version: "1.0.0".to_string(),
            kernel_version: "1.0.0".to_string(),
            hostname: "sigmaos".to_string(),
            uptime: 0,
            cpu: CpuInfo {
                model: "Unknown CPU".to_string(),
                cores: 4,
                threads: 8,
                frequency: 3000,
                architecture: "x86_64".to_string(),
            },
            memory: MemoryInfo {
                total: 16 * 1024 * 1024 * 1024,
                used: 8 * 1024 * 1024 * 1024,
                free: 8 * 1024 * 1024 * 1024,
                available: 8 * 1024 * 1024 * 1024,
                swap_total: 4 * 1024 * 1024 * 1024,
                swap_used: 0,
            },
            disks: Vec::new(),
            gpus: Vec::new(),
            networks: Vec::new(),
            shell: "/bin/sh".to_string(),
            terminal: "sigma-terminal".to_string(),
            resolution: "1920x1080".to_string(),
            theme: "Adwaita".to_string(),
            icons: "Adwaita".to_string(),
        }
    }

    /// Set OS name
    pub fn set_os_name(&mut self, name: String) {
        self.os_name = name;
    }

    /// Set OS version
    pub fn set_os_version(&mut self, version: String) {
        self.os_version = version;
    }

    /// Set kernel version
    pub fn set_kernel_version(&mut self, version: String) {
        self.kernel_version = version;
    }

    /// Set hostname
    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = hostname;
    }

    /// Set uptime
    pub fn set_uptime(&mut self, uptime: u64) {
        self.uptime = uptime;
    }

    /// Set CPU info
    pub fn set_cpu(&mut self, cpu: CpuInfo) {
        self.cpu = cpu;
    }

    /// Set memory info
    pub fn set_memory(&mut self, memory: MemoryInfo) {
        self.memory = memory;
    }

    /// Add disk
    pub fn add_disk(&mut self, disk: DiskInfo) {
        self.disks.push(disk);
    }

    /// Add GPU
    pub fn add_gpu(&mut self, gpu: GpuInfo) {
        self.gpus.push(gpu);
    }

    /// Add network
    pub fn add_network(&mut self, network: NetworkInfo) {
        self.networks.push(network);
    }

    /// Set shell
    pub fn set_shell(&mut self, shell: String) {
        self.shell = shell;
    }

    /// Set terminal
    pub fn set_terminal(&mut self, terminal: String) {
        self.terminal = terminal;
    }

    /// Set resolution
    pub fn set_resolution(&mut self, resolution: String) {
        self.resolution = resolution;
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: String) {
        self.theme = theme;
    }

    /// Set icons
    pub fn set_icons(&mut self, icons: String) {
        self.icons = icons;
    }

    /// Format uptime
    pub fn format_uptime(&self) -> String {
        let days = self.uptime / 86400;
        let hours = (self.uptime % 86400) / 3600;
        let minutes = (self.uptime % 3600) / 60;
        format!("{}d {}h {}m", days, hours, minutes)
    }

    /// Format memory
    pub fn format_memory(&self) -> String {
        let used_gb = self.memory.used / (1024 * 1024 * 1024);
        let total_gb = self.memory.total / (1024 * 1024 * 1024);
        format!("{}GiB / {}GiB", used_gb, total_gb)
    }

    /// Display as ASCII art
    pub fn display_ascii(&self) -> String {
        format!(
            r#"
   ____  _____  _____  
  / __ \|  ___||  __ \ 
 | |  | | |_   | |  | |
 | |  | |  _|  | |  | |
 | |__| | |___ | |__| |
  \____/|_____||_____/
  
  OS: {} {}
  Host: {}
  Kernel: {}
  Uptime: {}
  Shell: {}
  Terminal: {}
  CPU: {} ({} cores) @ {}MHz
  Memory: {}
  Resolution: {}
  Theme: {}
  Icons: {}
"#,
            self.os_name,
            self.os_version,
            self.hostname,
            self.kernel_version,
            self.format_uptime(),
            self.shell,
            self.terminal,
            self.cpu.model,
            self.cpu.cores,
            self.cpu.frequency,
            self.format_memory(),
            self.resolution,
            self.theme,
            self.icons
        )
    }
}

/// Global system info
static mut SYSTEM_INFO: Option<SystemInfo> = None;

/// Initialize system info
pub fn init_system_info() {
    unsafe {
        SYSTEM_INFO = Some(SystemInfo::new());
    }
}

/// Get system info
pub fn get_system_info() -> &'static mut SystemInfo {
    unsafe { SYSTEM_INFO.as_mut().expect("System info not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_creation() {
        let si = SystemInfo::new();
        assert_eq!(si.os_name, "SigmaOS");
    }

    #[test]
    fn test_format_uptime() {
        let mut si = SystemInfo::new();
        si.set_uptime(90061);
        let formatted = si.format_uptime();
        assert!(formatted.contains("1d"));
    }

    #[test]
    fn test_format_memory() {
        let si = SystemInfo::new();
        let formatted = si.format_memory();
        assert!(formatted.contains("GiB"));
    }
}
