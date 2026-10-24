// Integration tests for SigmaOS Periodic "Study → Improve Component → Prove Improvement" Engine

#[path = "../src/governance/periodic_study_improve_prove_engine.rs"]
mod periodic_study_improve_prove_engine;

use periodic_study_improve_prove_engine::*;

#[test]
fn test_periodic_cadence_metadata() {
    let cadences = [
        PeriodicCadence::Daily,
        PeriodicCadence::Weekly,
        PeriodicCadence::Biweekly,
        PeriodicCadence::Monthly,
        PeriodicCadence::Quarterly,
    ];

    for c in &cadences {
        assert!(!c.description().is_empty());
        assert!(!c.output_artifact().is_empty());
    }

    assert_eq!(
        PeriodicCadence::Daily.description(),
        "Keep build, tests, and boot status healthy"
    );
    assert_eq!(
        PeriodicCadence::Weekly.output_artifact(),
        "Upstream research note, ADR, small PR, test evidence"
    );
}

#[test]
fn test_component_domain_ownership_map() {
    let domains = [
        ComponentDomain::KernelCore,
        ComponentDomain::Memory,
        ComponentDomain::VfsFilesystems,
        ComponentDomain::Ipc,
        ComponentDomain::Syscalls,
        ComponentDomain::Security,
        ComponentDomain::Drivers,
        ComponentDomain::Networking,
        ComponentDomain::Packaging,
        ComponentDomain::DesktopZenith,
        ComponentDomain::InstallerUpdate,
        ComponentDomain::DistroCompatibility,
    ];

    for d in &domains {
        assert!(!d.name().is_empty());
        assert!(!d.primary_references().is_empty());
        assert!(!d.periodic_focus().is_empty());
        assert!(!d.required_proof().is_empty());
    }

    let domain = ComponentDomain::VfsFilesystems;
    assert_eq!(domain.name(), "VFS/filesystems");
    assert_eq!(domain.primary_references(), "Linux, xv6, FreeBSD");
    assert_eq!(domain.required_proof(), "Filesystem conformance tests");
}

#[test]
fn test_improvement_loop_steps() {
    let steps = [
        ImprovementLoopStep::Step1SelectComponent,
        ImprovementLoopStep::Step2MeasureCurrentState,
        ImprovementLoopStep::Step3SelectUpstreamReference,
        ImprovementLoopStep::Step4StudyMechanism,
        ImprovementLoopStep::Step5WriteImprovementProposal,
        ImprovementLoopStep::Step6ImplementSmallestSafeChange,
        ImprovementLoopStep::Step7RunTests,
        ImprovementLoopStep::Step8UpdateDocsAndStatus,
        ImprovementLoopStep::Step9AddRegressionProtection,
        ImprovementLoopStep::Step10ReviewMergeAndMonitor,
    ];

    for s in &steps {
        assert!(!s.name().is_empty());
    }

    assert_eq!(
        ImprovementLoopStep::Step1SelectComponent.name(),
        "1. SELECT COMPONENT"
    );
    assert_eq!(
        ImprovementLoopStep::Step10ReviewMergeAndMonitor.name(),
        "10. REVIEW, MERGE, AND MONITOR"
    );
}

#[test]
fn test_component_maturity_levels() {
    let levels = [
        ComponentMaturity::Proposed,
        ComponentMaturity::Prototype,
        ComponentMaturity::Integrated,
        ComponentMaturity::Hardened,
        ComponentMaturity::Supported,
    ];

    for lvl in &levels {
        assert!(!lvl.name().is_empty());
        assert!(!lvl.requirement().is_empty());
    }
}

