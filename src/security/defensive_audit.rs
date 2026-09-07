// SigmaOS Defensive Security Audit Trail Subsystem

use std::string::String;
use std::string::ToString;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct SecurityAuditRecord {
    pub timestamp_ms: u64,
    pub event_type: String,
    pub actor_id: String,
    pub target_resource: String,
    pub is_allowed: bool,
}

pub struct DefensiveAuditLog {
    pub records: Vec<SecurityAuditRecord>,
}

impl DefensiveAuditLog {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn log_event(&mut self, timestamp: u64, event: &str, actor: &str, target: &str, allowed: bool) {
        self.records.push(SecurityAuditRecord {
            timestamp_ms: timestamp,
            event_type: event.to_string(),
            actor_id: actor.to_string(),
            target_resource: target.to_string(),
            is_allowed: allowed,
        });
    }

    pub fn violation_count(&self) -> usize {
        self.records.iter().filter(|r| !r.is_allowed).count()
    }
}

impl Default for DefensiveAuditLog {
    fn default() -> Self {
        Self::new()
    }
}
