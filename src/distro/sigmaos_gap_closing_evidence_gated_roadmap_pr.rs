//! Distro-Style Evidence-Gated Gap-Closing Roadmap and PR Gateway Engine
//! (`src/distro/sigmaos_gap_closing_evidence_gated_roadmap_pr.rs`)
//!
//! Zero-dependency, `#![no_std]` compliant Rust engine implementing the proposal,
//! verification matrix, status taxonomy, 10-point definition of done, and patch generator
//! for closing gaps between documented ambitions and runtime execution.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Four-level Gap Severity Model (S0–S3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GapSeverity {
    S0Blocking,
    S1CoreParity,
    S2DistroUsability,
    S3Enhancement,
}

impl GapSeverity {
    pub fn code(&self) -> &'static str {
        match self {
            Self::S0Blocking => "S0",
            Self::S1CoreParity => "S1",
            Self::S2DistroUsability => "S2",
            Self::S3Enhancement => "S3",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::S0Blocking => "S0 — Blocking",
            Self::S1CoreParity => "S1 — Core parity",
            Self::S2DistroUsability => "S2 — Distro usability",
            Self::S3Enhancement => "S3 — Enhancement",
        }
    }

    pub fn closure_target(&self) -> &'static str {
        match self {
            Self::S0Blocking => "Fix immediately",
            Self::S1CoreParity => "Current milestone (M1)",
            Self::S2DistroUsability => "Next release (M2/M3)",
            Self::S3Enhancement => "Backlog",
        }
    }
}

/// 10-Point Minimum Proof Standard (Definition of Done)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProofStandardPoint {
    ImplementationInCanonicalPath = 1,
    UnitTestsPass = 2,
    IntegrationTestsPass = 3,
    RuntimeEvidenceInQemuOrHardware = 4,
    DocumentationMatchesReality = 5,
    RegressionTestAdded = 6,
    RollbackOrRecoveryPathDefined = 7,
    SecurityReviewCompleted = 8,
    CanonicalPageUpdated = 9,
    EvidenceLinkedAndDated = 10,
}

impl ProofStandardPoint {
    pub fn id(&self) -> usize {
        *self as usize
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ImplementationInCanonicalPath => "Implementation exists in canonical source path.",
            Self::UnitTestsPass => "Unit tests pass for isolated logic.",
            Self::IntegrationTestsPass => "Integration tests pass across module boundaries.",
            Self::RuntimeEvidenceInQemuOrHardware => "Runtime evidence exists from QEMU or named hardware.",
            Self::DocumentationMatchesReality => "Documentation matches reality, including limitations.",
            Self::RegressionTestAdded => "Regression test exists to prevent reversal.",
            Self::RollbackOrRecoveryPathDefined => "Rollback or recovery path is defined for installer, update, and boot changes.",
            Self::SecurityReviewCompleted => "Security review is complete for privilege, memory, crypto, sandbox, package, or network changes.",
            Self::CanonicalPageUpdated => "Status is updated on component's canonical wiki page.",
            Self::EvidenceLinkedAndDated => "Evidence is linked, dated, and reproducible.",
        }
    }
}

/// Canonical Status Vocabulary
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalStatus {
    Proposed,
    Prototype,
    Integrated,
    Supported,
}

impl CanonicalStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Proposed => "Proposed",
            Self::Prototype => "Prototype",
            Self::Integrated => "Integrated",
            Self::Supported => "Supported",
        }
    }

    pub fn allowed_evidence(&self) -> &'static str {
        match self {
            Self::Proposed => "Design document or issue",
            Self::Prototype => "Unit test or isolated demo",
            Self::Integrated => "Integration test plus runtime path",
            Self::Supported => "Reproducible QEMU/hardware test, release checklist, regression coverage",
        }
    }
}

/// 7 Core Workstreams
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapWorkstream {
    BootAndInstallation,
    KernelAndCoreRuntime,
    SecurityAndIsolation,
    PackagingUpdatesAndRecovery,
    HardwareEnablement,
    DesktopAndUserspace,
    DocumentationAndGovernance,
}

