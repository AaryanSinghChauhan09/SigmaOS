// SPDX-License-Identifier: MIT
// Integration Test for Sovereign Linux & BSD Ecosystem Gap Closure V75 PR Suite
// (`tests/test_linux_bsd_ecosystem_gap_closure_v75_pr.rs`)

#[path = "../src/distro/sovereign_linux_bsd_ecosystem_gap_closure_v75_pr.rs"]
mod sovereign_linux_bsd_ecosystem_gap_closure_v75_pr;

use sovereign_linux_bsd_ecosystem_gap_closure_v75_pr::*;

#[test]
fn test_integration_v75_ecosystem_gap_closure_pr_suite() {
    let mut suite = SovereignLinuxBsdEcosystemGapClosureV75PrSuite::new();

    // Verify initial PR suite state
    assert_eq!(suite.proposals.len(), 0);
    assert_eq!(suite.next_pr_id, 7500);

    // Verify individual subsystem proposal engine generators
    let futex_diff = suite.io_uring_futex2.register_futex_waiter(0x1000, 42, 0, 1);
    assert!(futex_diff.contains("sys_futex_waitv_v75"));

    let casper_diff = suite.freebsd_capsicum.register_casper_service("system.dns", 3, 0x01);
    assert!(casper_diff.contains("cap_service_open"));

    let softraid_diff = suite.openbsd_softraid.create_crypto_volume("sd0", 2);
    assert!(softraid_diff.contains("bioctl"));

    let hammer2_diff = suite.dragonfly_hammer2.create_pfs_snapshot("SNAP_V75", 45005);
    assert!(hammer2_diff.contains("hammer2 pfs-create"));

    let gentoo_diff = suite.gentoo_portage.add_ebuild("sys-kernel/sigma-sources-1.0", "0", "1.0", vec!["hardened"]);
    assert!(gentoo_diff.contains("EAPI=10"));

    let alpine_diff = suite.alpine_apk.generate_overlay("sigma.apkovl.tar.gz", 12);
    assert!(alpine_diff.contains("lbu commit"));

    let nix_diff = suite.nix_guix.add_closure("/nix/store/v75-test", "hash75", vec!["/nix/store/glibc"]);
    assert!(nix_diff.contains("nix store add-path"));

    let netbsd_diff = suite.netbsd_npf.add_rule(10, "pass", "vioif0");
    assert!(netbsd_diff.contains("npfctl add group"));

    let clear_diff = suite.clear_linux.set_pstate("performance", 0);
    assert!(clear_diff.contains("scaling_governor"));

    let illumos_diff = suite.illumos_crossbow.create_vnic("vnic75", "e1000g0", 10000);
    assert!(illumos_diff.contains("dladm create-vnic"));

    // Evaluate full gap closure parity flow
    let result = suite.evaluate_v75_gap_closure_parity();
    assert!(result);

    // Confirm that PR proposals were created and merged
    assert_eq!(suite.proposals.len(), 3);
    for proposal in suite.proposals.values() {
        assert_eq!(proposal.status, LinuxBsdPrStatusV75::Merged);
        assert!(!proposal.title.is_empty());
        assert!(!proposal.unified_diff.is_empty());
    }
}
