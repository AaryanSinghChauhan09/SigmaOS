use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEvent {
    Access,
    Modification,
    Security,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEventType {
    System,
    User,
    Kernel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditSeverity {
    Info,
    Warning,
    Critical,
}

pub struct SigmaAuditManager;
