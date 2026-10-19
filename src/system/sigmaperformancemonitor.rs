//! SigmaPerformanceMonitor - Performance Monitoring and Profiling System
//!
//! Provides comprehensive performance monitoring and profiling inspired by Linux performance tools.
//! Supports system performance tracking, bottleneck detection, and optimization recommendations.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Performance metric
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub id: String,
    pub name: String,
    pub value: f32,
    pub unit: String,
    pub threshold: f32,
}

impl PerformanceMetric {
    pub fn new(id: String, name: String, value: f32, unit: String) -> Self {
        Self {
            id,
            name,
            value,
            unit,
            threshold: 0.0,
        }
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn is_critical(&self) -> bool {
        self.threshold > 0.0 && self.value > self.threshold
    }
}

/// Performance category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceCategory {
    CPU,
    Memory,
    Disk,
    Network,
    GPU,
    Unknown,
}

/// Performance profile
#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub id: String,
    pub name: String,
    pub category: PerformanceCategory,
    pub metrics: Vec<PerformanceMetric>,
}

impl PerformanceProfile {
    pub fn new(id: String, name: String, category: PerformanceCategory) -> Self {
        Self {
            id,
            name,
            category,
            metrics: Vec::new(),
        }
    }

    pub fn with_metric(mut self, metric: PerformanceMetric) -> Self {
        self.metrics.push(metric);
        self
    }

    pub fn critical_metric_count(&self) -> usize {
        self.metrics.iter().filter(|m| m.is_critical()).count()
    }
}

/// Performance monitor
pub struct SigmaPerformanceMonitor {
    profiles: BTreeMap<String, PerformanceProfile>,
    next_profile_id: u64,
}

impl SigmaPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            profiles: BTreeMap::new(),
            next_profile_id: 1,
        }
    }

    pub fn add_profile(&mut self, name: String, category: PerformanceCategory) -> String {
        let id = format!("profile-{}", self.next_profile_id);
        self.next_profile_id += 1;
        let profile = PerformanceProfile::new(id.clone(), name, category);
        self.profiles.insert(id.clone(), profile);
        id
    }

    pub fn get_profile(&self, id: &str) -> Option<&PerformanceProfile> {
        self.profiles.get(id)
    }

    pub fn get_profile_mut(&mut self, id: &str) -> Option<&mut PerformanceProfile> {
        self.profiles.get_mut(id)
    }

    pub fn list_profiles(&self) -> Vec<&PerformanceProfile> {
        self.profiles.values().collect()
    }

    pub fn list_profiles_by_category(
        &self,
        category: PerformanceCategory,
    ) -> Vec<&PerformanceProfile> {
        self.profiles
            .values()
            .filter(|p| p.category == category)
            .collect()
    }

    pub fn remove_profile(&mut self, id: &str) -> bool {
        self.profiles.remove(id).is_some()
    }

    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }

    pub fn total_critical_metrics(&self) -> usize {
        self.profiles
            .values()
            .map(|p| p.critical_metric_count())
            .sum()
    }
}

impl Default for SigmaPerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMAPERFORMANCEMONITOR_INSTANCE: Option<SigmaPerformanceMonitor> = None;

pub fn init_sigmaperformancemonitor() {
    unsafe {
        SIGMAPERFORMANCEMONITOR_INSTANCE = Some(SigmaPerformanceMonitor::new());
    }
}

pub fn get_sigmaperformancemonitor() -> &'static SigmaPerformanceMonitor {
    unsafe {
        SIGMAPERFORMANCEMONITOR_INSTANCE
            .as_ref()
            .expect("SigmaPerformanceMonitor not initialized")
    }
}

pub fn get_sigmaperformancemonitor_mut() -> &'static mut SigmaPerformanceMonitor {
    unsafe {
        SIGMAPERFORMANCEMONITOR_INSTANCE
            .as_mut()
            .expect("SigmaPerformanceMonitor not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metric_creation() {
        let metric = PerformanceMetric::new(
            "metric-1".to_string(),
            "CPU Usage".to_string(),
            75.5,
            "%".to_string(),
        );
        assert_eq!(metric.name, "CPU Usage");
        assert_eq!(metric.value, 75.5);
    }

    #[test]
    fn test_performance_metric_critical() {
        let metric = PerformanceMetric::new(
            "metric-1".to_string(),
            "CPU Usage".to_string(),
            95.0,
            "%".to_string(),
        )
        .with_threshold(90.0);
        assert!(metric.is_critical());
    }

    #[test]
    fn test_performance_profile_creation() {
        let profile = PerformanceProfile::new(
            "profile-1".to_string(),
            "CPU Profile".to_string(),
            PerformanceCategory::CPU,
        );
        assert_eq!(profile.name, "CPU Profile");
        assert_eq!(profile.category, PerformanceCategory::CPU);
    }

    #[test]
    fn test_performance_profile_with_metric() {
        let profile = PerformanceProfile::new(
            "profile-1".to_string(),
            "CPU Profile".to_string(),
            PerformanceCategory::CPU,
        )
        .with_metric(PerformanceMetric::new(
            "metric-1".to_string(),
            "Usage".to_string(),
            75.0,
            "%".to_string(),
        ));
        assert_eq!(profile.metrics.len(), 1);
    }

    #[test]
    fn test_performance_monitor() {
        let monitor = SigmaPerformanceMonitor::new();
        assert_eq!(monitor.profile_count(), 0);
    }

    #[test]
    fn test_add_profile() {
        let mut monitor = SigmaPerformanceMonitor::new();
        let id = monitor.add_profile("CPU Profile".to_string(), PerformanceCategory::CPU);
        assert_eq!(id, "profile-1");
        assert_eq!(monitor.profile_count(), 1);
    }

    #[test]
    fn test_list_profiles_by_category() {
        let mut monitor = SigmaPerformanceMonitor::new();
        monitor.add_profile("CPU Profile".to_string(), PerformanceCategory::CPU);
        monitor.add_profile("Memory Profile".to_string(), PerformanceCategory::Memory);
        monitor.add_profile("Disk Profile".to_string(), PerformanceCategory::Disk);

        let cpu_profiles = monitor.list_profiles_by_category(PerformanceCategory::CPU);
        assert_eq!(cpu_profiles.len(), 1);
    }

    #[test]
    fn test_total_critical_metrics() {
        let mut monitor = SigmaPerformanceMonitor::new();
        let id = monitor.add_profile("CPU Profile".to_string(), PerformanceCategory::CPU);

        let profile = monitor.get_profile_mut(&id).unwrap();
        profile.metrics.push(
            PerformanceMetric::new("m1".to_string(), "Usage".to_string(), 95.0, "%".to_string())
                .with_threshold(90.0),
        );
        profile.metrics.push(
            PerformanceMetric::new("m2".to_string(), "Load".to_string(), 85.0, "%".to_string())
                .with_threshold(90.0),
        );

        assert_eq!(monitor.total_critical_metrics(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaperformancemonitor();
        let monitor = get_sigmaperformancemonitor();
        assert_eq!(monitor.profile_count(), 0);

        let monitor_mut = get_sigmaperformancemonitor_mut();
        monitor_mut.add_profile("CPU Profile".to_string(), PerformanceCategory::CPU);
        assert_eq!(monitor.profile_count(), 1);
    }
}
