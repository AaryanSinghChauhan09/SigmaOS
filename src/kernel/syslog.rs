// Linux-inspired Syslog (System Logging)
// Provides system logging with severity levels and facilities

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Syslog severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyslogSeverity {
    Emergency = 0,  // System is unusable
    Alert = 1,      // Action must be taken immediately
    Critical = 2,   // Critical conditions
    Error = 3,      // Error conditions
    Warning = 4,    // Warning conditions
    Notice = 5,     // Normal but significant condition
    Info = 6,       // Informational
    Debug = 7,      // Debug-level messages
}

impl SyslogSeverity {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(SyslogSeverity::Emergency),
            1 => Some(SyslogSeverity::Alert),
            2 => Some(SyslogSeverity::Critical),
            3 => Some(SyslogSeverity::Error),
            4 => Some(SyslogSeverity::Warning),
            5 => Some(SyslogSeverity::Notice),
            6 => Some(SyslogSeverity::Info),
            7 => Some(SyslogSeverity::Debug),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SyslogSeverity::Emergency => "emerg",
            SyslogSeverity::Alert => "alert",
            SyslogSeverity::Critical => "crit",
            SyslogSeverity::Error => "err",
            SyslogSeverity::Warning => "warning",
            SyslogSeverity::Notice => "notice",
            SyslogSeverity::Info => "info",
            SyslogSeverity::Debug => "debug",
        }
    }
}

/// Syslog facilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyslogFacility {
    Kernel = 0,
    User = 1,
    Mail = 2,
    Daemon = 3,
    Auth = 4,
    Syslog = 5,
    Lpr = 6,
    News = 7,
    Uucp = 8,
    Cron = 9,
    Authpriv = 10,
    Ftp = 11,
    Ntp = 12,
    Audit = 13,
    Alert = 14,
    Clock = 15,
    Local0 = 16,
    Local1 = 17,
    Local2 = 18,
    Local3 = 19,
    Local4 = 20,
    Local5 = 21,
    Local6 = 22,
    Local7 = 23,
}

impl SyslogFacility {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(SyslogFacility::Kernel),
            1 => Some(SyslogFacility::User),
            2 => Some(SyslogFacility::Mail),
            3 => Some(SyslogFacility::Daemon),
            4 => Some(SyslogFacility::Auth),
            5 => Some(SyslogFacility::Syslog),
            6 => Some(SyslogFacility::Lpr),
            7 => Some(SyslogFacility::News),
            8 => Some(SyslogFacility::Uucp),
            9 => Some(SyslogFacility::Cron),
            10 => Some(SyslogFacility::Authpriv),
            11 => Some(SyslogFacility::Ftp),
            12 => Some(SyslogFacility::Ntp),
            13 => Some(SyslogFacility::Audit),
            14 => Some(SyslogFacility::Alert),
            15 => Some(SyslogFacility::Clock),
            16 => Some(SyslogFacility::Local0),
            17 => Some(SyslogFacility::Local1),
            18 => Some(SyslogFacility::Local2),
            19 => Some(SyslogFacility::Local3),
            20 => Some(SyslogFacility::Local4),
            21 => Some(SyslogFacility::Local5),
            22 => Some(SyslogFacility::Local6),
            23 => Some(SyslogFacility::Local7),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SyslogFacility::Kernel => "kernel",
            SyslogFacility::User => "user",
            SyslogFacility::Mail => "mail",
            SyslogFacility::Daemon => "daemon",
            SyslogFacility::Auth => "auth",
            SyslogFacility::Syslog => "syslog",
            SyslogFacility::Lpr => "lpr",
            SyslogFacility::News => "news",
            SyslogFacility::Uucp => "uucp",
            SyslogFacility::Cron => "cron",
            SyslogFacility::Authpriv => "authpriv",
            SyslogFacility::Ftp => "ftp",
            SyslogFacility::Ntp => "ntp",
            SyslogFacility::Audit => "audit",
            SyslogFacility::Alert => "alert",
            SyslogFacility::Clock => "clock",
            SyslogFacility::Local0 => "local0",
            SyslogFacility::Local1 => "local1",
            SyslogFacility::Local2 => "local2",
            SyslogFacility::Local3 => "local3",
            SyslogFacility::Local4 => "local4",
            SyslogFacility::Local5 => "local5",
            SyslogFacility::Local6 => "local6",
            SyslogFacility::Local7 => "local7",
        }
    }
}

