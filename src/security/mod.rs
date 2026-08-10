// SigmaOS Security Subsystem
pub mod capability;
pub mod pledge;
pub mod vulnerability;
pub mod hardening;
pub mod defensive_audit;
pub mod parrot_parity;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use defensive_audit::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};
pub use parrot_parity::{AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, GLOBAL_ANONSURF, GLOBAL_SANDBOX, GLOBAL_FORENSIC};
