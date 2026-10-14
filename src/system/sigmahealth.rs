// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaHealth - System Health Monitoring Engine
//!
//! Provides comprehensive subsystem health scoring, periodic probe checks,
//! degradation detection, and diagnostic reporting across CPU, Memory, Storage,
//! Network, Kernel, and Security subsystems.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Subsystem identifier for health tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Subsystem {
    Cpu,
    Memory,
    Storage,
    Network,
    Kernel,
    Security,
}

/// Operational health state of a component or system
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HealthState {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

/// An individual health check probe
#[derive(Debug, Clone)]
pub struct HealthProbe {
    pub name: String,
    pub subsystem: Subsystem,
    pub state: HealthState,
    pub message: String,
    pub score: u8, // 0 - 100
    pub last_check_timestamp: u64,
}

impl HealthProbe {
    pub fn new(name: &str, subsystem: Subsystem) -> Self {
        Self {
            name: name.to_string(),
            subsystem,
            state: HealthState::Healthy,
            message: "Operational".to_string(),
            score: 100,
            last_check_timestamp: 0,
        }
    }

    pub fn update(&mut self, state: HealthState, score: u8, message: &str, timestamp: u64) {
        self.state = state;
        self.score = score.min(100);
        self.message = message.to_string();
        self.last_check_timestamp = timestamp;
    }
}

/// Aggregate system health report
#[derive(Debug, Clone)]
pub struct SystemHealthReport {
    pub overall_state: HealthState,
    pub overall_score: u8,
    pub subsystem_states: BTreeMap<Subsystem, HealthState>,
    pub active_issues_count: usize,
    pub timestamp: u64,
}

/// System Health Monitor
#[derive(Debug)]
pub struct SigmaHealth {
    probes: Vec<HealthProbe>,
    running: bool,
}

impl SigmaHealth {
    /// Create a new health monitor with baseline system probes
    pub fn new() -> Self {
        let mut mgr = Self {
            probes: Vec::new(),
            running: false,
        };
        mgr.init_default_probes();
        mgr
    }

    fn init_default_probes(&mut self) {
        self.probes.push(HealthProbe::new("cpu_load", Subsystem::Cpu));
        self.probes.push(HealthProbe::new("memory_pressure", Subsystem::Memory));
        self.probes.push(HealthProbe::new("root_disk_space", Subsystem::Storage));
        self.probes.push(HealthProbe::new("network_gateway", Subsystem::Network));
        self.probes.push(HealthProbe::new("kernel_oops", Subsystem::Kernel));
        self.probes.push(HealthProbe::new("tpm_security", Subsystem::Security));
    }

    /// Start the health monitor
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the health monitor
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Register a custom health probe
    pub fn register_probe(&mut self, probe: HealthProbe) {
        if let Some(existing) = self.probes.iter_mut().find(|p| p.name == probe.name) {
            *existing = probe;
        } else {
            self.probes.push(probe);
        }
    }

    /// Update status of a specific probe
    pub fn update_probe(
        &mut self,
        name: &str,
        state: HealthState,
        score: u8,
        message: &str,
        timestamp: u64,
    ) -> bool {
        if let Some(probe) = self.probes.iter_mut().find(|p| p.name == name) {
            probe.update(state, score, message, timestamp);
            true
        } else {
            false
        }
    }

    /// Get overall health state
    pub fn overall_state(&self) -> HealthState {
        if self.probes.iter().any(|p| p.state == HealthState::Critical) {
            HealthState::Critical
        } else if self.probes.iter().any(|p| p.state == HealthState::Degraded) {
            HealthState::Degraded
        } else if self.probes.iter().all(|p| p.state == HealthState::Healthy) {
            HealthState::Healthy
        } else {
            HealthState::Unknown
        }
    }

    /// Compute aggregate health score (0 - 100)
    pub fn overall_score(&self) -> u8 {
        if self.probes.is_empty() {
            return 100;
        }
        let total: u32 = self.probes.iter().map(|p| p.score as u32).sum();
        (total / (self.probes.len() as u32)) as u8
    }

    /// Get health state for a particular subsystem
    pub fn subsystem_state(&self, sub: Subsystem) -> HealthState {
        let matching: Vec<&HealthProbe> = self.probes.iter().filter(|p| p.subsystem == sub).collect();
        if matching.is_empty() {
            return HealthState::Unknown;
        }
        if matching.iter().any(|p| p.state == HealthState::Critical) {
            HealthState::Critical
        } else if matching.iter().any(|p| p.state == HealthState::Degraded) {
            HealthState::Degraded
        } else {
            HealthState::Healthy
        }
    }

    /// Generate comprehensive diagnostic report
    pub fn generate_report(&self, timestamp: u64) -> SystemHealthReport {
        let mut sub_map = BTreeMap::new();
        let subsystems = [
            Subsystem::Cpu,
            Subsystem::Memory,
            Subsystem::Storage,
            Subsystem::Network,
            Subsystem::Kernel,
            Subsystem::Security,
        ];
        for sub in &subsystems {
            sub_map.insert(*sub, self.subsystem_state(*sub));
        }

        let issues = self
            .probes
            .iter()
            .filter(|p| p.state != HealthState::Healthy)
            .count();

        SystemHealthReport {
            overall_state: self.overall_state(),
            overall_score: self.overall_score(),
            subsystem_states: sub_map,
            active_issues_count: issues,
            timestamp,
        }
    }

    /// Probe count
    pub fn probe_count(&self) -> usize {
        self.probes.len()
    }
}

impl Default for SigmaHealth {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Global Singleton & Convenience Functions
// ============================================================

static mut HEALTH_INSTANCE: Option<SigmaHealth> = None;

/// Initialize global SigmaHealth instance
pub fn init_sigmahealth() {
    unsafe {
        HEALTH_INSTANCE = Some(SigmaHealth::new());
        if let Some(ref mut health) = HEALTH_INSTANCE {
            health.start();
        }
    }
}

/// Get global SigmaHealth reference
pub fn get_sigmahealth() -> Option<&'static SigmaHealth> {
    unsafe { HEALTH_INSTANCE.as_ref() }
}

/// Get mutable global SigmaHealth reference
pub fn get_sigmahealth_mut() -> Option<&'static mut SigmaHealth> {
    unsafe { HEALTH_INSTANCE.as_mut() }
}

