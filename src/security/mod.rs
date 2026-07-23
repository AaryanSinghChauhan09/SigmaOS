// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod audit;
pub mod capability;
pub mod clipboard;
pub mod integrity;
pub mod intrusion;
pub mod mac;
pub mod password;
pub mod pki;
pub mod pledge;
pub mod secrets;
pub mod vault;
pub mod vpn;
pub mod vulnerability;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel, XorEncryption,
};
pub use integrity::{
    File, FileCapability, FileInfo, IntegrityError, IntegrityMonitor, IntegrityStatus,
    MonitorCapability, SimpleFile, SimpleIntegrityMonitor,
};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use mac::{
    ContextCapability, ContextID, EngineCapability, MACEngine, MACError, MACPolicy, MACStats,
    MLSPolicy, PolicyCapability, PolicyInfo, PolicyType, SecurityContext, SecurityDomain,
    SecurityLevel as MacSecurityLevel, SecurityOperation, SimpleMACEngine,
};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use pki::{
    Certificate, CertificateType, PKIError, PKIManager, SimpleCRL, SimpleCertificate,
    SimplePKIManager, CRL,
};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use secrets::{
    Keyring, KeyringCapability, KeyringStats, Secret, SecretCapability, SecretError, SecretInfo,
    SecretType, SimpleKeyring, SimpleSecret,
};
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
pub use vulnerability::{
    CIPipelineIntegration, ScanError, ScanReport, Severity as VulnerabilitySeverity,
    SimpleCIPipelineIntegration, SimpleScanReport, SimpleVulnerability, SimpleVulnerabilityScanner,
    Vulnerability, VulnerabilityID, VulnerabilityScanner,
};
