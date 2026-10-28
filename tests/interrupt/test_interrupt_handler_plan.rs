//! Test suite for verifying the SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan
//!
//! Validates `docs/INTERRUPT_HANDLER_PLAN.md` and `wiki/INTERRUPT_HANDLER_PLAN.md`.

use std::fs;
use std::path::Path;

#[test]
fn test_interrupt_handler_plan_docs_exist() {
    let doc_path = Path::new("docs/INTERRUPT_HANDLER_PLAN.md");
    let wiki_path = Path::new("wiki/INTERRUPT_HANDLER_PLAN.md");

    assert!(doc_path.exists(), "docs/INTERRUPT_HANDLER_PLAN.md must exist");
    assert!(wiki_path.exists(), "wiki/INTERRUPT_HANDLER_PLAN.md must exist");

    let doc_content = fs::read_to_string(doc_path).expect("Failed to read docs/INTERRUPT_HANDLER_PLAN.md");
    let wiki_content = fs::read_to_string(wiki_path).expect("Failed to read wiki/INTERRUPT_HANDLER_PLAN.md");

    assert_eq!(
        doc_content, wiki_content,
        "docs/INTERRUPT_HANDLER_PLAN.md and wiki/INTERRUPT_HANDLER_PLAN.md content must match"
    );
}

#[test]
fn test_interrupt_handler_plan_rules_and_invariants() {
    let content = fs::read_to_string("docs/INTERRUPT_HANDLER_PLAN.md")
        .expect("Failed to read docs/INTERRUPT_HANDLER_PLAN.md");

    let required_invariants = [
        "Separate architecture entry, IRQ flow, and controller operations.",
        "Keep hard-interrupt work bounded.",
        "Specify edge vs level semantics.",
        "Make ownership and teardown explicit.",
        "Define spurious and unhandled behavior.",
        "Do not sleep in hard-IRQ context.",
        "Treat interrupt numbers as mapped identities.",
        "SMP requires synchronization.",
        "Test by evidence tier.",
    ];

    for invariant in &required_invariants {
        assert!(
            content.contains(invariant),
            "Missing invariant: {}",
            invariant
        );
    }
}

#[test]
fn test_interrupt_handler_plan_pr_sequence() {
    let content = fs::read_to_string("docs/INTERRUPT_HANDLER_PLAN.md")
        .expect("Failed to read docs/INTERRUPT_HANDLER_PLAN.md");

    for pr_num in 1..=17 {
        let pr_marker = format!("| {} |", pr_num);
        assert!(
            content.contains(&pr_marker),
            "Missing PR number marker: {}",
            pr_marker
        );
    }

    assert!(
        content.contains("`docs(irq): audit interrupt paths and define support matrix`"),
        "Missing PR 1 title"
    );
    assert!(
        content.contains("`feat(irq): validate RISC-V PLIC dispatch on QEMU`"),
        "Missing PR 14 title"
    );
    assert!(
        content.contains("`docs(irq): publish driver guide and architecture support matrix`"),
        "Missing PR 17 title"
    );
}

#[test]
fn test_interrupt_handler_plan_cadence_and_rules() {
    let content = fs::read_to_string("docs/INTERRUPT_HANDLER_PLAN.md")
        .expect("Failed to read docs/INTERRUPT_HANDLER_PLAN.md");

    let cadences = [
        "Every PR",
        "Weekly",
        "Monthly",
        "Quarterly",
        "Each release candidate",
        "After each incident",
    ];

    for cadence in &cadences {
        assert!(
            content.contains(cadence),
            "Missing maintenance cadence: {}",
            cadence
        );
    }

    let completion_criteria = ["Prototype:", "Integrated:", "Supported:"];

    for criteria in &completion_criteria {
        assert!(
            content.contains(criteria),
            "Missing completion criteria: {}",
            criteria
        );
    }
}

#[test]
fn test_feature_status_toml_interrupt_registration() {
    let content = fs::read_to_string("FEATURE_STATUS.toml")
        .expect("Failed to read FEATURE_STATUS.toml");

    assert!(
        content.contains("[kernel.interrupt_framework]"),
        "FEATURE_STATUS.toml must contain [kernel.interrupt_framework]"
    );
    assert!(
        content.contains("docs/INTERRUPT_HANDLER_PLAN.md"),
        "FEATURE_STATUS.toml must reference docs/INTERRUPT_HANDLER_PLAN.md"
    );
    assert!(
        content.contains("[kernel.irq_subsystem]"),
        "FEATURE_STATUS.toml must contain [kernel.irq_subsystem]"
    );
}
