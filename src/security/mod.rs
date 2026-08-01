// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod pledge;
pub mod qubes_isolation;
pub mod unveil;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
