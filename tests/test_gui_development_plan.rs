// Integration test for SigmaOS GUI Development Plan
// (`tests/test_gui_development_plan.rs`)

#[path = "../src/desktop/sovereign_gui_development_and_pr_suite.rs"]
mod sovereign_gui_development_and_pr_suite;

use sovereign_gui_development_and_pr_suite::*;

#[test]
fn test_gui_development_plan_19_prs() {
    let tracker = GuiDevelopmentPlanTracker::new();
    assert_eq!(tracker.prs.len(), 19);

    assert_eq!(tracker.prs.get(&1).unwrap().stage, GuiPrStage::AuditBaseline);
    assert_eq!(tracker.prs.get(&3).unwrap().stage, GuiPrStage::BootableDesktopSlice);
    assert_eq!(tracker.prs.get(&9).unwrap().stage, GuiPrStage::UsableCoreUx);
    assert_eq!(tracker.prs.get(&13).unwrap().stage, GuiPrStage::SecurityAndAccessibility);
    assert_eq!(tracker.prs.get(&17).unwrap().stage, GuiPrStage::HardwareAndReleaseReadiness);
}

#[test]
fn test_gui_master_suite_integration() {
    let master = SovereignGuiDevelopmentAndPrSuiteMaster::new();
    assert!(master.health_check());
    let summary = master.summary_report();
    assert!(summary.contains("Total Defined PRs: 19"));
}
