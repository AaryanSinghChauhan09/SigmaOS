// SigmaOS Security Subsystem
pub mod capability;
pub mod pqc_enclave;
pub mod governance;
pub mod audit;
pub mod bsd_hardening;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod vault;
pub mod vpn;
pub mod exec_guard;

// SigmaOS Security Subsystem
pub mod seccomp;
pub mod seccomp_ebpf;
pub mod syscall_filter;
pub mod defensive_audit;
pub mod hardening;
pub mod kernel_hardening;
pub mod user_namespace;

pub mod bridge;
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod deobfuscation;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod libgksu;
pub mod mac;
pub mod openbsd_karl;
pub mod password;
pub mod pledge;
pub use deobfuscation::ArithmeticSubstitutionDeobfuscator;
pub mod kali_stack;
pub mod parrot;
pub mod parrot_kali;
pub mod parrot_linux;
pub mod parrot_parity;
pub mod pqc_measurement;
pub mod prism;
pub mod qubes_isolation;
pub mod root_improvement;
pub mod rules;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod selinux_advanced;
pub mod unveil;
pub mod vulnerability;
pub mod kali_components;
pub mod landlock;
pub use landlock::{LandlockEngine, LandlockPathBeneathAttr, LandlockRuleset};
pub mod landlock_sovereign;

pub use exec_guard::*;
pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use bsd_hardening::{
    AslrEngine, BsdHardeningSuite, BsdSysctlSecurelevelEnforcer, CapsicumCapability,
    CapsicumManager, MemoryPermission, PaxMprotect, PledgeManager as BsdPledgeManager,
    PledgePromise as BsdPledgePromise, SecurelevelState as BsdSecurelevelState,
    UnveilEntry as BsdUnveilEntry, UnveilManager as BsdUnveilManager,
    UnveilPermission as BsdUnveilPermission, WxEnforcer,
};
pub use capability::{
    CapabilityGate, CapabilityToken, LinuxCapability, LinuxCapabilitySet, Permission,
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
pub use defensive_audit::*;
pub use forensics::*;
pub use hardening::{
    MemoryProtectionState, RelroState, SecurityHardeningConfig, StackCanary,
};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use kali_stack::{
    CronDaemon, CronJob, DmesgLog, FirewallRule, IptablesFirewall, KaliAirgeddonWifiAudit,
    KaliError, KaliJohnTheRipperCracker, KaliMetasploitPayloadFilter, KaliSqlmapInjectionAuditor,
    KaliUndercoverThemeMode, KaliWiresharkPacketAnalyzer, PcapPacketHeader,
    PluggableAuthenticationModule, SudoPrivilegeEscalation, SwapSpaceManager, TmuxMultiplexer,
    TmuxPane, UndercoverDisguiseTheme, WifiFrameType,
};

pub use libgksu::{
    GksuAuthBackend, GksuDisplayServer, GksuExecutionRequest, GksuExecutionResult,
    GksuSecurityGuard, LibGksuGraphicalSudoEngine,
};
pub use openbsd_karl::{KarlKernelRelinker, KernelBinarySection, KernelSectionKind};
pub use parrot_kali::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, GLOBAL_ANONSURF,
    GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use parrot::ParrotSecurityFingerprintEngine;
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, Fido2Auth, FingerprintAuth,
    HardwareAuthManager, HardwareAuthPromptTarget, PasswordCategory, PasswordEntry, PasswordError,
    PasswordManager, PasswordManagerResult,
};
pub use pledge::{
    promises, PledgeError, PledgeManager, PledgePromise, PledgeManager as OriginalPledgeManager,
    PledgePromise as OriginalPledgePromise,
};
pub use qubes_isolation::*;
pub use root_improvement::*;
pub use pqc_measurement::{
    Dilithium5KernelSignatureVerifier, FedoraCryptoPolicyProfile, HybridPqcMeasurementEngine,
    SovereignFirmitasAttestationEngine, Tpm2PcrBank, Tpm2PcrRegister, TPM2_PCR_COUNT,
};
pub use rules::{
    AuditAccessType, AuditSyscallRule, AuditWatchRule, PfAction, PfFilterRule, PledgeRule,
    SecurelevelState, SovereignAuditRuleEngine, SovereignNetworkFilterRulesEngine,
    SovereignSandboxingRulesEngine, SovereignSecurelevelRuleEngine, SovereignSysctlHardeningRules,
    SysctlParameterRule, UnveilRule,
};
pub use selinux::{PolicyRule, SELinuxPolicy, SecurityContext, SigmaSELinux};
pub use selinux_advanced::{AdvancedSELinuxManager, MlsLevel, SELinuxBoolean, SELinuxModule};
pub use sigma_pledge::{PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter};
pub use sigma_unveil::{
    UnveilEntry as SigmaUnveilEntry, UnveilManager as SigmaUnveilManager, UnveilPermissions,
    UnveilState,
};
pub use vault::{
    Aes256GcmEncryption, ChaCha20Poly1305Encryption, EncryptedFile, EncryptedFileVault,
    EncryptionAlgorithm, Kyber1024Encryption, VaultEncryption, VaultError, VaultMetadata,
    VaultResult,
};
pub use vpn::{
    AuthMethod, ConnectionState, KillSwitchConfig, OpenVpnHandler, PiaDedicatedIpBinding,
    PiaMaceAdBlocker, PiaMultiHopShadowsocksBridge, PiaPortForwardingEngine, PiaServerRegion,
    PiaSplitTunnelGovernor, PiaStrictKillSwitch, PiaVpnManager, SecureVpnClient, SplitTunnelRule,
    VpnConfig, VpnConnectionResult, VpnError, VpnProtocol, VpnProtocolHandler, VpnStatistics,
    WireGuardHandler,
};
pub use crate::security::vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use crate::security::vulnerability::{SimpleVulnerability, SimpleVulnerabilityScanner};

// ─── Capability Monitor (pledge + unveil + Capsicum) ─────────────────────────
pub mod capability_monitor;
pub mod sudo_engine;
pub mod access_control_list;

pub use sudo_engine::*;
pub use access_control_list::*;
