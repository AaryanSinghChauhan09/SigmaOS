// SigmaOS Security Subsystem
pub mod audit;
pub mod capability;
pub mod hardening;
pub mod pledge;
pub mod vulnerability;

pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use vulnerability::{
    CIPipelineIntegration, ScanSummary, SimpleCIPipelineIntegration,
    SimpleVulnerability, Vulnerability, VulnerabilityScanner,
};
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod defensive_audit;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod mac;
pub mod nemoclaw;
pub mod parrot_parity;
pub mod password;
pub mod pki;
pub mod bridge;
pub mod prism;
pub mod sandbox;
pub mod qubes_isolation;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod selinux_integration;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod vault;
pub mod vpn;
pub mod parrot_linux;

pub use self::sigma_pledge::{PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter};
pub use self::sigma_unveil::{UnveilEntry, UnveilManager, UnveilPermissions, UnveilState};
pub use audit::{AuditLogger, AuditPolicy, LogFormat};
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
pub use bridge::{
    LegacySecurityType, SecurityBridge,
};
pub use prism::{
    SecurityFacet, SecurityPrism,
};
pub use sandbox::{
    SandboxRule, PrivacyFirstSandbox,
};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use selinux_integration::{
    SelinuxSyscallIntegration, SelinuxError, SelinuxStats, SyscallSecurityClass,
    initialize_selinux_integration, get_selinux_integration, check_syscall_selinux,
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
pub use integrity::{
    File as IntegrityFile, FileCapability, FileID, FileInfo, IntegrityError as SecIntegrityError,
    IntegrityStats, IntegrityStatus as SecIntegrityStatus, MonitorCapability, SimpleFile,
    SimpleIntegrityMonitor as SecSimpleIntegrityMonitor,
};
pub use mac::{
    ContextCapability, ContextID, EngineCapability as MacEngineCapability, MACStats, MLSPolicy,
    PolicyCapability as MacPolicyCapability, PolicyInfo as MacPolicyInfo,
    SecurityContext as MacSecurityContext, SecurityDomain, SecurityLevel as MacSecurityLevel,
    SimpleMACEngine,
};
pub use pki::{
    PKIManager as SecPKIManager, SimpleCRL, SimpleCertificate, SimplePKIManager, CRL as CrlTrait,
};
pub use secrets::{
    Keyring, KeyringCapability, KeyringStats, Secret, SecretCapability, SecretInfo, SimpleKeyring,
    SimpleSecret,
};
pub use vulnerability::{
    CIPipelineIntegration, ScanSummary, SimpleCIPipelineIntegration,
    SimpleVulnerability, Vulnerability, VulnerabilityScanner,
};
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub use parrot_parity::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
};
pub use parrot_linux::{
    AnonsurfEngine, AnonymityMode, ForensicsAuditTool, RecoveredFile, KaliSniffer,
    SniffedPacket, PentestAssistant, SecureWipeTool, SigmaIDS, IntrusionAlert,
};
// NemoClaw Security Primitives
pub use nemoclaw::{DefaultDenyNetworkPolicy, NemoClawError, OpenShellAgentSandbox, PrivacyRouter};

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
