// SigmaOS Security Subsystem
pub mod capability;
pub mod pledge;
pub mod vulnerability;
pub mod hardening;
pub mod defensive_audit;
pub mod parrot_parity;
pub mod parrot_linux;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use defensive_audit::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};
pub use parrot_parity::{AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, GLOBAL_ANONSURF, GLOBAL_SANDBOX, GLOBAL_FORENSIC};
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
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use selinux::{
    AppArmorManager, AppArmorProfile, ObjectType, SecurityContext, SecurityLabel, SecurityPolicy,
    SecurityRule, SelinuxPermission,
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
// Integrity: export the monitor trait and concrete types that actually exist
pub use integrity::{
    File as IntegrityFile, FileCapability, FileID, FileInfo, IntegrityError as SecIntegrityError,
    IntegrityStats, IntegrityStatus as SecIntegrityStatus, MonitorCapability, SimpleFile,
    SimpleIntegrityMonitor as SecSimpleIntegrityMonitor,
};
// MAC: export what the module actually defines
pub use mac::{
    ContextCapability, ContextID, EngineCapability as MacEngineCapability, MACStats, MLSPolicy,
    PolicyCapability as MacPolicyCapability, PolicyInfo as MacPolicyInfo,
    SecurityContext as MacSecurityContext, SecurityDomain, SecurityLevel as MacSecurityLevel,
    SimpleMACEngine,
};
// PKI: export actual types
pub use pki::{
    PKIManager as SecPKIManager, SimpleCRL, SimpleCertificate, SimplePKIManager, CRL as CrlTrait,
};
// Secrets: export actual types
pub use secrets::{
    Keyring, KeyringCapability, KeyringStats, Secret, SecretCapability, SecretInfo, SimpleKeyring,
    SimpleSecret,
};
// Vulnerability: export actual types
pub use vulnerability::{
    CIPipelineIntegration, ScanReport, ScanSummary, SimpleCIPipelineIntegration, SimpleScanReport,
    SimpleVulnerability, SimpleVulnerabilityScanner, Vulnerability, VulnerabilityScanner,
};
// Parrot Parity and Defensive Audit
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub use parrot_parity::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
    GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use parrot_linux::{
    AnonsurfEngine, AnonymityMode, ForensicsAuditTool, RecoveredFile, KaliSniffer,
    SniffedPacket, PentestAssistant, SecureWipeTool, SigmaIDS, IntrusionSeverity, IntrusionAlert,
};
// NemoClaw Security Primitives
pub use nemoclaw::{DefaultDenyNetworkPolicy, NemoClawError, OpenShellAgentSandbox, PrivacyRouter};

// Placeholder stubs for standard types to satisfy lib.rs exports
pub struct CronDaemon;
pub struct CronJob;
pub struct DmesgLog;
pub struct FirewallRule;
pub struct IptablesFirewall;
pub struct KaliError;
pub struct PluggableAuthenticationModule;
pub struct SudoPrivilegeEscalation;
pub struct SwapSpaceManager;
pub struct TmuxMultiplexer;
pub struct TmuxPane;
