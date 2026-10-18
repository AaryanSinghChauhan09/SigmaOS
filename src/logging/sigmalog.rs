// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaLog - System Logging Daemon
//!
//! SigmaLog is a comprehensive system logging daemon inspired by journald (systemd).
//! It provides structured logging, log rotation, filtering, and persistence.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Log priority levels following syslog standard
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SigmaLogLevel {
    Emergency = 0, // System is unusable
    Alert = 1,     // Action must be taken immediately
    Critical = 2,  // Critical conditions
    Error = 3,     // Error conditions
    Warning = 4,   // Warning conditions
    Notice = 5,    // Normal but significant condition
    Info = 6,      // Informational messages
    Debug = 7,     // Debug-level messages
}

impl fmt::Display for SigmaLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigmaLogLevel::Emergency => write!(f, "EMERGENCY"),
            SigmaLogLevel::Alert => write!(f, "ALERT"),
            SigmaLogLevel::Critical => write!(f, "CRITICAL"),
            SigmaLogLevel::Error => write!(f, "ERROR"),
            SigmaLogLevel::Warning => write!(f, "WARNING"),
            SigmaLogLevel::Notice => write!(f, "NOTICE"),
            SigmaLogLevel::Info => write!(f, "INFO"),
            SigmaLogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

/// Type alias for SigmaLogLevel for convenience
pub type LogLevel = SigmaLogLevel;

/// Log facility identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaLogFacility {
    Kernel,
    User,
    Mail,
    Daemon,
    Auth,
    Syslog,
    Lpr,
    News,
    Uucp,
    Cron,
    AuthPriv,
    Ftp,
    Local0,
    Local1,
    Local2,
    Local3,
    Local4,
    Local5,
    Local6,
    Local7,
}

impl fmt::Display for SigmaLogFacility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigmaLogFacility::Kernel => write!(f, "kernel"),
            SigmaLogFacility::User => write!(f, "user"),
            SigmaLogFacility::Mail => write!(f, "mail"),
            SigmaLogFacility::Daemon => write!(f, "daemon"),
            SigmaLogFacility::Auth => write!(f, "auth"),
            SigmaLogFacility::Syslog => write!(f, "syslog"),
            SigmaLogFacility::Lpr => write!(f, "lpr"),
            SigmaLogFacility::News => write!(f, "news"),
            SigmaLogFacility::Uucp => write!(f, "uucp"),
            SigmaLogFacility::Cron => write!(f, "cron"),
            SigmaLogFacility::AuthPriv => write!(f, "authpriv"),
            SigmaLogFacility::Ftp => write!(f, "ftp"),
            SigmaLogFacility::Local0 => write!(f, "local0"),
            SigmaLogFacility::Local1 => write!(f, "local1"),
            SigmaLogFacility::Local2 => write!(f, "local2"),
            SigmaLogFacility::Local3 => write!(f, "local3"),
            SigmaLogFacility::Local4 => write!(f, "local4"),
            SigmaLogFacility::Local5 => write!(f, "local5"),
            SigmaLogFacility::Local6 => write!(f, "local6"),
            SigmaLogFacility::Local7 => write!(f, "local7"),
        }
    }
}

/// Type alias for SigmaLogFacility for convenience
pub type LogFacility = SigmaLogFacility;

/// Log entry with structured data
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Unique identifier for the log entry
    pub id: String,
    /// Timestamp when the log was created
    pub timestamp: u64,
    /// Log priority level
    pub level: SigmaLogLevel,
    /// Log facility
    pub facility: SigmaLogFacility,
    /// Process identifier
    pub pid: u32,
    /// Process name
    pub process_name: String,
    /// Log message
    pub message: String,
    /// Source location (file, line)
    pub source: Option<String>,
    /// Additional structured fields
    pub fields: BTreeMap<String, String>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(
        level: SigmaLogLevel,
        facility: SigmaLogFacility,
        pid: u32,
        process_name: String,
        message: String,
    ) -> Self {
        Self {
            id: generate_log_id(),
            timestamp: 0, // Will be set by logger
            level,
            facility,
            pid,
            process_name,
            message,
            source: None,
            fields: BTreeMap::new(),
        }
    }

    /// Add a structured field to the log entry
    pub fn with_field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }

    /// Set source location
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
}

