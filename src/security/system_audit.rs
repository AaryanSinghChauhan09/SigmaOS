// System Audit Manager for SigmaOS
// System audit and logging per Wiki 07-Security.md
// Provides comprehensive audit logging and security event tracking

use std::string::{String, ToString};
use std::vec::Vec;

/// Audit event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventType {
    Authentication,
    Authorization,
    FileAccess,
    SystemChange,
    NetworkAccess,
    ProcessExecution,
    SecurityViolation,
}

impl AuditEventType {
    pub fn as_str(&self) -> &str {
        match self {
            AuditEventType::Authentication => "authentication",
            AuditEventType::Authorization => "authorization",
            AuditEventType::FileAccess => "file_access",
            AuditEventType::SystemChange => "system_change",
            AuditEventType::NetworkAccess => "network_access",
            AuditEventType::ProcessExecution => "process_execution",
            AuditEventType::SecurityViolation => "security_violation",
        }
    }
}

/// Audit event
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: u64,
    pub event_type: AuditEventType,
    pub timestamp: u64,
    pub user_id: u32,
    pub process_id: u32,
    pub resource: String,
    pub action: String,
    pub result: String,
    pub details: String,
}

impl AuditEvent {
    pub fn new(id: u64, event_type: AuditEventType, user_id: u32) -> Self {
        AuditEvent {
            id,
            event_type,
            timestamp: 0,
            user_id,
            process_id: 0,
            resource: String::new(),
            action: String::new(),
            result: String::new(),
            details: String::new(),
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "[{}] UID:{} PID:{} {} {} {} - {}",
            self.event_type.as_str(),
            self.user_id,
            self.process_id,
            self.resource,
            self.action,
            self.result,
            self.details
        )
    }
}

/// Audit rule
#[derive(Debug, Clone)]
pub struct AuditRule {
    pub event: String,
    pub path: String,
    pub action: AuditAction,
}

impl AuditRule {
    pub fn new(event: String, path: String, action: AuditAction) -> Self {
        AuditRule {
            event,
            path,
            action,
        }
    }
}

/// Audit action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditAction {
    Log,
    Alert,
    Block,
}

impl AuditAction {
    pub fn as_str(&self) -> &str {
        match self {
            AuditAction::Log => "log",
            AuditAction::Alert => "alert",
            AuditAction::Block => "block",
        }
    }
}

/// Audit configuration
#[derive(Debug, Clone)]
pub struct AuditConfig {
    pub log_file: String,
    pub log_level: String,
    pub max_log_size: u64,
    pub retention_days: u32,
}

impl AuditConfig {
    pub fn new() -> Self {
        AuditConfig {
            log_file: String::from("/var/log/audit.log"),
            log_level: String::from("info"),
            max_log_size: 100 * 1024 * 1024, // 100 MB
            retention_days: 30,
        }
    }
}

/// System audit manager
#[derive(Debug, Clone)]
pub struct SystemAuditManager {
    pub config: AuditConfig,
    pub events: Vec<AuditEvent>,
    pub rules: Vec<AuditRule>,
    pub next_event_id: u64,
}

impl Default for SystemAuditManager {
    fn default() -> Self {
        SystemAuditManager {
            config: AuditConfig::new(),
            events: Vec::new(),
            rules: Vec::new(),
            next_event_id: 1,
        }
    }
}

