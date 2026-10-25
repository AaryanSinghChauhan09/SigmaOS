//! Integration tests for Sovereign Security & Hardening Advancements V75 Suite.

#[path = "../src/security/sovereign_security_and_hardening_advancements_v75_pr.rs"]
mod sovereign_security_and_hardening_advancements_v75_pr;

use sovereign_security_and_hardening_advancements_v75_pr::*;

#[test]
fn test_v75_security_and_hardening_suite_full_integration() {
    let suite = SovereignSecurityAndHardeningAdvancementsV75Suite::new();
    let manifests = suite.generate_v75_pr_proposals();

    assert_eq!(manifests.len(), 3);
    assert_eq!(manifests[0].pr_id, 7501);
    assert_eq!(manifests[1].pr_id, 7502);
    assert_eq!(manifests[2].pr_id, 7503);

    assert!(manifests[0].title.contains("Landlock v9"));
    assert!(manifests[1].title.contains("Capsicum v3"));
    assert!(manifests[2].title.contains("Whonix Stream Isolation"));

    // Verify individual engine capabilities via suite accessor methods
    let _l = suite.linux_engine();
    let _b = suite.bsd_engine();
    let _a = suite.anti_forensic_engine();

    assert!(LinuxAccessControlAndSandboxingV75Engine::check_landlock_access(0b11, 0b01));
    assert!(BsdHardeningAndIsolationV75Engine::validate_pledge_promises("stdio rpath"));

    let mut ram_buf = [0xAAu8; 64];
    let wiped = AnonymityAndAntiForensicV75Engine::secure_memory_wipe(&mut ram_buf);
    assert_eq!(wiped, 64);
    assert_eq!(ram_buf[0], 0x00);
}