/// Generate a unique log ID
fn generate_log_id() -> String {
    // In a real implementation, this would use a proper UUID generator
    // For now, use a simple counter-based approach
    alloc::format!("LOG-{:016x}", 0xDEAD_BEEFu64) // Placeholder
}

/// Log storage backend
#[derive(Debug, Clone)]
pub enum LogStorage {
    /// In-memory storage
    Memory,
    /// File-based storage
    File { path: String },
    /// Network-based storage
    Network { endpoint: String },
}

/// Log filter criteria
#[derive(Debug, Clone)]
pub struct LogFilter {
    /// Minimum log level
    pub min_level: Option<SigmaLogLevel>,
    /// Maximum log level
    pub max_level: Option<SigmaLogLevel>,
    /// Facility filter
    pub facility: Option<SigmaLogFacility>,
    /// Process name filter
    pub process_name: Option<String>,
    /// PID filter
    pub pid: Option<u32>,
    /// Message contains text
    pub message_contains: Option<String>,
    /// Time range (start, end)
    pub time_range: Option<(u64, u64)>,
}

impl LogFilter {
    /// Create a new log filter
    pub fn new() -> Self {
        Self {
            min_level: None,
            max_level: None,
            facility: None,
            process_name: None,
            pid: None,
            message_contains: None,
            time_range: None,
        }
    }

    /// Set minimum log level
    pub fn with_min_level(mut self, level: LogLevel) -> Self {
        self.min_level = Some(level);
        self
    }

    /// Set maximum log level
    pub fn with_max_level(mut self, level: LogLevel) -> Self {
        self.max_level = Some(level);
        self
    }

    /// Set facility filter
    pub fn with_facility(mut self, facility: SigmaLogFacility) -> Self {
        self.facility = Some(facility);
        self
    }

    /// Set process name filter
    pub fn with_process_name(mut self, name: String) -> Self {
        self.process_name = Some(name);
        self
    }

    /// Set PID filter
    pub fn with_pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }

    /// Set message contains filter
    pub fn with_message_contains(mut self, text: String) -> Self {
        self.message_contains = Some(text);
        self
    }

    /// Set time range filter
    pub fn with_time_range(mut self, start: u64, end: u64) -> Self {
        self.time_range = Some((start, end));
        self
    }
}

impl Default for LogFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Log rotation policy
#[derive(Debug, Clone)]
pub struct RotationPolicy {
    /// Maximum size before rotation (in bytes)
    pub max_size: Option<u64>,
    /// Maximum age before rotation (in seconds)
    pub max_age: Option<u64>,
    /// Maximum number of rotated files to keep
    pub max_files: Option<u32>,
    /// Rotate on schedule (cron-like)
    pub rotate_on_schedule: Option<String>,
}

impl RotationPolicy {
    /// Create a new rotation policy
    pub fn new() -> Self {
        Self {
            max_size: Some(10 * 1024 * 1024), // 10 MB default
            max_age: Some(7 * 24 * 3600),     // 7 days default
            max_files: Some(5),               // Keep 5 rotated files
            rotate_on_schedule: None,
        }
    }

    /// Set maximum size
    pub fn with_max_size(mut self, size: u64) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Set maximum age
    pub fn with_max_age(mut self, age: u64) -> Self {
        self.max_age = Some(age);
        self
    }

    /// Set maximum files
    pub fn with_max_files(mut self, files: u32) -> Self {
        self.max_files = Some(files);
        self
    }

    /// Set rotation schedule
    pub fn with_schedule(mut self, schedule: String) -> Self {
        self.rotate_on_schedule = Some(schedule);
        self
    }
}

impl Default for RotationPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaLog configuration
#[derive(Debug, Clone)]
pub struct SigmaLogConfig {
    /// Log storage backend
    pub storage: LogStorage,
    /// Rotation policy
    pub rotation: RotationPolicy,
    /// Default log level
    pub default_level: SigmaLogLevel,
    /// Enable console output
    pub console_output: bool,
    /// Enable persistent storage
    pub persistent: bool,
    /// Maximum in-memory log entries
    pub max_memory_entries: usize,
}

