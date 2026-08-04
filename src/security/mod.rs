// SigmaOS Security Subsystem
pub mod capability;
pub mod pledge;
pub mod vulnerability;
pub mod hardening;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
