// SPDX-License-Identifier: MIT
// SigmaOS - Log Rotation Framework
// Inspired by logrotate (https://github.com/logrotate/logrotate)
// Provides automatic log rotation and management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Rotation schedule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationSchedule {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Size(u64), // Rotate when size exceeds bytes
;

/// Compression type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Gzip,
    Bzip2,
    Xz,
    Zstd,
;

/// Log file configuration
#[derive(Debug, Clone)]
pub struct LogFileConfig {
    pub path: String,
    pub schedule: RotationSchedule,
    pub rotate_count: u32, // Number of rotated files to keep
    pub compression: CompressionType,
    pub compress_delay: u32, // Delay compression by N rotations
    pub size_limit: u64, // Rotate when size exceeds
    pub missingok: bool, // Don't error if file is missing
    pub notifempty: bool, // Don't rotate if empty
    pub sharedscripts: bool, // Run scripts once per rotation
;

/// Log rotation entry
#[derive(Debug, Clone)]
pub struct LogRotationEntry {
    pub original_path: String,
    pub rotated_path: String,
    pub rotation_number: u32,
    pub timestamp: u64,
    pub compressed: bool,
;

/// Log rotation manager
pub struct LogRotateManager {
    pub configs: BTreeMap<String, LogFileConfig>,
    pub rotated_logs: Vec<LogRotationEntry>,
    pub next_config_id: AtomicU32,
    pub active: AtomicBool,
;

impl LogRotateManager {
    pub fn new() -> Self {
        Self {
            configs: BTreeMap::new(),
            rotated_logs: Vec::new(),
            next_config_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize log rotation manager
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Add log file configuration
    pub fn add_config(&mut self, path: String, config: LogFileConfig) {
        self.configs.insert(path, config);
    }
    
    /// Remove log file configuration
    pub fn remove_config(&mut self, path: &str) -> Result<(), &'static str> {
        self.configs.remove(path).ok_or("Config not found")?;
        Ok(())
    }
    
    /// Rotate log file
    pub fn rotate_log(&mut self, path: &str) -> Result<String, &'static str> {
        let config = self.configs.get(path).ok_or("Config not found")?;
        
        // Generate rotated path
        let rotation_number = self.get_next_rotation_number(path);
        let rotated_path = format!("{}.{}", path, rotation_number);
        
        // Add rotation entry
        let entry = LogRotationEntry {
            original_path: path.to_string(),
            rotated_path: rotated_path.clone(),
            rotation_number,
            timestamp: 0, // Would be current timestamp
            compressed: false,
        };
        self.rotated_logs.push(entry);
        
        // Enforce rotate count
        self.enforce_rotate_count(path, config.rotate_count);
        
        Ok(rotated_path)
    }
    
    /// Get next rotation number for a log file
    fn get_next_rotation_number(&self, path: &str) -> u32 {
        let count = self.rotated_logs
            .iter()
            .filter(|e| e.original_path == path)
            .count() as u32;
        count + 1
    }
    
    /// Enforce rotate count by removing old rotations
    fn enforce_rotate_count(&mut self, path: &str, rotate_count: u32) {
        let mut to_remove = Vec::new();
        
        for (i, entry) in self.rotated_logs.iter().enumerate() {
            if entry.original_path == path && entry.rotation_number > rotate_count {
                to_remove.push(i);
            }
        }
        
        // Remove in reverse order to maintain indices
        for i in to_remove.into_iter().rev() {
            self.rotated_logs.remove(i);
        }
    }
    
    /// Check if rotation is needed
    pub fn should_rotate(&self, path: &str, current_size: u64) -> bool {
        if let Some(config) = self.configs.get(path) {
            match config.schedule {
                RotationSchedule::Size(limit) => current_size >= limit,
                _ => false, // Time-based rotation handled by scheduler
            }
        } else {
            false
        }
    }
    
