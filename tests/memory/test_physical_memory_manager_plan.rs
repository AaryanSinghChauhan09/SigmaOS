//! Verification suite for SigmaOS Physical Memory Manager (PMM) Architecture & PR Plan.

use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum PmmRule {
    ExplicitlyUsableRamOnly,
    PhysicalVirtualAddressSeparation,
    CheckedArithmeticEverywhere,
    DefinedOwnershipAndPageStates,
    SeparatePmmVmmResponsibilities,
    ZeroMemoryBeforeExposure,
    ExplicitAllocationContext,
    DmaPolicyInDmaApi,
    NoPrematureNumaZones,
    ReadonlyObservability,
    EvidenceGatedCapabilityClaims,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompletionGate {
    Prototype,
    Integrated,
    Supported,
}

pub struct PmmPlan {
    pub rules: Vec<PmmRule>,
    pub pr_count: usize,
    pub recommended_milestone_prs: Vec<usize>,
}

impl PmmPlan {
    pub fn new() -> Self {
        Self {
            rules: vec![
                PmmRule::ExplicitlyUsableRamOnly,
                PmmRule::PhysicalVirtualAddressSeparation,
                PmmRule::CheckedArithmeticEverywhere,
                PmmRule::DefinedOwnershipAndPageStates,
                PmmRule::SeparatePmmVmmResponsibilities,
                PmmRule::ZeroMemoryBeforeExposure,
                PmmRule::ExplicitAllocationContext,
                PmmRule::DmaPolicyInDmaApi,
                PmmRule::NoPrematureNumaZones,
                PmmRule::ReadonlyObservability,
                PmmRule::EvidenceGatedCapabilityClaims,
            ],
            pr_count: 20,
            recommended_milestone_prs: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 13],
        }
    }

    pub fn verify_rules_count(&self) -> bool {
        self.rules.len() == 11
    }

    pub fn verify_pr_sequence_count(&self) -> bool {
        self.pr_count == 20
    }

    pub fn verify_recommended_first_milestone(&self) -> bool {
        self.recommended_milestone_prs.contains(&1)
            && self.recommended_milestone_prs.contains(&9)
            && self.recommended_milestone_prs.contains(&13)
            && self.recommended_milestone_prs.len() == 10
    }
}

impl Default for PmmPlan {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_pmm_plan_rules_and_invariants() {
    let plan = PmmPlan::new();
    assert!(plan.verify_rules_count());
    assert!(plan.rules.contains(&PmmRule::ExplicitlyUsableRamOnly));
    assert!(plan.rules.contains(&PmmRule::PhysicalVirtualAddressSeparation));
    assert!(plan.rules.contains(&PmmRule::CheckedArithmeticEverywhere));
    assert!(plan.rules.contains(&PmmRule::EvidenceGatedCapabilityClaims));
}

#[test]
fn test_pmm_pr_roadmap_and_milestones() {
    let plan = PmmPlan::new();
    assert!(plan.verify_pr_sequence_count());
    assert!(plan.verify_recommended_first_milestone());
}

#[test]
fn test_pmm_completion_gates() {
    let prototype = CompletionGate::Prototype;
    let integrated = CompletionGate::Integrated;
    let supported = CompletionGate::Supported;

    assert_ne!(prototype, integrated);
    assert_ne!(integrated, supported);
}

#[test]
fn test_pmm_documentation_plan_verification() {
    let doc_path = Path::new("docs/PHYSICAL_MEMORY_MANAGER_PLAN.md");
    let wiki_path = Path::new("wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md");

    assert!(doc_path.exists(), "docs/PHYSICAL_MEMORY_MANAGER_PLAN.md must exist");
    assert!(wiki_path.exists(), "wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md must exist");

    let doc_content = fs::read_to_string(doc_path).expect("Failed to read docs/PHYSICAL_MEMORY_MANAGER_PLAN.md");
    let wiki_content = fs::read_to_string(wiki_path).expect("Failed to read wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md");

    assert_eq!(
        doc_content, wiki_content,
        "docs/PHYSICAL_MEMORY_MANAGER_PLAN.md and wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md content must match exactly"
    );

    let required_invariants = [
        "Only explicitly usable RAM is allocatable.",
        "Keep physical and virtual addresses distinct.",
        "Use checked arithmetic everywhere.",
        "Define ownership and page states.",
        "Keep PMM and VMM responsibilities separate.",
        "Zero memory before exposing it to a less-trusted consumer.",
        "Make allocation context explicit.",
        "Keep DMA policy in a DMA API.",
        "Do not overbuild NUMA/zones prematurely.",
        "Support observability without exposing writable allocator internals.",
        "Evidence gates capability claims.",
    ];

    for invariant in &required_invariants {
        assert!(
            doc_content.contains(invariant),
            "Missing invariant in documentation: {}",
            invariant
        );
    }

    for pr_num in 1..=20 {
        let pr_marker = format!("| {} |", pr_num);
        assert!(
            doc_content.contains(&pr_marker),
            "Missing PR marker: {}",
            pr_marker
        );
    }

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
            doc_content.contains(cadence),
            "Missing maintenance cadence in doc: {}",
            cadence
        );
    }

    let completion_criteria = ["Prototype:", "Integrated:", "Supported:"];

    for criteria in &completion_criteria {
        assert!(
            doc_content.contains(criteria),
            "Missing completion criteria in doc: {}",
            criteria
        );
    }
}

#[test]
fn test_feature_status_toml_pmm_registration() {
    let content = fs::read_to_string("FEATURE_STATUS.toml")
        .expect("Failed to read FEATURE_STATUS.toml");

    assert!(
        content.contains("[kernel.physical_memory_manager]"),
        "FEATURE_STATUS.toml must contain [kernel.physical_memory_manager]"
    );
    assert!(
        content.contains("docs/PHYSICAL_MEMORY_MANAGER_PLAN.md"),
        "FEATURE_STATUS.toml must reference docs/PHYSICAL_MEMORY_MANAGER_PLAN.md"
    );
    assert!(
        content.contains("wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md"),
        "FEATURE_STATUS.toml must reference wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md"
    );
}
