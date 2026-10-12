// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaMonitor - System Monitor Daemon
//!
//! SigmaMonitor provides system monitoring for SigmaOS, inspired by htop/glances from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// CPU core statistics
#[derive(Debug, Clone)]
pub struct CpuCoreStats {
    /// Core ID
    pub core_id: u32,
    /// User time percentage
    pub user_percent: f32,
    /// System time percentage
    pub system_percent: f32,
    /// Idle time percentage
    pub idle_percent: f32,
}

impl CpuCoreStats {
    /// Create new CPU core stats
    pub fn new(core_id: u32) -> Self {
        Self {
            core_id,
            user_percent: 0.0,
            system_percent: 0.0,
            idle_percent: 100.0,
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total memory in MB
    pub total_mb: u64,
    /// Used memory in MB
    pub used_mb: u64,
    /// Free memory in MB
    pub free_mb: u64,
    /// Available memory in MB
    pub available_mb: u64,
    /// Used percentage
    pub used_percent: f32,
}

impl MemoryStats {
    /// Create new memory stats
    pub fn new(total_mb: u64) -> Self {
        Self {
            total_mb,
            used_mb: 0,
            free_mb: total_mb,
            available_mb: total_mb,
            used_percent: 0.0,
        }
    }
}

/// Disk statistics
#[derive(Debug, Clone)]
pub struct DiskStats {
    /// Device name
    pub device: String,
    /// Total space in GB
    pub total_gb: u64,
    /// Used space in GB
    pub used_gb: u64,
    /// Free space in GB
    pub free_gb: u64,
    /// Used percentage
    pub used_percent: f32,
}

impl DiskStats {
    /// Create new disk stats
    pub fn new(device: String, total_gb: u64) -> Self {
        Self {
            device,
            total_gb,
            used_gb: 0,
            free_gb: total_gb,
            used_percent: 0.0,
        }
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    /// Interface name
    pub interface: String,
    /// Bytes received
    pub bytes_rx: u64,
    /// Bytes transmitted
    pub bytes_tx: u64,
    /// Packets received
    pub packets_rx: u64,
    /// Packets transmitted
    pub packets_tx: u64,
}

impl NetworkStats {
    /// Create new network stats
    pub fn new(interface: String) -> Self {
        Self {
            interface,
            bytes_rx: 0,
            bytes_tx: 0,
            packets_rx: 0,
            packets_tx: 0,
        }
    }
}

/// System statistics
#[derive(Debug, Clone)]
pub struct SystemStats {
    /// CPU core statistics
    pub cpu_cores: Vec<CpuCoreStats>,
    /// Memory statistics
    pub memory: MemoryStats,
    /// Disk statistics
    pub disks: Vec<DiskStats>,
    /// Network statistics
    pub networks: Vec<NetworkStats>,
    /// Load average (1, 5, 15 minutes)
    pub load_avg: (f32, f32, f32),
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl SystemStats {
    /// Create new system stats
    pub fn new() -> Self {
        Self {
            cpu_cores: Vec::new(),
            memory: MemoryStats::new(0),
            disks: Vec::new(),
            networks: Vec::new(),
            load_avg: (0.0, 0.0, 0.0),
            uptime_seconds: 0,
        }
    }
}

/// SigmaMonitor - System Monitor Daemon
pub struct SigmaMonitor {
    /// Current system statistics
    pub stats: SystemStats,
    /// Running state
    running: bool,
    /// Update interval in seconds
    update_interval: u32,
}

impl SigmaMonitor {
    /// Create a new SigmaMonitor instance
    pub fn new() -> Self {
        Self {
            stats: SystemStats::new(),
            running: false,
            update_interval: 5,
        }
    }

    /// Start the monitor daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the monitor daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Set update interval
    pub fn set_update_interval(&mut self, interval: u32) {
        self.update_interval = interval;
    }

    /// Get update interval
    pub fn get_update_interval(&self) -> u32 {
        self.update_interval
    }

    /// Update system statistics
    pub fn update_stats(&mut self) {
        // In a real implementation, this would gather actual system statistics
        // For now, we just update the timestamp
        self.stats.uptime_seconds += self.update_interval as u64;
    }

    /// Get CPU usage
    pub fn get_cpu_usage(&self) -> f32 {
        if self.stats.cpu_cores.is_empty() {
            return 0.0;
        }
        
        let total_user: f32 = self.stats.cpu_cores.iter().map(|c| c.user_percent).sum();
        let total_system: f32 = self.stats.cpu_cores.iter().map(|c| c.system_percent).sum();
        (total_user + total_system) / self.stats.cpu_cores.len() as f32
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> f32 {
        self.stats.memory.used_percent
    }

    /// Get disk usage
    pub fn get_disk_usage(&self) -> f32 {
        if self.stats.disks.is_empty() {
            return 0.0;
        }
        
        let total_used: f32 = self.stats.disks.iter().map(|d| d.used_percent).sum();
        total_used / self.stats.disks.len() as f32
    }

    /// Get load average
    pub fn get_load_avg(&self) -> (f32, f32, f32) {
        self.stats.load_avg
    }

    /// Get uptime
    pub fn get_uptime(&self) -> u64 {
        self.stats.uptime_seconds
    }

    /// Add CPU core
    pub fn add_cpu_core(&mut self, core: CpuCoreStats) {
        self.stats.cpu_cores.push(core);
    }

    /// Add disk
    pub fn add_disk(&mut self, disk: DiskStats) {
        self.stats.disks.push(disk);
    }

    /// Add network interface
    pub fn add_network(&mut self, network: NetworkStats) {
        self.stats.networks.push(network);
    }

    /// Set memory stats
    pub fn set_memory(&mut self, memory: MemoryStats) {
        self.stats.memory = memory;
    }

    /// Set load average
    pub fn set_load_avg(&mut self, load_avg: (f32, f32, f32)) {
        self.stats.load_avg = load_avg;
    }

    /// Get monitor statistics
    pub fn monitor_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("cpu_cores".to_string(), self.stats.cpu_cores.len().to_string());
        stats.insert("cpu_usage".to_string(), format!("{:.1}", self.get_cpu_usage()));
        stats.insert("memory_usage".to_string(), format!("{:.1}", self.get_memory_usage()));
        stats.insert("disk_usage".to_string(), format!("{:.1}", self.get_disk_usage()));
        stats.insert("load_1min".to_string(), format!("{:.2}", self.stats.load_avg.0));
        stats.insert("load_5min".to_string(), format!("{:.2}", self.stats.load_avg.1));
        stats.insert("load_15min".to_string(), format!("{:.2}", self.stats.load_avg.2));
        stats.insert("uptime_seconds".to_string(), self.stats.uptime_seconds.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaMonitor instance (placeholder for actual global state)
static unsafe mut SIGMAMONITOR_INSTANCE: Option<SigmaMonitor> = None;

/// Initialize the global SigmaMonitor instance
pub fn init_sigmamonitor() {
    unsafe {
        SIGMAMONITOR_INSTANCE = Some(SigmaMonitor::new());
    }
}

/// Get the global SigmaMonitor instance
pub fn get_sigmamonitor() -> Option<&'static mut SigmaMonitor> {
    unsafe { SIGMAMONITOR_INSTANCE.as_mut() }
}

/// Convenience function to update statistics
pub fn update_system_stats() -> Result<(), String> {
    if let Some(monitor) = get_sigmamonitor() {
        monitor.update_stats();
        Ok(())
    } else {
        Err("SigmaMonitor not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_core_stats() {
        let core = CpuCoreStats::new(0);
        assert_eq!(core.core_id, 0);
        assert_eq!(core.idle_percent, 100.0);
    }

    #[test]
    fn test_memory_stats() {
        let memory = MemoryStats::new(8192);
        assert_eq!(memory.total_mb, 8192);
        assert_eq!(memory.free_mb, 8192);
    }

    #[test]
    fn test_disk_stats() {
        let disk = DiskStats::new("/dev/sda1".to_string(), 500);
        assert_eq!(disk.device, "/dev/sda1");
        assert_eq!(disk.total_gb, 500);
    }

    #[test]
    fn test_network_stats() {
        let network = NetworkStats::new("eth0".to_string());
        assert_eq!(network.interface, "eth0");
        assert_eq!(network.bytes_rx, 0);
    }

    #[test]
    fn test_sigmamonitor() {
        let mut monitor = SigmaMonitor::new();
        assert!(!monitor.is_running());
        monitor.start();
        assert!(monitor.is_running());
        monitor.stop();
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_sigmamonitor_set_update_interval() {
        let mut monitor = SigmaMonitor::new();
        monitor.set_update_interval(10);
        assert_eq!(monitor.get_update_interval(), 10);
    }

    #[test]
    fn test_sigmamonitor_add_cpu_core() {
        let mut monitor = SigmaMonitor::new();
        let core = CpuCoreStats::new(0);
        monitor.add_cpu_core(core);
        assert_eq!(monitor.stats.cpu_cores.len(), 1);
    }

    #[test]
    fn test_sigmamonitor_add_disk() {
        let mut monitor = SigmaMonitor::new();
        let disk = DiskStats::new("/dev/sda1".to_string(), 500);
        monitor.add_disk(disk);
        assert_eq!(monitor.stats.disks.len(), 1);
    }

    #[test]
    fn test_sigmamonitor_add_network() {
        let mut monitor = SigmaMonitor::new();
        let network = NetworkStats::new("eth0".to_string());
        monitor.add_network(network);
        assert_eq!(monitor.stats.networks.len(), 1);
    }

    #[test]
    fn test_sigmamonitor_set_memory() {
        let mut monitor = SigmaMonitor::new();
        let memory = MemoryStats::new(8192);
        monitor.set_memory(memory);
        assert_eq!(monitor.stats.memory.total_mb, 8192);
    }

    #[test]
    fn test_sigmamonitor_set_load_avg() {
        let mut monitor = SigmaMonitor::new();
        monitor.set_load_avg((1.5, 1.2, 1.0));
        assert_eq!(monitor.stats.load_avg, (1.5, 1.2, 1.0));
    }

    #[test]
    fn test_sigmamonitor_update_stats() {
        let mut monitor = SigmaMonitor::new();
        monitor.set_update_interval(5);
        monitor.update_stats();
        assert_eq!(monitor.stats.uptime_seconds, 5);
    }

    #[test]
    fn test_sigmamonitor_stats() {
        let mut monitor = SigmaMonitor::new();
        monitor.start();
        
        let core = CpuCoreStats::new(0);
        monitor.add_cpu_core(core);
        
        let stats = monitor.monitor_stats();
        assert_eq!(stats.get("cpu_cores"), Some(&"1".to_string()));
        assert_eq!(stats.get("running"), Some(&"true".to_string()));
    }

    #[test]
    fn test_update_system_stats_convenience() {
        init_sigmamonitor();
        
        let mut monitor = get_sigmamonitor().unwrap();
        monitor.set_update_interval(5);
        
        let result = update_system_stats();
        assert!(result.is_ok());
    }
}
