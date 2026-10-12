// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaAudit - System Audit Daemon
//!
//! SigmaAudit provides system audit logging for SigmaOS, inspired by auditd from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Audit event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventType {
    /// System call
    Syscall,
    /// File access
    FileAccess,
    /// Process execution
    ProcessExec,
    /// Network activity
    Network,
    /// Authentication
    Auth,
}

impl fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEventType::Syscall => write!(f, "syscall"),
            AuditEventType::FileAccess => write!(f, "file_access"),
            AuditEventType::ProcessExec => write!(f, "process_exec"),
            AuditEventType::Network => write!(f, "network"),
            AuditEventType::Auth => write!(f, "auth"),
        }
    }
}

/// Audit severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditSeverity {
    /// Info
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

impl fmt::Display for AuditSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditSeverity::Info => write!(f, "info"),
            AuditSeverity::Warning => write!(f, "warning"),
            AuditSeverity::Error => write!(f, "error"),
            AuditSeverity::Critical => write!(f, "critical"),
        }
    }
}

/// Audit event
#[derive(Debug, Clone)]
pub struct AuditEvent {
    /// Event ID
    pub event_id: String,
    /// Event type
    pub event_type: AuditEventType,
    /// Severity
    pub severity: AuditSeverity,
    /// Process ID
    pub pid: u32,
    /// User ID
    pub uid: u32,
    /// Message
    pub message: String,
    /// Timestamp
    pub timestamp: u64,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(event_id: String, event_type: AuditEventType, severity: AuditSeverity, pid: u32, uid: u32, message: String) -> Self {
        Self {
            event_id,
            event_type,
            severity,
            pid,
            uid,
            message,
            timestamp: 0,
        }
    }

    /// Set timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// SigmaAudit - System Audit Daemon
pub struct SigmaAudit {
    /// Audit events (event_id -> event)
    events: BTreeMap<String, AuditEvent>,
    /// Running state
    running: bool,
    /// Event counter
    event_counter: u32,
}

impl SigmaAudit {
    /// Create a new SigmaAudit instance
    pub fn new() -> Self {
        Self {
            events: BTreeMap::new(),
            running: false,
            event_counter: 0,
        }
    }

    /// Start the audit daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the audit daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Log an audit event
    pub fn log_event(&mut self, event: AuditEvent) -> Result<(), String> {
        if !self.running {
            return Err("SigmaAudit not running".to_string());
        }
        
        if self.events.contains_key(&event.event_id) {
            return Err(format!("Event '{}' already exists", event.event_id));
        }
        
        self.events.insert(event.event_id.clone(), event);
        self.event_counter += 1;
        Ok(())
    }

    /// Remove an audit event
    pub fn remove_event(&mut self, event_id: &str) -> Result<(), String> {
        if self.events.remove(event_id).is_none() {
            return Err(format!("Event '{}' not found", event_id));
        }
        Ok(())
    }

    /// Get an audit event
    pub fn get_event(&self, event_id: &str) -> Option<&AuditEvent> {
        self.events.get(event_id)
    }

    /// List all events
    pub fn list_events(&self) -> Vec<&AuditEvent> {
        self.events.values().collect()
    }