#[test]
fn test_maturity_promotion_rule_3_enforcement() {
    let mut engine = PeriodicImprovementEngine::new();
    assert_eq!(
        engine.active_loop_step,
        ImprovementLoopStep::Step1SelectComponent
    );

    // Default status is Prototype
    assert_eq!(
        engine.get_maturity(ComponentDomain::VfsFilesystems),
        ComponentMaturity::Prototype
    );

    let study = UpstreamStudyRecord {
        target_domain: ComponentDomain::VfsFilesystems,
        upstream_reference: "Linux VFS".into(),
        problem_solved: "Path traversal error code standardization".into(),
        core_structures_and_flow: "namei path walk".into(),
        failure_and_security_assumptions: "dentry cache safety".into(),
        sigmaos_adaptations: "Centralized path resolver".into(),
        explicit_rejections: "Complex symlink loops for now".into(),
        adr_title: "ADR-012: VFS Path-Resolution Hardening".into(),
    };

    // Attempting to promote to Integrated WITHOUT runtime evidence should trigger Rule 3 rejection
    let res = engine.record_weekly_improvement(
        ComponentDomain::VfsFilesystems,
        study.clone(),
        "research/decisions/ADR-012-vfs.md".into(),
        "VFS path resolution error handling".into(),
        14,   // unit tests added
        2,    // integration tests added
        None, // NO runtime evidence path
        ComponentMaturity::Integrated,
    );

    assert!(res.is_err());
    let err_msg = res.unwrap_err();
    assert!(err_msg.contains("Rule 3 Violation"));
    // Status remains Prototype
    assert_eq!(
        engine.get_maturity(ComponentDomain::VfsFilesystems),
        ComponentMaturity::Prototype
    );

    // Now promote WITH runtime evidence
    let res_ok = engine.record_weekly_improvement(
        ComponentDomain::VfsFilesystems,
        study,
        "research/decisions/ADR-012-vfs.md".into(),
        "VFS path resolution error handling".into(),
        14,
        2,
        Some("reports/boot/qemu-x86_64.log".into()), // Runtime evidence provided!
        ComponentMaturity::Integrated,
    );

    assert!(res_ok.is_ok());
    let record = res_ok.unwrap();
    assert_eq!(record.domain, ComponentDomain::VfsFilesystems);
    assert_eq!(record.adr_file, "research/decisions/ADR-012-vfs.md");
    assert_eq!(record.unit_tests_added, 14);
    assert_eq!(record.integration_tests_added, 2);
    assert_eq!(record.study.target_domain, ComponentDomain::VfsFilesystems);

    assert_eq!(
        engine.get_maturity(ComponentDomain::VfsFilesystems),
        ComponentMaturity::Integrated
    );
}

#[test]
fn test_monthly_milestone_completion_and_report() {
    let mut engine = PeriodicImprovementEngine::default();

    // Complete Month 1 milestone
    assert!(engine
        .complete_monthly_milestone(1, "reports/boot/qemu-x86_64-boot-matrix.json".into())
        .is_ok());

    assert!(engine.complete_monthly_milestone(99, "bad".into()).is_err());

    let m1 = &engine.monthly_milestones[0];
    assert_eq!(m1.domain, ComponentDomain::KernelCore);

    let summary = engine.generate_program_summary();
    assert!(summary.contains("SigmaOS Periodic Improvement Program Status Report"));
    assert!(summary.contains("Month 1: Reproducible QEMU x86_64 boot-to-shell [COMPLETED]"));
}

#[test]
fn test_periodic_pr_gateway_and_proposal_formatting() {
    let mut gateway = PeriodicStudyImproveProvePrGateway::default();
    assert_eq!(gateway.pr_proposals.len(), 6);
    assert_eq!(gateway.engine.component_maturities.len(), 12);

    let vfs_prs = gateway.search_by_domain(ComponentDomain::VfsFilesystems);
    assert_eq!(vfs_prs.len(), 1);
    assert_eq!(vfs_prs[0].pr_id, 103);

    let formatted_desc = vfs_prs[0].format_pull_request_description();
    assert!(formatted_desc.contains("## PR #103: VFS/Filesystems: Centralized Path Resolution"));
    assert!(formatted_desc.contains("ADR-012: VFS Path-Resolution Hardening"));
    assert!(formatted_desc.contains("reports/vfs/vfs_conformance.json"));
    assert!(formatted_desc.contains("Rule 3: Runtime evidence gated status promotion"));

    let summary_report = gateway.generate_pr_gateway_summary();
    assert!(summary_report.contains("SigmaOS Periodic Study-Improve-Prove PR Gateway Report"));
    assert!(summary_report.contains("Total Staged PR Proposals: 6"));

    // Verify Rule 3 rejection when submitting unevidenced proposal
    let bad_proposal = PeriodicImprovementPrProposal {
        pr_id: 999,
        pr_title: "Drivers: Untested driver promotion".into(),
        branch_name: "feat/untested-driver".into(),
        domain: ComponentDomain::Drivers,
        upstream_reference: "Linux drivers".into(),
        adr_title: "ADR-999".into(),
        initial_maturity: ComponentMaturity::Prototype,
        promoted_maturity: ComponentMaturity::Integrated,
        runtime_evidence_path: None,
        summary_diff: "+ bad_driver_code()".into(),
        pqc_signature: "DILITHIUM5_SIG_UNTESTED".into(),
        ci_status: "FAILED".into(),
    };

    let submit_res = gateway.submit_pr_proposal(bad_proposal);
    assert!(submit_res.is_err());
    assert!(submit_res.unwrap_err().contains("Rule 3 Violation"));
}
