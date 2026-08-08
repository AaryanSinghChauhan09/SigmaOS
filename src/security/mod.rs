// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod audit;
pub mod capability;
pub mod integrity;
pub mod mac;
pub mod obfuscator;
pub mod phantom;
pub mod pki;
pub mod pledge;
pub mod secrets;
pub mod securelevels;
pub mod unveil;
pub mod vulnerability;
||||||| 65885484f
pub mod hardening;
pub mod hardening;
pub mod qubes_isolation;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use obfuscator::{SovereignCodeHardener, SovereignThreatDetector};
pub use phantom::{CapabilityContext, KernelLevel, SecurityAdminLevel, UserLevel};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use securelevels::{LinuxCapability, Securelevel, SovereignSecurelevelManager};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
||||||| 65885484f
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use qubes_isolation::{DomainID, DomainType, IsolationError, IsolatedDomain, DomainOrchestrator};
