//! System Monitor
//!
//! System monitoring inspired by Linux Mint's system monitor and Omarchy's
//! system utilities, supporting CPU, memory, disk, and network monitoring.

use std::collections::HashMap;

/// CPU core
#[derive(Debug, Clone)]
pub struct CpuCore {
    pub id: u32,
    pub usage_percent: f32,
    pub frequency_mhz: u32,
}

impl CpuCore {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            usage_percent: 0.0,
            frequency_mhz: 0,
        }
    }

    pub fn set_usage(&mut self, usage: f32) {
        self.usage_percent = usage;
    }

    pub fn set_frequency(&mut self, freq: u32) {
        self.frequency_mhz = freq;
    }
}

/// Memory information
#[derive(Debug, Clone)]
pub struct RamMemoryInfo {
    pub total_mb: u64,
    pub used_mb: u64,
    pub free_mb: u64,
    pub available_mb: u64,
    pub swap_total_mb: u64,
    pub swap_used_mb: u64,
}

impl RamMemoryInfo {
    pub fn new() -> Self {
        Self {
            total_mb: 0,
            used_mb: 0,
            free_mb: 0,
            available_mb: 0,
            swap_total_mb: 0,
            swap_used_mb: 0,
        }
    }

    pub fn usage_percent(&self) -> f32 {
        if self.total_mb == 0 {
            0.0
        } else {
            (self.used_mb as f32 / self.total_mb as f32) * 100.0
        }
    }
}

impl Default for RamMemoryInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Disk partition
#[derive(Debug, Clone)]
pub struct DiskPartition {
    pub device: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_gb: u64,
    pub used_gb: u64,
    pub free_gb: u64,
}

impl DiskPartition {
    pub fn new(device: String, mount_point: String, file_system: String) -> Self {
        Self {
            device,
            mount_point,
            file_system,
            total_gb: 0,
            used_gb: 0,
            free_gb: 0,
        }
    }

    pub fn usage_percent(&self) -> f32 {
        if self.total_gb == 0 {
            0.0
        } else {
            (self.used_gb as f32 / self.total_gb as f32) * 100.0
        }
    }
}

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub packets_sent: u64,
    pub packets_recv: u64,
    pub is_up: bool,
}

impl NetworkInterface {
    pub fn new(name: String) -> Self {
        Self {
            name,
            bytes_sent: 0,
            bytes_recv: 0,
            packets_sent: 0,
            packets_recv: 0,
            is_up: false,
        }
    }

    pub fn set_up(&mut self, up: bool) {
        self.is_up = up;
    }
}

/// System monitor
#[derive(Debug)]
pub struct SystemMonitor {
    cpu_cores: Vec<CpuCore>,
    memory: RamMemoryInfo,
    partitions: HashMap<String, DiskPartition>,
    network_interfaces: HashMap<String, NetworkInterface>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut monitor = Self {
            cpu_cores: Vec::new(),
            memory: RamMemoryInfo::new(),
            partitions: HashMap::new(),
            network_interfaces: HashMap::new(),
        };

        // Initialize CPU cores (simulated 4 cores)
        for i in 0..4 {
            let mut core = CpuCore::new(i);
            core.set_frequency(2400); // 2.4 GHz
            monitor.cpu_cores.push(core);
        }

        // Initialize memory (simulated 8 GB)
        monitor.memory.total_mb = 8192;
        monitor.memory.used_mb = 4096;
        monitor.memory.free_mb = 4096;
        monitor.memory.available_mb = 4096;

        // Initialize partitions
        let mut root = DiskPartition::new(
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
        );
        root.total_gb = 100;
        root.used_gb = 50;
        root.free_gb = 50;
        monitor.partitions.insert("/dev/sda1".to_string(), root);

        // Initialize network interfaces
        let mut eth0 = NetworkInterface::new("eth0".to_string());
        eth0.set_up(true);
        monitor.network_interfaces.insert("eth0".to_string(), eth0);

        let mut wlan0 = NetworkInterface::new("wlan0".to_string());
        wlan0.set_up(true);
        monitor.network_interfaces.insert("wlan0".to_string(), wlan0);

