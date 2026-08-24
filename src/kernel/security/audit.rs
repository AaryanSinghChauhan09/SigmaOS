// Capability Audit Logging Subsystem for SigmaOS Security
// Location: src/kernel/security/audit.rs

// #![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityAuditEventType {
    Grant,
    Use,
    Revoke,
    Violation,
}

#[derive(Debug, Clone, Copy)]
pub struct CapabilityAuditEvent {
    pub timestamp: u64,
    pub token_id: u64,
    pub event_type: CapabilityAuditEventType,
    pub resource_id: u64,
    pub actor_pid: u32,
}

pub const MAX_AUDIT_LOGS: usize = 256;

pub struct CapabilityAuditLogger {
    logs: [CapabilityAuditEvent; MAX_AUDIT_LOGS],
    log_count: usize,
}

impl CapabilityAuditLogger {
    pub fn new() -> Self {
        CapabilityAuditLogger {
            logs: [CapabilityAuditEvent {
                timestamp: 0,
                token_id: 0,
                event_type: CapabilityAuditEventType::Violation,
                resource_id: 0,
                actor_pid: 0,
            }; MAX_AUDIT_LOGS],
            log_count: 0,
        }
    }

    pub fn log_event(&mut self, timestamp: u64, token_id: u64, event_type: CapabilityAuditEventType, resource_id: u64, actor_pid: u32) {
        if self.log_count < MAX_AUDIT_LOGS {
            self.logs[self.log_count] = CapabilityAuditEvent {
                timestamp,
                token_id,
                event_type,
                resource_id,
                actor_pid,
            };
            self.log_count += 1;
        }
    }

    pub fn count_violations(&self) -> usize {
        self.logs[..self.log_count].iter().filter(|e| e.event_type == CapabilityAuditEventType::Violation).count()
    }

    pub fn get_logs_count(&self) -> usize {
        self.log_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_audit_logger() {
        let mut logger = CapabilityAuditLogger::new();
        logger.log_event(100, 1, CapabilityAuditEventType::Grant, 5001, 10);
        logger.log_event(105, 1, CapabilityAuditEventType::Use, 5001, 10);
        logger.log_event(110, 2, CapabilityAuditEventType::Violation, 5001, 12);

        assert_eq!(logger.get_logs_count(), 3);
        assert_eq!(logger.count_violations(), 1);
    }
}
