#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Security Subsystem
pub mod capability;
pub mod defensive_audit;
pub mod hardening;
pub mod parrot_kali;
pub mod pledge;
pub mod qubes_isolation;
pub mod selinux;
pub mod vulnerability;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
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
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