impl GapWorkstream {
    pub fn id(&self) -> usize {
        match self {
            Self::BootAndInstallation => 1,
            Self::KernelAndCoreRuntime => 2,
            Self::SecurityAndIsolation => 3,
            Self::PackagingUpdatesAndRecovery => 4,
            Self::HardwareEnablement => 5,
            Self::DesktopAndUserspace => 6,
            Self::DocumentationAndGovernance => 7,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::BootAndInstallation => "Boot and installation",
            Self::KernelAndCoreRuntime => "Kernel and core runtime",
            Self::SecurityAndIsolation => "Security and isolation",
            Self::PackagingUpdatesAndRecovery => "Packaging, updates, and recovery",
            Self::HardwareEnablement => "Hardware enablement",
            Self::DesktopAndUserspace => "Desktop and userspace",
            Self::DocumentationAndGovernance => "Documentation and project governance",
        }
    }
}

/// 12 Industry-Standard Implementation Rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapClosingRule {
    Rule1SingleSourceOfTruth = 1,
    Rule2EvidenceGatedPromotion = 2,
    Rule3VerticalSlicesOverHorizontalPromises = 3,
    Rule4CiIsGateNotDashboard = 4,
    Rule5Reproducibility = 5,
    Rule6LeastPrivilege = 6,
    Rule7SecurityChangesRequireSeparateReview = 7,
    Rule8FailSafely = 8,
    Rule9NoUnsupportedClaims = 9,
    Rule10RegressionProtection = 10,
    Rule11ScopeDiscipline = 11,
    Rule12HumanAccountability = 12,
}

impl GapClosingRule {
    pub fn id(&self) -> usize {
        *self as usize
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Rule1SingleSourceOfTruth => "Single source of truth",
            Self::Rule2EvidenceGatedPromotion => "Evidence-gated promotion",
            Self::Rule3VerticalSlicesOverHorizontalPromises => "Vertical slices over horizontal promises",
            Self::Rule4CiIsGateNotDashboard => "CI is a gate, not a dashboard",
            Self::Rule5Reproducibility => "Reproducibility",
            Self::Rule6LeastPrivilege => "Least privilege",
            Self::Rule7SecurityChangesRequireSeparateReview => "Security changes require separate review",
            Self::Rule8FailSafely => "Fail safely",
            Self::Rule9NoUnsupportedClaims => "No unsupported claims",
            Self::Rule10RegressionProtection => "Regression protection",
            Self::Rule11ScopeDiscipline => "Scope discipline",
            Self::Rule12HumanAccountability => "Human accountability",
        }
    }
}

/// Gap Inventory Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GapInventoryRecord {
    pub gap_class: &'static str,
    pub current_signal: &'static str,
    pub closure_outcome: &'static str,
    pub severity: GapSeverity,
}

/// Release Gate Metric
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseGateMetric {
    pub metric: &'static str,
    pub m1_target: &'static str,
    pub current_baseline: &'static str,
    pub gate_condition: &'static str,
}

/// First 30 Days Milestone
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThirtyDayMilestone {
    pub timeframe: &'static str,
    pub title: &'static str,
    pub tasks: Vec<&'static str>,
}

/// Proposal Spec Descriptor for Evidence-Gated Roadmap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaOsGapClosingPrProposalSpec {
    pub pr_title: String,
    pub pr_branch: String,
    pub inventory_count: usize,
    pub workstream_count: usize,
    pub rule_count: usize,
    pub patch_digest: u64,
    pub status: String,
}

