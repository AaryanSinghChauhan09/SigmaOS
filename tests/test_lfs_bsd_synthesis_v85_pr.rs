//! Integration test suite for Sovereign LFS & BSD Synthesis V85 PR Gateway.

#[path = "../src/distro/sovereign_lfs_bsd_synthesis_v85_pr.rs"]
mod sovereign_lfs_bsd_synthesis_v85_pr;

use sovereign_lfs_bsd_synthesis_v85_pr::*;

#[test]
fn test_lfs_bsd_synthesis_v85_pr_gateway_integration() {
    let gateway = SovereignLfsBsdV85PrGateway::new();
    let manifests = gateway.execute_full_v85_pr_pipeline();

    assert_eq!(manifests.len(), 8);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 4111 && manifest.pr_id <= 4118);
        assert!(!manifest.title.is_empty());
        assert_eq!(manifest.status, LfsBsdPrStatusV85::Validated);
    }

    assert_eq!(gateway.calculate_subsystem_interop_score(), 100);

    // Verify LFS Toolchain Bootstrap Engine V85
    let mut lfs = LfsToolchainSysrootBootstrapEngineV85::new();
    assert!(lfs.execute_bootstrap_step("binutils"));
    assert!(lfs.execute_bootstrap_step("gcc"));
    assert!(lfs.execute_bootstrap_step("glibc"));
    assert_eq!(lfs.stage, LfsBootstrapStageV85::Pass2TempChroot);
    assert!(lfs.verify_dynamic_linker("/bin/sh"));

    // Verify BLFS DAG Scheduler V85
    let mut blfs = BlfsDependencyDagStagingEngineV85::new();
    let order = blfs.compute_topological_build_order("git").unwrap();
    assert_eq!(order, vec!["openssl", "nghttp2", "curl", "git"]);
    assert!(blfs.stage_file_check_collision("/usr/bin/git"));
    assert!(!blfs.stage_file_check_collision("/usr/bin/git"));

    // Verify FreeBSD ZFS Boot Environments & Bhyve Engine V85
    let mut freebsd = FreeBsdZfsBootEnvBhyveEngineV85::new();
    assert!(freebsd.activate_boot_environment("fallback"));
    assert_eq!(freebsd.active_be, "fallback");
    assert!(freebsd.verify_bhyve_hypercall(16));

    // Verify OpenBSD Pledge & Unveil Sentinel V85
    let openbsd = OpenBsdPledgeUnveilSyscallGuardEngineV85::new();
    assert!(openbsd.check_pledge("rpath"));
    assert!(openbsd.check_unveil_access("/etc/hosts", 'r'));
    assert!(!openbsd.check_unveil_access("/etc/hosts", 'w'));
    assert!(openbsd.validate_pinsyscall(0x0040_2000));

    // Verify DragonFly BSD HAMMER2 Cluster Replication V85
    let mut dfbsd = DragonFlyHammer2PfsClusterEngineV85::new();
    assert!(dfbsd.has_quorum());
    assert_eq!(dfbsd.commit_replicated_txg(), 4097);

    // Verify NetBSD Rump VFS & Veriexec V85
    let mut netbsd = NetBsdRumpFsVeriexecEngineV85::new();
    assert!(netbsd.audit_binary_execution(
        "/bin/init",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ));
    assert!(netbsd.mount_rump_vfs("nfs"));

    // Verify Universal Cross-ABI Syscall Translator V85
    let cross_abi = CrossAbiLinuxBsdSyscallTranslatorEngineV85::new();
    assert_eq!(
        cross_abi.translate_vfs_path("/usr/local/etc/httpd.conf"),
        "/etc/httpd.conf"
    );
    assert_eq!(
        cross_abi.translate_vfs_path("/usr/compat/linux/bin/cat"),
        "/bin/cat"
    );
    assert_eq!(cross_abi.translate_bsd_to_linux_syscall(3), 0);
    assert_eq!(cross_abi.supported_categories.len(), 44);
}
