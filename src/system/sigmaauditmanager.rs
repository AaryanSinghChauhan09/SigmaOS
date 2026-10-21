// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Audit event severity
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Audit event type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEventType {
    Authentication,
    Authorization,
    FileAccess,
    NetworkAccess,
    SystemChange,
    PackageOperation,
    ServiceOperation,
}

/// Audit event
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: String,
    pub event_type: AuditEventType,
    pub severity: AuditSeverity,
    pub source: String,
    pub target: String,
    pub description: String,
}

/// Audit manager for tracking system events
/// Inspired by Linux auditd and security auditing best practices
#[derive(Debug, Clone)]
pub struct SigmaAuditManager {
    events: Vec<AuditEvent>,
}

impl SigmaAuditManager {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    /// Log an audit event
    pub fn log_event(&mut self, event: AuditEvent) {
        self.events.push(event);
    }

    /// Get event by ID
    pub fn get_event(&self, id: &str) -> Option<&AuditEvent> {
        self.events.iter().find(|e| e.id == id)
    }

    /// Get all events
    pub fn get_events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Get events by type
    pub fn get_events_by_type(&self, event_type: AuditEventType) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// Get events by severity
    pub fn get_events_by_severity(&self, severity: AuditSeverity) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.severity == severity)
            .collect()
    }

    /// Get events by source
    pub fn get_events_by_source(&self, source: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.source == source)
            .collect()
    }

    /// Get critical events
    pub fn get_critical_events(&self) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.severity == AuditSeverity::Critical)
            .collect()
    }

    /// Clear all events
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Get event count
    pub fn get_event_count(&self) -> usize {
        self.events.len()
    }
}

impl Default for SigmaAuditManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_event() {
        let mut manager = SigmaAuditManager::new();
        let event = AuditEvent {
            id: "evt-1".to_string(),
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            event_type: AuditEventType::Authentication,
            severity: AuditSeverity::Info,
            source: "user".to_string(),
            target: "system".to_string(),
            description: "User login".to_string(),
        };
        manager.log_event(event);
        assert_eq!(manager.get_event_count(), 1);
    }

    #[test]
    fn test_get_events_by_type() {
        let mut manager = SigmaAuditManager::new();
        let event1 = AuditEvent {
            id: "evt-1".to_string(),
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            event_type: AuditEventType::Authentication,
            severity: AuditSeverity::Info,
            source: "user".to_string(),
            target: "system".to_string(),
            description: "User login".to_string(),
        };
        let event2 = AuditEvent {
            id: "evt-2".to_string(),
            timestamp: "2025-01-01T00:01:00Z".to_string(),
            event_type: AuditEventType::FileAccess,
            severity: AuditSeverity::Warning,
            source: "user".to_string(),
            target: "/etc/passwd".to_string(),
            description: "File access".to_string(),
        };
        manager.log_event(event1);
        manager.log_event(event2);
        assert_eq!(manager.get_events_by_type(AuditEventType::Authentication).len(), 1);
    }

    #[test]
    fn test_get_critical_events() {
        let mut manager = SigmaAuditManager::new();
        let event1 = AuditEvent {
            id: "evt-1".to_string(),
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            event_type: AuditEventType::Authentication,
            severity: AuditSeverity::Info,
            source: "user".to_string(),
            target: "system".to_string(),
            description: "User login".to_string(),
        };
        let event2 = AuditEvent {
            id: "evt-2".to_string(),
            timestamp: "2025-01-01T00:01:00Z".to_string(),
            event_type: AuditEventType::SystemChange,
            severity: AuditSeverity::Critical,
            source: "root".to_string(),
            target: "/etc/shadow".to_string(),
            description: "Critical system change".to_string(),
        };
        manager.log_event(event1);
        manager.log_event(event2);
        assert_eq!(manager.get_critical_events().len(), 1);
    }

    #[test]
    fn test_clear_events() {
        let mut manager = SigmaAuditManager::new();
        let event = AuditEvent {
            id: "evt-1".to_string(),
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            event_type: AuditEventType::Authentication,
            severity: AuditSeverity::Info,
            source: "user".to_string(),
            target: "system".to_string(),
            description: "User login".to_string(),
        };
        manager.log_event(event);
        manager.clear_events();
        assert_eq!(manager.get_event_count(), 0);
    }
}
