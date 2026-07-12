//! SigmaOS MAC (Mandatory Access Control) Module
//! SELinux-style policy system with modular architecture

pub mod policy_engine;

pub use policy_engine::{
    SecurityContext,
    SecurityClass,
    Permission,
    PolicyRule,
    PolicyEffect,
    PolicyModule,
    PolicyDecision,
    PolicyEngine,
    CapabilityToken,
    CapabilitySet,
    CapabilityChecker,
    AuditEntry,
    AuditLogger,
};

/// Re-export C-compatible functions
pub use policy_engine::{
    mac_policy_engine_init,
    mac_policy_engine_get,
    mac_capability_checker_init,
    mac_capability_checker_get,
    mac_audit_logger_init,
    mac_audit_logger_get,
    mac_check_permission,
    mac_add_module,
    mac_enable_module,
    mac_disable_module,
    mac_has_capability,
    mac_raise_capability,
    mac_drop_capability,
    mac_log_audit,
};
