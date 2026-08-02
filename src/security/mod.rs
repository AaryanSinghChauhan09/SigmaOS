// SigmaOS Security Subsystem
pub mod capability;
pub mod hardening;
pub mod pledge;
pub mod vulnerability;
pub mod qubes_isolation;
pub mod selinux;
pub mod root_improvement;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
    SecurityContext, SecurityLabel, SecurityPolicy, SecurityRule,
};
pub use selinux::{
    SelinuxPermission, DefensiveAuditSystem, ForensicBlock, MaliciousSignature, ObjectType,
};
pub use root_improvement::{
    SudoDoasElevator, SudoToken, PolkitEnforcer, PolkitAuthorization, PolkitRule,
    CapSplitter, LinuxCap, RootlessNamespaceManager, UidMapEntry, PamMfaAuthenticator,
};
