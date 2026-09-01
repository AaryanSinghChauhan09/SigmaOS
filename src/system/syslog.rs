#![cfg_attr(not(test), no_std)]
// SigmaOS Syslog System
// Linux/BSD distro-inspired logging system
// Handles system logging, log rotation, and log management

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Emergency, // 0
    Alert,     // 1
    Critical,  // 2
    Error,     // 3
    Warning,   // 4
    Notice,    // 5
    Info,      // 6
    Debug,     // 7
}

impl LogLevel {
    pub fn from_u8(level: u8) -> Option<Self> {
        match level {
            0 => Some(LogLevel::Emergency),
            1 => Some(LogLevel::Alert),
            2 => Some(LogLevel::Critical),
            3 => Some(LogLevel::Error),
            4 => Some(LogLevel::Warning),
            5 => Some(LogLevel::Notice),
            6 => Some(LogLevel::Info),
            7 => Some(LogLevel::Debug),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            LogLevel::Emergency => 0,
            LogLevel::Alert => 1,
            LogLevel::Critical => 2,
            LogLevel::Error => 3,
            LogLevel::Warning => 4,
            LogLevel::Notice => 5,
            LogLevel::Info => 6,
            LogLevel::Debug => 7,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LogLevel::Emergency => "EMERGENCY",
            LogLevel::Alert => "ALERT",
            LogLevel::Critical => "CRITICAL",
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARNING",
            LogLevel::Notice => "NOTICE",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

/// Log facility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFacility {
    Kernel,   // 0
    User,     // 1
    Mail,     // 2
    Daemon,   // 3
    Auth,     // 4
    Syslog,   // 5
    Lpr,      // 6
    News,     // 7
    Uucp,     // 8
    Cron,     // 9
    AuthPriv, // 10
    Ftp,      // 11
    Ntp,      // 12
    LogAudit, // 13
    LogAlert, // 14
    Clock,    // 15
    Local0,   // 16
    Local1,   // 17
    Local2,   // 18
    Local3,   // 19
    Local4,   // 20
    Local5,   // 21
    Local6,   // 22
    Local7,   // 23
}

impl LogFacility {
    pub fn from_u8(facility: u8) -> Option<Self> {
        match facility {
            0 => Some(LogFacility::Kernel),
            1 => Some(LogFacility::User),
            2 => Some(LogFacility::Mail),
            3 => Some(LogFacility::Daemon),
            4 => Some(LogFacility::Auth),
            5 => Some(LogFacility::Syslog),
            6 => Some(LogFacility::Lpr),
            7 => Some(LogFacility::News),
            8 => Some(LogFacility::Uucp),
            9 => Some(LogFacility::Cron),
            10 => Some(LogFacility::AuthPriv),
            11 => Some(LogFacility::Ftp),
            12 => Some(LogFacility::Ntp),
            13 => Some(LogFacility::LogAudit),
            14 => Some(LogFacility::LogAlert),
            15 => Some(LogFacility::Clock),
            16 => Some(LogFacility::Local0),
            17 => Some(LogFacility::Local1),
            18 => Some(LogFacility::Local2),
            19 => Some(LogFacility::Local3),
            20 => Some(LogFacility::Local4),
            21 => Some(LogFacility::Local5),
            22 => Some(LogFacility::Local6),
            23 => Some(LogFacility::Local7),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            LogFacility::Kernel => 0,
            LogFacility::User => 1,
            LogFacility::Mail => 2,
            LogFacility::Daemon => 3,
            LogFacility::Auth => 4,
            LogFacility::Syslog => 5,
            LogFacility::Lpr => 6,
            LogFacility::News => 7,
            LogFacility::Uucp => 8,
            LogFacility::Cron => 9,
            LogFacility::AuthPriv => 10,
            LogFacility::Ftp => 11,
            LogFacility::Ntp => 12,
            LogFacility::LogAudit => 13,
            LogFacility::LogAlert => 14,
            LogFacility::Clock => 15,
            LogFacility::Local0 => 16,
            LogFacility::Local1 => 17,
            LogFacility::Local2 => 18,
            LogFacility::Local3 => 19,
            LogFacility::Local4 => 20,
            LogFacility::Local5 => 21,
            LogFacility::Local6 => 22,
            LogFacility::Local7 => 23,
        }
    }
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub facility: LogFacility,
    pub level: LogLevel,
    pub process: String,
    pub pid: u32,
    pub message: String,
    pub hostname: String,
}

/// Syslog manager
pub struct SyslogManager {
    pub log_level: LogLevel,
    pub log_facility: LogFacility,
    pub log_file: String,
    pub log_entries: Vec<LogEntry>,
    pub log_rules: Vec<LogRule>,
    pub max_entries: usize,
}

impl SyslogManager {
    pub fn new(log_file: &str) -> Self {
        Self {
            log_level: LogLevel::Info,
            log_facility: LogFacility::User,
            log_file: String::from(log_file),
            log_entries: Vec::new(),
            log_rules: Vec::new(),
            max_entries: 10000,
        }
    }

    /// Initialize syslog manager
    pub fn initialize(&mut self) -> Result<(), SyslogError> {
        // Create default log rules
        self.create_default_rules();
        Ok(())
    }

    /// Log a message
    pub fn log(
        &mut self,
        level: LogLevel,
        facility: LogFacility,
        process: &str,
        pid: u32,
        message: &str,
    ) {
        if level.to_u8() > self.log_level.to_u8() {
            return;
        }

        let entry = LogEntry {
            timestamp: self.get_timestamp(),
            facility,
            level,
            process: String::from(process),
            pid,
            message: String::from(message),
            hostname: String::from("sigmaos"),
        };

        self.log_entries.push(entry);

        // Trim entries if max exceeded
        if self.log_entries.len() > self.max_entries {
            self.log_entries.remove(0);
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> String {
        // In real implementation, get actual timestamp
        String::from("2026-08-13T12:00:00Z")
    }

    /// Set log level
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    /// Set log facility
    pub fn set_log_facility(&mut self, facility: LogFacility) {
        self.log_facility = facility;
    }

    /// Add log rule
    pub fn add_rule(&mut self, rule: LogRule) {
        self.log_rules.push(rule);
    }

    /// Create default log rules
    fn create_default_rules(&mut self) {
        // Emergency messages always logged
        self.log_rules.push(LogRule {
            facility: LogFacility::Kernel,
            level: LogLevel::Emergency,
            action: LogAction::File(String::from("/var/log/emergency.log")),
        });

        // Auth messages to auth.log
        self.log_rules.push(LogRule {
            facility: LogFacility::Auth,
            level: LogLevel::Info,
            action: LogAction::File(String::from("/var/log/auth.log")),
        });

        // Cron messages to cron.log
        self.log_rules.push(LogRule {
            facility: LogFacility::Cron,
            level: LogLevel::Info,
            action: LogAction::File(String::from("/var/log/cron.log")),
        });
    }

    /// Get log entries by level
    pub fn get_entries_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.log_entries
            .iter()
            .filter(|entry| entry.level == level)
            .collect()
    }

    /// Get log entries by facility
    pub fn get_entries_by_facility(&self, facility: LogFacility) -> Vec<&LogEntry> {
        self.log_entries
            .iter()
            .filter(|entry| entry.facility == facility)
            .collect()
    }

    /// Get recent log entries
    pub fn get_recent_entries(&self, count: usize) -> Vec<&LogEntry> {
        let start = if self.log_entries.len() > count {
            self.log_entries.len() - count
        } else {
            0
        };
        self.log_entries[start..].iter().collect()
    }

    /// Clear log entries
    pub fn clear_entries(&mut self) {
        self.log_entries.clear();
    }

    /// Save logs to file
    pub fn save_logs(&self) -> Result<(), SyslogError> {
        // In real implementation, save to log file
        Ok(())
    }

    /// Load logs from file
    pub fn load_logs(&mut self) -> Result<(), SyslogError> {
        // In real implementation, load from log file
        Ok(())
    }
}

/// Log rule
#[derive(Debug, Clone)]
pub struct LogRule {
    pub facility: LogFacility,
    pub level: LogLevel,
    pub action: LogAction,
}

/// Log action
#[derive(Debug, Clone)]
pub enum LogAction {
    File(String),
    Console,
    Remote(String),
    Discard,
}

/// Syslog errors
#[derive(Debug)]
pub enum SyslogError {
    LogFileError(String),
    ConfigurationError(String),
    RotationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels() {
        assert_eq!(LogLevel::from_u8(3), Some(LogLevel::Error));
        assert_eq!(LogLevel::Error.to_u8(), 3);
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }

    #[test]
    fn test_log_facilities() {
        assert_eq!(LogFacility::from_u8(3), Some(LogFacility::Daemon));
        assert_eq!(LogFacility::Daemon.to_u8(), 3);
    }

    #[test]
    fn test_syslog_manager() {
        let mut manager = SyslogManager::new("/var/log/syslog");
        manager.initialize().unwrap();

        manager.log(
            LogLevel::Info,
            LogFacility::User,
            "test",
            1234,
            "Test message",
        );
        assert_eq!(manager.log_entries.len(), 1);
    }

    #[test]
    fn test_log_level_filtering() {
        let mut manager = SyslogManager::new("/var/log/syslog");
        manager.set_log_level(LogLevel::Error);
        manager.initialize().unwrap();

        manager.log(
            LogLevel::Debug,
            LogFacility::User,
            "test",
            1234,
            "Debug message",
        );
        manager.log(
            LogLevel::Error,
            LogFacility::User,
            "test",
            1234,
            "Error message",
        );

        assert_eq!(manager.log_entries.len(), 1);
    }

    #[test]
    fn test_log_filtering() {
        let mut manager = SyslogManager::new("/var/log/syslog");
        manager.initialize().unwrap();

        manager.log(
            LogLevel::Error,
            LogFacility::Auth,
            "test",
            1234,
            "Auth error",
        );
        manager.log(LogLevel::Info, LogFacility::Cron, "test", 1234, "Cron info");

        let auth_entries = manager.get_entries_by_facility(LogFacility::Auth);
        assert_eq!(auth_entries.len(), 1);
    }
}
