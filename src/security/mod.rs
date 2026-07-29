pub mod lsm;

pub mod audit;
pub mod capability;
pub mod capability_enforcer;
pub mod cleaner;
pub mod clipboard;
pub mod forensics;
pub mod integrity;
pub mod mac;
pub mod phantom;
pub mod pki;
pub mod pledge;
pub mod secrets;
pub mod vault;
pub mod vpn;
pub mod vulnerability;
#[cfg(any())]
pub mod clipboard;
#[cfg(any())]
pub mod intrusion;
#[cfg(any())]
pub mod password;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod vault;
pub mod vpn;
pub mod vulnerability;

pub use audit::{AuditEvent, AuditLogger, LogFormat, SimpleAuditEvent, SimpleAuditLogger};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use sigma_pledge::{PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter as SigmaSyscallFilter};
pub use sigma_unveil::{UnveilPermissions, UnveilEntry, UnveilState, UnveilManager};
pub use cleaner::{SecureCleaner, TorAnonymityGate, AmnesiaManager, MetadataScrubber};
pub use forensics::{ForensicAnalyzer, ExtractedMetadata, RecoveredFile};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel, XorEncryption,
};
pub use integrity::{File, IntegrityError, IntegrityMonitor, IntegrityStatus, SimpleIntegrityMonitor};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use mac::{MACEngine, MACPolicy, SecurityContext as MacSecurityContext, SimpleMACEngine};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use pki::{Certificate, PKIError, PKIManager};
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
// Integrity: export the monitor trait and concrete types that actually exist
pub use integrity::{
    File as IntegrityFile, FileCapability, FileID, FileInfo, IntegrityError, IntegrityMonitor,
    IntegrityStats, IntegrityStatus, MonitorCapability, SimpleFile, SimpleIntegrityMonitor,
};
// MAC: export what the module actually defines
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
    CIPipelineIntegration, ScanReport, ScanSummary, SimpleCIPipelineIntegration, SimpleScanReport,
    SimpleVulnerability, SimpleVulnerabilityScanner, Vulnerability, VulnerabilityScanner,
};
