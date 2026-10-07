//! System Manager
//!
//! Comprehensive system management inspired by Omarchy's system tools,
//! including system info, resource monitoring, and health checks.

use std::collections::HashMap;

/// System health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemHealth {
    Healthy,
    Warning,
    Critical,
}

impl SystemHealth {
    pub fn from_load(load: f64) -> Self {
        if load < 1.0 {
            SystemHealth::Healthy
        } else if load < 2.0 {
            SystemHealth::Warning
        } else {
            SystemHealth::Critical
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SystemHealth::Healthy => "Healthy",
            SystemHealth::Warning => "Warning",
            SystemHealth::Critical => "Critical",
        }
    }
}

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub kernel_version: String,
    pub os_version: String,
    pub architecture: String,
    pub uptime_seconds: u64,
    pub boot_time: u64,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            hostname: "sigmaos".to_string(),
            kernel_version: "6.6.0-sigma".to_string(),
            os_version: "SigmaOS 1.0".to_string(),
            architecture: "x86_64".to_string(),
            uptime_seconds: 0,
            boot_time: 0,
        }
    }

    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = hostname;
    }

    pub fn set_uptime(&mut self, uptime: u64) {
        self.uptime_seconds = uptime;
    }

    pub fn uptime_human(&self) -> String {
        let hours = self.uptime_seconds / 3600;
        let minutes = (self.uptime_seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// CPU information
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub threads: u32,
    pub frequency_mhz: u32,
    pub usage_percent: f64,
    pub temperature_celsius: f64,
}

impl CpuInfo {
    pub fn new() -> Self {
        Self {
            model: "Unknown CPU".to_string(),
            cores: 4,
            threads: 8,
            frequency_mhz: 2400,
            usage_percent: 0.0,
            temperature_celsius: 45.0,
        }
    }

    pub fn set_usage(&mut self, usage: f64) {
        self.usage_percent = usage.clamp(0.0, 100.0);
    }

    pub fn set_temperature(&mut self, temp: f64) {
        self.temperature_celsius = temp;
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub cached_bytes: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

impl MemoryInfo {
    pub fn new() -> Self {
        Self {
            total_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            available_bytes: 8 * 1024 * 1024 * 1024,
            used_bytes: 0,
            cached_bytes: 0,
            swap_total: 0,
            swap_used: 0,
        }
    }

    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
    }

    pub fn used_human(&self) -> String {
        Self::bytes_to_human(self.used_bytes)
    }

    pub fn total_human(&self) -> String {
        Self::bytes_to_human(self.total_bytes)
    }

    fn bytes_to_human(bytes: u64) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;

        if bytes >= GB {
            format!("{:.1} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else {
            format!("{} KB", bytes / 1024)
        }
    }
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Disk information
#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub mount_point: String,
    pub device: String,
    pub fs_type: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
}

impl DiskInfo {
    pub fn new(mount_point: String) -> Self {
        Self {
            mount_point,
            device: "/dev/sda1".to_string(),
            fs_type: "ext4".to_string(),
            total_bytes: 500 * 1024 * 1024 * 1024, // 500 GB
            used_bytes: 0,
            available_bytes: 500 * 1024 * 1024 * 1024,
        }
    }

    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
    }

    pub fn used_human(&self) -> String {
        MemoryInfo::bytes_to_human(self.used_bytes)
    }

    pub fn total_human(&self) -> String {
        MemoryInfo::bytes_to_human(self.total_bytes)
    }
}

/// Network information
#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub ip_address: String,
    pub mac_address: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub is_up: bool,
}

impl NetworkInfo {
    pub fn new(interface: String) -> Self {
        Self {
            interface,
            ip_address: "192.168.1.100".to_string(),
            mac_address: "00:00:00:00:00:00".to_string(),
            bytes_sent: 0,
            bytes_received: 0,
            is_up: true,
        }
    }

    pub fn update_traffic(&mut self, sent: u64, received: u64) {
        self.bytes_sent += sent;
        self.bytes_received += received;
    }
}

/// System manager
#[derive(Debug)]
pub struct SystemManager {
    system_info: SystemInfo,
    cpu_info: CpuInfo,
    memory_info: MemoryInfo,
    disks: HashMap<String, DiskInfo>,
    networks: HashMap<String, NetworkInfo>,
    health: SystemHealth,
}

impl SystemManager {
    pub fn new() -> Self {
        let mut manager = Self {
            system_info: SystemInfo::new(),
            cpu_info: CpuInfo::new(),
            memory_info: MemoryInfo::new(),
            disks: HashMap::new(),
            networks: HashMap::new(),
            health: SystemHealth::Healthy,
        };

        // Add default disk
        manager.add_disk(DiskInfo::new("/".to_string()));

        // Add default network
        manager.add_network(NetworkInfo::new("eth0".to_string()));

        manager
    }

    /// Get system information
    pub fn get_system_info(&self) -> &SystemInfo {
        &self.system_info
    }

    /// Get CPU information
    pub fn get_cpu_info(&self) -> &CpuInfo {
        &self.cpu_info
    }

    /// Get memory information
    pub fn get_memory_info(&self) -> &MemoryInfo {
        &self.memory_info
    }

    /// Get disk information
    pub fn get_disk(&self, mount_point: &str) -> Option<&DiskInfo> {
        self.disks.get(mount_point)
    }

