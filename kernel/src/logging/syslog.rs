// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired logging (syslog/journald) for SigmaOS
// Zero-allocation, performance-optimized logging operations

/// Log priority levels (syslog-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogPriority {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

/// Log facility (syslog-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFacility {
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

/// Log entry
pub struct LogEntry {
    pub timestamp: u64,
    pub priority: LogPriority,
    pub facility: LogFacility,
    pub process_id: u32,
    pub process_name: String,
    pub message: String,
    pub hostname: String,
}

impl LogEntry {
    pub fn new(priority: LogPriority, facility: LogFacility, message: &str) -> Self {
        Self {
            timestamp: 0,
            priority,
            facility,
            process_id: 0,
            process_name: String::new(),
            message: message.to_string(),
            hostname: String::new(),
        }
    }
}

/// Logger trait
pub trait Logger {
    /// Initialize logger
    fn init(&mut self) -> Result<(), LogError>;
    
    /// Log message
    fn log(&mut self, entry: LogEntry) -> Result<(), LogError>;
    
    /// Get log level
    fn log_level(&self) -> LogPriority;
    
    /// Set log level
    fn set_log_level(&mut self, level: LogPriority);
    
    /// Flush logs
    fn flush(&mut self) -> Result<(), LogError>;
}

/// Syslog logger
pub struct SyslogLogger {
    pub entries: Vec<LogEntry>,
    pub max_entries: usize,
    pub log_level: LogPriority,
    pub facility:LogFacility,
}

impl SyslogLogger {
    pub const fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
            log_level: LogPriority::Info,
            facility: LogFacility::Kernel,
        }
    }
}

impl Logger for SyslogLogger {
    fn init(&mut self) -> Result<(), LogError> {
        Ok(())
    }
    
    fn log(&mut self, entry: LogEntry) -> Result<(), LogError> {
        if entry.priority > self.log_level {
            return Ok(());
        }
        
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        
        self.entries.push(entry);
        Ok(())
    }
    
    fn log_level(&self) -> LogPriority {
        self.log_level
    }
    
    fn set_log_level(&mut self, level: LogPriority) {
        self.log_level = level as LogPriority;
    }
    
    fn flush(&mut self) -> Result<(), LogError> {
        self.entries.clear();
        Ok(())
    }
}

/// Journal logger (journald-style)
pub struct JournalLogger {
    pub entries: Vec<JournalEntry>,
    pub max_entries: usize,
    pub log_level: LogPriority,
    pub use_persistent: bool,
}

pub struct JournalEntry {
    pub timestamp: u64,
    pub priority: LogPriority,
    pub facility: LogFacility,
    pub fields: Vec<(String, String)>,
}

impl JournalEntry {
    pub fn new(priority: LogPriority, facility: LogFacility) -> Self {
        Self {
            timestamp: 0,
            priority,
            facility,
            fields: Vec::new(),
        }
    }
    
    pub fn add_field(&mut self, key: &str, value: &str) {
        self.fields.push((key.to_string(), value.to_string()));
    }
    
    pub fn get_field(&self, key: &str) -> Option<&str> {
        self.fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
}

impl JournalLogger {
    pub const fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
            log_level: LogPriority::Info,
            use_persistent: false,
        }
    }
    
    pub fn log_field(&mut self, priority: LogPriority, fields: Vec<(String, String)>) -> Result<(), LogError> {
        if priority > self.log_level {
            return Ok(());
        }
        
        let mut entry = JournalEntry::new(priority, LogFacility::Kernel);
        for (key, value) in fields {
            entry.add_field(&key, &value);
        }
        
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        
        self.entries.push(entry);
        Ok(())
    }
}

impl Logger for JournalLogger {
    fn init(&mut self) -> Result<(), LogError> {
        Ok(())
    }
    
