// SigmaOS Security Subsystem
pub mod address_sanitizer;
pub mod capability_enforcer;
pub mod memory_protection;
pub use address_sanitizer::{
    AddressSanitizer, AsanConfig, AsanRegion, AsanStatistics, ShadowState,
};
pub mod cap_rights;
pub use cap_rights::CapRightsMask;
pub mod acpi_aml_interpreter;
pub mod arm_pac;
pub mod audit;
pub mod bsd_hardening;
pub mod capability;
pub mod capsicum;
pub mod capsicum_wrapper;
pub mod filesystem_encryption;
pub mod governance;
pub mod hardware_privilege;
pub mod ima_evm;
pub mod intel_cet;
pub mod lockdown;
pub mod pledge_unveil;
pub mod pqc_enclave;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod system_audit;
pub mod vault;
pub mod vfs_hardening;
pub mod vpn;

pub use acpi_aml_interpreter::{AcpiSleepState, AmlInterpreter, AmlValue};
pub use arm_pac::{ArmPacController, PacKeyType};
pub use ima_evm::{ImaEvmController, ImaLogEntry};
pub use intel_cet::IntelCetEngine;
pub use lockdown::{KernelLockdownController, LockdownMode, LockdownReason};
pub use vfs_hardening::{VfsHardeningConfig, VfsHardeningEnforcer};

// SigmaOS Security Subsystem
pub mod defensive_audit;
pub use defensive_audit::DefensiveAuditSystem;
pub mod hardening;
pub mod kernel_hardening;
pub mod seccomp;
pub mod seccomp_ebpf;
pub mod syscall_filter;
pub mod user_namespace;

pub mod bridge;
pub mod capability_based_security;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod crypto_utils;
pub mod deobfuscation;
pub mod forensics;
pub mod integrity;
pub mod intrusion;
pub mod libgksu;
pub mod mac;
pub mod openbsd_karl;
pub mod pam;
pub mod password;
pub mod pledge;
pub use deobfuscation::ArithmeticSubstitutionDeobfuscator;
pub mod kali_components;
pub mod kali_stack;
pub mod landlock;
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
pub mod selinux_type_enforcement;
pub mod unveil;
pub mod vulnerability;
pub use landlock::{LandlockEngine, LandlockPathBeneathAttr, LandlockRuleset};
pub mod landlock_sovereign;
pub mod unified_mac_sandbox;
pub use unified_mac_sandbox::*;

pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use bsd_hardening::{
    AslrEngine, BsdHardeningSuite, CapsicumCapability, CapsicumManager, MemoryPermission,
    PaxMprotect, PledgeManager as BsdPledgeManager, PledgePromise as BsdPledgePromise,
    UnveilEntry as BsdUnveilEntry, UnveilManager as BsdUnveilManager,
    UnveilPermission as BsdUnveilPermission, WxEnforcer,
};
pub use capability::{
    CapabilityGate, CapabilityToken, LinuxCapability, LinuxCapabilitySet, Permission,
};
pub use capability_enforcer::{
    CapabilityEnforcer, CapabilitySet, ResourcePermission, ResourceType,
};
pub use capability_token::{
    CapabilityToken as AndroidStyleCapabilityToken,
    SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel as ClipboardSecurityLevel, XorEncryption,
};
pub use defensive_audit::*;
pub use filesystem_encryption::{
    EncryptionAlgorithm as FsEncryptionAlgorithm, EncryptionStatus as FsEncryptionStatus,
    EncryptionType as FsEncryptionType, FilesystemEncryptionManager, FscryptDirectory, LuksDevice,
};
pub use forensics::*;
pub use hardening::{MemoryProtectionState, RelroState, SecurityHardeningConfig, StackCanary};
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
pub use kernel_hardening::{
    GdtSegmentDescriptor, HardenedSyscallDispatcher, HardenedSyscallError, MemoryAccessError,
    PagePermissions, PledgePromise as KernelPledgePromise, PrivilegeRing,
    RetpolineKptiMitigationEngine, SmepSmapEnforcer, SovereignKaslrEngine,
    SovereignRing3UserModeTssEngine, SyscallCategory, TaskStateSegment64,
};
pub use system_audit::{
    AuditAction, AuditConfig, AuditEvent as SystemAuditEvent,
    AuditEventType as SystemAuditEventType, AuditRule, SystemAuditManager,
};

