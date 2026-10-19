//! SigmaSystemMonitor - System Monitoring System
//!
//! Provides comprehensive system monitoring inspired by Linux system monitoring tools.
//! Supports CPU, memory, disk, network, and temperature monitoring.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// CPU core
#[derive(Debug, Clone)]
pub struct CpuCore {
    pub id: u32,
    pub usage_percent: f32,
    pub frequency_mhz: u32,
    pub temperature_celsius: f32,
;

impl CpuCore {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            usage_percent: 0.0,
            frequency_mhz: 0,
            temperature_celsius: 0.0,
        }
    }

    pub fn with_usage(mut self, usage_percent: f32) -> Self {
        self.usage_percent = usage_percent;
        self
    }

    pub fn with_frequency(mut self, frequency_mhz: u32) -> Self {
        self.frequency_mhz = frequency_mhz;
        self
    }

    pub fn with_temperature(mut self, temperature_celsius: f32) -> Self {
        self.temperature_celsius = temperature_celsius;
        self
    }
;

/// Memory info
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub used_mb: u64,
    pub available_mb: u64,
    pub swap_total_mb: u64,
    pub swap_used_mb: u64,
;

impl MemoryInfo {
    pub fn new(total_mb: u64) -> Self {
        Self {
            total_mb,
            used_mb: 0,
            available_mb: total_mb,
            swap_total_mb: 0,
            swap_used_mb: 0,
        }
    }

    pub fn with_usage(mut self, used_mb: u64) -> Self {
        self.used_mb = used_mb;
        self.available_mb = self.total_mb.saturating_sub(used_mb);
        self
    }

    pub fn usage_percentage(&self) -> u32 {
        if self.total_mb == 0 {
            return 0;
        }
        ((self.used_mb * 100) / self.total_mb) as u32
    }
;

/// Network interface stats
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub interface: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
;

impl NetworkStats {
    pub fn new(interface: String) -> Self {
        Self {
            interface,
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
        }
    }

    pub fn with_traffic(mut self, bytes_sent: u64, bytes_received: u64) -> Self {
        self.bytes_sent = bytes_sent;
        self.bytes_received = bytes_received;
        self
    }
;

/// System monitor
pub struct SigmaSystemMonitor {
    cpu_cores: Vec<CpuCore>,
    memory: Option<MemoryInfo>,
    network_stats: BTreeMap<String, NetworkStats>,
    uptime_seconds: u64,
;

impl SigmaSystemMonitor {
    pub fn new(core_count: u32) -> Self {
        let mut cpu_cores = Vec::new();
        for i in 0..core_count {
            cpu_cores.push(CpuCore::new(i));
        }

        Self {
            cpu_cores,
            memory: None,
            network_stats: BTreeMap::new(),
            uptime_seconds: 0,
        }
    }

    pub fn get_cpu_core(&self, id: u32) -> Option<&CpuCore> {
        self.cpu_cores.iter().find(|c| c.id == id)
    }

    pub fn get_cpu_core_mut(&mut self, id: u32) -> Option<&mut CpuCore> {
        self.cpu_cores.iter_mut().find(|c| c.id == id)
    }

    pub fn list_cpu_cores(&self) -> Vec<&CpuCore> {
        self.cpu_cores.iter().collect()
    }

    pub fn set_memory(&mut self, memory: MemoryInfo) {
        self.memory = Some(memory);
    }

    pub fn get_memory(&self) -> Option<&MemoryInfo> {
        self.memory.as_ref()
    }

    pub fn add_network_stats(&mut self, stats: NetworkStats) {
        self.network_stats.insert(stats.interface.clone(), stats);
    }

    pub fn get_network_stats(&self, interface: &str) -> Option<&NetworkStats> {
        self.network_stats.get(interface)
    }

    pub fn list_network_stats(&self) -> Vec<&NetworkStats> {
        self.network_stats.values().collect()
    }

    pub fn set_uptime(&mut self, uptime_seconds: u64) {
        self.uptime_seconds = uptime_seconds;
    }

    pub fn get_uptime(&self) -> u64 {
        self.uptime_seconds
    }