    /// Get all disks
    pub fn get_disks(&self) -> Vec<&DiskInfo> {
        self.disks.values().collect()
    }

    /// Add disk
    pub fn add_disk(&mut self, disk: DiskInfo) {
        self.disks.insert(disk.mount_point.clone(), disk);
    }

    /// Get network information
    pub fn get_network(&self, interface: &str) -> Option<&NetworkInfo> {
        self.networks.get(interface)
    }

    /// Get all networks
    pub fn get_networks(&self) -> Vec<&NetworkInfo> {
        self.networks.values().collect()
    }

    /// Add network
    pub fn add_network(&mut self, network: NetworkInfo) {
        self.networks.insert(network.interface.clone(), network);
    }

    /// Update CPU usage
    pub fn update_cpu_usage(&mut self, usage: f64) {
        self.cpu_info.set_usage(usage);
        self.update_health();
    }

    /// Update memory usage
    pub fn update_memory_usage(&mut self, used: u64) {
        self.memory_info.used_bytes = used;
        self.update_health();
    }

    /// Update disk usage
    pub fn update_disk_usage(&mut self, mount_point: &str, used: u64) {
        if let Some(disk) = self.disks.get_mut(mount_point) {
            disk.used_bytes = used;
        }
    }

    /// Get system health
    pub fn get_health(&self) -> SystemHealth {
        self.health
    }

    /// Update system health based on resource usage
    fn update_health(&mut self) {
        // CPU and memory report percentages, not the load averages accepted by
        // SystemHealth::from_load. A single saturated resource is enough to
        // degrade the system, even when the other resource is idle.
        let usage = self.cpu_info.usage_percent.max(self.memory_info.usage_percent());
        self.health = if usage >= 90.0 {
            SystemHealth::Critical
        } else if usage >= 70.0 {
            SystemHealth::Warning
        } else {
            SystemHealth::Healthy
        };
    }

    /// Get system summary
    pub fn get_summary(&self) -> SystemSummary {
        SystemSummary {
            hostname: self.system_info.hostname.clone(),
            uptime: self.system_info.uptime_human(),
            cpu_usage: self.cpu_info.usage_percent,
            memory_usage: self.memory_info.usage_percent(),
            disk_usage: self.disks.values()
                .map(|d| d.usage_percent())
                .sum::<f64>() / self.disks.len() as f64,
            health: self.health,
        }
    }
}

impl Default for SystemManager {
    fn default() -> Self {
        Self::new()
    }
}

/// System summary
#[derive(Debug, Clone)]
pub struct SystemSummary {
    pub hostname: String,
    pub uptime: String,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub health: SystemHealth,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_health_from_load() {
        assert_eq!(SystemHealth::from_load(0.5), SystemHealth::Healthy);
        assert_eq!(SystemHealth::from_load(1.5), SystemHealth::Warning);
        assert_eq!(SystemHealth::from_load(2.5), SystemHealth::Critical);
    }

    #[test]
    fn test_system_info_creation() {
        let info = SystemInfo::new();
        assert_eq!(info.hostname, "sigmaos");
    }

    #[test]
    fn test_memory_info_usage() {
        let mut mem = MemoryInfo::new();
        mem.total_bytes = 8 * 1024 * 1024 * 1024;
        mem.used_bytes = 4 * 1024 * 1024 * 1024;
        assert_eq!(mem.usage_percent(), 50.0);
    }

    #[test]
    fn test_disk_info_usage() {
        let mut disk = DiskInfo::new("/".to_string());
        disk.total_bytes = 500 * 1024 * 1024 * 1024;
        disk.used_bytes = 250 * 1024 * 1024 * 1024;
        assert_eq!(disk.usage_percent(), 50.0);
    }

    #[test]
    fn test_system_manager_creation() {
        let manager = SystemManager::new();
        assert_eq!(manager.get_disks().len(), 1);
        assert_eq!(manager.get_networks().len(), 1);
    }

    #[test]
    fn test_update_cpu_usage() {
        let mut manager = SystemManager::new();
        manager.update_cpu_usage(75.0);
        assert_eq!(manager.get_cpu_info().usage_percent, 75.0);
    }

    #[test]
    fn test_update_memory_usage() {
        let mut manager = SystemManager::new();
        manager.update_memory_usage(4 * 1024 * 1024 * 1024);
        assert_eq!(manager.get_memory_info().used_bytes, 4 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_system_health_update() {
        let mut manager = SystemManager::new();
        manager.update_cpu_usage(75.0);
        assert_eq!(manager.get_health(), SystemHealth::Warning);

        manager.update_cpu_usage(150.0); // Clamped to 100%.
        assert_eq!(manager.get_cpu_info().usage_percent, 100.0);
        assert_eq!(manager.get_health(), SystemHealth::Critical);

        manager.update_cpu_usage(0.0);
        assert_eq!(manager.get_health(), SystemHealth::Healthy);
        manager.update_memory_usage(8 * 1024 * 1024 * 1024);
        assert_eq!(manager.get_health(), SystemHealth::Critical);
        manager.update_memory_usage(0);
        assert_eq!(manager.get_health(), SystemHealth::Healthy);
    }

    #[test]
    fn test_system_summary() {
        let manager = SystemManager::new();
        let summary = manager.get_summary();
        assert_eq!(summary.hostname, "sigmaos");
    }
}
