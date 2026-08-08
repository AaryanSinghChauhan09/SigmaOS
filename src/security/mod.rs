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
pub mod clipboard;
pub mod intrusion;
pub mod password;
pub mod parrot_linux;
pub mod selinux;
pub mod capability_sandbox;

pub use parrot_linux::{
    AnonymityMode, AnonsurfEngine, RecoveredFile, ForensicsAuditTool, SniffedPacket,
    KaliSniffer, PentestAssistant, SecureWipeTool, IntrusionSeverity, IntrusionAlert, SigmaIDS,
};
pub use selinux::{
    SecurityPolicy, SecurityLabel, SecurityRule, SecurityContext, SelinuxPermission, ObjectType,
    AppArmorProfile, AppArmorManager, AccessVectorCache, SelinuxBoolean, TypeTransitionRule,
};
pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use obfuscator::{SovereignCodeHardener, SovereignThreatDetector};
pub use phantom::{CapabilityContext, KernelLevel, SecurityAdminLevel, UserLevel};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use securelevels::{LinuxCapability, Securelevel, SovereignSecurelevelManager};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
pub use capability_sandbox::{CapabilitySandboxEnforcer, SandboxCapabilityToken, PORT_ALLOW_TCP, PORT_ALLOW_SSL};
