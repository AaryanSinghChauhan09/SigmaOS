// Integration test for Sovereign Open Source OS Missing Components V69 PR Suite

#[path = "../src/distro/open_source_os_missing_components_v69_pr.rs"]
mod open_source_os_missing_components_v69_pr;

use open_source_os_missing_components_v69_pr::*;

#[test]
fn test_open_source_os_missing_components_v69_pr_integration() {
    let suite = SovereignOpenSourceOsMissingComponentsV69PrSuite::new();

    // Verify all 10 engine components execute and pass verification
    assert!(suite.execute_all_v69_engines_and_verify());

    // Verify 100% score for all 44 subsystem interop harmony
    assert_eq!(suite.compute_all_44_subsystem_harmony_score(), 100);

    // Verify PR proposals metadata
    assert!(suite.pr_specs.len() >= 10);
    let sched_pr = suite
        .pr_specs
        .iter()
        .find(|spec| spec.pr_id == "PR-V69-001")
        .expect("PR-V69-001 should exist");
    assert_eq!(sched_pr.inspired_distro_or_os, "Linux 6.25+ / CachyOS");

    let portal_pr = suite
        .pr_specs
        .iter()
        .find(|spec| spec.pr_id == "PR-V69-010")
        .expect("PR-V69-010 should exist");
    assert_eq!(portal_pr.target_sigma_subsystems.len(), 44);
}
