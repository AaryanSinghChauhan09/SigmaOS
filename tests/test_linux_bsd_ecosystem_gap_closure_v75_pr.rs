// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Linux & BSD Ecosystem Gap Closure V75 PR Suite

#[path = "../src/distro/sovereign_linux_bsd_ecosystem_gap_closure_v75_pr.rs"]
mod sovereign_linux_bsd_ecosystem_gap_closure_v75_pr;

use sovereign_linux_bsd_ecosystem_gap_closure_v75_pr::*;

#[test]
fn test_linux_bsd_ecosystem_gap_closure_v75_pr_suite_full_cycle() {
    let mut suite = SovereignLinuxBsdEcosystemGapClosureV75PrSuite::new();

    let parity_ok = suite.evaluate_v75_gap_closure_parity();
    assert!(parity_ok, "V75 Gap Closure Parity evaluation failed");

    assert_eq!(suite.proposals.len(), 3, "Expected 3 initial PR proposals submitted & merged");

    for (id, proposal) in &suite.proposals {
        assert_eq!(proposal.status, LinuxBsdPrStatusV75::Merged);
        assert!(!proposal.title.is_empty());
        assert!(!proposal.author.is_empty());
        assert!(!proposal.subsystem.is_empty());
        assert!(!proposal.original_distro.is_empty());
        assert!(!proposal.patch_summary.is_empty());
        assert!(!proposal.patch_diff.is_empty());
    }

    // Individual engine sanity checks
    let mut linux_futex = LinuxIoUringFutex2V75PrEngine::new();
    let futex_diff = linux_futex.register_futex_waiter(0x3000, 2, 0, 1);
    assert!(futex_diff.contains("futex_waitv"));

    let mut freebsd_capsicum = FreeBsdCapsicumCasperV75PrEngine::new();
    let casper_diff = freebsd_capsicum.register_casper_service("system.dns", 8, 0x01);
    assert!(casper_diff.contains("cap_service_open"));

    let mut openbsd_softraid = OpenBsdSoftraidBioctlV75PrEngine::new();
    let softraid_diff = openbsd_softraid.create_crypto_volume("sd0", 1);
    assert!(softraid_diff.contains("bioctl"));

    let mut dragonfly_h2 = DragonFlyHammer2PfsV75PrEngine::new();
    let h2_diff = dragonfly_h2.create_pfs_snapshot("BACKUP_V75", 50001);
    assert!(h2_diff.contains("pfs-create"));

    let mut gentoo_portage = GentooPortageEapi10V75PrEngine::new();
    let gentoo_diff = gentoo_portage.add_ebuild("sys-apps/sigmaos", "0", "1.0.0", vec!["pqc", "hardened"]);
    assert!(gentoo_diff.contains("EAPI=10"));

    let mut alpine_apk = AlpineApkV4ApkovlV75PrEngine::new();
    let alpine_diff = alpine_apk.generate_overlay("sigmaos.apkovl.tar.gz", 12);
    assert!(alpine_diff.contains("lbu commit"));

    let mut nix_guix = NixGuixFlakeStoreV75PrEngine::new();
    let nix_diff = nix_guix.add_closure("/nix/store/sigmaos-v75", "sha256-abcdef", vec!["/nix/store/dep1"]);
    assert!(nix_diff.contains("nix store add-path"));

    let mut netbsd_npf = NetBsdNpfJitFirewallV75PrEngine::new();
    let npf_diff = netbsd_npf.add_rule(10, "pass", "eth0");
    assert!(npf_diff.contains("npfctl"));

    let mut clear_linux = ClearLinuxStatelessPstateV75PrEngine::new();
    let clear_diff = clear_linux.set_pstate("performance", 0);
    assert!(clear_diff.contains("performance"));

    let mut illumos_crossbow = IllumosDtraceCrossbowV75PrEngine::new();
    let illumos_diff = illumos_crossbow.create_vnic("vnic_v75", "e1000g0", 1000);
    assert!(illumos_diff.contains("create-vnic"));
}
