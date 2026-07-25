pub mod lsm;

pub mod audit;
pub mod capability;
pub mod clipboard;
pub mod forensics;
pub mod integrity;
pub mod mac;
pub mod pki;
pub mod pledge;
pub mod secrets;
pub mod selinux;
pub mod vault;
pub mod vpn;
pub mod vulnerability;
pub mod scanner;
pub mod forensics;
pub mod cleaner;
pub mod sigma_pledge;
pub mod sigma_unveil;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel, XorEncryption,
};
pub use forensics::{
    DecoyHoneyPot, ForensicAnalyzer, KaliSnifferAudit, PassComplexityAuditor, RecoveredFile,
    SigmaPortScanner,
};
pub use integrity::{IntegrityError, IntegrityMonitor, SimpleIntegrityMonitor};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use mac::{MACEngine, MACPolicy, SimpleMACEngine};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use pki::{Certificate, PKIError, PKIManager, SimplePKIManager};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use secrets::SecretType;
pub use vault::{
    Aes256GcmEncryption, ChaCha20Poly1305Encryption, EncryptedFile, EncryptedFileVault,
    EncryptionAlgorithm, Kyber1024Encryption, VaultEncryption, VaultError, VaultMetadata,
    VaultResult,
};
pub use prism::{
    SecurityFacet, SecurityPrism,
};
// Integrity: export the monitor trait and concrete types that actually exist
pub use integrity::{
    File as IntegrityFile, FileCapability, FileID, FileInfo, IntegrityError, IntegrityMonitor,
    IntegrityStats, IntegrityStatus, MonitorCapability, SimpleFile, SimpleIntegrityMonitor,
};
// MAC: export what the module defines
pub use mac::{
    ContextCapability, ContextID, EngineCapability as MacEngineCapability, MACEngine, MACPolicy,
    MACStats, MLSPolicy, PolicyCapability as MacPolicyCapability, PolicyInfo as MacPolicyInfo,
    SecurityContext as MacSecurityContext, SecurityDomain, SecurityLevel as MacSecurityLevel, SimpleMACEngine,
};
// PKI: export actual types
pub use pki::{
    Certificate, PKIManager, SimpleCRL, SimpleCertificate, SimplePKIManager, CRL as CrlTrait,
};
// Secrets: export actual types
pub use secrets::{
    Keyring, KeyringCapability, KeyringStats, Secret, SecretCapability, SecretInfo, SecretType,
    SimpleKeyring, SimpleSecret,
};
// Vulnerability: export actual types
pub use vulnerability::{
    KAslrHardener, Severity as VulnerabilitySeverity, StackCanaryGuard, WxorEPageGuard, ZeroizeSec,
};
pub use vulnerability::{
    KAslrHardener, Severity as VulnerabilitySeverity, StackCanaryGuard, WxorEPageGuard, ZeroizeSec,
};