    pub fn total_cpu_usage(&self) -> f32 {
        self.cpu_cores.iter().map(|c| c.usage_percent).sum()
    }

    pub fn average_cpu_usage(&self) -> f32 {
        if self.cpu_cores.is_empty() {
            return 0.0;
        }
        self.total_cpu_usage() / self.cpu_cores.len() as f32
    }

    pub fn cpu_count(&self) -> usize {
        self.cpu_cores.len()
    }
;

impl Default for SigmaSystemMonitor {
    fn default() -> Self {
        Self::new(1)
    }
;

static mut SIGMASYSTEMMONITOR_INSTANCE: Option<SigmaSystemMonitor> = None;

pub fn init_sigmasystemmonitor(core_count: u32) {
    unsafe {
        SIGMASYSTEMMONITOR_INSTANCE = Some(SigmaSystemMonitor::new(core_count));
    }
;

pub fn get_sigmasystemmonitor() -> &'static SigmaSystemMonitor {
    unsafe {
        SIGMASYSTEMMONITOR_INSTANCE
            .as_ref()
            .expect("SigmaSystemMonitor not initialized")
    }
;

pub fn get_sigmasystemmonitor_mut() -> &'static mut SigmaSystemMonitor {
    unsafe {
        SIGMASYSTEMMONITOR_INSTANCE
            .as_mut()
            .expect("SigmaSystemMonitor not initialized")
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_core_creation() {
        let core = CpuCore::new(0);
        assert_eq!(core.id, 0);
        assert_eq!(core.usage_percent, 0.0);
    }

    #[test]
    fn test_cpu_core_with_usage() {
        let core = CpuCore::new(0)
            .with_usage(50.0)
            .with_frequency(2400)
            .with_temperature(45.0);
        assert_eq!(core.usage_percent, 50.0);
        assert_eq!(core.frequency_mhz, 2400);
        assert_eq!(core.temperature_celsius, 45.0);
    }

    #[test]
    fn test_memory_info() {
        let memory = MemoryInfo::new(16000).with_usage(8000);
        assert_eq!(memory.total_mb, 16000);
        assert_eq!(memory.used_mb, 8000);
        assert_eq!(memory.available_mb, 8000);
        assert_eq!(memory.usage_percentage(), 50);
    }

    #[test]
    fn test_network_stats() {
        let stats = NetworkStats::new("eth0".to_string()).with_traffic(1000000, 2000000);
        assert_eq!(stats.interface, "eth0");
        assert_eq!(stats.bytes_sent, 1000000);
        assert_eq!(stats.bytes_received, 2000000);
    }

    #[test]
    fn test_system_monitor() {
        let monitor = SigmaSystemMonitor::new(4);
        assert_eq!(monitor.cpu_count(), 4);
    }

    #[test]
    fn test_total_cpu_usage() {
        let mut monitor = SigmaSystemMonitor::new(4);
        monitor.get_cpu_core_mut(0).unwrap().with_usage(25.0);
        monitor.get_cpu_core_mut(1).unwrap().with_usage(50.0);
        monitor.get_cpu_core_mut(2).unwrap().with_usage(75.0);
        monitor.get_cpu_core_mut(3).unwrap().with_usage(100.0);

        assert_eq!(monitor.total_cpu_usage(), 250.0);
    }

    #[test]
    fn test_average_cpu_usage() {
        let mut monitor = SigmaSystemMonitor::new(4);
        monitor.get_cpu_core_mut(0).unwrap().with_usage(25.0);
        monitor.get_cpu_core_mut(1).unwrap().with_usage(50.0);
        monitor.get_cpu_core_mut(2).unwrap().with_usage(75.0);
        monitor.get_cpu_core_mut(3).unwrap().with_usage(100.0);

        assert_eq!(monitor.average_cpu_usage(), 62.5);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasystemmonitor(4);
        let monitor = get_sigmasystemmonitor();
        assert_eq!(monitor.cpu_count(), 4);

        let monitor_mut = get_sigmasystemmonitor_mut();
        monitor_mut.set_uptime(3600);
        assert_eq!(monitor.get_uptime(), 3600);
    }
;
