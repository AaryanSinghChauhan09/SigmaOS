// SigmaOS Security Subsystem
pub mod capability;
pub mod crypto_utils;
pub mod hardening;
pub mod pledge;
pub mod vulnerability;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use crypto_utils::{constant_time_eq, hash_password_placeholder, CryptoError, SecureRandom};
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
