// SigmaOS Security Subsystem
pub mod capability;
pub mod pledge;
pub mod vulnerability;
pub mod hardening;
pub mod qubes_isolation;
pub mod selinux;
pub mod root_improvement;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
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
