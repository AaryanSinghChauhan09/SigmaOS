// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod clipboard;
pub mod integrity;
pub mod intrusion;
pub mod mac;
pub mod password;
pub mod pki;
pub mod pledge;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel, XorEncryption,
};
pub use integrity::{IntegrityCheck, IntegrityError, IntegrityVerifier};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use mac::{MacPolicy, MacRule, MacSecurity};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use pki::{Certificate, CertificateAuthority, PkiError, PkiManager};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use secrets::{SecretManager, SecretStorage, SecretType};
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
pub use vulnerability::{VulnerabilityDatabase, VulnerabilityScanner, VulnerabilitySeverity};
