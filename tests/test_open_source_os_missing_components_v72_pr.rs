// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS Missing Components V72 PR Suite

#[path = "../src/distro/open_source_os_missing_components_v72_pr.rs"]
mod open_source_os_missing_components_v72_pr;

use open_source_os_missing_components_v72_pr::*;

#[test]
fn test_open_source_os_missing_components_v72_pr_suite_full_cycle() {
    let suite = SovereignOpenSourceOsMissingComponentsV72PrSuite::new();
    let proposals = suite.execute_full_v72_pr_pipeline();

    assert_eq!(
        proposals.len(),
        11,
        "Expected 11 V72 open source OS PR proposals"
    );
    assert_eq!(
        suite.calculate_subsystem_interop_score(),
        100,
        "Subsystem interop score must be 100"
    );

    for proposal in &proposals {
        assert_eq!(proposal.status, OsPrStatusV72::Validated);
        assert!(!proposal.title.is_empty());
        assert!(!proposal.component_name.is_empty());
        assert!(!proposal.os_origin.is_empty());
        assert!(!proposal.impacted_subsystems.is_empty());
        assert!(!proposal.patch_diff.is_empty());
    }

    // Verify individual engine functionality
    let pr_001 = &proposals[0];
    assert_eq!(pr_001.pr_id, "PR-V72-001");
    assert!(pr_001.title.contains("Linux 6.27+"));
    assert!(pr_001
        .impacted_subsystems
        .contains(&"KernelScheduling".to_string()));

    let pr_002 = &proposals[1];
    assert_eq!(pr_002.pr_id, "PR-V72-002");
    assert!(pr_002.title.contains("FreeBSD 16.1+"));
    assert!(pr_002
        .impacted_subsystems
        .contains(&"SecuritySandboxing".to_string()));

    let pr_003 = &proposals[2];
    assert_eq!(pr_003.pr_id, "PR-V72-003");
    assert!(pr_003.title.contains("OpenBSD 8.7+"));

    let pr_010 = &proposals[9];
    assert_eq!(pr_010.pr_id, "PR-V72-010");
    assert!(pr_010
        .title
        .contains("Omarchy Linux Dynamic Wallust Theme Reloading"));

    let wiki_doc = suite
        .tech_media
        .generate_wiki_documentation_page("Sovereign SigmaOS V72 Open Source OS Architecture");
    assert!(wiki_doc.contains("Sovereign SigmaOS V72 Open Source OS Architecture"));
    assert!(wiki_doc.contains("Phoronix"));
    assert!(wiki_doc.contains("HackerNews"));

    let report = suite.comparison_matrix.generate_matrix_report();
    assert!(report.contains("SigmaOS vs Open Source Operating Systems Feature Parity Matrix V72"));
    assert!(report.contains("Linux 6.27+"));
    assert!(report.contains("FreeBSD 16.1+"));
}