impl SigmaLogConfig {
    /// Create a new default configuration
    pub fn new() -> Self {
        Self {
            storage: LogStorage::Memory,
            rotation: RotationPolicy::default(),
            default_level: SigmaLogLevel::Info,
            console_output: true,
            persistent: false,
            max_memory_entries: 10000,
        }
    }

    /// Set storage backend
    pub fn with_storage(mut self, storage: LogStorage) -> Self {
        self.storage = storage;
        self
    }

    /// Set rotation policy
    pub fn with_rotation(mut self, rotation: RotationPolicy) -> Self {
        self.rotation = rotation;
        self
    }

    /// Set default log level
    pub fn with_default_level(mut self, level: LogLevel) -> Self {
        self.default_level = level;
        self
    }

    /// Enable console output
    pub fn with_console_output(mut self, enabled: bool) -> Self {
        self.console_output = enabled;
        self
    }

    /// Enable persistent storage
    pub fn with_persistent(mut self, enabled: bool) -> Self {
        self.persistent = enabled;
        self
    }

    /// Set maximum memory entries
    pub fn with_max_memory_entries(mut self, entries: usize) -> Self {
        self.max_memory_entries = entries;
        self
    }
}

impl Default for SigmaLogConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaLog - System logging daemon
pub struct SigmaLog {
    /// Configuration
    config: SigmaLogConfig,
    /// In-memory log storage
    logs: Vec<LogEntry>,
    /// Current log ID counter
    log_counter: u64,
    /// Running state
    running: bool,
}

impl SigmaLog {
    /// Create a new SigmaLog instance
    pub fn new(config: SigmaLogConfig) -> Self {
        let max_entries = config.max_memory_entries;
        Self {
            config,
            logs: Vec::with_capacity(max_entries),
            log_counter: 0,
            running: false,
        }
    }

    /// Create SigmaLog with default configuration
    pub fn with_default_config() -> Self {
        Self::new(SigmaLogConfig::default())
    }

    /// Start the logging daemon
    pub fn start(&mut self) {
        self.running = true;
        // In a real implementation, this would start a background thread
        // to handle log processing, rotation, and network I/O
    }

