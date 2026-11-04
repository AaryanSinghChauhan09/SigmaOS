// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Arch Linux Missing Components V84 PR Engine
// (`tests/test_arch_linux_missing_components_v84_pr.rs`)

#[path = "../src/distro/arch_linux_missing_components_v84_pr.rs"]
mod arch_linux_missing_components_v84_pr;

use arch_linux_missing_components_v84_pr::*;

#[test]
fn test_arch_linux_v84_pr_engines_and_suite() {
    let suite = SovereignArchLinuxMissingComponentsV84PrSuite::new();
    let prs = suite.execute_full_v84_arch_pipeline();
    assert_eq!(prs.len(), 12);

    for pr in &prs {
        assert_eq!(pr.status, ArchV84PrStatus::Validated);
        assert!(pr.pr_id >= 8401 && pr.pr_id <= 8412);
        assert!(!pr.title.is_empty());
        assert!(!pr.subsystem.is_empty());
        assert!(!pr.target_repo.is_empty());
        assert!(!pr.summary.is_empty());
        assert!(!pr.patch_diff.is_empty());
    }

    // Test individual component methods
    let keyring = ArchPacmanKeyringPqcRotationEngineV84::new();
    assert!(keyring.verify_trust());

    let aurweb = ArchAurWebRpcClientAuditorEngineV84::new();
    assert!(aurweb.audit_pkgbuild_source("pkgname=test\npkgver=1.0"));
    assert!(!aurweb.audit_pkgbuild_source("rm -rf /"));

    let devtools = ArchDevtoolsCleanChrootNspawnEngineV84::new();
    assert!(devtools.verify_cleanroom());

    let reflector = ArchReflectorSpeedRankerEngineV84::new();
    assert_eq!(reflector.rank_mirrors(&[50, 10, 30]), vec![10, 30, 50]);

    let archiso = ArchisoCustomLiveIsoEngineV84::new();
    assert!(archiso.verify_iso_config());

    let snapper = ArchSnapperAutoBtrfsSnapshotEngineV84::new();
    assert!(snapper.should_snapshot());

    let sec_tracker = ArchSecurityTrackerCveScannerEngineV84::new();
    assert!(sec_tracker.scan_package("glibc"));

    let yay = ArchYayAstDependencySolverEngineV84::new();
    assert_eq!(yay.parallel_download_threads, 16);
}
