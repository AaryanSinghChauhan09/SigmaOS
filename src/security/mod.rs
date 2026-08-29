// SigmaOS Security Subsystem
pub mod audit;
pub mod capability;
pub mod hardening;
pub mod kernel_hardening;

pub mod bridge;
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod deobfuscation;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod mac;
pub mod password;
pub mod openbsd_karl;
pub mod pki;
pub mod pledge;
pub use deobfuscation::ArithmeticSubstitutionDeobfuscator;
pub mod prism;
pub mod qubes_isolation;
pub mod root_improvement;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub use sigma_unveil as unveil;
pub mod vault;
pub mod vpn;
pub mod vulnerability;
pub mod parrot_linux;
pub mod ai_anomaly_detection;

pub use openbsd_karl::{KarlKernelRelinker, KernelBinarySection, KernelSectionKind};
pub use qubes_isolation::*;
pub use root_improvement::*;
pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use capability::{CapabilityGate, CapabilityToken, LinuxCapability, LinuxCapabilitySet, Permission};
pub use capability_enforcer::{CapabilityToken as RuntimeCapabilityToken, SecurityEnforcer};
pub use capability_token::{
    CapabilityToken as AndroidStyleCapabilityToken,
    SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel as ClipboardSecurityLevel, XorEncryption,
};
pub use forensics::*;
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use kernel_hardening::{
    HardenedSyscallDispatcher, HardenedSyscallError, MemoryAccessError,
    PagePermissions, PledgePromise as KernelPledgePromise, RetpolineKptiMitigationEngine,
    SmepSmapEnforcer, SovereignKaslrEngine, SyscallCategory,
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
    AppArmorManager, AppArmorProfile, ObjectType, SecurityContext, SecurityLabel, SecurityPolicy,
    SecurityRule, SelinuxPermission,
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
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
    SimpleVulnerabilityScanner,
};
pub use ai_anomaly_detection::{
    AiAnomalyDetector, AnomalyEvent, AnomalySeverity, AnomalyStatistics, AnomalyType,
    BehavioralBaseline,
};
