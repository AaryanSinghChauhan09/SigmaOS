// SigmaOS Security Subsystem
pub mod capability;
pub mod defensive_audit;
pub mod parrot;
pub mod pledge;
pub mod vulnerability;
pub mod hardening;
pub mod deobfuscation;
pub mod kali_stack;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub use parrot::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
    GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};

pub use kali_stack::{
    PluggableAuthenticationModule, FirewallRule, IptablesFirewall, CronJob, CronDaemon,
    SudoPrivilegeEscalation, TmuxPane, TmuxMultiplexer, SwapSpaceManager, DmesgLog, KaliError,
};
