// SPDX-License-Identifier: MIT
// SigmaOS - System Monitor
// Inspired by htop, btop, and gnome-system-monitor
// Provides system resource monitoring

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Stopped,
    Zombie,
;

/// Process
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub memory_bytes: u64,
    pub user: String,
    pub priority: i32,
    pub threads: u32,
    pub command: String,
;

/// CPU core info
#[derive(Debug, Clone)]
pub struct CPUCore {
    pub id: u32,
    pub usage_percent: f32,
    pub frequency_mhz: u32,
    pub temperature: Option<f32>,
;

/// Memory info
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub available: u64,
    pub cached: u64,
    pub buffers: u64,
    pub swap_total: u64,
    pub swap_used: u64,
;

/// Disk info
#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub device: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub read_bytes_per_sec: u64,
    pub write_bytes_per_sec: u64,
;

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub packets_sent: u64,
    pub packets_recv: u64,
    pub bytes_sent_per_sec: u64,
    pub bytes_recv_per_sec: u64,
;

/// System monitor
pub struct SystemMonitor {
    pub processes: Vec<Process>,
    pub cpu_cores: Vec<CPUCore>,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub network_interfaces: Vec<NetworkInterface>,
    pub uptime_seconds: u64,
    pub load_average: [f32; 3],
    sort_by: String,
    sort_descending: bool,
    filter: String,
;

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            cpu_cores: Vec::new(),
            memory: MemoryInfo {
                total: 0,
                used: 0,
                free: 0,
                available: 0,
                cached: 0,
                buffers: 0,
                swap_total: 0,
                swap_used: 0,
            },
            disks: Vec::new(),
            network_interfaces: Vec::new(),
            uptime_seconds: 0,
            load_average: [0.0, 0.0, 0.0],
            sort_by: "cpu".to_string(),
            sort_descending: true,
            filter: String::new(),
        }
    }

    /// Refresh all data
    pub fn refresh(&mut self) {
        self.refresh_processes();
        self.refresh_cpu();
        self.refresh_memory();
        self.refresh_disks();
        self.refresh_network();
        self.refresh_uptime();
        self.refresh_load_average();
    }

    /// Refresh processes
    fn refresh_processes(&mut self) {
        // Simplified - in real implementation would read from /proc
        self.processes.clear();
    }

    /// Refresh CPU
    fn refresh_cpu(&mut self) {
        // Simplified - in real implementation would read CPU stats
        self.cpu_cores.clear();
    }

    /// Refresh memory
    fn refresh_memory(&mut self) {
        // Simplified - in real implementation would read from /proc/meminfo
    }

    /// Refresh disks
    fn refresh_disks(&mut self) {
        // Simplified - in real implementation would read from /proc/mounts and statfs
    }

    /// Refresh network
    fn refresh_network(&mut self) {
        // Simplified - in real implementation would read from /proc/net/dev
    }

    /// Refresh uptime
    fn refresh_uptime(&mut self) {
        // Simplified - in real implementation would read from /proc/uptime
    }

    /// Refresh load average
    fn refresh_load_average(&mut self) {
        // Simplified - in real implementation would read from /proc/loadavg
    }

    /// Kill process
    pub fn kill_process(&mut self, pid: u32, signal: i32) -> Result<(), &'static str> {
        let process = self.processes.iter().find(|p| p.pid == pid);
        if process.is_none() {
            return Err("Process not found");
        }
        // Simplified - in real implementation would send signal
        Ok(())
    }

    /// Set sort by
    pub fn set_sort_by(&mut self, field: String) {
        self.sort_by = field;
        self.sort_processes();
    }

    /// Toggle sort direction
    pub fn toggle_sort_direction(&mut self) {
        self.sort_descending = !self.sort_descending;
        self.sort_processes();
    }

    /// Sort processes
    fn sort_processes(&mut self) {
        match self.sort_by.as_str() {
            "pid" => {
                self.processes.sort_by(|a, b| {
                    if self.sort_descending {
                        b.pid.cmp(&a.pid)
                    } else {
                        a.pid.cmp(&b.pid)
                    }
                });
            }
            "name" => {
                self.processes.sort_by(|a, b| {
                    if self.sort_descending {
                        b.name.cmp(&a.name)
                    } else {
                        a.name.cmp(&b.name)
                    }
                });
            }
            "cpu" => {
                self.processes.sort_by(|a, b| {
                    if self.sort_descending {
                        b.cpu_percent
                            .partial_cmp(&a.cpu_percent)
                            .unwrap_or(core::cmp::Ordering::Equal)
                    } else {
                        a.cpu_percent
                            .partial_cmp(&b.cpu_percent)
                            .unwrap_or(core::cmp::Ordering::Equal)
                    }
                });
            }
            "memory" => {
                self.processes.sort_by(|a, b| {
                    if self.sort_descending {
                        b.memory_percent
                            .partial_cmp(&a.memory_percent)
                            .unwrap_or(core::cmp::Ordering::Equal)
                    } else {
                        a.memory_percent
                            .partial_cmp(&b.memory_percent)
                            .unwrap_or(core::cmp::Ordering::Equal)
                    }
                });
            }
            "user" => {
                self.processes.sort_by(|a, b| {
                    if self.sort_descending {
                        b.user.cmp(&a.user)
                    } else {
                        a.user.cmp(&b.user)
                    }
                });
            }
            _ => {}
        }
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }

    /// Get filtered processes
    pub fn get_filtered_processes(&self) -> Vec<&Process> {
        if self.filter.is_empty() {
            self.processes.iter().collect()
        } else {
            self.processes
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&self.filter.to_lowercase())
                        || p.command
                            .to_lowercase()
                            .contains(&self.filter.to_lowercase())
                })
                .collect()
        }
    }

    /// Get CPU usage percent
    pub fn get_cpu_usage_percent(&self) -> f32 {
        if self.cpu_cores.is_empty() {
            0.0
        } else {
            self.cpu_cores.iter().map(|c| c.usage_percent).sum::<f32>()
                / self.cpu_cores.len() as f32
        }
    }

    /// Get memory usage percent
    pub fn get_memory_usage_percent(&self) -> f32 {
        if self.memory.total == 0 {
            0.0
        } else {
            (self.memory.used as f32 / self.memory.total as f32) * 100.0
        }
    }

    /// Get swap usage percent
    pub fn get_swap_usage_percent(&self) -> f32 {
        if self.memory.swap_total == 0 {
            0.0
        } else {
            (self.memory.swap_used as f32 / self.memory.swap_total as f32) * 100.0
        }
    }

    /// Get total network bytes sent
    pub fn get_total_bytes_sent(&self) -> u64 {
        self.network_interfaces.iter().map(|n| n.bytes_sent).sum()
    }

    /// Get total network bytes received
    pub fn get_total_bytes_recv(&self) -> u64 {
        self.network_interfaces.iter().map(|n| n.bytes_recv).sum()
    }

    /// Get process by PID
    pub fn get_process(&self, pid: u32) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get thread count
    pub fn thread_count(&self) -> u32 {
        self.processes.iter().map(|p| p.threads).sum()
    }
