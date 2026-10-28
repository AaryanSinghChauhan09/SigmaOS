// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaLogRotate - Log Rotation Daemon
//!
//! SigmaLogRotate provides log rotation for SigmaOS, inspired by logrotate from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Rotation policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationPolicy {
    /// Rotate daily
    Daily,
    /// Rotate weekly
    Weekly,
    /// Rotate monthly
    Monthly,
    /// Rotate by size
    Size,
}

impl fmt::Display for RotationPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RotationPolicy::Daily => write!(f, "daily"),
            RotationPolicy::Weekly => write!(f, "weekly"),
            RotationPolicy::Monthly => write!(f, "monthly"),
            RotationPolicy::Size => write!(f, "size"),
        }
    }
}

/// Compression method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    /// No compression
    None,
    /// Gzip compression
    Gzip,
    /// Bzip2 compression
    Bzip2,
}

impl fmt::Display for CompressionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionMethod::None => write!(f, "none"),
            CompressionMethod::Gzip => write!(f, "gzip"),
            CompressionMethod::Bzip2 => write!(f, "bzip2"),
        }
    }
}

/// Log file configuration
#[derive(Debug, Clone)]
pub struct SigmaLogFileConfig {
    /// Log file path
    pub path: String,
    /// Rotation policy
    pub policy: RotationPolicy,
    /// Rotation count (number of rotated files to keep)
    pub rotate_count: u32,
    /// Size threshold in MB (for size-based rotation)
    pub size_threshold_mb: u64,
    /// Compression method
    pub compression: CompressionMethod,
    /// Enabled
    pub enabled: bool,
}

impl SigmaLogFileConfig {
    /// Create a new log file configuration
    pub fn new(path: String, policy: RotationPolicy, rotate_count: u32) -> Self {
        Self {
            path,
            policy,
            rotate_count,
            size_threshold_mb: 100,
            compression: CompressionMethod::Gzip,
            enabled: true,
        }
    }

    /// Set size threshold
    pub fn with_size_threshold(mut self, size_mb: u64) -> Self {
        self.size_threshold_mb = size_mb;
        self
    }

