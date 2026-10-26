// SPDX-License-Identifier: MIT
// Integration test for Sovereign Linux & BSD Ecosystem Gap Closure V77 PR Suite

use sigmaos::distro::sovereign_linux_bsd_ecosystem_gap_closure_v77_pr::*;

#[test]
fn test_linux_io_uring_futex2_v77_engine() {
    let mut engine = LinuxIoUringFutex2V77PrEngine::new();
    assert_eq!(engine.sq_ring_entries, 16384);
    assert_eq!(engine.sqpoll_cpu, 16);

    let diff = engine.register_futex_waiter(0x1000, 42, 0, 1);
    assert!(diff.contains("sys_futex_waitv_v77"));
    assert_eq!(engine.futex_waiters.len(), 1);
}

#[test]
fn test_freebsd_capsicum_casper_v77_engine() {
    let mut engine = FreeBsdCapsicumCasperV77PrEngine::new();
    assert!(engine.services.contains_key("system.dns"));

    let diff = engine.register_casper_service("system.sysctl", 4, 0x02);
    assert!(diff.contains("cap_service_open"));
    assert!(engine.services.contains_key("system.sysctl"));
}

#[test]
fn test_openbsd_softraid_bioctl_v77_engine() {
    let mut engine = OpenBsdSoftraidBioctlV77PrEngine::new();
    assert!(engine.volumes.contains_key("sd0"));

    let diff = engine.create_crypto_volume("sd1", 3);
    assert!(diff.contains("bioctl"));
    assert!(engine.volumes.contains_key("sd1"));
}

#[test]
fn test_dragonfly_hammer2_pfs_v77_engine() {
    let mut engine = DragonFlyHammer2PfsV77PrEngine::new();
    assert!(engine.pfs_list.contains_key("ROOT"));

    let diff = engine.create_pfs_snapshot("BACKUP", 77005);
    assert!(diff.contains("pfs-create"));
    assert!(engine.pfs_list.contains_key("BACKUP"));
}

#[test]
fn test_gentoo_portage_eapi10_v77_engine() {
    let mut engine = GentooPortageEapi10V77PrEngine::new();
    let diff = engine.add_ebuild("sys-devel/gcc-15.0", "0", "15", vec!["cxx", "fortran"]);
    assert!(diff.contains("EAPI=10"));
    assert!(engine.ebuilds.contains_key("sys-devel/gcc-15.0"));
}

#[test]
fn test_alpine_apk_v4_apkovl_v77_engine() {
    let mut engine = AlpineApkV4ApkovlV77PrEngine::new();
    let diff = engine.generate_overlay("server.apkovl.tar.gz", 30);
    assert!(diff.contains("lbu commit"));
    assert!(engine.overlays.contains_key("server.apkovl.tar.gz"));
}

#[test]
fn test_nix_guix_flake_store_v77_engine() {
    let mut engine = NixGuixFlakeStoreV77PrEngine::new();
    let diff = engine.add_closure("/nix/store/test-path", "sha256-digest", vec![]);
    assert!(diff.contains("nix store add-path"));
    assert!(engine.store_closures.contains_key("/nix/store/test-path"));
}

#[test]
fn test_netbsd_npf_jit_firewall_v77_engine() {
    let mut engine = NetBsdNpfJitFirewallV77PrEngine::new();
    let diff = engine.add_rule(10, "pass", "vioif1");
    assert!(diff.contains("npfctl"));
    assert!(engine.rules.contains_key(&10));
}

#[test]
fn test_clear_linux_stateless_pstate_v77_engine() {
    let mut engine = ClearLinuxStatelessPstateV77PrEngine::new();
    let diff = engine.set_pstate("performance", 0);
    assert!(diff.contains("performance"));
    assert_eq!(engine.pstate_config.governor, "performance");
}

#[test]
fn test_illumos_dtrace_crossbow_v77_engine() {
    let mut engine = IllumosDtraceCrossbowV77PrEngine::new();
    let diff = engine.create_vnic("vnic2", "e1000g0", 10000);
    assert!(diff.contains("create-vnic"));
    assert!(engine.vnics.contains_key("vnic2"));
}

#[test]
fn test_master_suite_v77_gap_closure_evaluation() {
    let mut suite = SovereignLinuxBsdEcosystemGapClosureV77PrSuite::new();
    let res = suite.evaluate_v77_gap_closure_parity();
    assert!(res);
    assert_eq!(suite.proposals.len(), 3);
}
