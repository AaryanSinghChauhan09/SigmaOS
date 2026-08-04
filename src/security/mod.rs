pub mod lsm;

pub mod audit;
pub mod capability;
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod clipboard;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod intrusion;
pub mod mac;
pub mod parrot_linux;
pub mod password;
pub mod password;
pub mod phantom;
pub mod pki;
||||||| 43be3a7e8
pub mod defensive_audit;
pub mod parrot;
||||||| 43be3a7e8
pub mod integrity;
pub mod mac;
pub mod phantom;
pub mod pki;
pub mod pledge;
pub mod qubes_isolation;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod selinux;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod vault;
pub mod vpn;
pub mod vulnerability;
pub mod clipboard;
pub mod intrusion;
pub mod password;
pub mod parrot_linux;
pub mod selinux;
||||||| 43be3a7e8
pub mod secrets;
pub mod securelevels;
pub mod unveil;
pub mod vulnerability;
pub mod clipboard;
pub mod intrusion;
pub mod password;
pub mod parrot_linux;
pub mod selinux;
||||||| 165ded71c
pub mod clipboard;
pub mod intrusion;
pub mod password;
pub mod parrot_linux;
pub mod selinux;

pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use capability_enforcer::SecurityEnforcer;
pub use capability_token::{PORT_ALLOW_SSL, PORT_ALLOW_TCP};
pub use cleaner::{AmnesiaManager, MetadataScrubber, SecureCleaner, TorAnonymityGate};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel, XorEncryption,
};
pub use forensics::{ExtractedMetadata, ForensicAnalyzer, RecoveredFile};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use parrot_linux::{
    AnonsurfEngine, AnonymityMode, ForensicsAuditTool, IntrusionAlert, IntrusionSeverity,
    KaliSniffer, PentestAssistant, RecoveredFile, SecureWipeTool, SigmaIDS, SniffedPacket,
};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
||||||| 43be3a7e8
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub use parrot::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
    GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
||||||| 43be3a7e8
pub use phantom::{CapabilityContext, KernelLevel, SecurityAdminLevel, UserLevel};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use selinux::{
    AccessVectorCache, AppArmorManager, AppArmorProfile, ObjectType, SecurityContext,
    SecurityLabel, SecurityPolicy, SecurityRule, SelinuxBoolean, SelinuxPermission,
    TypeTransitionRule,
};
pub use sigma_pledge::{
    PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter as SigmaSyscallFilter,
};
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
    SecurityContext as MacSecurityContext, SecurityDomain, SecurityLevel as MacSecurityLevel,
    SimpleMACEngine,
};
// PKI: export actual types
pub use pki::{
    Certificate, PKIError, PKIManager, SimpleCRL, SimpleCertificate, SimplePKIManager,
    CRL as CrlTrait,
};
// Secrets: export actual types
pub use secrets::{
    Keyring, KeyringCapability, KeyringStats, Secret, SecretCapability, SecretInfo, SecretManager,
    SecretStorage, SecretType, SimpleKeyring, SimpleSecret,
};
// Vulnerability: export actual types
pub use vulnerability::{
    CIPipelineIntegration, ScanReport, ScanSummary, SimpleCIPipelineIntegration, SimpleScanReport,
    SimpleVulnerability, SimpleVulnerabilityScanner, Vulnerability, VulnerabilityScanner,
};
||||||| 43be3a7e8
pub use securelevels::{LinuxCapability, Securelevel, SovereignSecurelevelManager};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
