pub mod lsm;

pub mod audit;
pub mod capability;
pub mod clipboard;
pub mod forensics;
pub mod integrity;
pub mod mac;
pub mod pki;
pub mod pledge;
pub mod bridge;
pub mod prism;

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
pub use vulnerability::{
    KAslrHardener, Severity as VulnerabilitySeverity, StackCanaryGuard, WxorEPageGuard, ZeroizeSec,
};
