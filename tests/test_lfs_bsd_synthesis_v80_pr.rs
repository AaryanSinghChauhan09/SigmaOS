//! Integration test suite for Sovereign LFS & BSD Synthesis V80 PR Gateway.

#[path = "../src/distro/sovereign_lfs_bsd_synthesis_v80_pr.rs"]
mod sovereign_lfs_bsd_synthesis_v80_pr;

use sovereign_lfs_bsd_synthesis_v80_pr::*;

#[test]
fn test_lfs_bsd_synthesis_v80_pr_gateway_integration() {
    let gateway = SovereignLfsBsdV80PrGateway::new();
    let manifests = gateway.execute_full_v80_pr_pipeline();

    assert_eq!(manifests.len(), 8);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 4071 && manifest.pr_id <= 4078);
        assert!(!manifest.title.is_empty());
        assert_eq!(manifest.status, LfsBsdPrStatusV80::Validated);
    }

    assert_eq!(gateway.calculate_subsystem_interop_score(), 100);

    // Verify LFS Toolchain Cross-Compiler Engine V80
    let mut lfs = Lfs122ToolchainCrossCompilerEngineV80::new();
    assert!(lfs.execute_bootstrap_step("binutils"));
    assert!(lfs.execute_bootstrap_step("gcc"));
    assert!(lfs.execute_bootstrap_step("glibc"));
    assert_eq!(lfs.stage, LfsBootstrapStageV80::Pass2TempChroot);

    // Verify BLFS DAG Scheduler V80
    let blfs = Blfs122BuildDependencySchedulerEngineV80::new();
    let order = blfs.compute_topological_build_order("git").unwrap();
    assert_eq!(order, vec!["openssl", "curl", "git"]);

    // Verify FreeBSD VNET & Casper Jail Engine V80
    let freebsd = FreeBsd142VnetCapsicumCasperJailEngineV80::new();
    assert!(freebsd.verify_jail_isolation(12));

    // Verify OpenBSD Pledge & FineIBT Sentinel V80
    let openbsd = OpenBsd77PledgeUnveilFineIbtSentinelEngineV80::new();
    assert!(openbsd.validate_syscall_entry(0x7fff_1000));

    // Verify DragonFly BSD HAMMER2 Multi-Master Replication V80
    let mut dfbsd = DragonFly70Hammer2MultiMasterReplicationEngineV80::new();
    assert_eq!(dfbsd.replicate_transaction_log(), 2049);

    // Verify NetBSD Rump Driver & Veriexec V80
    let netbsd = NetBsd100RumpVeriexecDriverEngineV80::new();
    assert!(netbsd.verify_and_dispatch_rump_driver(16));

    // Verify Universal Cross-ABI Syscall Translator V80
    let cross_abi = BsdLinuxCrossAbiSyscallTranslatorEngineV80::new();
    assert_eq!(
        cross_abi.translate_vfs_path("/usr/local/etc/httpd.conf"),
        "/etc/httpd.conf"
    );
    assert_eq!(cross_abi.supported_categories.len(), 44);
}
