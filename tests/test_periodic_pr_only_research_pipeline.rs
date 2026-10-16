// Integration test suite for SigmaOS Periodic PR-Only Research-and-Implementation Pipeline Engine

#[path = "../src/governance/periodic_pr_only_research_pipeline_engine.rs"]
mod periodic_pr_only_research_pipeline_engine;

use periodic_pr_only_research_pipeline_engine::*;
use std::fs;

#[test]
fn test_pipeline_engine_full_verification() {
    let engine = PeriodicPrOnlyResearchPipelineEngine::new();

    // Verify 5 cadence rules
    assert_eq!(engine.cadence_rules.len(), 5);
    let cadences: Vec<&str> = engine
        .cadence_rules
        .iter()
        .map(|c| c.cadence.as_str())
        .collect();
    assert!(cadences.contains(&"Daily"));
    assert!(cadences.contains(&"Weekly"));
    assert!(cadences.contains(&"Biweekly"));
    assert!(cadences.contains(&"Monthly"));
    assert!(cadences.contains(&"Quarterly"));

    // Verify missing component roadmap items (22 total)
    assert_eq!(engine.roadmap_items.len(), 22);

    let p0_items = engine.get_roadmap_by_priority(ComponentPriority::P0);
    assert_eq!(p0_items.len(), 4);
    assert!(p0_items
        .iter()
        .any(|i| i.component_name == "Reproducible CI/build gate"));
    assert!(p0_items
        .iter()
        .any(|i| i.component_name == "QEMU boot validator"));
    assert!(p0_items
        .iter()
        .any(|i| i.component_name == "Bootloader integration"));
    assert!(p0_items
        .iter()
        .any(|i| i.component_name == "Kernel entry and serial logging"));

    let p1_items = engine.get_roadmap_by_priority(ComponentPriority::P1);
    assert_eq!(p1_items.len(), 6);

    let p2_items = engine.get_roadmap_by_priority(ComponentPriority::P2);
    assert_eq!(p2_items.len(), 7);

    let p3_items = engine.get_roadmap_by_priority(ComponentPriority::P3);
    assert_eq!(p3_items.len(), 5);

    // Verify 12 mandatory review rules
    assert_eq!(engine.review_rules.len(), 12);
    for rule in &engine.review_rules {
        assert!(rule.enforced);
    }
    assert_eq!(engine.get_rule(1).unwrap().name, "Every change is a PR");
    assert_eq!(
        engine.get_rule(12).unwrap().name,
        "Human approval for high-risk merges"
    );

    // Verify 10 definition of done criteria
    assert_eq!(engine.definition_of_done.len(), 10);
    for criterion in &engine.definition_of_done {
        assert!(criterion.verified);
    }

    // Verify first 6 PR deliverables
    assert_eq!(engine.first_six_prs.len(), 6);
    assert_eq!(engine.first_six_prs[0].pr_number, 1);
    assert!(engine.first_six_prs[0].title.contains("chore(ci)"));
    assert!(engine.first_six_prs[3]
        .title
        .contains("feat(kernel): implement minimal boot-to-shell path"));

    // Verify 30-day schedule
    assert_eq!(engine.thirty_day_schedule.len(), 4);

    // Verify overall engine pipeline health
    assert!(engine.validate_pipeline_rules());
}

#[test]
fn test_gateway_report_content() {
    let engine = PeriodicPrOnlyResearchPipelineEngine::new();
    let report = engine.generate_pr_gateway_report();

    assert!(report.contains("Periodic PR-Only Research-and-Implementation Pipeline Report"));
    assert!(report.contains("Cadence Levels Configured: 5"));
    assert!(report.contains("Total Missing Component Roadmap Items: 22"));
    assert!(report.contains("Mandatory PR Review Rules Enforced: 12/12"));
    assert!(report.contains("Definition of Done Criteria: 10/10"));
    assert!(report.contains("VERIFIED PASS"));
}

#[test]
fn test_github_templates_exist() {
    // Check PR Template
    let pr_template = fs::read_to_string(".github/PULL_REQUEST_TEMPLATE.md")
        .expect("PULL_REQUEST_TEMPLATE.md should exist");
    assert!(pr_template.contains("Reference study") || pr_template.contains("Upstream study"));
    assert!(
        pr_template.contains("Kernel safety review") || pr_template.contains("What SigmaOS adopts")
    );
    assert!(
        pr_template.contains("Risks and rollback") || pr_template.contains("Safety and rollback")
    );
    assert!(pr_template.contains("Evidence") || pr_template.contains("Reviewer checklist"));

    // Check Study Issue Template
    let study_issue = fs::read_to_string(".github/ISSUE_TEMPLATE/study_issue.yml")
        .expect("study_issue.yml should exist");
    assert!(study_issue.contains("Study issue"));
    assert!(study_issue.contains("upstream_project"));

    // Check Missing Component Issue Template
    let missing_comp_issue =
        fs::read_to_string(".github/ISSUE_TEMPLATE/missing_component_issue.yml")
            .expect("missing_component_issue.yml should exist");
    assert!(missing_comp_issue.contains("Missing component issue"));
    assert!(missing_comp_issue.contains("minimum_viable_slice"));
}
