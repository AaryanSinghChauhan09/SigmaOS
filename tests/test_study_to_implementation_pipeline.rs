// Integration test suite for SigmaOS Study-to-Implementation Pipeline Engine

#[path = "../src/governance/study_to_implementation_pipeline_pr.rs"]
mod study_to_implementation_pipeline_pr;

use study_to_implementation_pipeline_pr::*;

#[test]
fn test_full_pipeline_verification() {
    let engine = StudyToImplementationPipelineEngine::new();

    // Verify study items (xv6, Linux, Redox, Arch, Mint, FreeBSD, OpenBSD, Debian, Fedora, NixOS, OSDev)
    assert!(engine.study_items.len() >= 11, "Expected at least 11 study items");

    let projects: Vec<&str> = engine.study_items.iter().map(|s| s.project_name.as_str()).collect();
    assert!(projects.contains(&"xv6"));
    assert!(projects.contains(&"Linux Kernel"));
    assert!(projects.contains(&"Redox OS"));
    assert!(projects.contains(&"Arch Linux"));
    assert!(projects.contains(&"Linux Mint"));
    assert!(projects.contains(&"FreeBSD"));
    assert!(projects.contains(&"OpenBSD"));
    assert!(projects.contains(&"Debian"));
    assert!(projects.contains(&"Fedora"));
    assert!(projects.contains(&"NixOS"));
    assert!(projects.contains(&"OSDev"));

    // Verify ADRs
    assert_eq!(engine.adrs.len(), 5);
    for adr in &engine.adrs {
        assert_eq!(adr.status, AdrStatus::Accepted);
        assert!(!adr.validation_method.is_empty());
    }

    // Verify curriculum phases
    assert_eq!(engine.curriculum_phases.len(), 5);
    for phase in &engine.curriculum_phases {
        assert!(phase.exit_test_passed);
    }

    // Verify 10-step repeatable cycle
    assert_eq!(engine.cycle_steps.len(), 10);

    // Verify 12 mandatory rules
    assert_eq!(engine.rules.len(), 12);
    for rule in &engine.rules {
        assert!(rule.enforced);
    }

    // Overall pipeline health check
    assert!(engine.validate_pipeline_health());
}

#[test]
fn test_gateway_pr_summary_output() {
    let gateway = StudyToImplementationPipelinePrGateway::new();
    let summary = gateway.generate_pr_proposal_summary();

    assert!(summary.contains("Study-to-Implementation Pipeline Gateway Report"));
    assert!(summary.contains("Total Study Items Registered: 11"));
    assert!(summary.contains("Accepted ADR Records: 5"));
    assert!(summary.contains("Overall Pipeline Status: VERIFIED PASS"));
}
