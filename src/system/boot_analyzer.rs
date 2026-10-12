// SPDX-License-Identifier: MIT
// SigmaOS - Boot Performance Analysis (systemd-analyze equivalent)
// Inspired by systemd-analyze
// Provides boot performance analysis and optimization

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Service boot time
#[derive(Debug, Clone)]
pub struct ServiceBootTime {
    pub name: String,
    pub start_time: u64, // milliseconds
    pub end_time: u64, // milliseconds
    pub duration: u64, // milliseconds
    pub status: ServiceStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Success,
    Failed,
    Timeout,
    Skipped,
}

/// Boot analysis result
#[derive(Debug, Clone)]
pub struct BootAnalysis {
    pub total_boot_time: u64, // milliseconds
    pub kernel_time: u64, // milliseconds
    pub initrd_time: u64, // milliseconds
    pub userspace_time: u64, // milliseconds
    pub services: Vec<ServiceBootTime>,
    pub slowest_services: Vec<ServiceBootTime>,
}

/// Boot analyzer
pub struct BootAnalyzer {
    pub boot_time: AtomicU64,
    pub service_times: BTreeMap<String, ServiceBootTime>,
    pub enabled: AtomicBool,
}

impl BootAnalyzer {
    pub fn new() -> Self {
        Self {
            boot_time: AtomicU64::new(0),
            service_times: BTreeMap::new(),
            enabled: AtomicBool::new(true),
        }
    }
    
    /// Initialize boot analyzer
    pub fn init(&mut self) {
        self.enabled.store(true, Ordering::Relaxed);
        self.boot_time.store(0, Ordering::Relaxed);
    }
    
    /// Record service start
    pub fn record_service_start(&mut self, name: String, start_time: u64) {
        let service = ServiceBootTime {
            name: name.clone(),
            start_time,
            end_time: 0,
            duration: 0,
            status: ServiceStatus::Success,
        };
        self.service_times.insert(name, service);
    }
    
    /// Record service end
    pub fn record_service_end(&mut self, name: String, end_time: u64, status: ServiceStatus) {
        if let Some(service) = self.service_times.get_mut(&name) {
            service.end_time = end_time;
            service.duration = end_time - service.start_time;
            service.status = status;
        }
    }
    
    /// Set total boot time
    pub fn set_boot_time(&self, time: u64) {
        self.boot_time.store(time, Ordering::Relaxed);
    }
    
    /// Get total boot time
    pub fn get_boot_time(&self) -> u64 {
        self.boot_time.load(Ordering::Relaxed)
    }
    
    /// Analyze boot
    pub fn analyze(&self) -> BootAnalysis {
        let total_boot_time = self.boot_time.load(Ordering::Relaxed);
        
        // Estimate kernel, initrd, userspace times
        let kernel_time = total_boot_time / 4;
        let initrd_time = total_boot_time / 4;
        let userspace_time = total_boot_time / 2;
        
        let services: Vec<_> = self.service_times.values().cloned().collect();
        
        // Find slowest services
        let mut slowest_services = services.clone();
        slowest_services.sort_by(|a, b| b.duration.cmp(&a.duration));
        slowest_services.truncate(10); // Top 10 slowest
        
        BootAnalysis {
            total_boot_time,
            kernel_time,
            initrd_time,
            userspace_time,
            services,
            slowest_services,
        }
    }
    
    /// Get critical path
    pub fn get_critical_path(&self) -> Vec<ServiceBootTime> {
        let mut services: Vec<_> = self.service_times.values().cloned().collect();
        services.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        services
    }
    
    /// Get service time
    pub fn get_service_time(&self, name: &str) -> Option<u64> {
        self.service_times.get(name).map(|s| s.duration)
    }
    
    /// Enable/disable analyzer
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }
}

impl Default for BootAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Global boot analyzer
static mut BOOT_ANALYZER: Option<BootAnalyzer> = None;

/// Initialize boot analyzer
pub fn init_boot_analyzer() {
    unsafe {
        BOOT_ANALYZER = Some(BootAnalyzer::new());
    }
}