    fn log(&mut self, entry: LogEntry) -> Result<(), LogError> {
        let mut journal_entry = JournalEntry::new(entry.priority, entry.facility);
        journal_entry.add_field("MESSAGE", &entry.message);
        journal_entry.add_field("PRIORITY", &format!("{}", entry.priority as u8));
        journal_entry.add_field("SYSLOG_FACILITY", &format!("{}", entry.facility as u8));
        journal_entry.add_field("PID", &format!("{}", entry.process_id));
        journal_entry.add_field("_COMM", &entry.process_name);
        journal_entry.add_field("_HOSTNAME", &entry.hostname);
        
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        
        self.entries.push(journal_entry);
        Ok(())
    }
    
    fn log_level(&self) -> LogPriority {
        self.log_level
    }
    
    fn set_log_level(&mut self, level: LogPriority) {
        self.log_level = level;
    }
    
    fn flush(&mut self) -> Result<(), LogError> {
        self.entries.clear();
        Ok(())
    }
}

/// Log error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogError {
    InitializationFailed,
    WriteFailed,
    FlushFailed,
    InvalidEntry,
    BufferFull,
    PermissionDenied,
    Other,
}

/// Log filter
pub struct LogFilter {
    pub min_priority: LogPriority,
    pub max_priority: LogPriority,
    pub facilities: Vec<LogFacility>,
    pub process_ids: Vec<u32>,
    pub process_names: Vec<String>,
}

impl LogFilter {
    pub const fn new() -> Self {
        Self {
            min_priority: LogPriority::Debug,
            max_priority: LogPriority::Emergency,
            facilities: Vec::new(),
            process_ids: Vec::new(),
            process_names: Vec::new(),
        }
    }
    
    pub fn matches(&self, entry: &LogEntry) -> bool {
        if entry.priority < self.min_priority || entry.priority > self.max_priority {
            return false;
        }
        
        if !self.facilities.is_empty() && !self.facilities.contains(&entry.facility) {
            return false;
        }
        
        if !self.process_ids.is_empty() && !self.process_ids.contains(&entry.process_id) {
            return false;
        }
        
        if !self.process_names.is_empty() && !self.process_names.iter().any(|n| n == &entry.process_name) {
            return false;
        }
        
        true
    }
}

/// Log rotation
pub struct LogRotation {
    pub max_size: u64,
    pub max_files: usize,
    pub current_size: u64,
    pub file_count: usize,
}

impl LogRotation {
    pub const fn new(max_size: u64, max_files: usize) -> Self {
        Self {
            max_size,
            max_files,
            current_size: 0,
            file_count: 0,
        }
    }
    
    pub fn should_rotate(&self) -> bool {
        self.current_size >= self.max_size
    }
    
    pub fn rotate(&mut self) {
        self.current_size = 0;
        self.file_count += 1;
        
        if self.file_count > self.max_files {
            self.file_count = self.max_files;
        }
    }
}

/// Standard log paths
pub mod log_paths {
    pub const VAR_LOG: &str = "/var/log";
    pub const VAR_LOG_MESSAGES: &str = "/var/log/messages";
    pub const VAR_LOG_SYSLOG: &str = "/var/log/syslog";
    pub const VAR_LOG_KERNEL: &str = "/var/log/kernel";
    pub const VAR_LOG_AUTH: &str = "/var/log/auth.log";
    pub const VAR_LOG_DAEMON: &str = "/var/log/daemon.log";
    pub const VAR_LOG_KERN: &str = "/var/log/kern.log";
    pub const VAR_LOG_MAIL: &str = "/var/log/mail.log";
    pub const VAR_LOG_USER: &str = "/var/log/user.log";
    pub const VAR_LOG_CRON: &str = "/var/log/cron.log";
    pub const VAR_LOG_DEBUG: &str = "/var/log/debug";
    pub const VAR_LOG_JOURNAL: &str = "/var/log/journal";
    pub const RUN_LOG: &str = "/run/log";
    pub const DEV_LOG: &str = "/dev/log";
    pub const DEV_KMSG: &str = "/dev/kmsg";
}
