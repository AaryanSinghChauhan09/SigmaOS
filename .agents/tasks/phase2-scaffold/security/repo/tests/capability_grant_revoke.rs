//! Integration tests: capability grant and revoke lifecycle
//!
//! Covers `CapabilitySet` grant/revoke semantics, bitmask correctness,
//! and the full grant-check-revoke-check cycle.

use sigmaos_security::{Capability, CapabilitySet};

// ── Initial state ─────────────────────────────────────────────────────────

#[test]
fn new_capability_set_is_empty() {
    let set = CapabilitySet::new();
    assert!(set.is_empty());
    assert_eq!(set.bits(), 0);
}

#[test]
fn default_capability_set_is_empty() {
    let set = CapabilitySet::default();
    assert!(set.is_empty());
}

// ── Grant ─────────────────────────────────────────────────────────────────

#[test]
fn granting_a_capability_is_reflected_in_contains() {
    let mut set = CapabilitySet::new();
    set.grant(Capability::Read);
    assert!(set.contains(Capability::Read));
    assert!(!set.is_empty());
}

#[test]
fn grant_all_capabilities_individually() {
    let mut set = CapabilitySet::new();
    let all = [
        Capability::Read,
        Capability::Write,
        Capability::Execute,
        Capability::Network,
        Capability::Mount,
        Capability::Admin,
    ];
    for cap in all {
        set.grant(cap);
    }
    for cap in all {
        assert!(set.contains(cap), "{:?} should be present", cap);
    }
}

#[test]
fn granting_same_capability_twice_is_idempotent() {
    let mut set = CapabilitySet::new();
    set.grant(Capability::Network);
    let bits_after_first = set.bits();
    set.grant(Capability::Network);
    assert_eq!(set.bits(), bits_after_first);
    assert!(set.contains(Capability::Network));
}

// ── Revoke ────────────────────────────────────────────────────────────────

#[test]
fn revoking_a_granted_capability_removes_it() {
    let mut set = CapabilitySet::new();
    set.grant(Capability::Write);
    assert!(set.contains(Capability::Write));

    set.revoke(Capability::Write);
    assert!(!set.contains(Capability::Write));
}

#[test]
fn revoking_a_missing_capability_is_a_no_op() {
    let mut set = CapabilitySet::new();
    // Admin was never granted — revoke should be safe
    set.revoke(Capability::Admin);
    assert!(!set.contains(Capability::Admin));
    assert!(set.is_empty());
}

#[test]
fn revoking_one_capability_leaves_others_intact() {
    let mut set = CapabilitySet::new();
    set.grant(Capability::Read);
    set.grant(Capability::Write);
    set.grant(Capability::Execute);

    set.revoke(Capability::Write);

    assert!(set.contains(Capability::Read));
    assert!(!set.contains(Capability::Write));
    assert!(set.contains(Capability::Execute));
}

// ── Full grant-check-revoke-check cycle ──────────────────────────────────

#[test]
fn full_grant_revoke_cycle() {
    let mut set = CapabilitySet::new();

    // Grant
    set.grant(Capability::Mount);
    assert!(set.contains(Capability::Mount));
    assert!(!set.is_empty());

    // Revoke
    set.revoke(Capability::Mount);
    assert!(!set.contains(Capability::Mount));
    assert!(set.is_empty());
}

// ── from_caps constructor ─────────────────────────────────────────────────

#[test]
fn from_caps_initialises_with_given_capabilities() {
    let set = CapabilitySet::from_caps(&[Capability::Read, Capability::Network]);
    assert!(set.contains(Capability::Read));
    assert!(set.contains(Capability::Network));
    assert!(!set.contains(Capability::Admin));
    assert!(!set.contains(Capability::Mount));
}

#[test]
fn from_caps_empty_slice_gives_empty_set() {
    let set = CapabilitySet::from_caps(&[]);
    assert!(set.is_empty());
}

// ── Bitmask uniqueness: each variant maps to a distinct bit ───────────────

#[test]
fn each_capability_has_a_unique_bit() {
    let all = [
        Capability::Read,
        Capability::Write,
        Capability::Execute,
        Capability::Network,
        Capability::Mount,
        Capability::Admin,
    ];
    let mut seen_bits = std::collections::HashSet::new();
    for cap in all {
        let mut set = CapabilitySet::new();
        set.grant(cap);
        let bit = set.bits();
        assert!(
            seen_bits.insert(bit),
            "Duplicate bitmask for {:?}: {:#018x}",
            cap,
            bit
        );
    }
}

// ── Grant reverts to non-empty after revoke + re-grant ───────────────────

#[test]
fn re_grant_after_revoke_restores_capability() {
    let mut set = CapabilitySet::new();
    set.grant(Capability::Execute);
    set.revoke(Capability::Execute);
    assert!(!set.contains(Capability::Execute));

    set.grant(Capability::Execute);
    assert!(set.contains(Capability::Execute));
    assert!(!set.is_empty());
}