/// Get global boot analyzer
pub fn get_boot_analyzer() -> Option<&'static mut BootAnalyzer> {
    unsafe {
        BOOT_ANALYZER.as_mut()
    }
}

/// Get boot time (equivalent to systemd-analyze time)
pub fn analyze_boot_time() -> String {
    if let Some(analyzer) = get_boot_analyzer() {
        let analysis = analyzer.analyze();
        format!(
            "Startup finished in {}ms (kernel) + {}ms (initrd) + {}ms (userspace) = {}ms",
            analysis.kernel_time,
            analysis.initrd_time,
            analysis.userspace_time,
            analysis.total_boot_time
        )
    } else {
        "Boot analyzer not initialized".to_string()
    }
}

/// Get blame (equivalent to systemd-analyze blame)
pub fn analyze_blame() -> String {
    if let Some(analyzer) = get_boot_analyzer() {
        let analysis = analyzer.analyze();
        let mut output = String::new();
        output.push_str("Slowest services:\n");
        for service in &analysis.slowest_services {
            output.push_str(&format!("{} {}ms\n", service.name, service.duration));
        }
        output
    } else {
        "Boot analyzer not initialized".to_string()
    }
}

/// Get critical path (equivalent to systemd-analyze critical-chain)
pub fn analyze_critical_chain() -> String {
    if let Some(analyzer) = get_boot_analyzer() {
        let critical_path = analyzer.get_critical_path();
        let mut output = String::new();
        output.push_str("Critical path:\n");
        for service in &critical_path {
            output.push_str(&format!("{} @{}ms +{}ms\n", service.name, service.start_time, service.duration));
        }
        output
    } else {
        "Boot analyzer not initialized".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_boot_analyzer_init() {
        let analyzer = BootAnalyzer::new();
        analyzer.init();
        assert!(analyzer.enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_record_service() {
        let mut analyzer = BootAnalyzer::new();
        analyzer.record_service_start("test_service".to_string(), 100);
        analyzer.record_service_end("test_service".to_string(), 200, ServiceStatus::Success);
        assert_eq!(analyzer.get_service_time("test_service"), Some(100));
    }
    
    #[test]
    fn test_analyze() {
        let mut analyzer = BootAnalyzer::new();
        analyzer.record_service_start("service1".to_string(), 100);
        analyzer.record_service_end("service1".to_string(), 200, ServiceStatus::Success);
        analyzer.record_service_start("service2".to_string(), 200);
        analyzer.record_service_end("service2".to_string(), 300, ServiceStatus::Success);
        analyzer.set_boot_time(4000);
        
        let analysis = analyzer.analyze();
        assert_eq!(analysis.total_boot_time, 4000);
        assert_eq!(analysis.services.len(), 2);
    }
    
    #[test]
    fn test_analyze_boot_time() {
        init_boot_analyzer();
        if let Some(analyzer) = get_boot_analyzer() {
            analyzer.set_boot_time(5000);
        }
        let result = analyze_boot_time();
        assert!(result.contains("5000ms"));
    }
    
    #[test]
    fn test_analyze_blame() {
        init_boot_analyzer();
        if let Some(analyzer) = get_boot_analyzer() {
            analyzer.record_service_start("slow".to_string(), 100);
            analyzer.record_service_end("slow".to_string(), 500, ServiceStatus::Success);
        }
        let result = analyze_blame();
        assert!(result.contains("Slowest services"));
    }
    
    #[test]
    fn test_analyze_critical_chain() {
        init_boot_analyzer();
        if let Some(analyzer) = get_boot_analyzer() {
            analyzer.record_service_start("first".to_string(), 100);
            analyzer.record_service_end("first".to_string(), 200, ServiceStatus::Success);
            analyzer.record_service_start("second".to_string(), 200);
            analyzer.record_service_end("second".to_string(), 300, ServiceStatus::Success);
        }
        let result = analyze_critical_chain();
        assert!(result.contains("Critical path"));
    }
}