/// Syslog entry
#[derive(Debug, Clone)]
pub struct SyslogEntry {
    pub timestamp: u64,
    pub facility: SyslogFacility,
    pub severity: SyslogSeverity,
    pub process: String,
    pub pid: u32,
    pub message: String,
}

impl SyslogEntry {
    pub fn new(
        facility: SyslogFacility,
        severity: SyslogSeverity,
        process: String,
        pid: u32,
        message: String,
    ) -> Self {
        Self {
            timestamp: 0, // Would be set to actual timestamp
            facility,
            severity,
            process,
            pid,
            message,
        }
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Calculate priority value (facility * 8 + severity)
    pub fn priority(&self) -> u8 {
        (self.facility as u8) * 8 + (self.severity as u8)
    }
}

/// Syslog buffer
pub struct SyslogBuffer {
    entries: VecDeque<SyslogEntry>,
    max_size: usize,
}

impl SyslogBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_size,
        }
    }

    /// Add an entry to the buffer
    pub fn add(&mut self, entry: SyslogEntry) {
        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    /// Get all entries
    pub fn get_all(&self) -> Vec<SyslogEntry> {
        self.entries.iter().cloned().collect()
    }

    /// Get entries by severity
    pub fn get_by_severity(&self, severity: SyslogSeverity) -> Vec<SyslogEntry> {
        self.entries
            .iter()
            .filter(|e| e.severity == severity)
            .cloned()
            .collect()
    }

    /// Get entries by facility
    pub fn get_by_facility(&self, facility: SyslogFacility) -> Vec<SyslogEntry> {
        self.entries
            .iter()
            .filter(|e| e.facility == facility)
            .cloned()
            .collect()
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for SyslogBuffer {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Syslog manager for system-wide logging
pub struct SyslogManager {
    buffer: Arc<Mutex<SyslogBuffer>>,
}

impl SyslogManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(SyslogBuffer::new(max_size))),
        }
    }

    /// Log a message
    pub fn log(
        &self,
        facility: SyslogFacility,
        severity: SyslogSeverity,
        process: String,
        pid: u32,
        message: String,
    ) {
        let entry = SyslogEntry::new(facility, severity, process, pid, message);
        let mut buffer = self.buffer.lock().unwrap();
        buffer.add(entry);
    }

    /// Get all entries
    pub fn get_all(&self) -> Vec<SyslogEntry> {
        let buffer = self.buffer.lock().unwrap();
        buffer.get_all()
    }

    /// Get entries by severity
    pub fn get_by_severity(&self, severity: SyslogSeverity) -> Vec<SyslogEntry> {
        let buffer = self.buffer.lock().unwrap();
        buffer.get_by_severity(severity)
    }

    /// Get entries by facility
    pub fn get_by_facility(&self, facility: SyslogFacility) -> Vec<SyslogEntry> {
        let buffer = self.buffer.lock().unwrap();
        buffer.get_by_facility(facility)
    }

    /// Clear all entries
    pub fn clear(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    /// Get entry count
    pub fn len(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }
}

impl Default for SyslogManager {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syslog_severity_from_u8() {
        assert_eq!(SyslogSeverity::from_u8(0), Some(SyslogSeverity::Emergency));
        assert_eq!(SyslogSeverity::from_u8(7), Some(SyslogSeverity::Debug));
        assert_eq!(SyslogSeverity::from_u8(8), None);
    }

    #[test]
    fn test_syslog_severity_as_str() {
        assert_eq!(SyslogSeverity::Emergency.as_str(), "emerg");
        assert_eq!(SyslogSeverity::Error.as_str(), "err");
        assert_eq!(SyslogSeverity::Info.as_str(), "info");
    }

    #[test]
    fn test_syslog_severity_ordering() {
        assert!(SyslogSeverity::Emergency < SyslogSeverity::Error);
        assert!(SyslogSeverity::Error < SyslogSeverity::Warning);
        assert!(SyslogSeverity::Info < SyslogSeverity::Debug);
    }

