// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaTuning - System Tuning and Kernel Parameter Manager
//!
//! Inspired by Linux `sysctl` (/proc/sys/) and tuning daemons such as `tuned`.
//! Provides runtime kernel parameter inspection, modification, boundary validation,
//! sysctl.conf parsing, and system performance profile presets.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Category of system tuning parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TuningCategory {
    /// General kernel parameters (kernel.*)
    Kernel,
    /// Virtual memory parameters (vm.*)
    Vm,
    /// Networking stack parameters (net.*)
    Net,
    /// Virtual and physical filesystems (fs.*)
    Fs,
    /// Hardening and security parameters (security.*)
    Security,
}

/// A kernel tunable parameter (sysctl entry)
#[derive(Debug, Clone)]
pub struct TunableParam {
    pub key: String,
    pub category: TuningCategory,
    pub current_value: i64,
    pub default_value: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub description: String,
    pub read_only: bool,
}

impl TunableParam {
    /// Create a new tunable parameter
    pub fn new(
        key: &str,
        category: TuningCategory,
        current_value: i64,
        min_value: i64,
        max_value: i64,
        description: &str,
    ) -> Self {
        Self {
            key: key.to_string(),
            category,
            current_value,
            default_value: current_value,
            min_value,
            max_value,
            description: description.to_string(),
            read_only: false,
        }
    }

    /// Mark parameter as read-only
    pub fn set_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Set a new value with range validation
    pub fn set_value(&mut self, new_val: i64) -> Result<(), &'static str> {
        if self.read_only {
            return Err("Parameter is read-only");
        }
        if new_val < self.min_value || new_val > self.max_value {
            return Err("Value out of valid range");
        }
        self.current_value = new_val;
        Ok(())
    }

    /// Reset to default value
    pub fn reset_to_default(&mut self) -> Result<(), &'static str> {
        if self.read_only {
            return Err("Parameter is read-only");
        }
        self.current_value = self.default_value;
        Ok(())
    }
}

/// System tuning performance profile
#[derive(Debug, Clone)]
pub struct TuningProfile {
    pub name: String,
    pub description: String,
    pub parameter_overrides: BTreeMap<String, i64>,
}

impl TuningProfile {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameter_overrides: BTreeMap::new(),
        }
    }

    pub fn with_override(mut self, key: &str, value: i64) -> Self {
        self.parameter_overrides.insert(key.to_string(), value);
        self
    }
}

/// System Tuning Manager (sysctl and tuned equivalent)
#[derive(Debug)]
pub struct SigmaTuning {
    parameters: BTreeMap<String, TunableParam>,
    profiles: BTreeMap<String, TuningProfile>,
    active_profile: Option<String>,
    running: bool,
}

impl SigmaTuning {
    /// Create a new SigmaTuning instance with core kernel default tunables
    pub fn new() -> Self {
        let mut mgr = Self {
            parameters: BTreeMap::new(),
            profiles: BTreeMap::new(),
            active_profile: None,
            running: false,
        };
        mgr.populate_defaults();
        mgr.populate_profiles();
        mgr
    }

    /// Populate standard system parameters
    fn populate_defaults(&mut self) {
        // Virtual Memory
        self.register(TunableParam::new(
            "vm.swappiness",
            TuningCategory::Vm,
            60,
            0,
            200,
            "Aggressiveness of memory swapping",
        ));
        self.register(TunableParam::new(
            "vm.dirty_ratio",
            TuningCategory::Vm,
            20,
            1,
            100,
            "Percentage of system memory that can be dirty before writeout",
        ));
        self.register(TunableParam::new(
            "vm.dirty_background_ratio",
            TuningCategory::Vm,
            10,
            1,
            100,
            "Percentage of memory at which background flush daemon wakes up",
        ));
        self.register(TunableParam::new(
            "vm.vfs_cache_pressure",
            TuningCategory::Vm,
            100,
            0,
            1000,
            "Tendency of kernel to reclaim memory used for directory and inode caches",
        ));

        // Network
        self.register(TunableParam::new(
            "net.ipv4.ip_forward",
            TuningCategory::Net,
            0,
            0,
            1,
            "Enable IPv4 packet forwarding",
        ));
        self.register(TunableParam::new(
            "net.ipv4.tcp_syncookies",
            TuningCategory::Net,
            1,
            0,
            1,
            "Enable SYN flood protection cookies",
        ));
        self.register(TunableParam::new(
            "net.core.somaxconn",
            TuningCategory::Net,
            4096,
            128,
            65535,
            "Maximum socket listen backlog queue length",
        ));
        self.register(TunableParam::new(
            "net.ipv4.tcp_max_syn_backlog",
            TuningCategory::Net,
            2048,
            128,
            65535,
            "Maximum outstanding embryonic TCP connections",
        ));

        // Filesystem
        self.register(TunableParam::new(
            "fs.file-max",
            TuningCategory::Fs,
            1048576,
            1024,
            104857600,
            "Maximum number of file handles system-wide",
        ));
        self.register(TunableParam::new(
            "fs.inotify.max_user_watches",
            TuningCategory::Fs,
            65536,
            1024,
            1048576,
            "Maximum number of file change watches per user",
        ));

        // Kernel
        self.register(TunableParam::new(
            "kernel.pid_max",
            TuningCategory::Kernel,
            32768,
            300,
            4194304,
            "Maximum allocated PID number",
        ));
        self.register(TunableParam::new(
            "kernel.panic_on_oops",
            TuningCategory::Kernel,
            0,
            0,
            1,
            "Trigger kernel panic on oops error",
        ));
        self.register(TunableParam::new(
            "kernel.sched_latency_ns",
            TuningCategory::Kernel,
            6000000,
            100000,
            1000000000,
            "Target scheduling latency in nanoseconds",
        ));

        // Security
        self.register(TunableParam::new(
            "security.dmesg_restrict",
            TuningCategory::Security,
            1,
            0,
            1,
            "Restrict dmesg buffer access to privileged users",
        ));
        self.register(TunableParam::new(
            "security.kptr_restrict",
            TuningCategory::Security,
            2,
            0,
            2,
            "Restrict kernel pointer exposure in /proc",
        ));
    }