;

/// Global system monitor
static mut SYSTEM_MONITOR: Option<SystemMonitor> = None;

/// Initialize system monitor
pub fn init_system_monitor() {
    unsafe {
        SYSTEM_MONITOR = Some(SystemMonitor::new());
    }
;

/// Get system monitor
pub fn get_system_monitor() -> &'static mut SystemMonitor {
    unsafe {
        SYSTEM_MONITOR
            .as_mut()
            .expect("System monitor not initialized")
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_creation() {
        let sm = SystemMonitor::new();
        assert_eq!(sm.process_count(), 0);
    }

    #[test]
    fn test_refresh() {
        let mut sm = SystemMonitor::new();
        sm.refresh();
        // Should not panic
    }

    #[test]
    fn test_set_sort_by() {
        let mut sm = SystemMonitor::new();
        sm.set_sort_by("cpu".to_string());
        assert_eq!(sm.sort_by, "cpu");
    }

    #[test]
    fn test_toggle_sort_direction() {
        let mut sm = SystemMonitor::new();
        sm.toggle_sort_direction();
        assert!(!sm.sort_descending);
    }

    #[test]
    fn test_set_filter() {
        let mut sm = SystemMonitor::new();
        sm.set_filter("test".to_string());
        assert_eq!(sm.filter, "test");
    }

    #[test]
    fn test_get_cpu_usage_percent() {
        let sm = SystemMonitor::new();
        let usage = sm.get_cpu_usage_percent();
        assert!(usage >= 0.0 && usage <= 100.0);
    }

    #[test]
    fn test_get_memory_usage_percent() {
        let sm = SystemMonitor::new();
        let usage = sm.get_memory_usage_percent();
        assert!(usage >= 0.0 && usage <= 100.0);
    }
;
