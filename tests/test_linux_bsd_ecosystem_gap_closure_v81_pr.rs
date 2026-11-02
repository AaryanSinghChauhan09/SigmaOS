// SPDX-License-Identifier: MIT
// Integration test for Sovereign Linux & BSD Ecosystem Gap Closure V81 PR Suite

#[path = "../src/distro/sovereign_linux_bsd_ecosystem_gap_closure_v81_pr.rs"]
mod sovereign_linux_bsd_ecosystem_gap_closure_v81_pr;

use sovereign_linux_bsd_ecosystem_gap_closure_v81_pr::*;

#[test]
fn test_linux_io_uring_futex2_v81_engine() {
    let mut engine = LinuxIoUringFutex2V81PrEngine::new();
    assert_eq!(engine.sq_ring_entries, 32768);
    assert_eq!(engine.sqpoll_cpu, 32);

    let diff = engine.register_futex_waiter(0x1000, 42, 0, 1);
    assert!(diff.contains("futex_waitv"));
    assert_eq!(engine.futex_waiters.len(), 1);
}

#[test]
fn test_freebsd_capsicum_casper_v81_engine() {
    let mut engine = FreeBsdCapsicumCasperV81PrEngine::new();
    assert_eq!(engine.services.len(), 1);

    let diff = engine.register_casper_service("system.pwd", 12, 0x00FF);
    assert!(diff.contains("cap_service_open"));
    assert_eq!(engine.services.len(), 2);
}

#[test]
fn test_openbsd_softraid_bioctl_v81_engine() {
    let mut engine = OpenBsdSoftraidBioctlV81PrEngine::new();
    assert_eq!(engine.volumes.len(), 1);

    let diff = engine.create_crypto_volume("sr1", 2);
    assert!(diff.contains("bioctl"));
    assert_eq!(engine.volumes.len(), 2);
}

#[test]
fn test_dragonfly_hammer3_pfs_v81_engine() {
    let mut engine = DragonFlyHammer3PfsV81PrEngine::new();
    assert_eq!(engine.pfs_list.len(), 1);

    let diff = engine.create_pfs_snapshot("BACKUP_SNAP", 90021);
    assert!(diff.contains("hammer3 pfs-create"));
    assert_eq!(engine.pfs_list.len(), 2);
}

#[test]
fn test_gentoo_portage_eapi10_v81_engine() {
    let mut engine = GentooPortageEapi10V81PrEngine::new();
    let diff = engine.add_ebuild("sys-devel/gcc-15", "15", "15.0", vec!["lto", "pgo"]);
    assert!(diff.contains("EAPI=10"));
    assert_eq!(engine.ebuilds.len(), 1);
}

#[test]
fn test_alpine_apk_v4_apkovl_v81_engine() {
    let mut engine = AlpineApkV4ApkovlV81PrEngine::new();
    let diff = engine.generate_overlay("sigmaos.apkovl.tar.gz", 42);
    assert!(diff.contains("lbu commit"));
    assert_eq!(engine.overlays.len(), 1);
}

#[test]
fn test_nix_guix_flake_store_v81_engine() {
    let mut engine = NixGuixFlakeStoreV81PrEngine::new();
    let diff = engine.add_closure("/nix/store/123", "blake3_hash", vec![]);
    assert!(diff.contains("nix store add-path"));
    assert_eq!(engine.store_closures.len(), 1);
}

#[test]
fn test_netbsd_npf_jit_firewall_v81_engine() {
    let mut engine = NetBsdNpfJitFirewallV81PrEngine::new();
    let diff = engine.add_rule(10, "pass stateful", "vioif1");
    assert!(diff.contains("npfctl"));
    assert_eq!(engine.rules.len(), 1);
}

#[test]
fn test_clear_linux_stateless_pstate_v81_engine() {
    let mut engine = ClearLinuxStatelessPstateV81PrEngine::new();
    let diff = engine.set_pstate("performance", 0);
    assert!(diff.contains("performance"));
    assert_eq!(engine.pstate_config.governor, "performance");
}

#[test]
fn test_illumos_dtrace_crossbow_v81_engine() {
    let mut engine = IllumosDtraceCrossbowV81PrEngine::new();
    let diff = engine.create_vnic("vnic2", "e1000g0", 10000);
    assert!(diff.contains("create-vnic"));
    assert!(engine.vnics.contains_key("vnic2"));
}

#[test]
fn test_master_suite_v81_gap_closure_evaluation() {
    let mut suite = SovereignLinuxBsdEcosystemGapClosureV81PrSuite::new();
    let res = suite.evaluate_v81_gap_closure_parity();
    assert!(res);
    assert_eq!(suite.proposals.len(), 3);
}