    /// List events by type
    pub fn list_by_type(&self, event_type: AuditEventType) -> Vec<&AuditEvent> {
        self.events.values()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// List events by severity
    pub fn list_by_severity(&self, severity: AuditSeverity) -> Vec<&AuditEvent> {
        self.events.values()
            .filter(|e| e.severity == severity)
            .collect()
    }

    /// List events by PID
    pub fn list_by_pid(&self, pid: u32) -> Vec<&AuditEvent> {
        self.events.values()
            .filter(|e| e.pid == pid)
            .collect()
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get audit statistics
    pub fn audit_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_events".to_string(), self.event_count().to_string());
        stats.insert("critical_events".to_string(), self.list_by_severity(AuditSeverity::Critical).len().to_string());
        stats.insert("error_events".to_string(), self.list_by_severity(AuditSeverity::Error).len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaAudit {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaAudit instance (placeholder for actual global state)
static unsafe mut SIGMAAUDIT_INSTANCE: Option<SigmaAudit> = None;

/// Initialize the global SigmaAudit instance
pub fn init_sigmait() {
    unsafe {
        SIGMAAUDIT_INSTANCE = Some(SigmaAudit::new());
    }
}

/// Get the global SigmaAudit instance
pub fn get_sigmait() -> Option<&'static mut SigmaAudit> {
    unsafe { SIGMAAUDIT_INSTANCE.as_mut() }
}

/// Convenience function to log an audit event
pub fn log_audit_event(event: AuditEvent) -> Result<(), String> {
    if let Some(audit) = get_sigmait() {
        audit.log_event(event)
    } else {
        Err("SigmaAudit not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_type_display() {
        assert_eq!(AuditEventType::Syscall.to_string(), "syscall");
        assert_eq!(AuditEventType::Auth.to_string(), "auth");
    }

    #[test]
    fn test_audit_severity_display() {
        assert_eq!(AuditSeverity::Info.to_string(), "info");
        assert_eq!(AuditSeverity::Critical.to_string(), "critical");
    }

    #[test]
    fn test_audit_event() {
        let event = AuditEvent::new(
            "evt1".to_string(),
            AuditEventType::Syscall,
            AuditSeverity::Info,
            1234,
            1000,
            "System call executed".to_string()
        )
        .with_timestamp(1234567890);

        assert_eq!(event.event_id, "evt1");
        assert_eq!(event.timestamp, 1234567890);
    }

    #[test]
    fn test_sigmait() {
        let mut audit = SigmaAudit::new();
        assert!(!audit.is_running());
        audit.start();
        assert!(audit.is_running());
        audit.stop();
        assert!(!audit.is_running());
    }

    #[test]
    fn test_sigmait_log_event() {
        let mut audit = SigmaAudit::new();
        audit.start();
        
        let event = AuditEvent::new(
            "evt1".to_string(),
            AuditEventType::Syscall,
            AuditSeverity::Info,
            1234,
            1000,
            "System call executed".to_string()
        );
        
        assert!(audit.log_event(event).is_ok());
        assert_eq!(audit.event_count(), 1);
    }

    #[test]
    fn test_sigmait_list_by_type() {
        let mut audit = SigmaAudit::new();
        audit.start();
        
        let event = AuditEvent::new(
            "evt1".to_string(),
            AuditEventType::Syscall,
            AuditSeverity::Info,
            1234,
            1000,
            "System call executed".to_string()
        );
        
        audit.log_event(event).unwrap();
        
        let events = audit.list_by_type(AuditEventType::Syscall);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_sigmait_list_by_severity() {
        let mut audit = SigmaAudit::new();
        audit.start();
        
        let event = AuditEvent::new(
            "evt1".to_string(),
            AuditEventType::Syscall,
            AuditSeverity::Critical,
            1234,
            1000,
            "System call executed".to_string()
        );
        
        audit.log_event(event).unwrap();
        
        let events = audit.list_by_severity(AuditSeverity::Critical);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_sigmait_list_by_pid() {
        let mut audit = SigmaAudit::new();
        audit.start();
        
        let event = AuditEvent::new(
            "evt1".to_string(),
            AuditEventType::Syscall,
            AuditSeverity::Info,
            1234,
            1000,
            "System call executed".to_string()
        );
        
        audit.log_event(event).unwrap();
        
        let events = audit.list_by_pid(1234);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_sigmait_stats() {
        let mut audit = SigmaAudit::new();
        audit.start();
        
        let event = AuditEvent::new(
            "evt1".to_string(),
            AuditEventType::Syscall,
            AuditSeverity::Critical,
            1234,
            1000,
            "System call executed".to_string()
        );
        
        audit.log_event(event).unwrap();
        
        let stats = audit.audit_stats();
        assert_eq!(stats.get("total_events"), Some(&"1".to_string()));
        assert_eq!(stats.get("critical_events"), Some(&"1".to_string()));
    }

    #[test]
    fn test_log_audit_event_convenience() {
        init_sigmait();
        
        let mut audit = get_sigmait().unwrap();
        audit.start();
        
        let event = AuditEvent::new(
            "evt1".to_string(),
            AuditEventType::Syscall,
            AuditSeverity::Info,
            1234,
            1000,
            "System call executed".to_string()
        );
        
        let result = log_audit_event(event);
        assert!(result.is_ok());
    }
}
