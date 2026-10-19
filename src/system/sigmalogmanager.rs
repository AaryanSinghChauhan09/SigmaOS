//! SigmaLogManager - System Logging Management
//!
//! Provides comprehensive system logging management inspired by Linux syslog, journald, and rsyslog.
//! Supports log levels, log rotation, log filtering, and persistent log storage.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
    Unknown,
}

/// Log source
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogSource {
    Kernel,
    System,
    Application(String),
    Network,
    Security,
    Unknown,
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: u64,
    pub timestamp: u64,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: String,
    pub component: String,
    pub process_id: Option<u32>,
}

impl LogEntry {
    pub fn new(id: u64, level: LogLevel, source: LogSource, message: String) -> Self {
        Self {
            id,
            timestamp: 0,
            level,
            source,
            message,
            component: String::new(),
            process_id: None,
        }
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_component(mut self, component: String) -> Self {
        self.component = component;
        self
    }

    pub fn with_process_id(mut self, pid: u32) -> Self {
        self.process_id = Some(pid);
        self
    }
}

/// Log rotation policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationPolicy {
    SizeBased(u64),
    TimeBased(u64),
    CountBased(u32),
    Disabled,
}

/// Log manager
pub struct SigmaLogManager {
    logs: BTreeMap<u64, LogEntry>,
    next_log_id: u64,
    max_log_size: usize,
    rotation_policy: RotationPolicy,
    current_size: u64,
    log_count: u32,
    enabled: bool,
}

impl SigmaLogManager {
    pub fn new() -> Self {
        Self {
            logs: BTreeMap::new(),
            next_log_id: 1,
            max_log_size: 10000,
            rotation_policy: RotationPolicy::SizeBased(10 * 1024 * 1024),
            current_size: 0,
            log_count: 0,
            enabled: true,
        }
    }

    pub fn log(&mut self, level: LogLevel, source: LogSource, message: String) -> u64 {
        let id = self.next_log_id;
        self.next_log_id += 1;

        let entry = LogEntry::new(id, level, source, message);
        let entry_size = entry.message.len() as u64;
        self.current_size += entry_size;
        self.log_count += 1;

        self.logs.insert(id, entry);

        self.check_rotation();

        id
    }

    pub fn get_log(&self, id: u64) -> Option<&LogEntry> {
        self.logs.get(&id)
    }

    pub fn list_logs(&self) -> Vec<&LogEntry> {
        self.logs.values().collect()
    }

    pub fn list_logs_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.logs.values().filter(|l| l.level == level).collect()
    }

    pub fn list_logs_by_source(&self, source: &LogSource) -> Vec<&LogEntry> {
        self.logs.values().filter(|l| &l.source == source).collect()
    }

    pub fn list_logs_by_component(&self, component: &str) -> Vec<&LogEntry> {
        self.logs
            .values()
            .filter(|l| l.component == component)
            .collect()
    }

    pub fn list_logs_since(&self, timestamp: u64) -> Vec<&LogEntry> {
        self.logs
            .values()
            .filter(|l| l.timestamp >= timestamp)
            .collect()
    }

    pub fn list_logs_with_level_min(&self, min_level: LogLevel) -> Vec<&LogEntry> {
        self.logs
            .values()
            .filter(|l| l.level >= min_level)
            .collect()
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
        self.current_size = 0;
        self.log_count = 0;
    }

    pub fn remove_log(&mut self, id: u64) -> bool {
        if let Some(entry) = self.logs.remove(&id) {
            self.current_size -= entry.message.len() as u64;
            self.log_count -= 1;
            true
        } else {
            false
        }
    }

    pub fn set_rotation_policy(&mut self, policy: RotationPolicy) {
        self.rotation_policy = policy;
    }

    pub fn get_rotation_policy(&self) -> RotationPolicy {
        self.rotation_policy
    }

    pub fn set_max_log_size(&mut self, size: usize) {
        self.max_log_size = size;
    }

    pub fn get_max_log_size(&self) -> usize {
        self.max_log_size
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn log_count(&self) -> usize {
        self.logs.len()
    }

    pub fn current_size(&self) -> u64 {
        self.current_size
    }

    fn check_rotation(&mut self) {
        match self.rotation_policy {
            RotationPolicy::SizeBased(max_size) => {
                if self.current_size >= max_size {
                    self.rotate_logs();
                }
            }
            RotationPolicy::CountBased(max_count) => {
                if self.log_count >= max_count {
                    self.rotate_logs();
                }
            }
            RotationPolicy::TimeBased(_) => {
                // Time-based rotation would require timestamp tracking
            }
            RotationPolicy::Disabled => {}
        }
    }

    fn rotate_logs(&mut self) {
        // Remove oldest logs (half of them)
        let remove_count = self.logs.len() / 2;
        let keys_to_remove: Vec<u64> = self.logs.keys().take(remove_count).cloned().collect();
        for key in keys_to_remove {
            self.remove_log(key);
        }
    }

    pub fn search_logs(&self, pattern: &str) -> Vec<&LogEntry> {
        self.logs
            .values()
            .filter(|l| l.message.contains(pattern))
            .collect()
    }
}

