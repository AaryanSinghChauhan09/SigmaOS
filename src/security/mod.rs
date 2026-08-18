// SigmaOS Security Subsystem
pub mod capability;
pub mod defensive_audit;
pub mod parrot;
pub mod pledge;
pub mod unveil;
pub mod selinux;
pub mod vulnerability;
pub mod hardening;
pub mod deobfuscation;
pub mod securelevels;
pub mod pam;
pub mod intrusion;
pub mod crypto_utils;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use intrusion::{
    CrowdStrikeFalconAi, DetectionResult, EventType, IntrusionDetectionSystem, RuleAction,
    SecurityEvent, Severity, SnortRule, SnortSignatureFirewall,
};
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub use parrot::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
    GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
pub use selinux::{
    AccessVectorCache, AvcKey, DynamicMacEnforcer, SeLinuxMode,
    SecurityContext as SelinuxSecurityContext, SelinuxEngine, SensitivityLevel,
};
pub use selinux::{
    SelinuxEngine as AppArmorManager, SeLinuxMode as AppArmorProfile,
    SelinuxEngine as SecurityPolicy, SelinuxEngine as SecurityLabel,
    SelinuxEngine as SecurityRule, SelinuxEngine as SelinuxPermission,
};
pub use securelevels::{Securelevel, LinuxCapability, SovereignSecurelevelManager};
pub use pam::{PamError, PamUser, PamGroup, SovereignPamManager};
pub use hardening::{
    secure_zeroize, AuditLogEntry, CpuMitigationFlags, HardenedAuditTrail,
    HardenedSyscallDispatcher, IntrusionMonitor, IntrusionSeverity, KaslrConfig, KaslrError,
    KaslrManager, KaslrSlide, KernelSection, MemoryRegionPermission, SmepSmapEngine,
    SmepSmapViolation, SyscallHardeningConfig, SyscallHardeningError, SyscallRegisterState,
    UserAccessGuard, UserPtr,
};
