// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod defensive_audit;
pub mod hardening;
pub mod pledge;
pub mod qubes_isolation;
pub mod selinux;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use defensive_audit::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use selinux::{
    AppArmorManager, AppArmorProfile, ObjectType, SecurityContext, SecurityLabel, SecurityPolicy,
    SecurityRule, SelinuxPermission,
};
