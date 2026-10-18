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

#[derive(Debug, Clone)]
pub struct DefensiveAuditLog {
    pub records: Vec<SecurityAuditRecord>,
}

impl DefensiveAuditLog {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn log_event(
        &mut self,
        timestamp: u64,
        event: &str,
        actor: &str,
        target: &str,
        allowed: bool,
    ) {
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

#[derive(Debug, Clone)]
pub struct DefensiveAuditSystem {
    pub anomaly_threshold: u32,
    pub log: DefensiveAuditLog,
}

impl DefensiveAuditSystem {
    pub fn new(threshold: u32) -> Self {
        Self {
            anomaly_threshold: threshold,
            log: DefensiveAuditLog::new(),
        }
    }

    pub fn log_event(&mut self, timestamp: u64, pid: u32, event_type: u32, payload: &[u8]) -> Result<(), &'static str> {
        let payload_str = std::string::String::from_utf8_lossy(payload);
        self.log.log_event(timestamp, &event_type.to_string(), &pid.to_string(), &payload_str, true);
        Ok(())
    }

    pub fn evaluate_anomaly_score(&self, payload: &[u8]) -> u32 {
        let s = std::string::String::from_utf8_lossy(payload);
        if s.contains("/bin/sh") || s.contains("rm -rf") || s.contains("sudo") {
            85
        } else {
            10
        }
    }

    pub fn check_payload_safety(&self, payload: &[u8]) -> bool {
        self.evaluate_anomaly_score(payload) < self.anomaly_threshold
    }
}
