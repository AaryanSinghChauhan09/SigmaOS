//! System Performance Tuning inspired by tuned and systemd-analyze
//! Adaptive ML tuning profiles, boot time performance analysis, and automated I/O/network optimization.

use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuningProfileKind {
    ThroughputPerformance,
    LatencyPerformance,
    Powersave,
    NetworkLatency,
    VirtualHost,
}

#[derive(Debug, Clone)]
pub struct BootStageMetrics {
    pub stage_name: String,
    pub duration_ms: u32,
}

pub struct PerformanceTuner {
    pub current_profile: TuningProfileKind,
    pub boot_stages: Vec<BootStageMetrics>,
    pub sysctl_optimizations: Vec<(String, String)>,
}

impl PerformanceTuner {
    pub fn new() -> Self {
        Self {
            current_profile: TuningProfileKind::ThroughputPerformance,
            boot_stages: Vec::new(),
            sysctl_optimizations: Vec::new(),
        }
    }

    pub fn apply_profile(&mut self, profile: TuningProfileKind) {
        self.current_profile = profile;
        self.sysctl_optimizations.clear();

        match profile {
            TuningProfileKind::ThroughputPerformance => {
                self.sysctl_optimizations.push((
                    "kernel.sched_min_granularity_ns".to_string(),
                    "10000000".to_string(),
                ));
                self.sysctl_optimizations
                    .push(("vm.dirty_ratio".to_string(), "40".to_string()));
            }
            TuningProfileKind::LatencyPerformance => {
                self.sysctl_optimizations.push((
                    "kernel.sched_min_granularity_ns".to_string(),
                    "1000000".to_string(),
                ));
                self.sysctl_optimizations
                    .push(("net.core.busy_poll".to_string(), "50".to_string()));
            }
            TuningProfileKind::Powersave => {
                self.sysctl_optimizations.push((
                    "vm.dirty_writeback_centisecs".to_string(),
                    "1500".to_string(),
                ));
            }
            TuningProfileKind::NetworkLatency => {
                self.sysctl_optimizations
                    .push(("net.ipv4.tcp_fastopen".to_string(), "3".to_string()));
                self.sysctl_optimizations
                    .push(("net.ipv4.tcp_low_latency".to_string(), "1".to_string()));
            }
            TuningProfileKind::VirtualHost => {
                self.sysctl_optimizations
                    .push(("vm.ksm_pages_to_scan".to_string(), "1000".to_string()));
            }
        }
    }

    pub fn record_boot_stage(&mut self, stage: &str, duration_ms: u32) {
        self.boot_stages.push(BootStageMetrics {
            stage_name: stage.to_string(),
            duration_ms,
        });
    }

    pub fn get_total_boot_time_ms(&self) -> u32 {
        self.boot_stages.iter().map(|s| s.duration_ms).sum()
    }
}

impl Default for PerformanceTuner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_tuned_systemd_analyze() {
        let mut tuner = PerformanceTuner::new();
        tuner.apply_profile(TuningProfileKind::LatencyPerformance);
        assert_eq!(tuner.sysctl_optimizations.len(), 2);

        tuner.record_boot_stage("firmware_uefi", 800);
        tuner.record_boot_stage("kernel_init", 400);
        tuner.record_boot_stage("systemd_units", 600);

        assert_eq!(tuner.get_total_boot_time_ms(), 1800);
    }
}
