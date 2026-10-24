// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS Missing Components V73 PR Suite

#[path = "../src/distro/open_source_os_missing_components_v73_pr.rs"]
mod open_source_os_missing_components_v73_pr;

use open_source_os_missing_components_v73_pr::*;

#[test]
fn test_open_source_os_missing_components_v73_pr_suite_full_cycle() {
    let suite = SovereignOpenSourceOsMissingComponentsV73PrSuite::new();
    let proposals = suite.execute_full_v73_pr_pipeline();

    assert_eq!(
        proposals.len(),
        11,
        "Expected 11 V73 open source OS PR proposals"
    );
    assert_eq!(
        suite.calculate_subsystem_interop_score(),
        100,
        "Subsystem interop score must be 100"
    );

    for proposal in &proposals {
        assert_eq!(proposal.status, OsPrStatusV73::Validated);
        assert!(!proposal.title.is_empty());
        assert!(!proposal.component_name.is_empty());
        assert!(!proposal.os_origin.is_empty());
        assert!(!proposal.impacted_subsystems.is_empty());
        assert!(!proposal.patch_diff.is_empty());
    }

    // Verify individual engine functionality
    let pr_001 = &proposals[0];
    assert_eq!(pr_001.pr_id, "PR-V73-001");
    assert!(pr_001.title.contains("Linux 6.29+"));
    assert!(pr_001
        .impacted_subsystems
        .contains(&"KernelScheduling".to_string()));

    let pr_002 = &proposals[1];
    assert_eq!(pr_002.pr_id, "PR-V73-002");
    assert!(pr_002.title.contains("FreeBSD 16.2+"));
    assert!(pr_002
        .impacted_subsystems
        .contains(&"SecuritySandboxing".to_string()));

    let pr_003 = &proposals[2];
    assert_eq!(pr_003.pr_id, "PR-V73-003");
    assert!(pr_003.title.contains("OpenBSD 8.8+"));

    let pr_010 = &proposals[9];
    assert_eq!(pr_010.pr_id, "PR-V73-010");
    assert!(pr_010
        .title
        .contains("Omarchy Linux Dynamic Wallust Theme Reloading"));

    let wiki_doc = suite
        .tech_media
        .generate_wiki_documentation_page("Sovereign SigmaOS V73 Open Source OS Architecture");
    assert!(wiki_doc.contains("Sovereign SigmaOS V73 Open Source OS Architecture"));
    assert!(wiki_doc.contains("Phoronix"));
    assert!(wiki_doc.contains("HackerNews"));
}