    /// Populate tuning profiles
    fn populate_profiles(&mut self) {
        // Throughput Performance (servers, compute)
        let throughput = TuningProfile::new(
            "throughput-performance",
            "Optimize for sustained computation and max throughput",
        )
        .with_override("vm.swappiness", 10)
        .with_override("vm.dirty_ratio", 40)
        .with_override("vm.dirty_background_ratio", 20)
        .with_override("net.core.somaxconn", 16384)
        .with_override("fs.file-max", 2097152);

        // Low Latency (realtime audio, gaming, interactive)
        let low_latency = TuningProfile::new(
            "low-latency",
            "Optimize for minimal response time and jitter",
        )
        .with_override("vm.swappiness", 1)
        .with_override("vm.dirty_ratio", 10)
        .with_override("vm.dirty_background_ratio", 5)
        .with_override("kernel.sched_latency_ns", 1000000);

        // Power Save (laptops, battery)
        let powersave = TuningProfile::new(
            "powersave",
            "Minimize power draw and reduce disk spin-ups",
        )
        .with_override("vm.swappiness", 80)
        .with_override("vm.dirty_ratio", 50)
        .with_override("vm.dirty_background_ratio", 30);

        self.profiles.insert(throughput.name.clone(), throughput);
        self.profiles.insert(low_latency.name.clone(), low_latency);
        self.profiles.insert(powersave.name.clone(), powersave);
    }

    /// Start the tuning manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the tuning manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Register a parameter
    pub fn register(&mut self, param: TunableParam) {
        self.parameters.insert(param.key.clone(), param);
    }

    /// Get a parameter value by key
    pub fn get_value(&self, key: &str) -> Option<i64> {
        self.parameters.get(key).map(|p| p.current_value)
    }