    /// Stop the logging daemon
    pub fn stop(&mut self) {
        self.running = false;
        // In a real implementation, this would gracefully shutdown
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Log a message
    pub fn log(&mut self, entry: LogEntry) {
        if !self.running {
            return;
        }

        // Apply default level filter
        if entry.level > self.config.default_level {
            return;
        }

        // In a real implementation, set actual timestamp
        let mut entry = entry;
        entry.timestamp = self.get_current_time();

        // Store in memory
        if self.logs.len() >= self.config.max_memory_entries {
            // Remove oldest entry
            self.logs.remove(0);
        }
        self.logs.push(entry.clone());

        // Output to console if enabled
        if self.config.console_output {
            self.output_to_console(&entry);
        }

        // Persist if enabled
        if self.config.persistent {
            self.persist_log(&entry);
        }
    }

    /// Get current time (placeholder)
    fn get_current_time(&self) -> u64 {
        // In a real implementation, this would get the actual system time
        0
    }

    /// Output log to console
    fn output_to_console(&self, entry: &LogEntry) {
        // In a real implementation, this would write to stdout/stderr
        // Format: [TIMESTAMP] LEVEL[PROCESS_NAME:PID]: MESSAGE
        let _ = alloc::format!(
            "[{}] {}[{}:{}]: {}",
            entry.timestamp,
            entry.level,
            entry.process_name,
            entry.pid,
            entry.message
        );
    }

    /// Persist log to storage
    fn persist_log(&self, entry: &LogEntry) {
        // In a real implementation, this would write to file or network
        match &self.config.storage {
            LogStorage::Memory => {
                // Already in memory
            }
            LogStorage::File { path } => {
                // Write to file
                let _ = path;
            }
            LogStorage::Network { endpoint } => {
                // Send to network endpoint
                let _ = endpoint;
            }
        }
    }

    /// Query logs with filter
    pub fn query(&self, filter: &LogFilter) -> Vec<LogEntry> {
        self.logs
            .iter()
            .filter(|entry| {
                // Apply level filter
                if let Some(min) = filter.min_level {
                    if entry.level < min {
                        return false;
                    }
                }
                if let Some(max) = filter.max_level {
                    if entry.level > max {
                        return false;
                    }
                }

                // Apply facility filter
                if let Some(facility) = filter.facility {
                    if entry.facility != facility {
                        return false;
                    }
                }

                // Apply process name filter
                if let Some(name) = &filter.process_name {
                    if &entry.process_name != name {
                        return false;
                    }
                }

                // Apply PID filter
                if let Some(pid) = filter.pid {
                    if entry.pid != pid {
                        return false;
                    }
                }

                // Apply message contains filter
                if let Some(text) = &filter.message_contains {
                    if !entry.message.contains(text) {
                        return false;
                    }
                }

                // Apply time range filter
                if let Some((start, end)) = filter.time_range {
                    if entry.timestamp < start || entry.timestamp > end {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Get all logs
    pub fn get_all_logs(&self) -> Vec<LogEntry> {
        self.logs.clone()
    }

    /// Get log count
    pub fn log_count(&self) -> usize {
        self.logs.len()
    }

    /// Clear all logs
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    /// Rotate logs
    pub fn rotate_logs(&mut self) {
        // In a real implementation, this would rotate log files
        // according to the rotation policy
    }

    /// Get configuration
    pub fn config(&self) -> &SigmaLogConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: SigmaLogConfig) {
        self.config = config;
    }
}

impl Default for SigmaLog {
    fn default() -> Self {
        Self::with_default_config()
    }
}

/// Global SigmaLog instance (placeholder for actual global state)
static mut SIGMALOG_INSTANCE: Option<SigmaLog> = None;

/// Initialize the global SigmaLog instance
pub fn init_sigmalog(config: SigmaLogConfig) {
    unsafe {
        SIGMALOG_INSTANCE = Some(SigmaLog::new(config));
    }
}

/// Initialize SigmaLog with default configuration
pub fn init_sigmalog_default() {
    init_sigmalog(SigmaLogConfig::default());
}

/// Get the global SigmaLog instance
pub fn get_sigmalog() -> Option<&'static mut SigmaLog> {
    unsafe { SIGMALOG_INSTANCE.as_mut() }
}

/// Convenience function to log a message
pub fn log_message(
    level: SigmaLogLevel,
    facility: SigmaLogFacility,
    pid: u32,
    process_name: &str,
    message: &str,
) {
    if let Some(logger) = get_sigmalog() {
        let entry = LogEntry::new(
            level,
            facility,
            pid,
            process_name.to_string(),
            message.to_string(),
        );
        logger.log(entry);
    }
}

/// Convenience macro for logging
#[macro_export]
macro_rules! log {
    (level: $level:expr, facility: $facility:expr, pid: $pid:expr, process: $process:expr, $($arg:tt)*) => {
        $crate::logging::sigmalog::log_message(
            $level,
            $facility,
            $pid,
            $process,
            &alloc::format!($($arg)*)
        );
    };
    (error, $($arg:tt)*) => {
        log!(level: $crate::logging::sigmalog::SigmaLogLevel::Error, facility: $crate::logging::sigmalog::SigmaLogFacility::User, pid: 0, process: "sigmaos", $($arg)*);
    };
    (warn, $($arg:tt)*) => {
        log!(level: $crate::logging::sigmalog::SigmaLogLevel::Warning, facility: $crate::logging::sigmalog::SigmaLogFacility::User, pid: 0, process: "sigmaos", $($arg)*);
    };
    (info, $($arg:tt)*) => {
        log!(level: $crate::logging::sigmalog::SigmaLogLevel::Info, facility: $crate::logging::sigmalog::SigmaLogFacility::User, pid: 0, process: "sigmaos", $($arg)*);
    };
    (debug, $($arg:tt)*) => {
        log!(level: $crate::logging::sigmalog::SigmaLogLevel::Debug, facility: $crate::logging::sigmalog::SigmaLogFacility::User, pid: 0, process: "sigmaos", $($arg)*);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_display() {
        assert_eq!(SigmaLogLevel::Error.to_string(), "ERROR");
        assert_eq!(SigmaLogLevel::Info.to_string(), "INFO");
    }

    #[test]
    fn test_log_facility_display() {
        assert_eq!(SigmaLogFacility::Kernel.to_string(), "kernel");
        assert_eq!(SigmaLogFacility::User.to_string(), "user");
    }

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(
            SigmaLogLevel::Info,
            SigmaLogFacility::User,
            1234,
            "test".to_string(),
            "Test message".to_string(),
        );
        assert_eq!(entry.level, SigmaLogLevel::Info);
        assert_eq!(entry.facility, SigmaLogFacility::User);
        assert_eq!(entry.pid, 1234);
        assert_eq!(entry.process_name, "test");
        assert_eq!(entry.message, "Test message");
    }

    #[test]
    fn test_log_entry_with_field() {
        let entry = LogEntry::new(
            SigmaLogLevel::Info,
            SigmaLogFacility::User,
            1234,
            "test".to_string(),
            "Test message".to_string(),
        )
        .with_field("key".to_string(), "value".to_string());
        assert_eq!(entry.fields.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_log_filter() {
        let filter = LogFilter::new()
            .with_min_level(SigmaLogLevel::Warning)
            .with_facility(SigmaLogFacility::Kernel);
        assert_eq!(filter.min_level, Some(SigmaLogLevel::Warning));
        assert_eq!(filter.facility, Some(SigmaLogFacility::Kernel));
    }

    #[test]
    fn test_rotation_policy() {
        let policy = RotationPolicy::new()
            .with_max_size(100_000_000)
            .with_max_files(10);
        assert_eq!(policy.max_size, Some(100_000_000));
        assert_eq!(policy.max_files, Some(10));
    }

    #[test]
    fn test_sigmalog_config() {
        let config = SigmaLogConfig::new()
            .with_default_level(SigmaLogLevel::Debug)
            .with_console_output(false);
        assert_eq!(config.default_level, SigmaLogLevel::Debug);
        assert_eq!(config.console_output, false);
    }

    #[test]
    fn test_sigmalog_creation() {
        let mut logger = SigmaLog::with_default_config();
        assert!(!logger.is_running());
        logger.start();
        assert!(logger.is_running());
        logger.stop();
        assert!(!logger.is_running());
    }

    #[test]
    fn test_sigmalog_logging() {
        let mut logger = SigmaLog::with_default_config();
        logger.start();

        let entry = LogEntry::new(
            SigmaLogLevel::Info,
            SigmaLogFacility::User,
            1234,
            "test".to_string(),
            "Test message".to_string(),
        );
        logger.log(entry);

        assert_eq!(logger.log_count(), 1);
    }

    #[test]
    fn test_sigmalog_query() {
        let mut logger = SigmaLog::with_default_config();
        logger.start();

        let entry1 = LogEntry::new(
            SigmaLogLevel::Error,
            SigmaLogFacility::User,
            1234,
            "test".to_string(),
            "Error message".to_string(),
        );
        let entry2 = LogEntry::new(
            SigmaLogLevel::Info,
            SigmaLogFacility::User,
            1234,
            "test".to_string(),
            "Info message".to_string(),
        );
        logger.log(entry1);
        logger.log(entry2);

        let filter = LogFilter::new().with_min_level(SigmaLogLevel::Error);
        let results = logger.query(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].level, SigmaLogLevel::Error);
    }

    #[test]
    fn test_sigmalog_clear() {
        let mut logger = SigmaLog::with_default_config();
        logger.start();

        let entry = LogEntry::new(
            SigmaLogLevel::Info,
            SigmaLogFacility::User,
            1234,
            "test".to_string(),
            "Test message".to_string(),
        );
        logger.log(entry);

        assert_eq!(logger.log_count(), 1);
        logger.clear_logs();
        assert_eq!(logger.log_count(), 0);
    }
}
