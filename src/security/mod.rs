// SigmaOS Security Subsystem
pub mod capability;
pub mod pqc_enclave;
pub mod governance;

pub use pqc_enclave::{
    KyberKem, DilithiumSignature, RotatableToken, PqcTokenRotationBus, PqcZeroTrustGater,
};
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod mac;
pub mod password;
pub mod pki;
pub mod pledge;
pub mod unveil;
pub mod selinux;
pub mod vulnerability;
pub mod hardening;
pub mod deobfuscation;
pub mod securelevels;
pub mod audit;
pub mod bsd_hardening;
pub mod kali_components;
pub mod qubes_isolation;
pub mod root_improvement;
pub mod pam;
pub mod crypto_utils;
pub mod publication_permissions;

pub use publication_permissions::{
    AppPermissionRecord, FineGrainedAccessControlMatrix, HardwarePeripheralBounds,
    MatrixPolicyAction, PermissionGrantState, PortalPermissionScope,
    SovereignPublicationInspiredPermissionEngine,
};




pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use bsd_hardening::{
    AslrEngine, BsdHardeningSuite, CapsicumCapability, CapsicumManager, MemoryPermission,
    PaxMprotect, PledgeManager as BsdPledgeManager, PledgePromise as BsdPledgePromise,
    UnveilEntry as BsdUnveilEntry, UnveilManager as BsdUnveilManager,
    UnveilPermission as BsdUnveilPermission, WxEnforcer,
};
pub use capability::{
    CapabilityGate, CapabilityToken,LinuxCapabilitySet, Permission,
};
pub use capability_enforcer::{CapabilityToken as RuntimeCapabilityToken, SecurityEnforcer};
pub use capability_token::{
    CapabilityToken as AndroidStyleCapabilityToken,
    SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel as ClipboardSecurityLevel, XorEncryption,
};
pub use governance::{
    ComplianceFramework, ComplianceProfileEngine, ContainerSecurityPolicyEngine,
    DefaultSecurePosture, DeveloperKeyRotator, EncryptedHomeOptIn, GovernanceCharterManager,
    ImmutableAuditTrail, IncidentResponsePlaybook, LicensingAuditor, MacPolicyEngine,
    MacPolicyMode, NetworkZeroTrustEngine, PrivacyDashboardControls, PrivacyPreservingTelemetry,
    RuntimeAppSandbox, SbomManager, SecureUpdateChannel, SecurityPrivacyGovernanceMasterSuite,
    SystemSecretsKeyring, TpmHardwareAttestation, VulnerabilityDisclosureManager,
};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use hardening::{
    MemoryProtectionState, RelroState, SecurityHardeningConfig, StackCanary,
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
};
pub use vulnerability::{SimpleVulnerability, SimpleVulnerabilityScanner};
