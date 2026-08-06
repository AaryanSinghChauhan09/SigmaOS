// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod audit;
pub mod capability;
pub mod capability_enforcer;
pub mod capability_token;
pub mod cleaner;
pub mod clipboard;
pub mod defensive_audit;
pub mod hardening;
pub mod parrot_kali;
pub mod password;
pub mod pledge;
pub mod qubes_isolation;
pub mod scanner;
pub mod secrets;
pub mod selinux;
pub mod sigma_pledge;
pub mod sigma_unveil;
pub mod vault;
pub mod jails;
pub mod securelevels;
pub mod vpn;
pub mod vulnerability;

pub use self::jails::{Jail, JailCapabilities, JailManager};
pub use self::securelevels::{Securelevel, get_securelevel, set_securelevel, check_raw_disk_write_allowed, check_kernel_memory_write_allowed, check_immutable_flag_change_allowed, check_disk_partition_allowed, check_time_adjustment_allowed, check_firewall_modification_allowed};
pub use self::sigma_pledge::{PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter};
pub use self::sigma_unveil::{UnveilEntry, UnveilManager, UnveilPermissions, UnveilState};
pub use audit::{AuditLogger, AuditPolicy, LogFormat};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use defensive_audit::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use parrot_kali::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
    GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use qubes_isolation::{
    DomainID, DomainOrchestrator, DomainType, IsolatedDomain, IsolationError,
};
pub use selinux::{
    AppArmorManager, AppArmorProfile, ObjectType, SecurityContext, SecurityLabel, SecurityPolicy,
    SecurityRule, SelinuxPermission,
};
