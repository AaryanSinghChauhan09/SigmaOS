// SPDX-License-Identifier: MIT
// SigmaOS - System Monitor Framework
// Inspired by htop, iotop, nethogs, glances
// Provides system monitoring capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// CPU statistics
#[derive(Debug, Clone, Copy)]
pub struct CpuStats {
    pub user: f32,
    pub system: f32,
    pub idle: f32,
    pub iowait: f32,
;

/// Memory statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_used: u64,
;

/// Process statistics
#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub state: ProcessState,
;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Stopped,
    Zombie,
;

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub interface: String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub packets_sent: u64,
    pub packets_recv: u64,
;

/// Disk statistics
#[derive(Debug, Clone)]
pub struct DiskStats {
    pub device: String,
    pub reads: u64,
    pub writes: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
;

/// System monitor
pub struct SystemMonitor {
    pub cpu_stats: CpuStats,
    pub memory_stats: MemoryStats,
    pub processes: Vec<ProcessStats>,
    pub network_stats: BTreeMap<String, NetworkStats>,
    pub disk_stats: BTreeMap<String, DiskStats>,
    pub active: AtomicBool,
;

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            cpu_stats: CpuStats {
                user: 0.0,
                system: 0.0,
                idle: 100.0,
                iowait: 0.0,
            },
            memory_stats: MemoryStats {
                total: 0,
                used: 0,
                free: 0,
                cached: 0,
                swap_total: 0,
                swap_used: 0,
            },
            processes: Vec::new(),
            network_stats: BTreeMap::new(),
            disk_stats: BTreeMap::new(),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize system monitor
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Update CPU statistics
    pub fn update_cpu_stats(&mut self, user: f32, system: f32, idle: f32, iowait: f32) {
        self.cpu_stats = CpuStats {
            user,
            system,
            idle,
            iowait,
        };
    }

    /// Update memory statistics
    pub fn update_memory_stats(
        &mut self,
        total: u64,
        used: u64,
        free: u64,
        cached: u64,
        swap_total: u64,
        swap_used: u64,
    ) {
        self.memory_stats = MemoryStats {
            total,
            used,
            free,
            cached,
            swap_total,
            swap_used,
        };
    }

    /// Add process statistics
    pub fn add_process(
        &mut self,
        pid: u32,
        name: String,
        cpu_usage: f32,
        memory_usage: u64,
        state: ProcessState,
    ) {
        self.processes.push(ProcessStats {
            pid,
            name,
            cpu_usage,
            memory_usage,
            state,
        });
    }

    /// Clear processes
    pub fn clear_processes(&mut self) {
        self.processes.clear();
    }

    /// Update network statistics
    pub fn update_network_stats(
        &mut self,
        interface: String,
        bytes_sent: u64,
        bytes_recv: u64,
        packets_sent: u64,
        packets_recv: u64,
    ) {
        self.network_stats.insert(
            interface.clone(),
            NetworkStats {
                interface,
                bytes_sent,
                bytes_recv,
                packets_sent,
                packets_recv,
            },
        );
    }

    /// Update disk statistics
    pub fn update_disk_stats(
        &mut self,
        device: String,
        reads: u64,
        writes: u64,
        read_bytes: u64,
        write_bytes: u64,
    ) {
        self.disk_stats.insert(
            device.clone(),
            DiskStats {
                device,
                reads,
                writes,
                read_bytes,
                write_bytes,
            },
        );
    }

    /// Get CPU usage percentage
    pub fn get_cpu_usage(&self) -> f32 {
        100.0 - self.cpu_stats.idle
    }

    /// Get memory usage percentage
    pub fn get_memory_usage(&self) -> f32 {
        if self.memory_stats.total == 0 {
            0.0
        } else {
            (self.memory_stats.used as f32 / self.memory_stats.total as f32) * 100.0
        }
    }

    /// Get swap usage percentage
    pub fn get_swap_usage(&self) -> f32 {
        if self.memory_stats.swap_total == 0 {
            0.0
        } else {
            (self.memory_stats.swap_used as f32 / self.memory_stats.swap_total as f32) * 100.0
        }
    }

    /// Get top processes by CPU
    pub fn get_top_cpu_processes(&self, limit: usize) -> Vec<&ProcessStats> {
        let mut sorted = self.processes.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        sorted.into_iter().take(limit).collect()
    }

    /// Get top processes by memory
    pub fn get_top_memory_processes(&self, limit: usize) -> Vec<&ProcessStats> {
        let mut sorted = self.processes.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
        sorted.into_iter().take(limit).collect()
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
;

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
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

/// Get global system monitor
pub fn get_system_monitor() -> Option<&'static mut SystemMonitor> {
    unsafe { SYSTEM_MONITOR.as_mut() }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_init() {
        let mut monitor = SystemMonitor::new();
        monitor.init();
        assert!(monitor.is_active());
    }

    #[test]
    fn test_update_cpu_stats() {
        let mut monitor = SystemMonitor::new();
        monitor.update_cpu_stats(10.0, 5.0, 85.0, 0.0);
        assert_eq!(monitor.get_cpu_usage(), 15.0);
    }

    #[test]
    fn test_update_memory_stats() {
        let mut monitor = SystemMonitor::new();
        monitor.update_memory_stats(16000, 8000, 8000, 0, 8000, 4000);
        assert_eq!(monitor.get_memory_usage(), 50.0);
        assert_eq!(monitor.get_swap_usage(), 50.0);
    }

    #[test]
    fn test_add_process() {
        let mut monitor = SystemMonitor::new();
        monitor.add_process(1, "test".to_string(), 10.0, 1000, ProcessState::Running);
        assert_eq!(monitor.processes.len(), 1);
    }

    #[test]
    fn test_top_cpu_processes() {
        let mut monitor = SystemMonitor::new();
        monitor.add_process(1, "low".to_string(), 5.0, 1000, ProcessState::Running);
        monitor.add_process(2, "high".to_string(), 50.0, 2000, ProcessState::Running);
        monitor.add_process(3, "medium".to_string(), 25.0, 1500, ProcessState::Running);

        let top = monitor.get_top_cpu_processes(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].pid, 2);
        assert_eq!(top[1].pid, 3);
    }
;
