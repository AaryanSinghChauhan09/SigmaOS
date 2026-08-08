// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod defensive_audit;
pub mod parrot;
pub mod pledge;
pub mod bridge;
pub mod prism;
pub mod sandbox;

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
pub use bridge::{
    LegacySecurityType, SecurityBridge,
};
pub use prism::{
    SecurityFacet, SecurityPrism,
};
pub use sandbox::{
    SandboxRule, PrivacyFirstSandbox,
};