    /// Set compression method
    pub fn with_compression(mut self, compression: CompressionMethod) -> Self {
        self.compression = compression;
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// SigmaLogRotate - Log Rotation Daemon
pub struct SigmaLogRotate {
    /// Log file configurations (path -> config)
    configs: BTreeMap<String, SigmaLogFileConfig>,
    /// Running state
    running: bool,
    /// Rotation counter
    rotation_counter: u32,
}

impl SigmaLogRotate {
    /// Create a new SigmaLogRotate instance
    pub fn new() -> Self {
        Self {
            configs: BTreeMap::new(),
            running: false,
            rotation_counter: 0,
        }
    }

    /// Start the rotation daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the rotation daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a log file configuration
    pub fn add_config(&mut self, config: SigmaLogFileConfig) -> Result<(), String> {
        if self.configs.contains_key(&config.path) {
            return Err(format!("Config for '{}' already exists", config.path));
        }

        self.configs.insert(config.path.clone(), config);
        Ok(())
    }

    /// Remove a log file configuration
    pub fn remove_config(&mut self, path: &str) -> Result<(), String> {
        if self.configs.remove(path).is_none() {
            return Err(format!("Config for '{}' not found", path));
        }
        Ok(())
    }

    /// Get a log file configuration
    pub fn get_config(&self, path: &str) -> Option<&SigmaLogFileConfig> {
        self.configs.get(path)
    }

    /// Enable a log file configuration
    pub fn enable_config(&mut self, path: &str) -> Result<(), String> {
        match self.configs.get_mut(path) {
            Some(config) => {
                config.enabled = true;
                Ok(())
            }
            None => Err(format!("Config for '{}' not found", path)),
        }
    }

    /// Disable a log file configuration
    pub fn disable_config(&mut self, path: &str) -> Result<(), String> {
        match self.configs.get_mut(path) {
            Some(config) => {
                config.enabled = false;
                Ok(())
            }
            None => Err(format!("Config for '{}' not found", path)),
        }
    }

    /// Rotate a log file
    pub fn rotate_log(&mut self, path: &str) -> Result<(), String> {
        match self.configs.get_mut(path) {
            Some(config) => {
                if !config.enabled {
                    return Err(format!("Config for '{}' is disabled", path));
                }

                // In a real implementation, this would perform the actual rotation
                // For now, we just increment the counter
                self.rotation_counter += 1;
                Ok(())
            }
            None => Err(format!("Config for '{}' not found", path)),
        }
    }

    /// Rotate all enabled log files
    pub fn rotate_all(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaLogRotate not running".to_string());
        }

        for (path, config) in self.configs.iter() {
            if config.enabled {
                self.rotation_counter += 1;
            }
        }

        Ok(())
    }

    /// List all configurations
    pub fn list_configs(&self) -> Vec<&SigmaLogFileConfig> {
        self.configs.values().collect()
    }

    /// List enabled configurations
    pub fn list_enabled(&self) -> Vec<&SigmaLogFileConfig> {
        self.configs.values().filter(|c| c.enabled).collect()
    }

    /// Get configuration count
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Get rotation statistics
    pub fn rotation_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_configs".to_string(), self.config_count().to_string());
        stats.insert(
            "enabled_configs".to_string(),
            self.list_enabled().len().to_string(),
        );
        stats.insert(
            "total_rotations".to_string(),
            self.rotation_counter.to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaLogRotate {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaLogRotate instance (placeholder for actual global state)
static mut SIGMALOGROTATE_INSTANCE: Option<SigmaLogRotate> = None;

/// Initialize the global SigmaLogRotate instance
pub fn init_sigmalogrotate() {
    unsafe {
        SIGMALOGROTATE_INSTANCE = Some(SigmaLogRotate::new());
    }
}

/// Get the global SigmaLogRotate instance
pub fn get_sigmalogrotate() -> Option<&'static mut SigmaLogRotate> {
    unsafe { SIGMALOGROTATE_INSTANCE.as_mut() }
}

/// Convenience function to rotate a log file
pub fn rotate_log_file(path: &str) -> Result<(), String> {
    if let Some(logrotate) = get_sigmalogrotate() {
        logrotate.rotate_log(path)
    } else {
        Err("SigmaLogRotate not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_policy_display() {
        assert_eq!(RotationPolicy::Daily.to_string(), "daily");
        assert_eq!(RotationPolicy::Weekly.to_string(), "weekly");
    }

    #[test]
    fn test_compression_method_display() {
        assert_eq!(CompressionMethod::Gzip.to_string(), "gzip");
        assert_eq!(CompressionMethod::Bzip2.to_string(), "bzip2");
    }

    #[test]
    fn test_log_file_config() {
        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7)
            .with_size_threshold(50)
            .with_compression(CompressionMethod::Gzip)
            .with_enabled(true);

        assert_eq!(config.path, "/var/log/syslog");
        assert_eq!(config.rotate_count, 7);
        assert_eq!(config.size_threshold_mb, 50);
    }

    #[test]
    fn test_sigmalogrotate() {
        let mut logrotate = SigmaLogRotate::new();
        assert!(!logrotate.is_running());
        logrotate.start();
        assert!(logrotate.is_running());
        logrotate.stop();
        assert!(!logrotate.is_running());
    }

    #[test]
    fn test_sigmalogrotate_add_config() {
        let mut logrotate = SigmaLogRotate::new();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7);

        assert!(logrotate.add_config(config).is_ok());
        assert_eq!(logrotate.config_count(), 1);
    }

    #[test]
    fn test_sigmalogrotate_enable_config() {
        let mut logrotate = SigmaLogRotate::new();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7)
            .with_enabled(false);

        logrotate.add_config(config).unwrap();
        assert!(logrotate.enable_config("/var/log/syslog").is_ok());

        let config = logrotate.get_config("/var/log/syslog").unwrap();
        assert!(config.enabled);
    }

    #[test]
    fn test_sigmalogrotate_disable_config() {
        let mut logrotate = SigmaLogRotate::new();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7);

        logrotate.add_config(config).unwrap();
        assert!(logrotate.disable_config("/var/log/syslog").is_ok());

        let config = logrotate.get_config("/var/log/syslog").unwrap();
        assert!(!config.enabled);
    }

    #[test]
    fn test_sigmalogrotate_rotate_log() {
        let mut logrotate = SigmaLogRotate::new();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7);

        logrotate.add_config(config).unwrap();
        assert!(logrotate.rotate_log("/var/log/syslog").is_ok());
        assert_eq!(logrotate.rotation_counter, 1);
    }

    #[test]
    fn test_sigmalogrotate_rotate_all() {
        let mut logrotate = SigmaLogRotate::new();
        logrotate.start();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7);

        logrotate.add_config(config).unwrap();
        assert!(logrotate.rotate_all().is_ok());
        assert_eq!(logrotate.rotation_counter, 1);
    }

    #[test]
    fn test_sigmalogrotate_list_enabled() {
        let mut logrotate = SigmaLogRotate::new();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7);

        logrotate.add_config(config).unwrap();

        let enabled = logrotate.list_enabled();
        assert_eq!(enabled.len(), 1);
    }

    #[test]
    fn test_sigmalogrotate_stats() {
        let mut logrotate = SigmaLogRotate::new();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7);

        logrotate.add_config(config).unwrap();

        let stats = logrotate.rotation_stats();
        assert_eq!(stats.get("total_configs"), Some(&"1".to_string()));
        assert_eq!(stats.get("enabled_configs"), Some(&"1".to_string()));
    }

    #[test]
    fn test_rotate_log_file_convenience() {
        init_sigmalogrotate();

        let mut logrotate = get_sigmalogrotate().unwrap();

        let config = SigmaLogFileConfig::new("/var/log/syslog".to_string(), RotationPolicy::Daily, 7);

        logrotate.add_config(config).unwrap();

        let result = rotate_log_file("/var/log/syslog");
        assert!(result.is_ok());
    }
}
