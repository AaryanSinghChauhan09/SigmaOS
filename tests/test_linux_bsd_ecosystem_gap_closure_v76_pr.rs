// SPDX-License-Identifier: MIT
// Integration Test for Sovereign Linux & BSD Ecosystem Gap Closure V76 PR Suite
// (`tests/test_linux_bsd_ecosystem_gap_closure_v76_pr.rs`)

#[path = "../src/distro/sovereign_linux_bsd_ecosystem_gap_closure_v76_pr.rs"]
mod sovereign_linux_bsd_ecosystem_gap_closure_v76_pr;

use sovereign_linux_bsd_ecosystem_gap_closure_v76_pr::*;

#[test]
fn test_v76_linux_bsd_gap_closure_suite_integration() {
    let mut suite = SovereignLinuxBsdEcosystemGapClosureV76PrSuite::new();

    // Verify initial state
    assert_eq!(suite.proposals.len(), 0);

    // Run full gap closure evaluation
    let parity_ok = suite.evaluate_v76_gap_closure_parity();
    assert!(parity_ok);
    assert_eq!(suite.proposals.len(), 3);

    // Check individual proposals status
    for prop in suite.proposals.values() {
        assert_eq!(prop.status, LinuxBsdPrStatusV76::Merged);
        assert!(!prop.unified_diff.is_empty());
    }

    // Test individual engines
    let mut linux = LinuxIoUringFutex2V76PrEngine::new();
    let fdiff = linux.register_futex_waiter(0x2000, 10, 0x1, 0xff);
    assert!(fdiff.contains("futex_waitv"));

    let mut freebsd = FreeBsdCapsicumCasperV76PrEngine::new();
    let cdiff = freebsd.register_casper_service("system.storage", 6, 0x08);
    assert!(cdiff.contains("cap_service_open"));

    let mut openbsd = OpenBsdSoftraidBioctlV76PrEngine::new();
    let sdiff = openbsd.create_crypto_volume("sd3", 4);
    assert!(sdiff.contains("bioctl"));

    let mut dragonfly = DragonFlyHammer2PfsV76PrEngine::new();
    let hdiff = dragonfly.create_pfs_snapshot("BACKUP", 76003);
    assert!(hdiff.contains("pfs-create"));

    let mut gentoo = GentooPortageEapi10V76PrEngine::new();
    let gdiff = gentoo.add_ebuild("sys-kernel/sigmaos-kernel-6.30", "0", "6.30", vec!["ebpf"]);
    assert!(gdiff.contains("EAPI=10"));

    let mut alpine = AlpineApkV4ApkovlV76PrEngine::new();
    let adiff = alpine.generate_overlay("custom.apkovl.tar.gz", 12);
    assert!(adiff.contains("lbu commit"));

    let mut nix = NixGuixFlakeStoreV76PrEngine::new();
    let ndiff = nix.add_closure("/nix/store/xyz", "hash789", vec![]);
    assert!(ndiff.contains("nix store add-path"));

    let mut netbsd = NetBsdNpfJitFirewallV76PrEngine::new();
    let npfdiff = netbsd.add_rule(10, "pass", "vioif1");
    assert!(npfdiff.contains("npfctl"));

    let mut clear = ClearLinuxStatelessPstateV76PrEngine::new();
    let cldiff = clear.set_pstate("performance", 0);
    assert!(cldiff.contains("performance"));

    let mut illumos = IllumosDtraceCrossbowV76PrEngine::new();
    let ildiff = illumos.create_vnic("vnic2", "e1000g1", 10000);
    assert!(ildiff.contains("create-vnic"));
}