impl Default for SigmaLogManager {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMALOGMANAGER_INSTANCE: Option<SigmaLogManager> = None;

pub fn init_sigmalogmanager() {
    unsafe {
        SIGMALOGMANAGER_INSTANCE = Some(SigmaLogManager::new());
    }
}

pub fn get_sigmalogmanager() -> &'static SigmaLogManager {
    unsafe {
        SIGMALOGMANAGER_INSTANCE
            .as_ref()
            .expect("SigmaLogManager not initialized")
    }
}

pub fn get_sigmalogmanager_mut() -> &'static mut SigmaLogManager {
    unsafe {
        SIGMALOGMANAGER_INSTANCE
            .as_mut()
            .expect("SigmaLogManager not initialized")
    }
}

pub fn log_message(level: LogLevel, source: LogSource, message: String) -> u64 {
    let manager = get_sigmalogmanager_mut();
    manager.log(level, source, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(
            1,
            LogLevel::Info,
            LogSource::System,
            "Test message".to_string(),
        );
        assert_eq!(entry.id, 1);
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.source, LogSource::System);
        assert_eq!(entry.message, "Test message");
    }

    #[test]
    fn test_log_entry_builder() {
        let entry = LogEntry::new(
            1,
            LogLevel::Error,
            LogSource::Kernel,
            "Error message".to_string(),
        )
        .with_timestamp(1234567890)
        .with_component("TestComponent".to_string())
        .with_process_id(1234);

        assert_eq!(entry.timestamp, 1234567890);
        assert_eq!(entry.component, "TestComponent");
        assert_eq!(entry.process_id, Some(1234));
    }

    #[test]
    fn test_log_manager() {
        let manager = SigmaLogManager::new();
        assert_eq!(manager.log_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_log_message() {
        let mut manager = SigmaLogManager::new();
        let id = manager.log(
            LogLevel::Info,
            LogSource::System,
            "Test log message".to_string(),
        );
        assert_eq!(id, 1);
        assert_eq!(manager.log_count(), 1);
    }

    #[test]
    fn test_list_logs_by_level() {
        let mut manager = SigmaLogManager::new();
        manager.log(
            LogLevel::Info,
            LogSource::System,
            "Info message".to_string(),
        );
        manager.log(
            LogLevel::Error,
            LogSource::Kernel,
            "Error message".to_string(),
        );
        manager.log(
            LogLevel::Warning,
            LogSource::System,
            "Warning message".to_string(),
        );

        let error_logs = manager.list_logs_by_level(LogLevel::Error);
        assert_eq!(error_logs.len(), 1);
    }

    #[test]
    fn test_list_logs_by_source() {
        let mut manager = SigmaLogManager::new();
        manager.log(
            LogLevel::Info,
            LogSource::System,
            "System message".to_string(),
        );
        manager.log(
            LogLevel::Error,
            LogSource::Kernel,
            "Kernel message".to_string(),
        );
        manager.log(
            LogLevel::Warning,
            LogSource::System,
            "Another system message".to_string(),
        );

        let system_logs = manager.list_logs_by_source(&LogSource::System);
        assert_eq!(system_logs.len(), 2);
    }

    #[test]
    fn test_list_logs_by_component() {
        let mut manager = SigmaLogManager::new();
        let mut entry1 = LogEntry::new(
            1,
            LogLevel::Info,
            LogSource::System,
            "Message 1".to_string(),
        );
        entry1.component = "ComponentA".to_string();
        let mut entry2 = LogEntry::new(
            2,
            LogLevel::Error,
            LogSource::Kernel,
            "Message 2".to_string(),
        );
        entry2.component = "ComponentB".to_string();
        manager.logs.insert(1, entry1);
        manager.logs.insert(2, entry2);

        let component_a_logs = manager.list_logs_by_component("ComponentA");
        assert_eq!(component_a_logs.len(), 1);
    }

    #[test]
    fn test_list_logs_with_level_min() {
        let mut manager = SigmaLogManager::new();
        manager.log(
            LogLevel::Debug,
            LogSource::System,
            "Debug message".to_string(),
        );
        manager.log(
            LogLevel::Info,
            LogSource::System,
            "Info message".to_string(),
        );
        manager.log(
            LogLevel::Error,
            LogSource::Kernel,
            "Error message".to_string(),
        );

        let min_info_logs = manager.list_logs_with_level_min(LogLevel::Info);
        assert_eq!(min_info_logs.len(), 2);
    }

    #[test]
    fn test_clear_logs() {
        let mut manager = SigmaLogManager::new();
        manager.log(LogLevel::Info, LogSource::System, "Message 1".to_string());
        manager.log(LogLevel::Error, LogSource::Kernel, "Message 2".to_string());

        manager.clear_logs();
        assert_eq!(manager.log_count(), 0);
        assert_eq!(manager.current_size(), 0);
    }

    #[test]
    fn test_remove_log() {
        let mut manager = SigmaLogManager::new();
        manager.log(LogLevel::Info, LogSource::System, "Message 1".to_string());
        manager.log(LogLevel::Error, LogSource::Kernel, "Message 2".to_string());

        assert!(manager.remove_log(1));
        assert_eq!(manager.log_count(), 1);
        assert!(!manager.remove_log(999));
    }

    #[test]
    fn test_rotation_policy() {
        let mut manager = SigmaLogManager::new();
        manager.set_rotation_policy(RotationPolicy::CountBased(5));
        assert_eq!(manager.get_rotation_policy(), RotationPolicy::CountBased(5));
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = SigmaLogManager::new();
        assert!(manager.is_enabled());

        manager.disable();
        assert!(!manager.is_enabled());

        manager.enable();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_search_logs() {
        let mut manager = SigmaLogManager::new();
        manager.log(
            LogLevel::Info,
            LogSource::System,
            "Error occurred".to_string(),
        );
        manager.log(
            LogLevel::Error,
            LogSource::Kernel,
            "Warning message".to_string(),
        );
        manager.log(
            LogLevel::Warning,
            LogSource::System,
            "Another error".to_string(),
        );

        let results = manager.search_logs("error");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmalogmanager();
        let manager = get_sigmalogmanager();
        assert!(manager.is_enabled());

        let manager_mut = get_sigmalogmanager_mut();
        manager_mut.disable();
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_log_message_convenience() {
        init_sigmalogmanager();
        let id = log_message(
            LogLevel::Info,
            LogSource::System,
            "Convenience log".to_string(),
        );
        assert_eq!(id, 1);

        let manager = get_sigmalogmanager();
        assert_eq!(manager.log_count(), 1);
    }
}
