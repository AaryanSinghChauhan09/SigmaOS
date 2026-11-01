//! Integration tests: privilege separation boundary enforcement
//!
//! Validates that `PrivilegeSeparator` correctly grants, denies, and revokes
//! capabilities, and that privilege escalation is blocked once `no_new_privs`
//! is applied.

use sigmaos_security::{Capability, PrivilegeSeparator, SecurityError};

// ── Grant / check round-trip ──────────────────────────────────────────────

#[test]
fn granted_capability_passes_check() {
    // Arrange
    let mut sep = PrivilegeSeparator::new("org.sigmaos.filemanager");
    // Act
    sep.grant(Capability::Read);
    // Assert
    assert!(sep.check(Capability::Read).is_ok());
}

#[test]
fn unganted_capability_fails_check() {
    let sep = PrivilegeSeparator::new("org.sigmaos.browser");
    assert_eq!(
        sep.check(Capability::Admin),
        Err(SecurityError::CapabilityMissing)
    );
}

// ── Revoke removes the capability ────────────────────────────────────────

#[test]
fn revoked_capability_is_denied() {
    // Arrange
    let mut sep = PrivilegeSeparator::new("org.sigmaos.network");
    sep.grant(Capability::Network);
    assert!(sep.check(Capability::Network).is_ok());
    // Act
    sep.revoke(Capability::Network);
    // Assert
    assert_eq!(
        sep.check(Capability::Network),
        Err(SecurityError::CapabilityMissing)
    );
}

// ── Untrusted process has no capabilities by default ─────────────────────

#[test]
fn new_separator_has_empty_capability_set() {
    let sep = PrivilegeSeparator::new("org.sigmaos.untrusted");
    assert!(sep.capability_set().is_empty());
    for cap in [
        Capability::Read,
        Capability::Write,
        Capability::Execute,
        Capability::Network,
        Capability::Mount,
        Capability::Admin,
    ] {
        assert_eq!(
            sep.check(cap),
            Err(SecurityError::CapabilityMissing),
            "Expected denial for {:?}",
            cap
        );
    }
}

// ── Privileged capabilities are identified correctly ─────────────────────

#[test]
fn mount_and_admin_are_privileged() {
    assert!(Capability::Mount.is_privileged());
    assert!(Capability::Admin.is_privileged());
}

#[test]
fn read_write_execute_network_are_not_privileged() {
    assert!(!Capability::Read.is_privileged());
    assert!(!Capability::Write.is_privileged());
    assert!(!Capability::Execute.is_privileged());
    assert!(!Capability::Network.is_privileged());
}

// ── no_new_privs prevents capability escalation ──────────────────────────

#[test]
fn no_new_privs_is_initially_unset() {
    let sep = PrivilegeSeparator::new("org.sigmaos.app");
    assert!(!sep.has_no_new_privs());
}

#[test]
fn apply_no_new_privs_sets_the_flag() {
    let mut sep = PrivilegeSeparator::new("org.sigmaos.app");
    sep.apply_no_new_privs();
    assert!(sep.has_no_new_privs());
}

#[test]
fn no_new_privs_does_not_remove_existing_capabilities() {
    // Capabilities granted *before* no_new_privs are retained.
    let mut sep = PrivilegeSeparator::new("org.sigmaos.service");
    sep.grant(Capability::Network);
    sep.apply_no_new_privs();
    assert!(sep.check(Capability::Network).is_ok());
}

// ── Process name is preserved ────────────────────────────────────────────

#[test]
fn process_name_is_stored_correctly() {
    let sep = PrivilegeSeparator::new("org.sigmaos.init");
    assert_eq!(sep.process_name, "org.sigmaos.init");
}

// ── Multiple capabilities on one separator ───────────────────────────────

#[test]
fn multiple_capabilities_can_be_granted_independently() {
    let mut sep = PrivilegeSeparator::new("org.sigmaos.compositor");
    sep.grant(Capability::Read);
    sep.grant(Capability::Write);
    sep.grant(Capability::Execute);

    assert!(sep.check(Capability::Read).is_ok());
    assert!(sep.check(Capability::Write).is_ok());
    assert!(sep.check(Capability::Execute).is_ok());
    // Network was never granted
    assert_eq!(
        sep.check(Capability::Network),
        Err(SecurityError::CapabilityMissing)
    );
}

#[test]
fn revoking_one_capability_does_not_affect_others() {
    let mut sep = PrivilegeSeparator::new("org.sigmaos.editor");
    sep.grant(Capability::Read);
    sep.grant(Capability::Write);
    sep.revoke(Capability::Write);

    assert!(sep.check(Capability::Read).is_ok());
    assert_eq!(
        sep.check(Capability::Write),
        Err(SecurityError::CapabilityMissing)
    );
}
