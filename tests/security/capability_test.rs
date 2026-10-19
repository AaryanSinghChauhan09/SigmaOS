//! SigmaOS Security & Capability Enforcement Test Suite
//!
//! Validates process capability token verification, pledge/unveil sandboxing,
//! privilege dropping, fail-closed error propagation, and negative security
//! assertions across kernel boundary calls.

#[path = "../../src/userland/security_sandbox.rs"]
mod security_sandbox;

use security_sandbox::{PledgeCapability, UserlandSecuritySandbox};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapToken {
    CapSystemAdmin = 1 << 0,
    CapMount       = 1 << 1,
    CapNetAdmin    = 1 << 2,
    CapProcessCtrl = 1 << 3,
    CapDeviceAccess= 1 << 4,
    CapAuditLog    = 1 << 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSecurityContext {
    pub pid: u32,
    pub uid: u32,
    pub capabilities: u64,
    pub sandbox: UserlandSecuritySandbox,
}

impl ProcessSecurityContext {
    pub fn new(pid: u32, uid: u32, initial_caps: u64) -> Self {
        Self {
            pid,
            uid,
            capabilities: initial_caps,
            sandbox: UserlandSecuritySandbox::new(pid as u64),
        }
    }

    pub fn has_capability(&self, cap: CapToken) -> bool {
        (self.capabilities & (cap as u64)) != 0
    }

    pub fn drop_capability(&mut self, cap: CapToken) {
        self.capabilities &= !(cap as u64);
    }

    pub fn check_privileged_operation(&self, required_cap: CapToken) -> Result<(), &'static str> {
        if !self.has_capability(required_cap) {
            return Err("EPERM: Permission denied - missing required capability token");
        }
        if required_cap == CapToken::CapNetAdmin && !self.sandbox.check_pledge(PledgeCapability::INet) {
            return Err("EACCES: Operation restricted by active pledge profile");
        }
        Ok(())
    }
}

#[test]
fn test_capability_presence_and_dropping() {
    let mut ctx = ProcessSecurityContext::new(101, 1000, (CapToken::CapMount as u64) | (CapToken::CapNetAdmin as u64));

    assert!(ctx.has_capability(CapToken::CapMount));
    assert!(ctx.has_capability(CapToken::CapNetAdmin));
    assert!(!ctx.has_capability(CapToken::CapSystemAdmin));

    // Verify privileged operation succeeds when capability present
    assert!(ctx.check_privileged_operation(CapToken::CapMount).is_ok());

    // Permanently drop CapMount
    ctx.drop_capability(CapToken::CapMount);
    assert!(!ctx.has_capability(CapToken::CapMount));

    // Negative assertion: Attempting mount without CapMount must fail with EPERM
    let res = ctx.check_privileged_operation(CapToken::CapMount);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "EPERM: Permission denied - missing required capability token");
}

#[test]
fn test_unprivileged_process_privilege_escalation_denied() {
    // Unprivileged context (no capabilities)
    let ctx = ProcessSecurityContext::new(202, 1001, 0);

    assert_eq!(
        ctx.check_privileged_operation(CapToken::CapSystemAdmin),
        Err("EPERM: Permission denied - missing required capability token")
    );
    assert_eq!(
        ctx.check_privileged_operation(CapToken::CapDeviceAccess),
        Err("EPERM: Permission denied - missing required capability token")
    );
    assert_eq!(
        ctx.check_privileged_operation(CapToken::CapAuditLog),
        Err("EPERM: Permission denied - missing required capability token")
    );
}

#[test]
fn test_userland_pledge_and_unveil_integration() {
    let mut ctx = ProcessSecurityContext::new(303, 1000, 0xFFFFFFFFFFFFFFFF);

    // Apply OpenBSD pledge "stdio rpath" (dropping INet)
    assert!(ctx.sandbox.pledge("stdio rpath").is_ok());

    // NetAdmin fails because INet pledge is missing
    let res = ctx.check_privileged_operation(CapToken::CapNetAdmin);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "EACCES: Operation restricted by active pledge profile");

    // Apply unveil rules
    assert!(ctx.sandbox.unveil("/var/log", "r").is_ok());
    assert!(ctx.sandbox.check_unveil("/var/log/syslog", "r"));
    assert!(!ctx.sandbox.check_unveil("/var/log/syslog", "w"));
    assert!(!ctx.sandbox.check_unveil("/etc/shadow", "r"));

    // W^X memory protection check
    assert!(!ctx.sandbox.check_memory_protection(true, true)); // W^X violation
}
