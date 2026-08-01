// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod pledge;
pub mod qubes_isolation;
pub mod parrot_kali;
pub mod defensive_audit;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{DomainID, DomainType, IsolationError, IsolatedDomain, DomainOrchestrator};

pub use parrot_kali::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, GLOBAL_ANONSURF, GLOBAL_FORENSIC,
    GLOBAL_SANDBOX, RoutingMode, SandboxPolicy,
};

pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
