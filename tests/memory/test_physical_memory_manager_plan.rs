//! Verification suite for SigmaOS Physical Memory Manager (PMM) Architecture & PR Plan.

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
