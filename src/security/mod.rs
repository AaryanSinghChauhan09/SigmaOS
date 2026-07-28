// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod audit;
pub mod capability;
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod defensive_audit;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod mac;
pub mod password;
pub mod pki;
pub mod pledge;
pub mod qubes_isolation;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod vault;
pub mod vpn;
pub mod vulnerability;

pub use audit::{AuditLogger, AuditPolicy, LogFormat};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use capability_enforcer::{CapabilityToken as RuntimeCapabilityToken, SecurityEnforcer};
pub use forensics::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
};
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub const GLOBAL_ANONSURF: u32 = 0;
pub const GLOBAL_FORENSIC: u32 = 0;
pub const GLOBAL_SANDBOX: u32 = 0;
pub use capability_token::{CapabilityToken as AndroidStyleCapabilityToken, SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP};
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
    AppArmorManager, AppArmorProfile, ObjectType, SelinuxPermission, SecurityContext as SelinuxSecurityContext,
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
// Integrity: export the monitor trait and concrete types that actually exist
pub use integrity::{
    File as IntegrityFile, FileCapability, FileID, FileInfo, IntegrityError, IntegrityMonitor,
    IntegrityStats, IntegrityStatus, MonitorCapability, SimpleFile, SimpleIntegrityMonitor,
};
// MAC: export what the module actually defines
pub use mac::{
    ContextCapability, ContextID, EngineCapability as MacEngineCapability, MACEngine, MACPolicy,
    MACStats, MLSPolicy, PolicyCapability as MacPolicyCapability, PolicyInfo as MacPolicyInfo,
    SecurityContext, SecurityDomain, SecurityLevel as MacSecurityLevel, SimpleMACEngine,
};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
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
