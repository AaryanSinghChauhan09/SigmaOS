// Integration test for SigmaOS GUI Development Plan
// (`tests/test_gui_development_plan.rs`)

#![allow(dead_code)]

use std::fs;
use std::path::Path;

#[path = "../src/desktop/sovereign_gui_development_and_pr_suite.rs"]
mod sovereign_gui_development_and_pr_suite;

use sovereign_gui_development_and_pr_suite::*;

#[test]
fn test_gui_development_plan_documentation_verification() {
    let docs_path = "/home/aaryansinghchauhan/SigmaOS.wiki/30-SigmaOS-GUI-Development-Plan.md";
    let wiki_path = "/home/aaryansinghchauhan/SigmaOS.wiki/30-SigmaOS-GUI-Development-Plan.md";

    assert!(
        Path::new(docs_path).exists(),
        "docs/SIGMAOS_GUI_DEVELOPMENT_PLAN.md must exist"
    );
    assert!(
        Path::new(wiki_path).exists(),
        "wiki/30-SigmaOS-GUI-Development-Plan.md must exist"
    );

    let docs_content = fs::read_to_string(docs_path).expect("Read docs plan");
    let wiki_content = fs::read_to_string(wiki_path).expect("Read wiki plan");

    assert_eq!(
        docs_content, wiki_content,
        "Wiki plan must match docs plan byte-for-byte"
    );

    // Verify key sections and contracts exist in documentation
    assert!(docs_content.contains("SigmaOS GUI Development Plan"));
    assert!(docs_content.contains("Design and Acceptance Rules"));
    assert!(docs_content.contains("Proposed 19-PR Sequence Roadmap"));
    assert!(docs_content.contains("Periodic Maintenance PR Cadences"));
    assert!(docs_content.contains("Component Rules"));
    assert!(docs_content.contains("PR Review and Merge Gates"));
    assert!(docs_content.contains("Rollout and Exit Criteria"));
}

#[test]
fn test_gui_development_plan_19_prs() {
    let tracker = GuiDevelopmentPlanTracker::new();
    assert_eq!(tracker.prs.len(), 19);

    for pr_id in 1..=19 {
        let pr = tracker.prs.get(&pr_id).unwrap_or_else(|| panic!("PR {} missing", pr_id));
        assert_eq!(pr.pr_number, pr_id);
        assert!(!pr.title.is_empty());
        assert!(!pr.scope.is_empty());
        assert!(!pr.acceptance_criteria.is_empty());
    }

    assert_eq!(tracker.prs.get(&1).unwrap().stage, GuiPrStage::AuditBaseline);
    assert_eq!(tracker.prs.get(&2).unwrap().stage, GuiPrStage::AuditBaseline);
    assert_eq!(tracker.prs.get(&3).unwrap().stage, GuiPrStage::BootableDesktopSlice);
    assert_eq!(tracker.prs.get(&8).unwrap().stage, GuiPrStage::BootableDesktopSlice);
    assert_eq!(tracker.prs.get(&9).unwrap().stage, GuiPrStage::UsableCoreUx);
    assert_eq!(tracker.prs.get(&12).unwrap().stage, GuiPrStage::UsableCoreUx);
    assert_eq!(tracker.prs.get(&13).unwrap().stage, GuiPrStage::SecurityAndAccessibility);
    assert_eq!(tracker.prs.get(&16).unwrap().stage, GuiPrStage::SecurityAndAccessibility);
    assert_eq!(tracker.prs.get(&17).unwrap().stage, GuiPrStage::HardwareAndReleaseReadiness);
    assert_eq!(tracker.prs.get(&19).unwrap().stage, GuiPrStage::HardwareAndReleaseReadiness);
}

#[test]
fn test_gui_master_suite_integration() {
    let master = SovereignGuiDevelopmentAndPrSuiteMaster::new();
    assert!(master.health_check());
    let summary = master.summary_report();
    assert!(summary.contains("Total Defined PRs: 19"));
}