/// FNV-1a checksum calculation for PR patch content verification
pub fn fnv1a_gap_closing_digest(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Master PR Gateway Engine for Distro-Style Evidence-Gated Gap Closing
#[derive(Debug, Clone, Default)]
pub struct SigmaOsGapClosingPrGatewayEngine;

impl SigmaOsGapClosingPrGatewayEngine {
    pub fn new() -> Self {
        Self
    }

    /// Query current gap inventory.
    pub fn get_inventory(&self) -> Vec<GapInventoryRecord> {
        alloc::vec![
            GapInventoryRecord {
                gap_class: "Boot and installation",
                current_signal: "QEMU x86_64 boot success is 0%; UEFI/BIOS marked not started",
                closure_outcome: "Reproducible boot-to-shell and clean-install path",
                severity: GapSeverity::S0Blocking,
            },
            GapInventoryRecord {
                gap_class: "Runtime integration",
                current_signal: "Source modules are not proof of working runtime features",
                closure_outcome: "End-to-end subsystem tests connected to boot",
                severity: GapSeverity::S1CoreParity,
            },
            GapInventoryRecord {
                gap_class: "Capability truth",
                current_signal: "Many plans, broad status vocabulary, historical documents",
                closure_outcome: "Evidence-backed component status",
                severity: GapSeverity::S1CoreParity,
            },
            GapInventoryRecord {
                gap_class: "Hardware enablement",
                current_signal: "No real hardware tested; no graphics, Wi-Fi, audio, USB, or suspend support",
                closure_outcome: "Tiered device support with test logs",
                severity: GapSeverity::S2DistroUsability,
            },
            GapInventoryRecord {
                gap_class: "Packaging and updates",
                current_signal: "Packaging/update recovery remains a stated priority",
                closure_outcome: "Safe install, update, rollback, and recovery workflow",
                severity: GapSeverity::S2DistroUsability,
            },
            GapInventoryRecord {
                gap_class: "Security",
                current_signal: "Security is a core component but requires separate assurance",
                closure_outcome: "Threat model, capability tests, and review gates",
                severity: GapSeverity::S1CoreParity,
            },
            GapInventoryRecord {
                gap_class: "Documentation sprawl",
                current_signal: "165 wiki pages and many historical planning documents",
                closure_outcome: "Canonical component pages and archived plans",
                severity: GapSeverity::S2DistroUsability,
            },
        ]
    }

    /// Query 10-point proof standards.
    pub fn get_proof_standards(&self) -> Vec<ProofStandardPoint> {
        alloc::vec![
            ProofStandardPoint::ImplementationInCanonicalPath,
            ProofStandardPoint::UnitTestsPass,
            ProofStandardPoint::IntegrationTestsPass,
            ProofStandardPoint::RuntimeEvidenceInQemuOrHardware,
            ProofStandardPoint::DocumentationMatchesReality,
            ProofStandardPoint::RegressionTestAdded,
            ProofStandardPoint::RollbackOrRecoveryPathDefined,
            ProofStandardPoint::SecurityReviewCompleted,
            ProofStandardPoint::CanonicalPageUpdated,
            ProofStandardPoint::EvidenceLinkedAndDated,
        ]
    }

    /// Query 7 workstreams.
    pub fn get_workstreams(&self) -> Vec<GapWorkstream> {
        alloc::vec![
            GapWorkstream::BootAndInstallation,
            GapWorkstream::KernelAndCoreRuntime,
            GapWorkstream::SecurityAndIsolation,
            GapWorkstream::PackagingUpdatesAndRecovery,
            GapWorkstream::HardwareEnablement,
            GapWorkstream::DesktopAndUserspace,
            GapWorkstream::DocumentationAndGovernance,
        ]
    }

    /// Query 12 rules.
    pub fn get_rules(&self) -> Vec<GapClosingRule> {
        alloc::vec![
            GapClosingRule::Rule1SingleSourceOfTruth,
            GapClosingRule::Rule2EvidenceGatedPromotion,
            GapClosingRule::Rule3VerticalSlicesOverHorizontalPromises,
            GapClosingRule::Rule4CiIsGateNotDashboard,
            GapClosingRule::Rule5Reproducibility,
            GapClosingRule::Rule6LeastPrivilege,
            GapClosingRule::Rule7SecurityChangesRequireSeparateReview,
            GapClosingRule::Rule8FailSafely,
            GapClosingRule::Rule9NoUnsupportedClaims,
            GapClosingRule::Rule10RegressionProtection,
            GapClosingRule::Rule11ScopeDiscipline,
            GapClosingRule::Rule12HumanAccountability,
        ]
    }

    /// Query First 30 Days schedule.
    pub fn get_first_30_days(&self) -> Vec<ThirtyDayMilestone> {
        alloc::vec![
            ThirtyDayMilestone {
                timeframe: "Days 1–3",
                title: "Setup and Governance",
                tasks: alloc::vec![
                    "Create gap register, severity model, evidence policy, and canonical page template.",
                ],
            },
            ThirtyDayMilestone {
                timeframe: "Days 4–7",
                title: "Claim Audit",
                tasks: alloc::vec![
                    "Audit repository claims and downgrade unevidenced status claims.",
                ],
            },
            ThirtyDayMilestone {
                timeframe: "Days 8–14",
                title: "Boot Validator",
                tasks: alloc::vec![
                    "Implement QEMU x86_64 boot validator and retain serial logs.",
                ],
            },
            ThirtyDayMilestone {
                timeframe: "Days 15–21",
                title: "Boot Blocker Closure",
                tasks: alloc::vec![
                    "Solve primary boot blocker until boot-to-shell is reproducible.",
                ],
            },
            ThirtyDayMilestone {
                timeframe: "Days 22–27",
                title: "Install and Rollback",
                tasks: alloc::vec![
                    "Add clean-install and update rollback tests.",
                ],
            },
            ThirtyDayMilestone {
                timeframe: "Days 28–30",
                title: "M1 Report",
                tasks: alloc::vec![
                    "Publish first evidence-backed M1 status report and public known-issues list.",
                ],
            },
        ]
    }

    /// Generate PR proposal spec descriptor.
    pub fn generate_proposal_spec(&self) -> SigmaOsGapClosingPrProposalSpec {
        let inventory = self.get_inventory();
        let workstreams = self.get_workstreams();
        let rules = self.get_rules();

        let mut payload = String::from("PR: feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap\n");
        for inv in &inventory {
            payload.push_str(inv.gap_class);
            payload.push_str(": ");
            payload.push_str(inv.severity.code());
            payload.push('\n');
        }

        let digest = fnv1a_gap_closing_digest(payload.as_bytes());

        SigmaOsGapClosingPrProposalSpec {
            pr_title: String::from("feat(roadmap): Implement Distro-Style Evidence-Gated Gap-Closing Roadmap"),
            pr_branch: String::from("feature/evidence-gated-gap-closing-roadmap"),
            inventory_count: inventory.len(),
            workstream_count: workstreams.len(),
            rule_count: rules.len(),
            patch_digest: digest,
            status: String::from("Approved / Ready for Integration"),
        }
    }

    /// Generate diff patch sample.
    pub fn generate_patch_diff(&self) -> String {
        let spec = self.generate_proposal_spec();
        format!(
            "--- a/docs/roadmap/PR_SPEC_SIGMAOS_GAP_CLOSING_EVIDENCE_GATED_ROADMAP.md\n\
             +++ b/docs/roadmap/PR_SPEC_SIGMAOS_GAP_CLOSING_EVIDENCE_GATED_ROADMAP.md\n\
             @@ -0,0 +1,50 @@\n\
             +# PR Title: {}\n\
             +# PR Branch: {}\n\
             +# Inventory Count: {}\n\
             +# Workstreams: {}\n\
             +# Rules Count: {}\n\
             +# Patch Digest: {:016x}\n\
             +# Status: {}\n",
            spec.pr_title,
            spec.pr_branch,
            spec.inventory_count,
            spec.workstream_count,
            spec.rule_count,
            spec.patch_digest,
            spec.status
        )
    }

    /// Verify 10-point definition of done compliance.
    pub fn verify_definition_of_done(&self, checklist_points_satisfied: usize) -> bool {
        checklist_points_satisfied >= 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_severity_and_canonical_status() {
        assert_eq!(GapSeverity::S0Blocking.code(), "S0");
        assert_eq!(GapSeverity::S1CoreParity.closure_target(), "Current milestone (M1)");
        assert_eq!(CanonicalStatus::Supported.as_str(), "Supported");
    }

    #[test]
    fn test_proof_standards_and_rules() {
        let engine = SigmaOsGapClosingPrGatewayEngine::new();
        let standards = engine.get_proof_standards();
        let rules = engine.get_rules();
        assert_eq!(standards.len(), 10);
        assert_eq!(rules.len(), 12);
        assert_eq!(standards[0].id(), 1);
        assert_eq!(rules[11].id(), 12);
    }

    #[test]
    fn test_inventory_and_workstreams() {
        let engine = SigmaOsGapClosingPrGatewayEngine::new();
        let inv = engine.get_inventory();
        let ws = engine.get_workstreams();
        assert_eq!(inv.len(), 7);
        assert_eq!(ws.len(), 7);
        assert_eq!(ws[0].id(), 1);
    }

    #[test]
    fn test_spec_and_patch_generation() {
        let engine = SigmaOsGapClosingPrGatewayEngine::new();
        let spec = engine.generate_proposal_spec();
        assert_eq!(spec.pr_branch, "feature/evidence-gated-gap-closing-roadmap");
        assert_ne!(spec.patch_digest, 0);

        let patch = engine.generate_patch_diff();
        assert!(patch.contains("feature/evidence-gated-gap-closing-roadmap"));
        assert!(patch.contains("Patch Digest:"));
    }

    #[test]
    fn test_definition_of_done_verifier() {
        let engine = SigmaOsGapClosingPrGatewayEngine::new();
        assert!(engine.verify_definition_of_done(10));
        assert!(!engine.verify_definition_of_done(9));
    }
}
