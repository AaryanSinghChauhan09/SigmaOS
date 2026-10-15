//! Test suite for SigmaOS Distro-Style Evidence-Gated Gap-Closing Roadmap Engine

#[path = "../src/distro/sigmaos_gap_closing_evidence_gated_roadmap_pr.rs"]
mod sigmaos_gap_closing_evidence_gated_roadmap_pr;

use sigmaos_gap_closing_evidence_gated_roadmap_pr::*;

#[test]
fn test_gap_closing_distro_practices() {
    let engine = GapClosingPrEngine::new();
    let practices = engine.get_distro_practices();

    assert_eq!(practices.len(), 7);
    assert_eq!(practices[0].practice, "Debian installer first");
    assert!(practices[0].application.contains("QEMU x86_64 boot-to-shell"));
    assert_eq!(practices[1].practice, "Arch reproducible build discipline");
    assert_eq!(practices[2].practice, "FreeBSD release engineering");
    assert_eq!(practices[3].practice, "OpenBSD least privilege");
    assert_eq!(practices[4].practice, "Fedora CI/gating");
    assert_eq!(practices[5].practice, "NixOS declarative configuration");
    assert_eq!(practices[6].practice, "Linux hardware enablement");
}

#[test]
fn test_priority_gap_classes() {
    let engine = GapClosingPrEngine::new();
    let gap_classes = engine.get_priority_gap_classes();

    assert_eq!(gap_classes.len(), 7);
    assert_eq!(gap_classes[0].name, "Boot and installation");
    assert!(gap_classes[0].current_signal.contains("QEMU x86_64 boot success is 0%"));
    assert_eq!(gap_classes[1].name, "Runtime integration");
    assert_eq!(gap_classes[2].name, "Capability truth");
    assert_eq!(gap_classes[3].name, "Hardware enablement");
    assert_eq!(gap_classes[4].name, "Packaging and updates");
    assert_eq!(gap_classes[5].name, "Security");
    assert_eq!(gap_classes[6].name, "Documentation sprawl");
}

#[test]
fn test_gap_severities_and_closure_targets() {
    let engine = GapClosingPrEngine::new();
    let severities = engine.get_severities();

    assert_eq!(severities.len(), 4);
    assert_eq!(severities[0].code(), "S0");
    assert_eq!(severities[0].closure_target(), "Fix immediately");
    assert_eq!(severities[1].code(), "S1");
    assert_eq!(severities[1].closure_target(), "Current milestone");
    assert_eq!(severities[2].code(), "S2");
    assert_eq!(severities[2].closure_target(), "Next release");
    assert_eq!(severities[3].code(), "S3");
    assert_eq!(severities[3].closure_target(), "Backlog");
}

#[test]
fn test_canonical_status_vocabulary() {
    assert_eq!(CanonicalStatus::Proposed.as_str(), "Proposed");
    assert_eq!(CanonicalStatus::Prototype.as_str(), "Prototype");
    assert_eq!(CanonicalStatus::Integrated.as_str(), "Integrated");
    assert_eq!(CanonicalStatus::Supported.as_str(), "Supported");

    assert!(CanonicalStatus::Proposed.allowed_evidence().contains("Design document"));
    assert!(CanonicalStatus::Prototype.allowed_evidence().contains("Unit test"));
    assert!(CanonicalStatus::Integrated.allowed_evidence().contains("Integration test"));
    assert!(CanonicalStatus::Supported.allowed_evidence().contains("Reproducible QEMU/hardware test"));
}

#[test]
fn test_gap_closing_rules() {
    let engine = GapClosingPrEngine::new();
    let rules = engine.get_rules();

    assert_eq!(rules.len(), 12);
    assert_eq!(rules[0].id(), 1);
    assert_eq!(rules[0].name(), "Single source of truth");
    assert_eq!(rules[11].id(), 12);
    assert_eq!(rules[11].name(), "Human accountability");
}

#[test]
fn test_workstreams_and_exit_criteria() {
    let engine = GapClosingPrEngine::new();
    let workstreams = engine.get_workstreams();

    assert_eq!(workstreams.len(), 7);
    assert_eq!(workstreams[0].id, 1);
    assert_eq!(workstreams[0].title, "Boot and installation");
    assert!(workstreams[0].exit_criterion.contains("100% boot success"));
    assert_eq!(workstreams[6].id, 7);
    assert_eq!(workstreams[6].title, "Documentation and project governance");
}

#[test]
fn test_backlog_p0_to_p3() {
    let engine = GapClosingPrEngine::new();
    let backlog = engine.get_backlog();

    assert_eq!(backlog.len(), 20);
    assert_eq!(backlog[0].priority, "P0");
    assert_eq!(backlog[0].id, 1);
    assert_eq!(backlog[5].priority, "P1");
    assert_eq!(backlog[12].priority, "P2");
    assert_eq!(backlog[17].priority, "P3");
}

#[test]
fn test_minimum_proof_checklist_verification() {
    let mut checklist = EvidenceProofChecklist::default();
    assert_eq!(checklist.satisfied_count(), 0);
    assert!(!checklist.is_gap_closed());

    checklist.implementation_exists = true;
    checklist.unit_tests_pass = true;
    checklist.integration_tests_pass = true;
    checklist.runtime_evidence_exists = true;
    checklist.documentation_matches_reality = true;
    checklist.regression_test_exists = true;
    checklist.rollback_recovery_path_defined = true;
    checklist.security_review_complete = true;
    checklist.status_updated_on_canonical_page = true;
    checklist.evidence_linked_and_reproducible = true;

    assert_eq!(checklist.satisfied_count(), 10);
    assert!(checklist.is_gap_closed());
}

#[test]
fn test_pr_proposal_and_patch_diff_generation() {
    let engine = GapClosingPrEngine::new();
    let spec = engine.generate_proposal_spec();

    assert_eq!(spec.pr_title, "feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap");
    assert_eq!(spec.pr_branch, "feature/gap-closing-evidence-gated-roadmap");
    assert_eq!(spec.severities_count, 4);
    assert_eq!(spec.rules_count, 12);
    assert_eq!(spec.workstreams_count, 7);
    assert_ne!(spec.patch_digest, 0);

    let patch = engine.generate_patch_diff();
    assert!(patch.contains("feature/gap-closing-evidence-gated-roadmap"));
    assert!(patch.contains("Patch Digest:"));
}
