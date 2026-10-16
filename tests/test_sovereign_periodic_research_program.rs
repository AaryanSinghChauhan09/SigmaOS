// Integration test suite for Sovereign Periodic Research-and-Implementation Program Engine

#[path = "../src/governance/sovereign_periodic_research_program_engine.rs"]
mod sovereign_periodic_research_program_engine;

use sovereign_periodic_research_program_engine::*;

#[test]
fn test_cadences_and_outputs() {
    assert_eq!(
        ResearchCadence::Daily.required_output(),
        "Build/test/boot status report"
    );
    assert_eq!(
        ResearchCadence::Weekly.required_output(),
        "Research note + ADR + small PR + test evidence"
    );
    assert_eq!(
        ResearchCadence::Monthly.required_output(),
        "Runtime-validated feature"
    );
    assert_eq!(
        ResearchCadence::Quarterly.required_output(),
        "Updated roadmap and gap register"
    );
}

#[test]
fn test_curriculum_cycles() {
    assert_eq!(CurriculumCycle::Xv6AndOsDev.title(), "Cycle 1: xv6 and OSDev");
    assert_eq!(CurriculumCycle::LinuxKernel.title(), "Cycle 2: Linux kernel");
    assert_eq!(
        CurriculumCycle::ArchLinuxAndArchWiki.title(),
        "Cycle 3: Arch Linux and ArchWiki"
    );
    assert_eq!(CurriculumCycle::LinuxMint.title(), "Cycle 4: Linux Mint");
    assert_eq!(CurriculumCycle::Fedora.title(), "Cycle 5: Fedora");
    assert_eq!(
        CurriculumCycle::FreeBsdAndOpenBsd.title(),
        "Cycle 6: FreeBSD and OpenBSD"
    );
    assert_eq!(
        CurriculumCycle::RedoxOsAndRustOsDesign.title(),
        "Cycle 7: Redox OS and Rust OS design"
    );
}

#[test]
fn test_engine_adrs_and_rules() {
    let engine = SovereignPeriodicResearchEngine::new();
    assert_eq!(engine.monthly_roadmap.len(), 10);
    assert_eq!(engine.adrs.len(), 6);
    assert!(engine.validate_industry_rules());

    let summary = engine.generate_summary_report();
    assert!(summary.contains("Sovereign Periodic Research Program Status"));
    assert!(summary.contains("ADR-001"));
    assert!(summary.contains("ADR-006"));
    assert!(summary.contains("Month 1: Boot and kernel entry [COMPLETED]"));
}
