// Integration tests for SigmaOS Periodic PR-Only Component-Rule System Engine

#[path = "../src/governance/periodic_pr_component_rules_engine.rs"]
mod periodic_pr_component_rules_engine;

use periodic_pr_component_rules_engine::*;

#[test]
fn test_periodic_cadence_metadata() {
    assert_eq!(PeriodicCadence::Daily.activity(), "Build, test, boot, and status health");
    assert_eq!(PeriodicCadence::Weekly.pr_types(), "docs/research, feat/<component>");
    assert_eq!(PeriodicCadence::Biweekly.required_output(), "Regression/runtime report");
    assert_eq!(PeriodicCadence::Monthly.activity(), "Complete one component milestone");
    assert_eq!(PeriodicCadence::Quarterly.pr_types(), "docs/roadmap, refactor/architecture");
}

#[test]
fn test_component_lifecycle_steps() {
    assert_eq!(ComponentLifecycleStep::Step1SelectComponent.number(), 1);
    assert_eq!(ComponentLifecycleStep::Step1SelectComponent.title(), "SELECT COMPONENT");

    assert_eq!(ComponentLifecycleStep::Step6OpenResearchAdrPr.number(), 6);
    assert_eq!(ComponentLifecycleStep::Step6OpenResearchAdrPr.title(), "OPEN RESEARCH/ADR PR");

    assert_eq!(ComponentLifecycleStep::Step12MonitorForRegressions.number(), 12);
    assert_eq!(ComponentLifecycleStep::Step12MonitorForRegressions.title(), "MONITOR FOR REGRESSIONS");
}

#[test]
fn test_component_domain_mapping_and_gates() {
    let kernel = ComponentDomain::Kernel;
    assert_eq!(kernel.name(), "Kernel");
    assert_eq!(kernel.rules_file(), "docs/component-rules/kernel.md");
    assert_eq!(kernel.additional_gate(), "QEMU boot smoke test");

    let vfs = ComponentDomain::VfsFilesystems;
    assert_eq!(vfs.name(), "VFS/filesystems");
    assert_eq!(vfs.rules_file(), "docs/component-rules/vfs.md");
    assert_eq!(vfs.additional_gate(), "Filesystem conformance and QEMU RAM-disk test");

    let sec = ComponentDomain::Security;
    assert_eq!(sec.name(), "Security");
    assert_eq!(sec.additional_gate(), "Denial tests and fuzzing");
}

#[test]
fn test_industry_standard_rules() {
    assert_eq!(IndustryStandardRule::Rule1EverythingIsPullRequest.number(), 1);
    assert_eq!(IndustryStandardRule::Rule1EverythingIsPullRequest.title(), "Everything is a pull request");

    assert_eq!(IndustryStandardRule::Rule5RuntimeEvidenceControlsStatus.number(), 5);
    assert_eq!(IndustryStandardRule::Rule5RuntimeEvidenceControlsStatus.title(), "Runtime evidence controls status");

    assert_eq!(IndustryStandardRule::Rule14ArchiveStaleResearch.number(), 14);
    assert_eq!(IndustryStandardRule::Rule14ArchiveStaleResearch.title(), "Archive stale research");
}

#[test]
fn test_study_schedule_and_milestones() {
    let schedule = PeriodicPrComponentRulesEngine::study_schedule();
    assert_eq!(schedule.len(), 12);
    assert_eq!(schedule[0].week, 1);
    assert_eq!(schedule[0].study_focus, "xv6/OSDev boot and interrupts");
    assert_eq!(schedule[11].week, 12);
    assert_eq!(schedule[11].component_improved, "Zenith desktop");

    let milestones = PeriodicPrComponentRulesEngine::monthly_milestones();
    assert_eq!(milestones.len(), 10);
    assert_eq!(milestones[0].month, 1);
    assert_eq!(milestones[0].milestone_description, "QEMU x86_64 boot-to-shell");
    assert_eq!(milestones[9].month, 10);
    assert_eq!(milestones[9].promotion_target, "Desktop: Prototype or Integrated");
}

#[test]
fn test_pr_validation_rule_enforcement() {
    let mut engine = PeriodicPrComponentRulesEngine::new();

    // 1. Rejection due to missing ADR (Rule 3)
    let invalid_adr_rec = PrValidationRecord {
        pr_title: "feat(vfs): path resolver".into(),
        branch_name: "feat/vfs-path-resolver".into(),
        component: ComponentDomain::VfsFilesystems,
        study_issue_id: Some(42),
        has_adr: false, // Missing ADR!
        unit_tests_passed: true,
        integration_tests_passed: true,
        runtime_evidence_path: Some("reports/qemu.log".into()),
        security_review_done: false,
        status_promoted: false,
    };
    let res1 = engine.validate_pr(invalid_adr_rec);
    assert!(res1.is_err());
    assert!(res1.unwrap_err().contains("Rule 3 Violation"));

    // 2. Rejection due to status promotion without runtime evidence (Rule 5)
    let invalid_evidence_rec = PrValidationRecord {
        pr_title: "feat(vfs): path resolver".into(),
        branch_name: "feat/vfs-path-resolver".into(),
        component: ComponentDomain::VfsFilesystems,
        study_issue_id: Some(42),
        has_adr: true,
        unit_tests_passed: true,
        integration_tests_passed: true,
        runtime_evidence_path: None, // Missing runtime evidence!
        security_review_done: false,
        status_promoted: true, // Attempted promotion
    };
    let res2 = engine.validate_pr(invalid_evidence_rec);
    assert!(res2.is_err());
    assert!(res2.unwrap_err().contains("Rule 5 Violation"));

    // 3. Rejection due to security change without security review (Rule 8)
    let invalid_sec_rec = PrValidationRecord {
        pr_title: "security(pledge): sandbox filters".into(),
        branch_name: "security/pledge-filters".into(),
        component: ComponentDomain::Security,
        study_issue_id: Some(45),
        has_adr: true,
        unit_tests_passed: true,
        integration_tests_passed: true,
        runtime_evidence_path: Some("reports/security.log".into()),
        security_review_done: false, // Security review missing!
        status_promoted: false,
    };
    let res3 = engine.validate_pr(invalid_sec_rec);
    assert!(res3.is_err());
    assert!(res3.unwrap_err().contains("Rule 8 Violation"));

    // 4. Valid PR proposal
    let valid_rec = PrValidationRecord {
        pr_title: "feat(vfs): centralized path resolver".into(),
        branch_name: "feat/vfs-path-resolver-043".into(),
        component: ComponentDomain::VfsFilesystems,
        study_issue_id: Some(42),
        has_adr: true,
        unit_tests_passed: true,
        integration_tests_passed: true,
        runtime_evidence_path: Some("reports/boot/qemu-ramdisk.log".into()),
        security_review_done: false,
        status_promoted: true,
    };
    let res4 = engine.validate_pr(valid_rec);
    assert!(res4.is_ok());

    let report = engine.generate_governance_report();
    assert!(report.contains("SigmaOS Periodic PR-Only Component-Rule Governance Report"));
    assert!(report.contains("feat(vfs): centralized path resolver"));
}
