// Linux-inspired Resource Monitoring
// Provides system resource monitoring (CPU, memory, disk, network)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// CPU statistics
#[derive(Debug, Clone, Copy)]
pub struct CpuStats {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
}

impl CpuStats {
    pub fn new() -> Self {
        Self {
            user: 0,
            nice: 0,
            system: 0,
            idle: 100,
            iowait: 0,
            irq: 0,
            softirq: 0,
            steal: 0,
            guest: 0,
        }
    }

    /// Calculate total CPU time
    pub fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq + self.steal + self.guest
    }

    /// Calculate CPU usage percentage (based on idle time)
    pub fn usage_percent(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            ((total - self.idle) as f64 / total as f64) * 100.0
        }
    }
}

impl Default for CpuStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

impl MemoryStats {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            free: total,
            available: total,
            buffers: 0,
            cached: 0,
            swap_total: 0,
            swap_free: 0,
        }
    }

    /// Calculate used memory
    pub fn used(&self) -> u64 {
        self.total - self.free
    }

    /// Calculate memory usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.used() as f64 / self.total as f64) * 100.0
        }
    }

    /// Calculate swap usage percentage
    pub fn swap_usage_percent(&self) -> f64 {
        if self.swap_total == 0 {
            0.0
        } else {
            ((self.swap_total - self.swap_free) as f64 / self.swap_total as f64) * 100.0
        }
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self::new(1024 * 1024 * 1024) // 1GB default
    }
}

/// Disk statistics
#[derive(Debug, Clone, Copy)]
pub struct DiskStats {
    pub reads_completed: u64,
    pub reads_merged: u64,
    pub sectors_read: u64,
    pub read_time_ms: u64,
    pub writes_completed: u64,
    pub writes_merged: u64,
    pub sectors_written: u64,
    pub write_time_ms: u64,
    pub io_in_progress: u64,
    pub io_time_ms: u64,
    pub weighted_io_time_ms: u64,
}

impl DiskStats {
    pub fn new() -> Self {
        Self {
            reads_completed: 0,
            reads_merged: 0,
            sectors_read: 0,
            read_time_ms: 0,
            writes_completed: 0,
            writes_merged: 0,
            sectors_written: 0,
            write_time_ms: 0,
            io_in_progress: 0,
            io_time_ms: 0,
            weighted_io_time_ms: 0,
        }
    }

    /// Calculate total reads
    pub fn total_reads(&self) -> u64 {
        self.reads_completed
    }

    /// Calculate total writes
    pub fn total_writes(&self) -> u64 {
        self.writes_completed
    }

    /// Calculate total IO operations
    pub fn total_io(&self) -> u64 {
        self.reads_completed + self.writes_completed
    }
}

impl Default for DiskStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Network statistics
#[derive(Debug, Clone, Copy)]
pub struct NetworkStats {
    pub bytes_received: u64,
    pub packets_received: u64,
    pub errs_in: u64,
    pub drops_in: u64,
    pub bytes_sent: u64,
    pub packets_sent: u64,
    pub errs_out: u64,
    pub drops_out: u64,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            bytes_received: 0,
            packets_received: 0,
            errs_in: 0,
            drops_in: 0,
            bytes_sent: 0,
            packets_sent: 0,
            errs_out: 0,
            drops_out: 0,
        }
    }

    /// Calculate total bytes
    pub fn total_bytes(&self) -> u64 {
        self.bytes_received + self.bytes_sent
    }

    /// Calculate total packets
    pub fn total_packets(&self) -> u64 {
        self.packets_received + self.packets_sent
    }

    /// Calculate total errors
    pub fn total_errors(&self) -> u64 {
        self.errs_in + self.errs_out
    }

    /// Calculate total drops
    pub fn total_drops(&self) -> u64 {
        self.drops_in + self.drops_out
    }
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource monitor for system-wide resource monitoring
pub struct ResourceMonitor {
    cpu_stats: Arc<Mutex<CpuStats>>,
    memory_stats: Arc<Mutex<MemoryStats>>,
    disk_stats: Arc<Mutex<HashMap<String, DiskStats>>>,
    network_stats: Arc<Mutex<HashMap<String, NetworkStats>>>,
}