pub use crate::security::vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use crate::security::vulnerability::{SimpleVulnerability, SimpleVulnerabilityScanner};
pub use capability_based_security::{CapabilityGrant, CapabilityManager, CapabilityType};
pub use libgksu::{
    GksuAuthBackend, GksuDisplayServer, GksuExecutionRequest, GksuExecutionResult,
    GksuSecurityGuard, LibGksuGraphicalSudoEngine,
};
pub use openbsd_karl::{KarlKernelRelinker, KernelBinarySection, KernelSectionKind};
pub use pam::{
    AccountTallyModule, PamError, PamGroup, PamModule, PamUser, PasswordQualityModule,
    SovereignPamManager,
};
pub use parrot::ParrotSecurityFingerprintEngine;
pub use parrot_kali::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, GLOBAL_ANONSURF,
    GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use pledge::{
    promises, PledgeError, PledgeManager, PledgeManager as OriginalPledgeManager, PledgePromise,
    PledgePromise as OriginalPledgePromise,
};
pub use pqc_measurement::{
    Dilithium5KernelSignatureVerifier, FedoraCryptoPolicyProfile, HybridPqcMeasurementEngine,
    SovereignFirmitasAttestationEngine, Tpm2PcrBank, Tpm2PcrRegister, TPM2_PCR_COUNT,
};
pub use qubes_isolation::*;
pub use root_improvement::*;
pub use rules::{
    AuditAccessType, AuditSyscallRule, AuditWatchRule, PfAction, PfFilterRule, PledgeRule,
    SecurelevelState, SovereignAuditRuleEngine, SovereignNetworkFilterRulesEngine,
    SovereignSandboxingRulesEngine, SovereignSecurelevelRuleEngine, SovereignSysctlHardeningRules,
    SysctlParameterRule, UnveilRule,
};
pub use selinux::{PolicyRule, SELinuxPolicy, SecurityContext, SigmaSELinux};
pub use selinux_advanced::{AdvancedSELinuxManager, MlsLevel, SELinuxBoolean, SELinuxModule};
pub use selinux_type_enforcement::{
    get_selinux_enforcement, init_selinux_enforcement, SecurityContext as SelinuxSecurityContext,
    SelinuxTypeEnforcement, TePolicy, TeRule,
};
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

// ─── Capability Monitor (pledge + unveil + Capsicum) ─────────────────────────
pub mod capability_monitor;
pub mod phase_audit;

pub use hardware_privilege::*;

pub use capsicum::{CapEntry, CapMode, CapRight, CapabilitySandbox};
pub use capsicum_wrapper::{
    CapRights, CapSandboxMode, CapsicumFdEntry, CapsicumManager as CapsicumWrapperManager,
    CapsicumStatistics,
};
pub use memory_protection::{
    AslrConfig, MemoryProtectionManager, MemoryProtectionMode, StackCanaryConfig,
};
pub use pledge_unveil::{
    PledgePromise as OpenBsdPledgePromise, PledgeSandbox, Sandbox, UnveilPermission, UnveilSandbox,
};
pub mod access_control_list;
pub mod ai_anomaly_detection;
pub mod binary_protection;
pub mod bpf_lsm_sovereign;
pub mod bsd_pledge;
pub mod capability_sandbox;
pub mod cgroups;
pub mod comprehensive_access_control;
pub mod dm_crypt;
pub mod exec_guard;
pub mod firewall;
pub mod firewall_manager;
pub mod governance_privacy;
pub mod hardened_sysctl;
pub mod ids_rule_parser;
pub mod input_validation;
pub mod jails;
pub mod kernel_security;
pub mod keys;
pub mod landlock_v4;
pub mod leanstral;
pub mod lsm;
pub mod mac_vfs_integration;
pub mod malware;
pub mod mandatory_access_control;
pub mod namespaces;
pub mod nemoclaw;
pub mod obfuscation;
pub mod obfuscator;
pub mod phantom;
pub mod pki;
pub mod pledge_impl;
pub mod privacy_manager;
pub mod publication_permissions;
pub mod safe_absorption_protocol;
pub mod sandbox;
pub mod seccomp_filter;
pub mod securelevels;
pub mod selinux_integration;
pub mod sigma_seccomp_bpf;
pub mod sovereign_portable_sandbox_engine;
pub mod sovereign_security_and_hardening_advancements_pr;
pub mod sudo_engine;
pub mod system_policy_rules;
pub mod tpm2;
pub mod trust_zone;
pub mod unified_access_control;
pub mod visa_harness;
pub use sovereign_security_and_hardening_advancements_pr::*;
pub mod sovereign_security_management_plan;
pub use sovereign_security_management_plan::*;
