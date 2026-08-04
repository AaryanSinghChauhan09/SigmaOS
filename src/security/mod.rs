// SigmaOS Security Module
// Comprehensive security subsystem: MAC, sandboxing, crypto, audit, networking security

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod audit;
pub mod bridge;
pub mod capability;
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod defensive_audit;
pub mod forensics;
pub mod hardening;
pub mod integrity;
pub mod intrusion;
pub mod kali_stack;
pub mod leanstral;
pub mod lsm;
pub mod mac;
pub mod nemoclaw;
pub mod parrot;
pub mod parrot_kali;
pub mod parrot_linux;
pub mod parrot_parity;
pub mod password;
pub mod phantom;
pub mod pki;
pub mod pledge;
pub mod prism;
pub mod qubes_isolation;
pub mod root_improvement;
pub mod sandbox;
pub mod scanner;
pub mod secrets;
pub mod securelevels;
pub mod selinux;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod unveil;
pub mod vault;
pub mod visa_harness;
pub mod vpn;
pub mod vulnerability;

pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use capability_enforcer::SecurityEnforcer;
pub use capability_token::{PORT_ALLOW_SSL, PORT_ALLOW_TCP};
pub use cleaner::{AmnesiaManager, MetadataScrubber, SecureCleaner, TorAnonymityGate};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel, XorEncryption,
};
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub use forensics::{ExtractedMetadata, ForensicAnalyzer, RecoveredFile};
pub use integrity::{
    File as IntegrityFile, FileCapability, FileID, FileInfo, IntegrityError, IntegrityMonitor,
    IntegrityStats, IntegrityStatus, MonitorCapability, SimpleFile, SimpleIntegrityMonitor,
};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use mac::{
    ContextCapability, ContextID, EngineCapability as MacEngineCapability, MACEngine, MACPolicy,
    MACStats, MLSPolicy, PolicyCapability as MacPolicyCapability, PolicyInfo as MacPolicyInfo,
    SecurityContext as MacSecurityContext, SecurityDomain, SecurityLevel as MacSecurityLevel,
    SimpleMACEngine,
};
pub use parrot_linux::{
    AnonsurfEngine, AnonymityMode, ForensicsAuditTool, IntrusionAlert, IntrusionSeverity,
    KaliSniffer, PentestAssistant, SecureWipeTool, SigmaIDS, SniffedPacket,
};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use phantom::{CapabilityContext, KernelLevel, SecurityAdminLevel, UserLevel};
pub use pki::{
    Certificate, PKIError, PKIManager, SimpleCRL, SimpleCertificate, SimplePKIManager,
    CRL as CrlTrait,
};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use secrets::{
    Keyring, KeyringCapability, KeyringStats, Secret, SecretCapability, SecretInfo, SecretManager,
    SecretStorage, SecretType, SimpleKeyring, SimpleSecret,
};
pub use securelevels::{LinuxCapability, Securelevel, SovereignSecurelevelManager};
pub use selinux::{
    AccessVectorCache, AppArmorManager, AppArmorProfile, ObjectType, SecurityContext,
    SecurityLabel, SecurityPolicy, SecurityRule, SelinuxBoolean, SelinuxPermission,
    TypeTransitionRule,
};
pub use sigma_pledge::{
    PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter as SigmaSyscallFilter,
};
pub use sigma_unveil::{UnveilEntry, UnveilManager as SigmaUnveilManager, UnveilPermissions, UnveilState};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
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
    CIPipelineIntegration, ScanReport, ScanSummary, SimpleCIPipelineIntegration, SimpleScanReport,
    SimpleVulnerability, SimpleVulnerabilityScanner, Vulnerability, VulnerabilityScanner,
};