    /// Compress rotated log
    pub fn compress_log(&mut self, path: &str) -> Result<(), &'static str> {
        for entry in self.rotated_logs.iter_mut() {
            if entry.rotated_path == path {
                entry.compressed = true;
                return Ok(());
            }
        }
        Err("Log entry not found")
    }
    
    /// Get configuration
    pub fn get_config(&self, path: &str) -> Option<&LogFileConfig> {
        self.configs.get(path)
    }
    
    /// Get all configurations
    pub fn get_all_configs(&self) -> Vec<(&String, &LogFileConfig)> {
        self.configs.iter().collect()
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
;

impl Default for LogRotateManager {
    fn default() -> Self {
        Self::new()
    }
;

/// Global log rotation manager
static mut LOG_ROTATE: Option<LogRotateManager> = None;

/// Initialize log rotation manager
pub fn init_log_rotate() {
    unsafe {
        LOG_ROTATE = Some(LogRotateManager::new());
    }
;

/// Get global log rotation manager
pub fn get_log_rotate() -> Option<&'static mut LogRotateManager> {
    unsafe {
        LOG_ROTATE.as_mut()
    }
;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_rotate_init() {
        let mut manager = LogRotateManager::new();
        manager.init();
        assert!(manager.is_active());
    }
    
    #[test]
    fn test_add_config() {
        let mut manager = LogRotateManager::new();
        let config = LogFileConfig {
            path: "/var/log/test.log".to_string(),
            schedule: RotationSchedule::Daily,
            rotate_count: 7,
            compression: CompressionType::Gzip,
            compress_delay: 0,
            size_limit: 100_000_000,
            missingok: true,
            notifempty: true,
            sharedscripts: false,
        };
        manager.add_config("/var/log/test.log".to_string(), config);
        assert!(manager.get_config("/var/log/test.log").is_some());
    }
    
    #[test]
    fn test_rotate_log() {
        let mut manager = LogRotateManager::new();
        let config = LogFileConfig {
            path: "/var/log/test.log".to_string(),
            schedule: RotationSchedule::Daily,
            rotate_count: 7,
            compression: CompressionType::Gzip,
            compress_delay: 0,
            size_limit: 100_000_000,
            missingok: true,
            notifempty: true,
            sharedscripts: false,
        };
        manager.add_config("/var/log/test.log".to_string(), config);
        
        let rotated = manager.rotate_log("/var/log/test.log").unwrap();
        assert_eq!(rotated, "/var/log/test.log.1");
        assert_eq!(manager.rotated_logs.len(), 1);
    }
    
    #[test]
    fn test_should_rotate() {
        let mut manager = LogRotateManager::new();
        let config = LogFileConfig {
            path: "/var/log/test.log".to_string(),
            schedule: RotationSchedule::Size(1000),
            rotate_count: 7,
            compression: CompressionType::Gzip,
            compress_delay: 0,
            size_limit: 1000,
            missingok: true,
            notifempty: true,
            sharedscripts: false,
        };
        manager.add_config("/var/log/test.log".to_string(), config);
        
        assert!(!manager.should_rotate("/var/log/test.log", 500));
        assert!(manager.should_rotate("/var/log/test.log", 1500));
    }
    
    #[test]
    fn test_enforce_rotate_count() {
        let mut manager = LogRotateManager::new();
        let config = LogFileConfig {
            path: "/var/log/test.log".to_string(),
            schedule: RotationSchedule::Daily,
            rotate_count: 3,
            compression: CompressionType::Gzip,
            compress_delay: 0,
            size_limit: 100_000_000,
            missingok: true,
            notifempty: true,
            sharedscripts: false,
        };
        manager.add_config("/var/log/test.log".to_string(), config);
        
        // Rotate multiple times
        for _ in 0..5 {
            let _ = manager.rotate_log("/var/log/test.log");
        }
        
        // Should only keep 3 rotations
        assert_eq!(manager.rotated_logs.len(), 3);
    }
;