/// Convenience function to get overall health state
pub fn system_health_state() -> HealthState {
    if let Some(health) = get_sigmahealth() {
        health.overall_state()
    } else {
        HealthState::Unknown
    }
}

/// Convenience function to get overall health score
pub fn system_health_score() -> u8 {
    if let Some(health) = get_sigmahealth() {
        health.overall_score()
    } else {
        0
    }
}

/// Convenience function to generate diagnostic report
pub fn generate_health_report(timestamp: u64) -> Option<SystemHealthReport> {
    get_sigmahealth().map(|h| h.generate_report(timestamp))
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_monitor_lifecycle() {
        let mut mgr = SigmaHealth::new();
        mgr.start();
        assert!(mgr.is_running());

        assert_eq!(mgr.overall_state(), HealthState::Healthy);
        assert_eq!(mgr.overall_score(), 100);

        // Degrade a probe
        assert!(mgr.update_probe("cpu_load", HealthState::Degraded, 70, "High CPU load", 100));
        assert_eq!(mgr.overall_state(), HealthState::Degraded);
        assert!(mgr.overall_score() < 100);

        // Critical probe
        assert!(mgr.update_probe("root_disk_space", HealthState::Critical, 20, "Disk 99% full", 105));
        assert_eq!(mgr.overall_state(), HealthState::Critical);
        assert_eq!(mgr.subsystem_state(Subsystem::Storage), HealthState::Critical);

        let report = mgr.generate_report(110);
        assert_eq!(report.overall_state, HealthState::Critical);
        assert_eq!(report.active_issues_count, 2);
    }

    #[test]
    fn test_global_singleton() {
        init_sigmahealth();
        assert_eq!(system_health_state(), HealthState::Healthy);
        assert_eq!(system_health_score(), 100);
        assert!(generate_health_report(100).is_some());
    }
}
