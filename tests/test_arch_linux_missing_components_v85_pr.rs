// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Arch Linux Missing Components V85 PR Engine
// (`tests/test_arch_linux_missing_components_v85_pr.rs`)

#[path = "../src/distro/arch_linux_missing_components_v85_pr.rs"]
mod arch_linux_missing_components_v85_pr;

use arch_linux_missing_components_v85_pr::*;

#[test]
fn test_arch_linux_v85_pr_engines_and_suite() {
    let suite = SovereignArchLinuxMissingComponentsV85PrSuite::new();
    let prs = suite.execute_full_v85_arch_pipeline();
    assert_eq!(prs.len(), 12);

    for pr in &prs {
        assert_eq!(pr.status, ArchV85PrStatus::Validated);
        assert!(pr.pr_id >= 8501 && pr.pr_id <= 8512);
        assert!(!pr.title.is_empty());
        assert!(!pr.subsystem.is_empty());
        assert!(!pr.target_repo.is_empty());
        assert!(!pr.summary.is_empty());
        assert!(!pr.patch_diff.is_empty());
    }

    // Test individual component methods
    let keyring = ArchPacmanKeyringPqcRotationEngineV85::new();
    assert!(keyring.verify_trust());

    let aurweb = ArchAurWebRpcClientAuditorEngineV85::new();
    assert!(aurweb.audit_pkgbuild_source("pkgname=test\npkgver=1.0"));
    assert!(!aurweb.audit_pkgbuild_source("rm -rf /"));

    let devtools = ArchDevtoolsCleanChrootNspawnEngineV85::new();
    assert!(devtools.verify_cleanroom());

    let reflector = ArchReflectorSpeedRankerEngineV85::new();
    assert_eq!(reflector.rank_mirrors(&[50, 10, 30]), vec![10, 30, 50]);

    let archiso = ArchisoCustomLiveIsoEngineV85::new();
    assert!(archiso.verify_iso_config());

    let snapper = ArchSnapperAutoBtrfsSnapshotEngineV85::new();
    assert!(snapper.should_snapshot());

    let sec_tracker = ArchSecurityTrackerCveScannerEngineV85::new();
    assert!(sec_tracker.scan_package("glibc"));

    let modprobed = ArchModprobedDbTrimmerEngineV85::new();
    assert_eq!(modprobed.kernel_modules_count, 1024);
    assert!(modprobed.trim_unused);

    let conflict = ArchPacmanFileConflictResolverEngineV85::new();
    assert!(conflict.resolve_orphans);
    assert!(!conflict.overwrite_allowed);

    let supervisor = ArchSystemdServiceSupervisorEngineV85::new();
    assert!(supervisor.auto_restart);
    assert_eq!(supervisor.tracked_services, 128);

    let yay = ArchYayAstDependencySolverEngineV85::new();
    assert_eq!(yay.parallel_download_threads, 16);
    assert!(yay.aur_comments_filter);

    let uki = ArchMkinitcpioSystemdUkiEngineV85::new();
    assert!(uki.microcode_stubs);
    assert!(uki.unified_kernel_image);
}
