// SigmaOS Security Subsystem
pub mod capability;
pub mod capability_enforcer;
pub mod defensive_audit;
pub mod parrot;
pub mod pledge;
pub mod unveil;
pub mod vulnerability;
pub mod hardening;
pub mod kernel_hardening;
pub mod intrusion;
pub mod deobfuscation;
pub mod binary_protection;

pub use binary_protection::{BinaryProtectionManager, RelroMode, AslrMap, ChecksecReport};

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use hardening::{
    secure_zeroize, AuditLogEntry, CpuMitigationFlags, HardenedAuditTrail,
    HardenedSyscallDispatcher, IntrusionMonitor, IntrusionSeverity, KaslrConfig, KaslrError,
    KaslrManager, KaslrSlide, KernelSection, MemoryRegionPermission, SmepSmapEngine,
    SmepSmapViolation, SyscallHardeningConfig, SyscallHardeningError, SyscallRegisterState,
    UserAccessGuard, UserPtr,
};
pub use capability_enforcer::{SecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP};
pub use kernel_hardening::{HardenedSyscallDispatcher, SmepSmapEnforcer, SovereignKaslrEngine};
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
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise, ThreadSubPledgeContext};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};

pub use kali_stack::{
    PluggableAuthenticationModule, FirewallRule, IptablesFirewall, CronJob, CronDaemon,
    SudoPrivilegeEscalation, TmuxPane, TmuxMultiplexer, SwapSpaceManager, DmesgLog, KaliError,
};
