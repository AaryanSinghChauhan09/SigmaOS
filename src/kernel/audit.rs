// Linux-inspired audit subsystem
// System call and security event logging for SigmaOS

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// Audit event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditEventType {
    Syscall,
    FileAccess,
    ProcessExec,
    NetworkConnect,
    SecurityEvent,
    CapabilityChange,
}

/// Audit event result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventResult {
    Success,
    Failure,
    PermissionDenied,
    NotFound,
}

/// Audit event
#[derive(Debug, Clone)]
pub struct AuditEvent {
    id: u64,
    event_type: AuditEventType,
    timestamp: u64,
    pid: u32,
    uid: u32,
    gid: u32,
    result: AuditEventResult,
    message: String,
}

impl AuditEvent {
    pub fn new(id: u64, event_type: AuditEventType, timestamp: u64, pid: u32, uid: u32, gid: u32, result: AuditEventResult, message: String) -> Self {
        AuditEvent {
            id,
            event_type,
            timestamp,
            pid,
            uid,
            gid,
            result,
            message,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn event_type(&self) -> AuditEventType {
        self.event_type
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }

    pub fn result(&self) -> AuditEventResult {
        self.result
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Audit log (collection of events)
#[derive(Debug, Clone)]
pub struct AuditLog {
    id: u64,
    name: String,
    events: Vec<AuditEvent>,
    max_size: usize,
}

impl AuditLog {
    pub fn new(id: u64, name: String, max_size: usize) -> Self {
        AuditLog {
            id,
            name,
            events: Vec::new(),
            max_size,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_event(&mut self, event: AuditEvent) {
        if self.events.len() >= self.max_size {
            self.events.remove(0); // Remove oldest event
        }
        self.events.push(event);
    }

    pub fn get_events(&self) -> &[AuditEvent] {
        &self.events
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

/// Audit manager for the system
pub struct AuditManager {
    logs: BTreeMap<u64, Arc<Mutex<AuditLog>>>,
    next_log_id: u64,
    next_event_id: u64,
}

impl AuditManager {
    pub fn new() -> Self {
        AuditManager {
            logs: BTreeMap::new(),
            next_log_id: 1,
            next_event_id: 1,
        }
    }

    /// Create a new audit log
    pub fn create_log(&mut self, name: String, max_size: usize) -> u64 {
        let id = self.next_log_id;
        self.next_log_id += 1;

        let log = Arc::new(Mutex::new(AuditLog::new(id, name, max_size)));
        self.logs.insert(id, log);

        id
    }

    /// Get audit log
    pub fn get_log(&self, log_id: u64) -> Option<Arc<Mutex<AuditLog>>> {
        self.logs.get(&log_id).cloned()
    }

    /// Log an event
    pub fn log_event(&mut self, log_id: u64, event_type: AuditEventType, timestamp: u64, pid: u32, uid: u32, gid: u32, result: AuditEventResult, message: String) -> Result<u64, String> {
        let log = self.logs.get(&log_id)
            .ok_or_else(|| format!("Audit log not found: {}", log_id))?;
        
        let event_id = self.next_event_id;
        self.next_event_id += 1;

        let event = AuditEvent::new(event_id, event_type, timestamp, pid, uid, gid, result, message);
        
        let mut log_guard = log.lock().unwrap();
        log_guard.add_event(event);
        
        Ok(event_id)
    }

    /// Get events from log
    pub fn get_events(&self, log_id: u64) -> Result<Vec<AuditEvent>, String> {
        let log = self.logs.get(&log_id)
            .ok_or_else(|| format!("Audit log not found: {}", log_id))?;
        
        let log_guard = log.lock().unwrap();
        Ok(log_guard.get_events().to_vec())
    }

    /// Clear audit log
    pub fn clear_log(&self, log_id: u64) -> Result<(), String> {
        let log = self.logs.get(&log_id)
            .ok_or_else(|| format!("Audit log not found: {}", log_id))?;
        
        let mut log_guard = log.lock().unwrap();
        log_guard.clear();
        
        Ok(())
    }

    /// Remove audit log
    pub fn remove_log(&mut self, log_id: u64) -> Result<(), String> {
        self.logs.remove(&log_id)
            .ok_or_else(|| format!("Audit log not found: {}", log_id))?;
        Ok(())
    }

    /// Get log count
    pub fn log_count(&self) -> usize {
        self.logs.len()
    }
}

impl Default for AuditManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(
            1,
            AuditEventType::Syscall,
            1234567890,
            100,
            0,
            0,
            AuditEventResult::Success,
            "test syscall".to_string()
        );
        
        assert_eq!(event.id(), 1);
        assert_eq!(event.event_type(), AuditEventType::Syscall);
    }

    #[test]
    fn test_audit_log_creation() {
        let log = AuditLog::new(1, "test_log".to_string(), 100);
        assert_eq!(log.id(), 1);
        assert_eq!(log.name(), "test_log");
        assert_eq!(log.event_count(), 0);
    }

    #[test]
    fn test_audit_log_add_event() {
        let mut log = AuditLog::new(1, "test_log".to_string(), 100);
        let event = AuditEvent::new(
            1,
            AuditEventType::Syscall,
            1234567890,
            100,
            0,
            0,
            AuditEventResult::Success,
            "test syscall".to_string()
        );
        
        log.add_event(event);
        assert_eq!(log.event_count(), 1);
    }

    #[test]
    fn test_audit_log_max_size() {
        let mut log = AuditLog::new(1, "test_log".to_string(), 2);
        
        log.add_event(AuditEvent::new(1, AuditEventType::Syscall, 1, 100, 0, 0, AuditEventResult::Success, "event1".to_string()));
        log.add_event(AuditEvent::new(2, AuditEventType::Syscall, 2, 100, 0, 0, AuditEventResult::Success, "event2".to_string()));
        log.add_event(AuditEvent::new(3, AuditEventType::Syscall, 3, 100, 0, 0, AuditEventResult::Success, "event3".to_string()));
        
        assert_eq!(log.event_count(), 2); // Oldest event removed
    }

    #[test]
    fn test_audit_log_clear() {
        let mut log = AuditLog::new(1, "test_log".to_string(), 100);
        log.add_event(AuditEvent::new(1, AuditEventType::Syscall, 1, 100, 0, 0, AuditEventResult::Success, "event1".to_string()));
        
        log.clear();
        assert_eq!(log.event_count(), 0);
    }

    #[test]
    fn test_audit_manager_creation() {
        let manager = AuditManager::new();
        assert_eq!(manager.log_count(), 0);
    }

    #[test]
    fn test_audit_manager_create_log() {
        let mut manager = AuditManager::new();
        let id = manager.create_log("test_log".to_string(), 100);
        
        assert_eq!(id, 1);
        assert_eq!(manager.log_count(), 1);
    }

    #[test]
    fn test_audit_manager_log_event() {
        let mut manager = AuditManager::new();
        let log_id = manager.create_log("test_log".to_string(), 100);
        
        let event_id = manager.log_event(
            log_id,
            AuditEventType::Syscall,
            1234567890,
            100,
            0,
            0,
            AuditEventResult::Success,
            "test syscall".to_string()
        ).unwrap();
        
        assert_eq!(event_id, 1);
    }

    #[test]
    fn test_audit_manager_get_events() {
        let mut manager = AuditManager::new();
        let log_id = manager.create_log("test_log".to_string(), 100);
        
        manager.log_event(
            log_id,
            AuditEventType::Syscall,
            1234567890,
            100,
            0,
            0,
            AuditEventResult::Success,
            "test syscall".to_string()
        ).unwrap();
        
        let events = manager.get_events(log_id).unwrap();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_audit_manager_clear_log() {
        let mut manager = AuditManager::new();
        let log_id = manager.create_log("test_log".to_string(), 100);
        
        manager.log_event(
            log_id,
            AuditEventType::Syscall,
            1234567890,
            100,
            0,
            0,
            AuditEventResult::Success,
            "test syscall".to_string()
        ).unwrap();
        
        assert!(manager.clear_log(log_id).is_ok());
    }

    #[test]
    fn test_audit_manager_remove_log() {
        let mut manager = AuditManager::new();
        let log_id = manager.create_log("test_log".to_string(), 100);
        
        assert!(manager.remove_log(log_id).is_ok());
        assert_eq!(manager.log_count(), 0);
    }
}