    /// Set a parameter value by key
    pub fn set_value(&mut self, key: &str, value: i64) -> Result<(), &'static str> {
        if let Some(param) = self.parameters.get_mut(key) {
            param.set_value(value)
        } else {
            Err("Parameter not found")
        }
    }

    /// Get parameter metadata
    pub fn get_param(&self, key: &str) -> Option<&TunableParam> {
        self.parameters.get(key)
    }

    /// Reset parameter to its default value
    pub fn reset_param(&mut self, key: &str) -> Result<(), &'static str> {
        if let Some(param) = self.parameters.get_mut(key) {
            param.reset_to_default()
        } else {
            Err("Parameter not found")
        }
    }

    /// Apply a named performance tuning profile
    pub fn apply_profile(&mut self, name: &str) -> Result<(), &'static str> {
        let profile = self.profiles.get(name).cloned().ok_or("Profile not found")?;
        for (key, val) in &profile.parameter_overrides {
            if let Some(param) = self.parameters.get_mut(key) {
                let _ = param.set_value(*val);
            }
        }
        self.active_profile = Some(name.to_string());
        Ok(())
    }

    /// Get currently active tuning profile
    pub fn active_profile(&self) -> Option<&str> {
        self.active_profile.as_deref()
    }

    /// List parameters in a specific category
    pub fn list_by_category(&self, category: TuningCategory) -> Vec<&TunableParam> {
        self.parameters
            .values()
            .filter(|p| p.category == category)
            .collect()
    }

    /// Total registered parameter count
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Parse and apply `sysctl.conf` style text
    pub fn load_sysctl_conf(&mut self, conf: &str) -> usize {
        let mut count = 0;
        for line in conf.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }
            if let Some(eq_pos) = trimmed.find('=') {
                let key = trimmed[..eq_pos].trim();
                let val_str = trimmed[eq_pos + 1..].trim();
                if let Ok(val) = val_str.parse::<i64>() {
                    if self.set_value(key, val).is_ok() {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Dump all parameters in sysctl format
    pub fn dump_sysctl(&self) -> String {
        let mut out = String::new();
        for (key, param) in &self.parameters {
            out.push_str(&format!("{} = {}\n", key, param.current_value));
        }
        out
    }
}

impl Default for SigmaTuning {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Global Singleton & Convenience Functions
// ============================================================

static mut TUNING_INSTANCE: Option<SigmaTuning> = None;

/// Initialize global SigmaTuning instance
pub fn init_sigmatuning() {
    unsafe {
        TUNING_INSTANCE = Some(SigmaTuning::new());
        if let Some(ref mut tuning) = TUNING_INSTANCE {
            tuning.start();
        }
    }
}

/// Get global SigmaTuning reference
pub fn get_sigmatuning() -> Option<&'static SigmaTuning> {
    unsafe { TUNING_INSTANCE.as_ref() }
}

/// Get mutable global SigmaTuning reference
pub fn get_sigmatuning_mut() -> Option<&'static mut SigmaTuning> {
    unsafe { TUNING_INSTANCE.as_mut() }
}

/// Convenience function to read a sysctl value
pub fn sysctl_get(key: &str) -> Option<i64> {
    if let Some(tuning) = get_sigmatuning() {
        tuning.get_value(key)
    } else {
        None
    }
}

/// Convenience function to write a sysctl value
pub fn sysctl_set(key: &str, value: i64) -> Result<(), &'static str> {
    if let Some(tuning) = get_sigmatuning_mut() {
        tuning.set_value(key, value)
    } else {
        Err("Tuning manager not initialized")
    }
}

/// Convenience function to apply a profile
pub fn apply_tuning_profile(profile_name: &str) -> bool {
    if let Some(tuning) = get_sigmatuning_mut() {
        tuning.apply_profile(profile_name).is_ok()
    } else {
        false
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_validation() {
        let mut param = TunableParam::new("vm.swappiness", TuningCategory::Vm, 60, 0, 200, "Swappiness");
        assert_eq!(param.current_value, 60);

        assert!(param.set_value(10).is_ok());
        assert_eq!(param.current_value, 10);

        // Out of bounds
        assert!(param.set_value(250).is_err());
        assert_eq!(param.current_value, 10);

        // Reset
        assert!(param.reset_to_default().is_ok());
        assert_eq!(param.current_value, 60);
    }

    #[test]
    fn test_tuning_manager_defaults() {
        let mut mgr = SigmaTuning::new();
        mgr.start();
        assert!(mgr.is_running());

        assert_eq!(mgr.get_value("vm.swappiness"), Some(60));
        assert_eq!(mgr.get_value("fs.file-max"), Some(1048576));
        assert!(mgr.parameter_count() >= 10);

        assert!(mgr.set_value("vm.swappiness", 10).is_ok());
        assert_eq!(mgr.get_value("vm.swappiness"), Some(10));
    }

    #[test]
    fn test_tuning_profiles() {
        let mut mgr = SigmaTuning::new();
        assert!(mgr.apply_profile("throughput-performance").is_ok());
        assert_eq!(mgr.active_profile(), Some("throughput-performance"));
        assert_eq!(mgr.get_value("vm.swappiness"), Some(10));
        assert_eq!(mgr.get_value("net.core.somaxconn"), Some(16384));

        assert!(mgr.apply_profile("low-latency").is_ok());
        assert_eq!(mgr.active_profile(), Some("low-latency"));
        assert_eq!(mgr.get_value("vm.swappiness"), Some(1));
    }

    #[test]
    fn test_sysctl_conf_loader() {
        let mut mgr = SigmaTuning::new();
        let conf = r#"
            # System sysctl configuration
            vm.swappiness = 20
            net.ipv4.ip_forward = 1
            fs.file-max = 2000000
        "#;
        let applied = mgr.load_sysctl_conf(conf);
        assert_eq!(applied, 3);
        assert_eq!(mgr.get_value("vm.swappiness"), Some(20));
        assert_eq!(mgr.get_value("net.ipv4.ip_forward"), Some(1));
        assert_eq!(mgr.get_value("fs.file-max"), Some(2000000));
    }

    #[test]
    fn test_global_singleton() {
        init_sigmatuning();
        assert_eq!(sysctl_get("vm.swappiness"), Some(60));
        assert!(sysctl_set("vm.swappiness", 15).is_ok());
        assert_eq!(sysctl_get("vm.swappiness"), Some(15));
        assert!(apply_tuning_profile("low-latency"));
        assert_eq!(sysctl_get("vm.swappiness"), Some(1));
    }
}
