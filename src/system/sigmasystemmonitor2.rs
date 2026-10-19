//! SigmaSystemMonitor - System Monitoring Dashboard
//!
//! Inspired by system monitors in Linux distributions, this module provides comprehensive
//! system monitoring including CPU, memory, disk, network, and process monitoring.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Monitor type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitorType {
    Cpu,
    Memory,
    Disk,
    Network,
    Process,
    Temperature,
    Unknown,
}

/// Metric data structure
#[derive(Debug, Clone)]
pub struct Metric {
    pub metric_id: String,
    pub name: String,
    pub monitor_type: MonitorType,
    pub value: u64,
    pub unit: String,
}

impl Metric {
    pub fn new(
        metric_id: String,
        name: String,
        monitor_type: MonitorType,
        value: u64,
        unit: String,
    ) -> Self {
        Self {
            metric_id,
            name,
            monitor_type,
            value,
            unit,
        }
    }
}

/// System monitor
pub struct SigmaSystemMonitor {
    metrics: BTreeMap<String, Metric>,
    cpu_usage: AtomicU64,
    memory_usage: AtomicU64,
    running: AtomicBool,
}

impl SigmaSystemMonitor {
    pub fn new() -> Self {
        Self {
            metrics: BTreeMap::new(),
            cpu_usage: AtomicU64::new(0),
            memory_usage: AtomicU64::new(0),
            running: AtomicBool::new(false),
        }
    }

    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn add_metric(&mut self, metric: Metric) -> bool {
        self.metrics
            .insert(metric.metric_id.clone(), metric)
            .is_some()
    }

    pub fn get_metric(&self, metric_id: &str) -> Option<&Metric> {
        self.metrics.get(metric_id)
    }

    pub fn list_metrics(&self) -> Vec<&Metric> {
        self.metrics.values().collect()
    }

    pub fn update_cpu_usage(&self, usage: u64) {
        self.cpu_usage.store(usage, Ordering::SeqCst);
    }

    pub fn get_cpu_usage(&self) -> u64 {
        self.cpu_usage.load(Ordering::SeqCst)
    }

    pub fn update_memory_usage(&self, usage: u64) {
        self.memory_usage.store(usage, Ordering::SeqCst);
    }

    pub fn get_memory_usage(&self) -> u64 {
        self.memory_usage.load(Ordering::SeqCst)
    }

    pub fn metric_count(&self) -> usize {
        self.metrics.len()
    }
}

impl Default for SigmaSystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMASYSTEMMONITOR_INSTANCE: Option<SigmaSystemMonitor> = None;

pub fn init_sigmasystemmonitor() {
    unsafe {
        SIGMASYSTEMMONITOR_INSTANCE = Some(SigmaSystemMonitor::new());
    }
}

pub fn get_sigmasystemmonitor() -> Option<&'static SigmaSystemMonitor> {
    unsafe { SIGMASYSTEMMONITOR_INSTANCE.as_ref() }
}

pub fn get_sigmasystemmonitor_mut() -> Option<&'static mut SigmaSystemMonitor> {
    unsafe { SIGMASYSTEMMONITOR_INSTANCE.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sigmasystemmonitor_creation() {
        let monitor = SigmaSystemMonitor::new();
        assert!(!monitor.is_running());
    }
}
