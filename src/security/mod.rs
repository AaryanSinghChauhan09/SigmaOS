// SigmaOS Security Subsystem
pub mod capability;
pub mod pqc_enclave;

pub use pqc_enclave::{
    KyberKem, DilithiumSignature, RotatableToken, PqcTokenRotationBus, PqcZeroTrustGater,
};
pub mod capability_enforcer;
pub mod defensive_audit;
pub mod parrot;
pub mod pledge;
pub mod unveil;
pub mod vulnerability;
pub mod hardening;
pub mod qubes_isolation;
pub mod bridge;
pub mod prism;
pub mod sandbox;

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
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use unveil::{UnveilManager, UnveilPermission, UnveilRestriction};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use qubes_isolation::{DomainID, DomainType, IsolationError, IsolatedDomain, DomainOrchestrator};
pub use bridge::{
    LegacySecurityType, SecurityBridge,
};
pub use prism::{
    SecurityFacet, SecurityPrism,
};
pub use sandbox::{
    SandboxRule, PrivacyFirstSandbox,
};
