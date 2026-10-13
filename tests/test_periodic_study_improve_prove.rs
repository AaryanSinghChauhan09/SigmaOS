// SPDX-License-Identifier: MIT
// Integration and Unit Test Suite for Periodic Improvement Program

#[path = "../src/governance/periodic_study_improve_prove_engine.rs"]
mod periodic_study_improve_prove_engine;

use periodic_study_improve_prove_engine::*;

#[test]
fn test_periodic_program_full_compliance() {
    let engine = PeriodicStudyImproveProveEngine::new();
    assert!(engine.validate_program_compliance());
    assert_eq!(engine.schedule.cadences.len(), 5);
    assert_eq!(engine.ownership_map.components.len(), 12);
    assert_eq!(engine.weekly_cycle.days.len(), 7);
    assert_eq!(engine.monthly_roadmap.milestones.len(), 12);
    assert_eq!(engine.rules_engine.rules.len(), 14);
}

#[test]
fn test_periodic_industry_rules_enforcement() {
    let rules_engine = PeriodicProgramRulesEngine::new();
    assert!(rules_engine.validate_all_rules());
    for rule in &rules_engine.rules {
        assert!(rule.is_enforced);
        assert!(!rule.title.is_empty());
        assert!(!rule.requirement.is_empty());
    }
}

#[test]
fn test_component_maturity_promotion_pipeline() {
    let mut engine = PeriodicStudyImproveProveEngine::new();

    // Verify initial maturity state
    assert_eq!(
        engine.component_states.get("Kernel core"),
        Some(&ComponentMaturityLevel::Integrated)
    );

    // Promote component with full evidence
    let promoted = engine.promote_component(
        "Kernel core",
        true, // ADR
        true, // Unit tests
        true, // Runtime evidence
        true, // Fuzzing & negative tests
        true, // Reproducible release artifact
    );
    assert_eq!(promoted, ComponentMaturityLevel::Supported);
    assert_eq!(
        engine.component_states.get("Kernel core"),
        Some(&ComponentMaturityLevel::Supported)
    );
}

#[test]
fn test_improvement_report_generation() {
    let engine = PeriodicStudyImproveProveEngine::new();
    let report = engine.generate_improvement_report("Memory");
    assert!(report.contains("# Periodic Component Improvement Report — Memory"));
    assert!(report.contains("Maturity Level: Integrated"));
    assert!(report.contains("QEMU Boot Pass Rate: 100.0%"));
    assert!(report.contains("Compliance Rules Enforced: 14/14"));
}
