// SPDX-License-Identifier: MIT
// SigmaOS - Log Manager
// Log management inspired by Arch (journalctl), Debian (rsyslog), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Critical = 4,
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
    pub process_id: Option<u32>,
}

/// Log manager
pub struct SigmaLogManager {
    logs: Vec<LogEntry>,
    max_logs: usize,
}

impl SigmaLogManager {
    pub fn new(max_logs: usize) -> Self {
        Self {
            logs: Vec::new(),
            max_logs,
        }
    }

    pub fn log(&mut self, level: LogLevel, source: String, message: String, process_id: Option<u32>) {
        let entry = LogEntry {
            timestamp: 0,
            level,
            source,
            message,
            process_id,
        };
        self.logs.push(entry);
        if self.logs.len() > self.max_logs {
            self.logs.remove(0);
        }
    }

    pub fn get_logs(&self) -> &[LogEntry] {
        &self.logs
    }

    pub fn get_logs_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.level == level).collect()
    }

    pub fn get_logs_by_source(&self, source: &str) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.source == source).collect()
    }

    pub fn get_logs_by_process(&self, pid: u32) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.process_id == Some(pid)).collect()
    }

    pub fn get_error_logs(&self) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.level >= LogLevel::Error).collect()
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    pub fn get_log_count(&self) -> usize {
        self.logs.len()
    }

    pub fn search_logs(&self, query: &str) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.message.contains(query)).collect()
    }
}

impl Default for SigmaLogManager {
    fn default() -> Self {
        Self::new(10000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_manager_creation() {
        let manager = SigmaLogManager::new(100);
        assert_eq!(manager.get_logs().len(), 0);
        assert_eq!(manager.get_log_count(), 0);
    }

    #[test]
    fn test_log() {
        let mut manager = SigmaLogManager::new(100);
        manager.log(LogLevel::Info, "test".to_string(), "Test message".to_string(), Some(1));
        assert_eq!(manager.get_logs().len(), 1);
    }

    #[test]
    fn test_logs_by_level() {
        let mut manager = SigmaLogManager::new(100);
        manager.log(LogLevel::Info, "test".to_string(), "Info message".to_string(), Some(1));
        manager.log(LogLevel::Error, "test".to_string(), "Error message".to_string(), Some(1));
        assert_eq!(manager.get_logs_by_level(LogLevel::Error).len(), 1);
    }

    #[test]
    fn test_logs_by_source() {
        let mut manager = SigmaLogManager::new(100);
        manager.log(LogLevel::Info, "kernel".to_string(), "Kernel message".to_string(), Some(1));
        manager.log(LogLevel::Info, "user".to_string(), "User message".to_string(), Some(1));
        assert_eq!(manager.get_logs_by_source("kernel").len(), 1);
    }

    #[test]
    fn test_error_logs() {
        let mut manager = SigmaLogManager::new(100);
        manager.log(LogLevel::Info, "test".to_string(), "Info message".to_string(), Some(1));
        manager.log(LogLevel::Error, "test".to_string(), "Error message".to_string(), Some(1));
        manager.log(LogLevel::Critical, "test".to_string(), "Critical message".to_string(), Some(1));
        assert_eq!(manager.get_error_logs().len(), 2);
    }

    #[test]
    fn test_max_logs() {
        let mut manager = SigmaLogManager::new(3);
        manager.log(LogLevel::Info, "test".to_string(), "Message 1".to_string(), Some(1));
        manager.log(LogLevel::Info, "test".to_string(), "Message 2".to_string(), Some(1));
        manager.log(LogLevel::Info, "test".to_string(), "Message 3".to_string(), Some(1));
        manager.log(LogLevel::Info, "test".to_string(), "Message 4".to_string(), Some(1));
        assert_eq!(manager.get_logs().len(), 3);
    }

    #[test]
    fn test_search_logs() {
        let mut manager = SigmaLogManager::new(100);
        manager.log(LogLevel::Info, "test".to_string(), "Error occurred".to_string(), Some(1));
        manager.log(LogLevel::Info, "test".to_string(), "Info message".to_string(), Some(1));
        assert_eq!(manager.search_logs("Error").len(), 1);
    }
}
