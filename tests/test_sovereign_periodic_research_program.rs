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
    assert_eq!(
        CurriculumCycle::Xv6AndOsDev.title(),
        "Cycle 1: xv6 and OSDev"
    );
    assert_eq!(
        CurriculumCycle::LinuxKernel.title(),
        "Cycle 2: Linux kernel"
    );
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
fn test_weekly_study_stages() {
    assert_eq!(
        WeeklyStudyStage::Day1SelectMechanism.description(),
        "Day 1: Select one narrowly bounded OS mechanism"
    );
    assert_eq!(
        WeeklyStudyStage::Day2StudyUpstream.description(),
        "Day 2: Read upstream source, docs, and record structures/flow"
    );
    assert_eq!(
        WeeklyStudyStage::Day3WriteDesignNoteAdr.description(),
        "Day 3: Write an Architecture Decision Record (ADR)"
    );
    assert_eq!(
        WeeklyStudyStage::Day4To5ImplementSmallestSlice.description(),
        "Days 4–5: Implement the smallest connected vertical slice"
    );
    assert_eq!(
        WeeklyStudyStage::Day6To7ValidateAndPublish.description(),
        "Days 6–7: Run full test suite, QEMU boot, and publish weekly report"
    );
}

#[test]
fn test_engine_adrs_and_rules() {
    let engine = SovereignPeriodicResearchEngine::new();
    assert_eq!(engine.monthly_roadmap.len(), 10);
    assert_eq!(engine.adrs.len(), 6);
    assert!(engine.validate_industry_rules());
    assert_eq!(INDUSTRY_RULES.len(), 14);

    let summary = engine.generate_summary_report();
    assert!(summary.contains("Sovereign Periodic Research Program Status"));
    assert!(summary.contains("ADR-001"));
    assert!(summary.contains("ADR-006"));
    assert!(summary.contains("Month 1: Boot and kernel entry [COMPLETED]"));
}

#[test]
fn test_task_toml_generators_and_pr_manifest() {
    let engine = SovereignPeriodicResearchEngine::new();

    let weekly_task = engine.generate_weekly_task_toml(
        "STUDY-2026-W42",
        "Study Linux VFS path resolution",
        "filesystem",
        &["linux", "xv6"],
    );
    assert!(weekly_task.contains("STUDY-2026-W42"));
    assert!(weekly_task.contains("filesystem"));
    assert!(weekly_task.contains("cycle = \"weekly\""));

    let monthly_task = engine.generate_monthly_task_toml(
        "CAP-2026-M10",
        "Deliver QEMU x86_64 boot-to-shell",
        "P0",
        "boot-runtime-agent",
    );
    assert!(monthly_task.contains("CAP-2026-M10"));
    assert!(monthly_task.contains("P0"));
    assert!(monthly_task.contains("cycle = \"monthly\""));

    let adr_template = engine.generate_adr_template(
        "ADR-007",
        "Minimal scheduler model",
        "xv6 uses a simple context-switch model; Linux uses runnable queues.",
    );
    assert!(adr_template.contains("ADR-007: Minimal scheduler model"));

    let pr_manifest = engine.generate_pr_proposal_manifest();
    assert_eq!(pr_manifest.pr_id, 4221);
    assert_eq!(
        pr_manifest.title,
        "governance: enforce periodic research and implementation program"
    );
    assert_eq!(pr_manifest.status, "Ready for Integration");
    assert_eq!(pr_manifest.target_files.len(), 7);
}
