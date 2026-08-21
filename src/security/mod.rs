// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod audit;
pub mod capability;
pub mod pqc_enclave;
pub mod hardening;
pub mod qubes_isolation;

pub use pqc_enclave::{
    KyberKem, DilithiumSignature, RotatableToken, PqcTokenRotationBus, PqcZeroTrustGater,
};

pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod mac;
pub mod password;
pub mod pki;
pub mod pledge;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod vault;
pub mod vpn;
pub mod vulnerability;
pub mod governance_privacy;

pub use governance_privacy::*;

pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use capability_enforcer::{CapabilityToken as RuntimeCapabilityToken, SecurityEnforcer};
pub use capability_token::{
    CapabilityToken as AndroidStyleCapabilityToken,
    SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel as ClipboardSecurityLevel, XorEncryption,
};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use selinux::{
    AppArmorManager, AppArmorProfile, ObjectType, Permission as SelinuxPermission, SecurityContext,
    SecurityLabel, SecurityPolicy, SecurityRule,
};
pub use sigma_pledge::{PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter};
pub use sigma_unveil::{UnveilEntry, UnveilManager, UnveilPermissions, UnveilState};
pub use vault::{
    Aes256GcmEncryption, ChaCha20Poly1305Encryption, EncryptedFile, EncryptedFileVault,
    EncryptionAlgorithm, Kyber1024Encryption, VaultEncryption, VaultError, VaultMetadata,
    VaultResult,
};
pub use vpn::{
    AuthMethod, ConnectionState, KillSwitchConfig, OpenVpnHandler, SecureVpnClient, VpnConfig,
    VpnConnectionResult, VpnError, VpnProtocol, VpnProtocolHandler, VpnStatistics,
    WireGuardHandler,
};
pub use vulnerability::{VulnerabilityDatabase, VulnerabilityScanner};
pub use qubes_isolation::{DomainID, DomainType, IsolationError, IsolatedDomain, DomainOrchestrator};
