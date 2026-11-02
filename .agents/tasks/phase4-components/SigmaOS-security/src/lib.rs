//! SigmaOS Security subsystem — secure defaults, privilege model, package trust,
//! audit logging, and security event reporting.
//!
//! Implements the `security-trust.md` platform contract.

#![allow(dead_code)]

use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Trust levels
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Signed by the SigmaOS project key (highest trust).
    SigmaOS,
    /// Signed by a community maintainer key in the keyring.
    Community,
    /// No signature (user explicitly opted in).
    Unsigned,
    /// Unknown / not verified.
    Unknown,
}

// ---------------------------------------------------------------------------
// Privilege model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Privilege {
    User,
    Admin,
    Root,
}

#[derive(Debug, Clone)]
pub struct PrivilegeCheck {
    pub action: String,
    pub required: Privilege,
    pub granted: bool,
}

pub fn check_privilege(action: &str, caller: Privilege, required: Privilege) -> PrivilegeCheck {
    let granted = (caller as u8) >= (required as u8);
    PrivilegeCheck { action: action.to_string(), required, granted }
}

// ---------------------------------------------------------------------------
// Audit log
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEventKind {
    PackageInstall,
    PackageRemove,
    ConfigChange,
    PrivilegeEscalation,
    AuthSuccess,
    AuthFailure,
    ServiceStart,
    ServiceStop,
    SecurityViolation,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub kind: AuditEventKind,
    pub actor: String,
    pub detail: String,
    pub severity: u8,   // 1 (info) – 5 (critical)
}

pub struct AuditLog {
    events: Vec<AuditEvent>,
    max_events: usize,
}

impl AuditLog {
    pub fn new(max_events: usize) -> Self {
        Self { events: Vec::new(), max_events }
    }

    pub fn record(&mut self, event: AuditEvent) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn events(&self) -> &[AuditEvent] { &self.events }

    pub fn security_violations(&self) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| e.kind == AuditEventKind::SecurityViolation).collect()
    }

    pub fn high_severity(&self) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| e.severity >= 4).collect()
    }
}

// ---------------------------------------------------------------------------
// Package trust verification
// ---------------------------------------------------------------------------

pub fn verify_package(name: &str, signature_present: bool, key_trusted: bool) -> TrustLevel {
    match (signature_present, key_trusted) {
        (true, true)   => TrustLevel::SigmaOS,
        (true, false)  => TrustLevel::Community,
        (false, _)     => TrustLevel::Unsigned,
    }
}

pub fn trust_gate(name: &str, level: TrustLevel, allow_unsigned: bool) -> Result<(), String> {
    if level == TrustLevel::Unknown {
        return Err(format!("{}: unknown trust level — blocked", name));
    }
    if level == TrustLevel::Unsigned && !allow_unsigned {
        return Err(format!("{}: unsigned package — blocked (use --allow-unsigned to override)", name));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Hardening profile
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardeningProfile {
    Default,
    Workstation,
    Server,
    Hardened,
}

pub struct HardeningConfig {
    pub profile: HardeningProfile,
    pub apparmor_enabled: bool,
    pub firewall_default_deny: bool,
    pub kernel_lockdown: bool,
    pub secure_boot: bool,
    pub audit_all_syscalls: bool,
}

impl HardeningConfig {
    pub fn for_profile(p: HardeningProfile) -> Self {
        match p {
            HardeningProfile::Default => Self { profile: p, apparmor_enabled: true, firewall_default_deny: false, kernel_lockdown: false, secure_boot: false, audit_all_syscalls: false },
            HardeningProfile::Workstation => Self { profile: p, apparmor_enabled: true, firewall_default_deny: false, kernel_lockdown: false, secure_boot: true, audit_all_syscalls: false },
            HardeningProfile::Server => Self { profile: p, apparmor_enabled: true, firewall_default_deny: true, kernel_lockdown: false, secure_boot: true, audit_all_syscalls: false },
            HardeningProfile::Hardened => Self { profile: p, apparmor_enabled: true, firewall_default_deny: true, kernel_lockdown: true, secure_boot: true, audit_all_syscalls: true },
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_verification() {
        assert_eq!(verify_package("foo", true, true), TrustLevel::SigmaOS);
        assert_eq!(verify_package("foo", true, false), TrustLevel::Community);
        assert_eq!(verify_package("foo", false, false), TrustLevel::Unsigned);
    }

    #[test]
    fn trust_gate_blocks_unsigned() {
        assert!(trust_gate("pkg", TrustLevel::Unsigned, false).is_err());
        assert!(trust_gate("pkg", TrustLevel::Unsigned, true).is_ok());
        assert!(trust_gate("pkg", TrustLevel::SigmaOS, false).is_ok());
    }

    #[test]
    fn privilege_check() {
        let r = check_privilege("install", Privilege::Admin, Privilege::User);
        assert!(r.granted);
        let r2 = check_privilege("install", Privilege::User, Privilege::Root);
        assert!(!r2.granted);
    }

    #[test]
    fn audit_log_records_and_filters() {
        let mut log = AuditLog::new(100);
        log.record(AuditEvent { kind: AuditEventKind::PackageInstall, actor: "alice".into(), detail: "installed foo".into(), severity: 2 });
        log.record(AuditEvent { kind: AuditEventKind::SecurityViolation, actor: "unknown".into(), detail: "blocked syscall".into(), severity: 5 });
        assert_eq!(log.events().len(), 2);
        assert_eq!(log.security_violations().len(), 1);
        assert_eq!(log.high_severity().len(), 1);
    }

    #[test]
    fn hardening_profiles() {
        let h = HardeningConfig::for_profile(HardeningProfile::Hardened);
        assert!(h.firewall_default_deny);
        assert!(h.kernel_lockdown);
        assert!(h.audit_all_syscalls);
        let d = HardeningConfig::for_profile(HardeningProfile::Default);
        assert!(!d.firewall_default_deny);
    }
}
