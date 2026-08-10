// SigmaOS Security Subsystem
pub mod capability;
pub mod hardening;
pub mod pledge;
pub mod vulnerability;
pub mod parrot_parity;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use parrot_parity::{
    RoutingMode, AnonSurfShunt, SandboxPolicy, AppSandboxEngine, ForensicStorageFilter,
    GLOBAL_ANONSURF, GLOBAL_SANDBOX, GLOBAL_FORENSIC,
};