impl ResourceMonitor {
    pub fn new() -> Self {
        Self {
            cpu_stats: Arc::new(Mutex::new(CpuStats::new())),
            memory_stats: Arc::new(Mutex::new(MemoryStats::new(1024 * 1024 * 1024))),
            disk_stats: Arc::new(Mutex::new(HashMap::new())),
            network_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get CPU statistics
    pub fn get_cpu_stats(&self) -> CpuStats {
        let stats = self.cpu_stats.lock().unwrap();
        *stats
    }

    /// Update CPU statistics
    pub fn update_cpu_stats(&self, stats: CpuStats) {
        let mut cpu_stats = self.cpu_stats.lock().unwrap();
        *cpu_stats = stats;
    }

    /// Get memory statistics
    pub fn get_memory_stats(&self) -> MemoryStats {
        let stats = self.memory_stats.lock().unwrap();
        *stats
    }

    /// Update memory statistics
    pub fn update_memory_stats(&self, stats: MemoryStats) {
        let mut memory_stats = self.memory_stats.lock().unwrap();
        *memory_stats = stats;
    }

    /// Get disk statistics for a device
    pub fn get_disk_stats(&self, device: &str) -> Option<DiskStats> {
        let stats = self.disk_stats.lock().unwrap();
        stats.get(device).copied()
    }

    /// Update disk statistics for a device
    pub fn update_disk_stats(&self, device: String, stats: DiskStats) {
        let mut disk_stats = self.disk_stats.lock().unwrap();
        disk_stats.insert(device, stats);
    }

    /// Get all disk statistics
    pub fn get_all_disk_stats(&self) -> HashMap<String, DiskStats> {
        let stats = self.disk_stats.lock().unwrap();
        stats.clone()
    }

    /// Get network statistics for an interface
    pub fn get_network_stats(&self, interface: &str) -> Option<NetworkStats> {
        let stats = self.network_stats.lock().unwrap();
        stats.get(interface).copied()
    }

    /// Update network statistics for an interface
    pub fn update_network_stats(&self, interface: String, stats: NetworkStats) {
        let mut network_stats = self.network_stats.lock().unwrap();
        network_stats.insert(interface, stats);
    }

    /// Get all network statistics
    pub fn get_all_network_stats(&self) -> HashMap<String, NetworkStats> {
        let stats = self.network_stats.lock().unwrap();
        stats.clone()
    }
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_stats() {
        let stats = CpuStats::new();
        assert_eq!(stats.total(), 100);
        assert_eq!(stats.usage_percent(), 0.0);
    }

    #[test]
    fn test_cpu_stats_usage() {
        let mut stats = CpuStats::new();
        stats.user = 30;
        stats.system = 20;
        stats.idle = 50;

        assert_eq!(stats.total(), 100);
        assert_eq!(stats.usage_percent(), 50.0);
    }

    #[test]
    fn test_memory_stats() {
        let stats = MemoryStats::new(1024 * 1024 * 1024);
        assert_eq!(stats.total, 1024 * 1024 * 1024);
        assert_eq!(stats.used(), 0);
        assert_eq!(stats.usage_percent(), 0.0);
    }

    #[test]
    fn test_memory_stats_usage() {
        let mut stats = MemoryStats::new(1024 * 1024 * 1024);
        stats.free = 512 * 1024 * 1024;

        assert_eq!(stats.used(), 512 * 1024 * 1024);
        assert_eq!(stats.usage_percent(), 50.0);
    }

    #[test]
    fn test_disk_stats() {
        let stats = DiskStats::new();
        assert_eq!(stats.total_reads(), 0);
        assert_eq!(stats.total_writes(), 0);
        assert_eq!(stats.total_io(), 0);
    }

    #[test]
    fn test_disk_stats_with_data() {
        let mut stats = DiskStats::new();
        stats.reads_completed = 100;
        stats.writes_completed = 50;

        assert_eq!(stats.total_reads(), 100);
        assert_eq!(stats.total_writes(), 50);
        assert_eq!(stats.total_io(), 150);
    }

    #[test]
    fn test_network_stats() {
        let stats = NetworkStats::new();
        assert_eq!(stats.total_bytes(), 0);
        assert_eq!(stats.total_packets(), 0);
        assert_eq!(stats.total_errors(), 0);
    }

    #[test]
    fn test_network_stats_with_data() {
        let mut stats = NetworkStats::new();
        stats.bytes_received = 1000;
        stats.bytes_sent = 500;
        stats.errs_in = 1;
        stats.errs_out = 2;

        assert_eq!(stats.total_bytes(), 1500);
        assert_eq!(stats.total_errors(), 3);
    }

    #[test]
    fn test_resource_monitor() {
        let monitor = ResourceMonitor::new();

        let cpu_stats = monitor.get_cpu_stats();
        assert_eq!(cpu_stats.total(), 100);

        let mem_stats = monitor.get_memory_stats();
        assert_eq!(mem_stats.total, 1024 * 1024 * 1024);
    }

    #[test]
    fn test_resource_monitor_update() {
        let monitor = ResourceMonitor::new();

        let mut cpu_stats = CpuStats::new();
        cpu_stats.user = 50;
        cpu_stats.idle = 50;
        monitor.update_cpu_stats(cpu_stats);

        let current = monitor.get_cpu_stats();
        assert_eq!(current.usage_percent(), 50.0);
    }

    #[test]
    fn test_resource_monitor_disk() {
        let monitor = ResourceMonitor::new();

        let mut disk_stats = DiskStats::new();
        disk_stats.reads_completed = 100;
        monitor.update_disk_stats("sda".to_string(), disk_stats);

        let retrieved = monitor.get_disk_stats("sda").unwrap();
        assert_eq!(retrieved.total_reads(), 100);
    }

    #[test]
    fn test_resource_monitor_network() {
        let monitor = ResourceMonitor::new();

        let mut net_stats = NetworkStats::new();
        net_stats.bytes_received = 1000;
        monitor.update_network_stats("eth0".to_string(), net_stats);

        let retrieved = monitor.get_network_stats("eth0").unwrap();
        assert_eq!(retrieved.bytes_received, 1000);
    }
}
