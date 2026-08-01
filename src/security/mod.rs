// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod pledge;
pub mod qubes_isolation;
pub mod selinux;
pub mod defensive_audit;
pub mod hardening;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{DomainID, DomainType, IsolationError, IsolatedDomain, DomainOrchestrator};
pub use selinux::{
    SecurityPolicy, SecurityRule, SecurityLabel, SelinuxPermission, ObjectType, SecurityContext,
    AppArmorProfile, AppArmorManager,
};
pub use defensive_audit::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};
pub use hardening::{IntrusionMonitor, IntrusionSeverity, AuditLogEntry, HardenedAuditTrail, secure_zeroize};