impl SystemAuditManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config(&mut self, config: AuditConfig) {
        self.config = config;
    }

    pub fn add_rule(&mut self, rule: AuditRule) {
        self.rules.push(rule);
    }

    pub fn log_event(&mut self, event_type: AuditEventType, user_id: u32, resource: String, action: String, result: String) -> u64 {
        let mut event = AuditEvent::new(self.next_event_id, event_type, user_id);
        event.resource = resource;
        event.action = action;
        event.result = result;

        let id = event.id;
        self.events.push(event);
        self.next_event_id += 1;

        id
    }

    pub fn log_file_access(&mut self, path: String, user_id: u32, action: String) -> u64 {
        self.log_event(
            AuditEventType::FileAccess,
            user_id,
            path,
            action,
            String::from("success"),
        )
    }

    pub fn log_security_event(&mut self, event_type: AuditEventType, user_id: u32, details: String) -> u64 {
        let mut event = AuditEvent::new(self.next_event_id, event_type, user_id);
        event.details = details;

        let id = event.id;
        self.events.push(event);
        self.next_event_id += 1;

        id
    }

    pub fn get_event(&self, id: u64) -> Option<&AuditEvent> {
        self.events.iter().find(|e| e.id == id)
    }

    pub fn query_events(&self, event_type: AuditEventType) -> Vec<AuditEvent> {
        self.events.iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect()
    }

    pub fn query_user_events(&self, user_id: u32) -> Vec<AuditEvent> {
        self.events.iter()
            .filter(|e| e.user_id == user_id)
            .cloned()
            .collect()
    }

    pub fn query_file_access(&self, path: &str) -> Vec<AuditEvent> {
        self.events.iter()
            .filter(|e| e.event_type == AuditEventType::FileAccess && e.resource.contains(path))
            .cloned()
            .collect()
    }

    pub fn list_recent_events(&self, count: usize) -> Vec<AuditEvent> {
        let recent: Vec<_> = self.events.iter().rev().take(count).cloned().collect();
        recent
    }

    pub fn list_security_events(&self) -> Vec<AuditEvent> {
        self.events.iter()
            .filter(|e| e.event_type == AuditEventType::SecurityViolation)
            .cloned()
            .collect()
    }

    pub fn clear_old_events(&mut self, older_than: u64) {
        self.events.retain(|e| e.timestamp >= older_than);
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Audit Statistics:\n");
        stats.push_str(&format!("Total events: {}\n", self.events.len()));
        stats.push_str(&format!("Rules configured: {}\n", self.rules.len()));

        let auth_count = self
            .events
            .iter()
            .filter(|e| e.event_type == AuditEventType::Authentication)
            .count();
        let file_count = self
            .events
            .iter()
            .filter(|e| e.event_type == AuditEventType::FileAccess)
            .count();
        let security_count = self
            .events
            .iter()
            .filter(|e| e.event_type == AuditEventType::SecurityViolation)
            .count();
        let auth_count = self.events.iter().filter(|e| e.event_type == AuditEventType::Authentication).count();
        let file_count = self.events.iter().filter(|e| e.event_type == AuditEventType::FileAccess).count();
        let security_count = self.events.iter().filter(|e| e.event_type == AuditEventType::SecurityViolation).count();

        stats.push_str(&format!("Authentication events: {}\n", auth_count));
        stats.push_str(&format!("File access events: {}\n", file_count));
        stats.push_str(&format!("Security violations: {}\n", security_count));

        stats
    }

    pub fn export_log(&self) -> String {
        let mut log = String::from("Audit Log Export:\n");
        log.push_str(&format!("Log file: {}\n", self.config.log_file));
        log.push_str(&format!("Log level: {}\n", self.config.log_level));
        log.push_str(&format!(
            "Retention: {} days\n\n",
            self.config.retention_days
        ));
        log.push_str(&format!("Retention: {} days\n\n", self.config.retention_days));

        for event in &self.events {
            log.push_str(&format!("{}\n", event.get_summary()));
        }

        log
    }

    pub fn check_rules(&self, event: &AuditEvent) -> Vec<AuditAction> {
        let mut actions = Vec::new();

        for rule in &self.rules {
            if rule.event == event.event_type.as_str() {
                if event.resource.contains(&rule.path) || rule.path.is_empty() {
                    actions.push(rule.action);
                }
            }
        }

        actions
    }

    pub fn set_log_level(&mut self, level: String) {
        self.config.log_level = level;
    }

    pub fn set_retention_days(&mut self, days: u32) {
        self.config.retention_days = days;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_type_as_str() {
        assert_eq!(AuditEventType::Authentication.as_str(), "authentication");
        assert_eq!(AuditEventType::FileAccess.as_str(), "file_access");
    }

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(1, AuditEventType::Authentication, 1000);
        assert_eq!(event.id, 1);
        assert_eq!(event.user_id, 1000);
    }

    #[test]
    fn test_audit_event_get_summary() {
        let mut event = AuditEvent::new(1, AuditEventType::FileAccess, 1000);
        event.resource = String::from("/etc/passwd");
        event.action = String::from("read");
        event.result = String::from("success");

        let summary = event.get_summary();
        assert!(summary.contains("file_access"));
        assert!(summary.contains("/etc/passwd"));
    }

    #[test]
    fn test_audit_rule_creation() {
        let rule = AuditRule::new(
            String::from("file_access"),
            String::from("/etc/shadow"),
            AuditAction::Log,
        );
        assert_eq!(rule.event, "file_access");
        assert_eq!(rule.path, "/etc/shadow");
    }

    #[test]
    fn test_audit_action_as_str() {
        assert_eq!(AuditAction::Log.as_str(), "log");
        assert_eq!(AuditAction::Alert.as_str(), "alert");
        assert_eq!(AuditAction::Block.as_str(), "block");
    }

    #[test]
    fn test_audit_config_creation() {
        let config = AuditConfig::new();
        assert_eq!(config.log_file, "/var/log/audit.log");
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_system_audit_manager_creation() {
        let manager = SystemAuditManager::new();
        assert_eq!(manager.events.len(), 0);
        assert_eq!(manager.next_event_id, 1);
    }

    #[test]
    fn test_system_audit_manager_log_event() {
        let mut manager = SystemAuditManager::new();
        let id = manager.log_event(
            AuditEventType::Authentication,
            1000,
            String::from("/login"),
            String::from("attempt"),
            String::from("success"),
        );

        assert_eq!(id, 1);
        assert_eq!(manager.events.len(), 1);
    }

    #[test]
    fn test_system_audit_manager_log_file_access() {
        let mut manager = SystemAuditManager::new();
        let id = manager.log_file_access(String::from("/etc/passwd"), 1000, String::from("read"));

        assert_eq!(id, 1);
        assert_eq!(manager.events.len(), 1);
    }

    #[test]
    fn test_system_audit_manager_get_event() {
        let mut manager = SystemAuditManager::new();
        manager.log_event(
            AuditEventType::Authentication,
            1000,
            String::from("/login"),
            String::from("attempt"),
            String::from("success"),
        );

        let event = manager.get_event(1);
        assert!(event.is_some());
        assert_eq!(event.unwrap().user_id, 1000);
    }

    #[test]
    fn test_system_audit_manager_query_events() {
        let mut manager = SystemAuditManager::new();
        manager.log_event(
            AuditEventType::Authentication,
            1000,
            String::from("/login"),
            String::from("attempt"),
            String::from("success"),
        );

        let auth_events = manager.query_events(AuditEventType::Authentication);
        assert_eq!(auth_events.len(), 1);
    }

    #[test]
    fn test_system_audit_manager_query_user_events() {
        let mut manager = SystemAuditManager::new();
        manager.log_event(
            AuditEventType::Authentication,
            1000,
            String::from("/login"),
            String::from("attempt"),
            String::from("success"),
        );

        let user_events = manager.query_user_events(1000);
        assert_eq!(user_events.len(), 1);
    }

    #[test]
    fn test_system_audit_manager_query_file_access() {
        let mut manager = SystemAuditManager::new();
        manager.log_file_access(String::from("/etc/passwd"), 1000, String::from("read"));

        let file_events = manager.query_file_access("/etc/passwd");
        assert_eq!(file_events.len(), 1);
    }

    #[test]
    fn test_system_audit_manager_list_recent_events() {
        let mut manager = SystemAuditManager::new();
        manager.log_event(
            AuditEventType::Authentication,
            1000,
            String::from("/login"),
            String::from("attempt"),
            String::from("success"),
        );
        manager.log_event(
            AuditEventType::FileAccess,
            1000,
            String::from("/etc/passwd"),
            String::from("read"),
            String::from("success"),
        );

        let recent = manager.list_recent_events(1);
        assert_eq!(recent.len(), 1);
    }

    #[test]
    fn test_system_audit_manager_get_statistics() {
        let mut manager = SystemAuditManager::new();
        manager.log_event(
            AuditEventType::Authentication,
            1000,
            String::from("/login"),
            String::from("attempt"),
            String::from("success"),
        );

        let stats = manager.get_statistics();
        assert!(stats.contains("Total events: 1"));
    }

    #[test]
    fn test_system_audit_manager_add_rule() {
        let mut manager = SystemAuditManager::new();
        manager.add_rule(AuditRule::new(
            String::from("file_access"),
            String::from("/etc/shadow"),
            AuditAction::Log,
        ));

        assert_eq!(manager.rules.len(), 1);
    }

    #[test]
    fn test_system_audit_manager_check_rules() {
        let mut manager = SystemAuditManager::new();
        manager.add_rule(AuditRule::new(
            String::from("file_access"),
            String::from("/etc/shadow"),
            AuditAction::Log,
        ));

        let mut event = AuditEvent::new(1, AuditEventType::FileAccess, 1000);
        event.resource = String::from("/etc/shadow");

        let actions = manager.check_rules(&event);
        assert_eq!(actions.len(), 1);
    }
}
