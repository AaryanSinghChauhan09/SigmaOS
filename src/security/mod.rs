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
<<<<<<< HEAD
pub mod hardening;
pub mod qubes_isolation;
||||||| 23ef22a4a
pub mod clipboard;
pub mod intrusion;
pub mod password;
pub mod parrot_linux;
pub mod selinux;
=======
pub mod clipboard;
pub mod intrusion;
pub mod password;
pub mod parrot_linux;
pub mod selinux;
pub mod capability_sandbox;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

pub use capability::{CapabilityGate, CapabilityToken, Permission};
<<<<<<< HEAD
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use qubes_isolation::{DomainID, DomainType, IsolationError, IsolatedDomain, DomainOrchestrator};
||||||| 23ef22a4a
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, Severity, Vulnerability, SimpleVulnerability,
    VulnerabilityScanner, SimpleVulnerabilityScanner, ScanSummary, ScanReport,
    SimpleScanReport, CIPipelineIntegration, SimpleCIPipelineIntegration,
};
=======
pub use obfuscator::{SovereignCodeHardener, SovereignThreatDetector};
pub use phantom::{CapabilityContext, KernelLevel, SecurityAdminLevel, UserLevel};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use securelevels::{LinuxCapability, Securelevel, SovereignSecurelevelManager};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
pub use capability_sandbox::{CapabilitySandboxEnforcer, SandboxCapabilityToken, PORT_ALLOW_TCP, PORT_ALLOW_SSL};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
