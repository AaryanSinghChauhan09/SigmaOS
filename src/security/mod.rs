// SigmaOS Security Subsystem
pub mod capability;
pub mod pledge;
pub mod vulnerability;
pub mod hardening;
pub mod defensive_audit;
pub mod selinux;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use defensive_audit::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};
pub use selinux::{
    SecurityPolicy, SecurityRule, SecurityLabel, SecurityContext, SelinuxPermission, ObjectType,
    AppArmorProfile, AppArmorManager,
};
