// Integration tests for SigmaOS Periodic "Study → Improve Component → Prove Improvement" Engine

#[path = "../src/governance/periodic_study_improve_prove_engine.rs"]
mod periodic_study_improve_prove_engine;

use periodic_study_improve_prove_engine::*;

#[test]
fn test_periodic_cadence_metadata() {
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
    let domain = ComponentDomain::VfsFilesystems;
    assert_eq!(domain.name(), "VFS/filesystems");
    assert_eq!(domain.primary_references(), "Linux, xv6, FreeBSD");
    assert_eq!(domain.required_proof(), "Filesystem conformance tests");
}

#[test]
fn test_improvement_loop_steps() {
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
fn test_maturity_promotion_rule_3_enforcement() {
    let mut engine = PeriodicImprovementEngine::new();

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
        14, // unit tests added
        2,  // integration tests added
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
    assert_eq!(
        engine.get_maturity(ComponentDomain::VfsFilesystems),
        ComponentMaturity::Integrated
    );
}

#[test]
fn test_monthly_milestone_completion_and_report() {
    let mut engine = PeriodicImprovementEngine::new();

    // Complete Month 1 milestone
    assert!(engine
        .complete_monthly_milestone(
            1,
            "reports/boot/qemu-x86_64-boot-matrix.json".into()
        )
        .is_ok());

    let summary = engine.generate_program_summary();
    assert!(summary.contains("SigmaOS Periodic Improvement Program Status Report"));
    assert!(summary.contains("Month 1: Reproducible QEMU x86_64 boot-to-shell [COMPLETED]"));
}
