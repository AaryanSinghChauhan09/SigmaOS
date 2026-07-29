pub mod lsm;

pub mod audit;
pub mod capability;
pub mod cleaner;
pub mod forensics;
pub mod integrity;
pub mod mac;
pub mod phantom;
pub mod pki;
pub mod pledge;
<<<<<<< HEAD
pub mod vulnerability;
pub mod parrot_parity;
=======
pub mod secrets;
pub mod vault;
pub mod vpn;
pub mod vulnerability;
<<<<<<< HEAD
pub mod cleaner;
pub mod forensics;
pub mod sigma_pledge;
pub mod sigma_unveil;
>>>>>>> origin/jules-18101178622594638830-97dc43c6
=======
#[cfg(any())]
pub mod clipboard;
#[cfg(any())]
pub mod intrusion;
#[cfg(any())]
pub mod password;
>>>>>>> origin/jules-6565657164915217370-c04e8c01

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
<<<<<<< HEAD
pub use vulnerability::{
    VulnerabilityClass, VulnerabilityReport, SecurityScanner, ExploitPayload,
    PenetrationAssistant,
};
pub use parrot_parity::{
    RoutingMode, AnonSurfShunt, SandboxPolicy, AppSandboxEngine, ForensicStorageFilter,
    GLOBAL_ANONSURF, GLOBAL_SANDBOX, GLOBAL_FORENSIC,
};
=======
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
>>>>>>> origin/jules-18101178622594638830-97dc43c6
