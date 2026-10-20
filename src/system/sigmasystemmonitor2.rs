// SPDX-License-Identifier: MIT
// SigmaOS - System Monitor 2
// System monitoring inspired by Arch (htop, iotop), Debian (top), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
}

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub priority: i32,
}

/// System metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_in: u64,
    pub network_out: u64,
    pub uptime_seconds: u64,
}

/// System monitor
pub struct SigmaSystemMonitor2 {
    processes: Vec<ProcessInfo>,
    metrics: SystemMetrics,
}

impl SigmaSystemMonitor2 {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            metrics: SystemMetrics {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
                network_in: 0,
                network_out: 0,
                uptime_seconds: 0,
            },
        }
    }

    pub fn add_process(&mut self, process: ProcessInfo) {
        self.processes.push(process);
    }

    pub fn get_processes(&self) -> &[ProcessInfo] {
        &self.processes
    }

    pub fn get_process(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn update_metrics(&mut self, metrics: SystemMetrics) {
        self.metrics = metrics;
    }

    pub fn get_metrics(&self) -> &SystemMetrics {
        &self.metrics
    }

    pub fn get_top_cpu_processes(&self, limit: usize) -> Vec<&ProcessInfo> {
        let mut processes = self.processes.iter().collect::<Vec<_>>();
        processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
        processes.into_iter().take(limit).collect()
    }

    pub fn get_top_memory_processes(&self, limit: usize) -> Vec<&ProcessInfo> {
        let mut processes = self.processes.iter().collect::<Vec<_>>();
        processes.sort_by(|a, b| b.memory_percent.partial_cmp(&a.memory_percent).unwrap());
        processes.into_iter().take(limit).collect()
    }

    pub fn kill_process(&mut self, pid: u32) -> Result<(), String> {
        if let Some(idx) = self.processes.iter().position(|p| p.pid == pid) {
            self.processes.remove(idx);
            Ok(())
        } else {
            Err(format!("Process {} not found", pid))
        }
    }
}

impl Default for SigmaSystemMonitor2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_creation() {
        let monitor = SigmaSystemMonitor2::new();
        assert_eq!(monitor.get_processes().len(), 0);
        assert_eq!(monitor.get_metrics().cpu_usage, 0.0);
    }

    #[test]
    fn test_add_process() {
        let mut monitor = SigmaSystemMonitor2::new();
        let process = ProcessInfo {
            pid: 1,
            name: "test".to_string(),
            state: ProcessState::Running,
            cpu_percent: 10.0,
            memory_percent: 5.0,
            priority: 0,
        };
        monitor.add_process(process);
        assert_eq!(monitor.get_processes().len(), 1);
    }

    #[test]
    fn test_update_metrics() {
        let mut monitor = SigmaSystemMonitor2::new();
        let metrics = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_usage: 70.0,
            network_in: 1000,
            network_out: 2000,
            uptime_seconds: 3600,
        };
        monitor.update_metrics(metrics);
        assert_eq!(monitor.get_metrics().cpu_usage, 50.0);
    }

    #[test]
    fn test_top_cpu_processes() {
        let mut monitor = SigmaSystemMonitor2::new();
        monitor.add_process(ProcessInfo {
            pid: 1,
            name: "low".to_string(),
            state: ProcessState::Running,
            cpu_percent: 10.0,
            memory_percent: 5.0,
            priority: 0,
        });
        monitor.add_process(ProcessInfo {
            pid: 2,
            name: "high".to_string(),
            state: ProcessState::Running,
            cpu_percent: 90.0,
            memory_percent: 5.0,
            priority: 0,
        });
        let top = monitor.get_top_cpu_processes(1);
        assert_eq!(top[0].name, "high");
    }

    #[test]
    fn test_kill_process() {
        let mut monitor = SigmaSystemMonitor2::new();
        monitor.add_process(ProcessInfo {
            pid: 1,
            name: "test".to_string(),
            state: ProcessState::Running,
            cpu_percent: 10.0,
            memory_percent: 5.0,
            priority: 0,
        });
        monitor.kill_process(1).unwrap();
        assert_eq!(monitor.get_processes().len(), 0);
    }
}