        monitor
    }

    /// Get CPU cores
    pub fn get_cpu_cores(&self) -> &Vec<CpuCore> {
        &self.cpu_cores
    }

    /// Get CPU usage (average)
    pub fn get_cpu_usage(&self) -> f32 {
        if self.cpu_cores.is_empty() {
            0.0
        } else {
            let total: f32 = self.cpu_cores.iter().map(|c| c.usage_percent).sum();
            total / self.cpu_cores.len() as f32
        }
    }

    /// Get memory info
    pub fn get_memory(&self) -> &RamMemoryInfo {
        &self.memory
    }

    /// Set memory info
    pub fn set_memory(&mut self, memory: RamMemoryInfo) {
        self.memory = memory;
    }

    /// Add a partition
    pub fn add_partition(&mut self, partition: DiskPartition) {
        self.partitions.insert(partition.device.clone(), partition);
    }

    /// Get a partition
    pub fn get_partition(&self, device: &str) -> Option<&DiskPartition> {
        self.partitions.get(device)
    }

    /// List all partitions
    pub fn list_partitions(&self) -> Vec<&DiskPartition> {
        self.partitions.values().collect()
    }

    /// Add a network interface
    pub fn add_network_interface(&mut self, interface: NetworkInterface) {
        self.network_interfaces.insert(interface.name.clone(), interface);
    }

    /// Get a network interface
    pub fn get_network_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.network_interfaces.get(name)
    }

    /// List all network interfaces
    pub fn list_network_interfaces(&self) -> Vec<&NetworkInterface> {
        self.network_interfaces.values().collect()
    }

    /// Get total disk usage
    pub fn get_total_disk_usage(&self) -> (u64, u64) {
        let total: u64 = self.partitions.values().map(|p| p.total_gb).sum();
        let used: u64 = self.partitions.values().map(|p| p.used_gb).sum();
        (total, used)
    }

    /// Get total network traffic
    pub fn get_total_network_traffic(&self) -> (u64, u64) {
        let sent: u64 = self.network_interfaces.values().map(|n| n.bytes_sent).sum();
        let recv: u64 = self.network_interfaces.values().map(|n| n.bytes_recv).sum();
        (sent, recv)
    }

    /// Update simulated values
    pub fn update(&mut self) {
        // Simulate CPU usage changes
        for core in &mut self.cpu_cores {
            let usage = (core.id as f32 * 10.0 + 20.0) % 100.0;
            core.set_usage(usage);
        }

        // Simulate memory usage changes
        let used = (self.memory.total_mb as f32 * 0.5) as u64;
        self.memory.used_mb = used;
        self.memory.free_mb = self.memory.total_mb - used;
        self.memory.available_mb = self.memory.free_mb;

        // Simulate network traffic
        for interface in self.network_interfaces.values_mut() {
            interface.bytes_sent += 1024;
            interface.bytes_recv += 2048;
            interface.packets_sent += 10;
            interface.packets_recv += 20;
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SystemMonitorStatistics {
        let cpu_usage = self.get_cpu_usage();
        let memory_usage = self.memory.usage_percent();
        let (disk_total, disk_used) = self.get_total_disk_usage();
        let disk_usage = if disk_total == 0 { 0.0 } else { (disk_used as f32 / disk_total as f32) * 100.0 };
        let network_up = self.network_interfaces.values().filter(|n| n.is_up).count();

        SystemMonitorStatistics {
            cpu_cores: self.cpu_cores.len(),
            cpu_usage,
            memory_usage,
            disk_partitions: self.partitions.len(),
            disk_usage,
            network_interfaces: self.network_interfaces.len(),
            network_up,
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// System monitor statistics
#[derive(Debug, Clone)]
pub struct SystemMonitorStatistics {
    pub cpu_cores: usize,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_partitions: usize,
    pub disk_usage: f32,
    pub network_interfaces: usize,
    pub network_up: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_core_creation() {
        let core = CpuCore::new(0);
        assert_eq!(core.id, 0);
    }

    #[test]
    fn test_memory_info_creation() {
        let mem = RamMemoryInfo::new();
        assert_eq!(mem.total_mb, 0);
    }

    #[test]
    fn test_memory_usage_percent() {
        let mut mem = RamMemoryInfo::new();
        mem.total_mb = 1000;
        mem.used_mb = 500;
        assert_eq!(mem.usage_percent(), 50.0);
    }

    #[test]
    fn test_disk_partition_creation() {
        let part = DiskPartition::new(
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
        );
        assert_eq!(part.mount_point, "/");
    }

    #[test]
    fn test_disk_usage_percent() {
        let mut part = DiskPartition::new(
            "/dev/sda1".to_string(),
            "/".to_string(),
            "ext4".to_string(),
        );
        part.total_gb = 100;
        part.used_gb = 50;
        assert_eq!(part.usage_percent(), 50.0);
    }

    #[test]
    fn test_system_monitor_creation() {
        let monitor = SystemMonitor::new();
        assert_eq!(monitor.cpu_cores.len(), 4);
    }

    #[test]
    fn test_get_cpu_usage() {
        let monitor = SystemMonitor::new();
        let usage = monitor.get_cpu_usage();
        assert!(usage >= 0.0 && usage <= 100.0);
    }

    #[test]
    fn test_add_partition() {
        let mut monitor = SystemMonitor::new();
        let part = DiskPartition::new(
            "/dev/sda2".to_string(),
            "/home".to_string(),
            "ext4".to_string(),
        );
        monitor.add_partition(part);
        assert!(monitor.get_partition("/dev/sda2").is_some());
    }

    #[test]
    fn test_add_network_interface() {
        let mut monitor = SystemMonitor::new();
        let iface = NetworkInterface::new("eth1".to_string());
        monitor.add_network_interface(iface);
        assert!(monitor.get_network_interface("eth1").is_some());
    }

    #[test]
    fn test_update() {
        let mut monitor = SystemMonitor::new();
        monitor.update();
        let stats = monitor.get_statistics();
        assert!(stats.cpu_usage > 0.0);
    }

    #[test]
    fn test_statistics() {
        let monitor = SystemMonitor::new();
        let stats = monitor.get_statistics();
        assert_eq!(stats.cpu_cores, 4);
    }
}
