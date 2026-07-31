// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod pledge;
pub mod vulnerability;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    VulnerabilityClass, VulnerabilityReport, SecurityScanner, ExploitPayload,
    PenetrationAssistant,
};