    #[test]
    fn test_syslog_facility_from_u8() {
        assert_eq!(SyslogFacility::from_u8(0), Some(SyslogFacility::Kernel));
        assert_eq!(SyslogFacility::from_u8(1), Some(SyslogFacility::User));
        assert_eq!(SyslogFacility::from_u8(23), Some(SyslogFacility::Local7));
        assert_eq!(SyslogFacility::from_u8(24), None);
    }

    #[test]
    fn test_syslog_facility_as_str() {
        assert_eq!(SyslogFacility::Kernel.as_str(), "kernel");
        assert_eq!(SyslogFacility::Daemon.as_str(), "daemon");
        assert_eq!(SyslogFacility::Local0.as_str(), "local0");
    }

    #[test]
    fn test_syslog_entry() {
        let entry = SyslogEntry::new(
            SyslogFacility::Daemon,
            SyslogSeverity::Info,
            "test".to_string(),
            123,
            "Test message".to_string(),
        );

        assert_eq!(entry.facility, SyslogFacility::Daemon);
        assert_eq!(entry.severity, SyslogSeverity::Info);
        assert_eq!(entry.process, "test");
        assert_eq!(entry.pid, 123);
        assert_eq!(entry.message, "Test message");
    }

    #[test]
    fn test_syslog_entry_priority() {
        let entry = SyslogEntry::new(
            SyslogFacility::Daemon,
            SyslogSeverity::Info,
            "test".to_string(),
            123,
            "Test".to_string(),
        );

        // Daemon (3) * 8 + Info (6) = 30
        assert_eq!(entry.priority(), 30);
    }

    #[test]
    fn test_syslog_buffer() {
        let mut buffer = SyslogBuffer::new(3);

        let entry1 = SyslogEntry::new(
            SyslogFacility::Daemon,
            SyslogSeverity::Info,
            "test".to_string(),
            123,
            "Message 1".to_string(),
        );

        let entry2 = SyslogEntry::new(
            SyslogFacility::Daemon,
            SyslogSeverity::Warning,
            "test".to_string(),
            123,
            "Message 2".to_string(),
        );

        buffer.add(entry1);
        buffer.add(entry2);

        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_syslog_buffer_max_size() {
        let mut buffer = SyslogBuffer::new(2);

        for i in 0..3 {
            let entry = SyslogEntry::new(
                SyslogFacility::Daemon,
                SyslogSeverity::Info,
                "test".to_string(),
                123,
                format!("Message {}", i),
            );
            buffer.add(entry);
        }

        assert_eq!(buffer.len(), 2);
        let entries = buffer.get_all();
        assert_eq!(entries[0].message, "Message 1");
        assert_eq!(entries[1].message, "Message 2");
    }

    #[test]
    fn test_syslog_buffer_filter() {
        let mut buffer = SyslogBuffer::new(10);

        buffer.add(SyslogEntry::new(
            SyslogFacility::Daemon,
            SyslogSeverity::Error,
            "test".to_string(),
            123,
            "Error".to_string(),
        ));

        buffer.add(SyslogEntry::new(
            SyslogFacility::Daemon,
            SyslogSeverity::Info,
            "test".to_string(),
            123,
            "Info".to_string(),
        ));

        let errors = buffer.get_by_severity(SyslogSeverity::Error);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Error");
    }

    #[test]
    fn test_syslog_manager() {
        let manager = SyslogManager::new(10);

        manager.log(
            SyslogFacility::Daemon,
            SyslogSeverity::Info,
            "test".to_string(),
            123,
            "Test message".to_string(),
        );

        assert_eq!(manager.len(), 1);

        let entries = manager.get_all();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].message, "Test message");
    }

    #[test]
    fn test_syslog_manager_filter() {
        let manager = SyslogManager::new(10);

        manager.log(
            SyslogFacility::Daemon,
            SyslogSeverity::Error,
            "test".to_string(),
            123,
            "Error".to_string(),
        );

        manager.log(
            SyslogFacility::Kernel,
            SyslogSeverity::Info,
            "test".to_string(),
            123,
            "Info".to_string(),
        );

        let errors = manager.get_by_severity(SyslogSeverity::Error);
        assert_eq!(errors.len(), 1);

        let daemon = manager.get_by_facility(SyslogFacility::Daemon);
        assert_eq!(daemon.len(), 1);
    }
}
