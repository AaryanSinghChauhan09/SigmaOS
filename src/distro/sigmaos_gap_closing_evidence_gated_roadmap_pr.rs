//! SigmaOS Distro-Style Evidence-Gated Gap-Closing Roadmap Engine
//! (`src/distro/sigmaos_gap_closing_evidence_gated_roadmap_pr.rs`)
//!
//! `#![no_std]` compliant Rust engine implementing the distro-style gap closing model,
//! severity classification (S0-S3), 10-point minimum proof standard, canonical status vocabulary,
//! 7 core workstreams, 12 industry rules, P0-P3 backlog, and PR proposal generation.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Gap Severity Levels S0 through S3
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
            Self::S1CoreParity => "S1 — Core Parity",
            Self::S2DistroUsability => "S2 — Distro Usability",
            Self::S3Enhancement => "S3 — Enhancement",
        }
    }

    pub fn definition(&self) -> &'static str {
        match self {
            Self::S0Blocking => "Prevents boot, install, update, recovery, or basic use",
            Self::S1CoreParity => "Required for a minimal usable OS",
            Self::S2DistroUsability => "Expected by desktop Linux/BSD users",
            Self::S3Enhancement => "Valuable but not required for core use",
        }
    }

    pub fn closure_target(&self) -> &'static str {
        match self {
            Self::S0Blocking => "Fix immediately",
            Self::S1CoreParity => "Current milestone",
            Self::S2DistroUsability => "Next release",
            Self::S3Enhancement => "Backlog",
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

    pub fn prohibited_claim(&self) -> &'static str {
        match self {
            Self::Proposed => "Works or supported",
            Self::Prototype => "Boot-ready or production-ready",
            Self::Integrated => "Hardware supported without hardware log",
            Self::Supported => "Universal compatibility without test corpus",
        }
    }
}

/// Distro & BSD Practice Mapping Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroPracticeEntry {
    pub practice: &'static str,
    pub application: &'static str,
}

/// Priority Gap Class Descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorityGapClass {
    pub name: &'static str,
    pub current_signal: &'static str,
    pub closure_outcome: &'static str,
}

/// Workstream Spec
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkstreamSpec {
    pub id: u8,
    pub title: &'static str,
    pub gap_description: &'static str,
    pub closure_plan: Vec<&'static str>,
    pub exit_criterion: &'static str,
}

/// 12 Industry Rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapClosingRule {
    Rule1SingleSourceOfTruth,
    Rule2EvidenceGatedPromotion,
    Rule3VerticalSlicesOverHorizontalPromises,
    Rule4CiIsAGate,
    Rule5Reproducibility,
    Rule6LeastPrivilege,
    Rule7SecurityChangesRequireSeparateReview,
    Rule8FailSafely,
    Rule9NoUnsupportedClaims,
    Rule10RegressionProtection,
    Rule11ScopeDiscipline,
    Rule12HumanAccountability,
}

impl GapClosingRule {
    pub fn id(&self) -> u8 {
        match self {
            Self::Rule1SingleSourceOfTruth => 1,
            Self::Rule2EvidenceGatedPromotion => 2,
            Self::Rule3VerticalSlicesOverHorizontalPromises => 3,
            Self::Rule4CiIsAGate => 4,
            Self::Rule5Reproducibility => 5,
            Self::Rule6LeastPrivilege => 6,
            Self::Rule7SecurityChangesRequireSeparateReview => 7,
            Self::Rule8FailSafely => 8,
            Self::Rule9NoUnsupportedClaims => 9,
            Self::Rule10RegressionProtection => 10,
            Self::Rule11ScopeDiscipline => 11,
            Self::Rule12HumanAccountability => 12,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Rule1SingleSourceOfTruth => "Single source of truth",
            Self::Rule2EvidenceGatedPromotion => "Evidence-gated promotion",
            Self::Rule3VerticalSlicesOverHorizontalPromises => "Vertical slices over horizontal promises",
            Self::Rule4CiIsAGate => "CI is a gate, not a dashboard",
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

/// Backlog Task Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacklogTask {
    pub priority: &'static str,
    pub id: u8,
    pub title: &'static str,
}

/// 10-Point Minimum Proof Standard Checklist Evaluator
#[derive(Debug, Clone, Default)]
pub struct EvidenceProofChecklist {
    pub implementation_exists: bool,
    pub unit_tests_pass: bool,
    pub integration_tests_pass: bool,
    pub runtime_evidence_exists: bool,
    pub documentation_matches_reality: bool,
    pub regression_test_exists: bool,
    pub rollback_recovery_path_defined: bool,
    pub security_review_complete: bool,
    pub status_updated_on_canonical_page: bool,
    pub evidence_linked_and_reproducible: bool,
}

impl EvidenceProofChecklist {
    pub fn new_all_passed() -> Self {
        Self {
            implementation_exists: true,
            unit_tests_pass: true,
            integration_tests_pass: true,
            runtime_evidence_exists: true,
            documentation_matches_reality: true,
            regression_test_exists: true,
            rollback_recovery_path_defined: true,
            security_review_complete: true,
            status_updated_on_canonical_page: true,
            evidence_linked_and_reproducible: true,
        }
    }

    pub fn satisfied_count(&self) -> usize {
        let mut count = 0;
        if self.implementation_exists { count += 1; }
        if self.unit_tests_pass { count += 1; }
        if self.integration_tests_pass { count += 1; }
        if self.runtime_evidence_exists { count += 1; }
        if self.documentation_matches_reality { count += 1; }
        if self.regression_test_exists { count += 1; }
        if self.rollback_recovery_path_defined { count += 1; }
        if self.security_review_complete { count += 1; }
        if self.status_updated_on_canonical_page { count += 1; }
        if self.evidence_linked_and_reproducible { count += 1; }
        count
    }

    pub fn is_gap_closed(&self) -> bool {
        self.satisfied_count() == 10
    }
}

/// PR Proposal Spec Descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GapClosingPrProposalSpec {
    pub pr_title: String,
    pub pr_branch: String,
    pub severities_count: usize,
    pub rules_count: usize,
    pub workstreams_count: usize,
    pub patch_digest: u64,
    pub status: String,
}

/// FNV-1a checksum calculation
pub fn fnv1a_gap_closing_digest(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Master PR Gateway Engine for Gap Closing Evidence-Gated Roadmap
#[derive(Debug, Clone, Default)]
pub struct GapClosingPrEngine;

impl GapClosingPrEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn get_distro_practices(&self) -> Vec<DistroPracticeEntry> {
        alloc::vec![
            DistroPracticeEntry {
                practice: "Debian installer first",
                application: "Prove QEMU x86_64 boot-to-shell before desktop ambitions",
            },
            DistroPracticeEntry {
                practice: "Arch reproducible build discipline",
                application: "Every artifact must be rebuildable from a locked toolchain and commit",
            },
            DistroPracticeEntry {
                practice: "FreeBSD release engineering",
                application: "Maintain a release checklist, known issues, and rollback path",
            },
            DistroPracticeEntry {
                practice: "OpenBSD least privilege",
                application: "Security boundaries are reviewed separately and never bypassed for speed",
            },
            DistroPracticeEntry {
                practice: "Fedora CI/gating",
                application: "CI must block regressions, not merely record them",
            },
            DistroPracticeEntry {
                practice: "NixOS declarative configuration",
                application: "Make system configuration, tests, and build inputs explicit",
            },
            DistroPracticeEntry {
                practice: "Linux hardware enablement",
                application: "Maintain a support matrix with logs, dates, and tested device models",
            },
        ]
    }

    pub fn get_priority_gap_classes(&self) -> Vec<PriorityGapClass> {
        alloc::vec![
            PriorityGapClass {
                name: "Boot and installation",
                current_signal: "QEMU x86_64 boot success is 0%; UEFI/BIOS marked not started",
                closure_outcome: "Reproducible boot-to-shell and clean-install path",
            },
            PriorityGapClass {
                name: "Runtime integration",
                current_signal: "Source modules are not proof of working runtime features",
                closure_outcome: "End-to-end subsystem tests connected to boot",
            },
            PriorityGapClass {
                name: "Capability truth",
                current_signal: "Many plans, broad status vocabulary, historical documents",
                closure_outcome: "Evidence-backed component status",
            },
            PriorityGapClass {
                name: "Hardware enablement",
                current_signal: "No real hardware tested; missing graphics, Wi-Fi, audio, USB, suspend",
                closure_outcome: "Tiered device support with test logs",
            },
            PriorityGapClass {
                name: "Packaging and updates",
                current_signal: "Packaging/update recovery remains a stated priority",
                closure_outcome: "Safe install, update, rollback, and recovery workflow",
            },
            PriorityGapClass {
                name: "Security",
                current_signal: "Security is a core component but requires separate assurance",
                closure_outcome: "Threat model, capability tests, and review gates",
            },
            PriorityGapClass {
                name: "Documentation sprawl",
                current_signal: "165 wiki pages and many historical planning documents",
                closure_outcome: "Canonical component pages and archived plans",
            },
        ]
    }

    pub fn get_severities(&self) -> Vec<GapSeverity> {
        alloc::vec![
            GapSeverity::S0Blocking,
            GapSeverity::S1CoreParity,
            GapSeverity::S2DistroUsability,
            GapSeverity::S3Enhancement,
        ]
    }

    pub fn get_rules(&self) -> Vec<GapClosingRule> {
        alloc::vec![
            GapClosingRule::Rule1SingleSourceOfTruth,
            GapClosingRule::Rule2EvidenceGatedPromotion,
            GapClosingRule::Rule3VerticalSlicesOverHorizontalPromises,
            GapClosingRule::Rule4CiIsAGate,
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

    pub fn get_workstreams(&self) -> Vec<WorkstreamSpec> {
        alloc::vec![
            WorkstreamSpec {
                id: 1,
                title: "Boot and installation",
                gap_description: "QEMU x86_64 target has 0% boot evidence.",
                closure_plan: alloc::vec![
                    "Build clean ISO from pinned commit and toolchain",
                    "Test UEFI and BIOS boot separately in QEMU",
                    "Capture serial console, kernel log, boot duration",
                    "Define minimum boot contract",
                    "Add boot-to-shell smoke suite and clean-install test",
                ],
                exit_criterion: "100% boot success across supported QEMU x86_64 CI images.",
            },
            WorkstreamSpec {
                id: 2,
                title: "Kernel and core runtime",
                gap_description: "Core subsystems lack runtime integration proof.",
                closure_plan: alloc::vec![
                    "Select minimum viable kernel path",
                    "Write integration test per subsystem",
                    "Add runtime dependency map",
                    "Treat non-reachable subsystems as Prototype",
                    "Add fault-injection tests",
                ],
                exit_criterion: "Boot-to-shell run exercises every claimed core subsystem with trace.",
            },
            WorkstreamSpec {
                id: 3,
                title: "Security and isolation",
                gap_description: "Security boundaries require explicit threat model and capability tests.",
                closure_plan: alloc::vec![
                    "Create threat model covering untrusted packages/binaries/network",
                    "Define capability boundaries",
                    "Add negative denial tests",
                    "Separate security review from feature review",
                    "Use fuzzing for parsers and syscall handlers",
                ],
                exit_criterion: "Documented threat model, capability matrix, and passing denial tests.",
            },
            WorkstreamSpec {
                id: 4,
                title: "Packaging, updates, and recovery",
                gap_description: "Package installation and update recovery need atomic transaction verification.",
                closure_plan: alloc::vec![
                    "Phase 1 metadata parse",
                    "Phase 2 dependency resolution and dry-run",
                    "Phase 3 isolated test image installation",
                    "Phase 4 sandboxed script execution",
                    "Phase 5 atomic update and rollback tests",
                ],
                exit_criterion: "Failed update completes atomically or restores prior state proven by test.",
            },
            WorkstreamSpec {
                id: 5,
                title: "Hardware enablement",
                gap_description: "No real hardware tested; missing GPU, Wi-Fi, audio, USB, suspend.",
                closure_plan: alloc::vec![
                    "Keep QEMU x86_64 as Tier 1",
                    "Add ARM64 QEMU after x86_64 target",
                    "Maintain device matrix with exact models and logs",
                    "Start with virtio devices",
                    "Real hardware after QEMU parity",
                ],
                exit_criterion: "Every support claim has linked test logs.",
            },
            WorkstreamSpec {
                id: 6,
                title: "Desktop and userspace",
                gap_description: "Desktop compositor and apps should not precede reliable boot and runtime.",
                closure_plan: alloc::vec![
                    "Define minimal desktop contract",
                    "Add automated UI smoke tests",
                    "Add crash reporting and diagnostics",
                ],
                exit_criterion: "QEMU boot reaches desktop, launches app, accepts input, shuts down cleanly.",
            },
            WorkstreamSpec {
                id: 7,
                title: "Documentation and project governance",
                gap_description: "165 wiki pages create confusion over current status.",
                closure_plan: alloc::vec![
                    "Declare component wiki pages canonical",
                    "Move historical plans to archive namespace",
                    "Add front-matter verification blocks",
                    "Add contributor guide",
                ],
                exit_criterion: "Every active component page has current status and evidence links.",
            },
        ]
    }

    pub fn get_backlog(&self) -> Vec<BacklogTask> {
        alloc::vec![
            BacklogTask { priority: "P0", id: 1, title: "Create machine-readable gap register" },
            BacklogTask { priority: "P0", id: 2, title: "Audit all component pages and downgrade unevidenced claims" },
            BacklogTask { priority: "P0", id: 3, title: "Add CI gates for clean build, unit tests, formatting, and test scripts" },
            BacklogTask { priority: "P0", id: 4, title: "Create QEMU x86_64 boot-to-shell validator" },
            BacklogTask { priority: "P0", id: 5, title: "Publish Verified Today report from actual test evidence" },
            BacklogTask { priority: "P1", id: 6, title: "Prove UEFI boot in QEMU" },
            BacklogTask { priority: "P1", id: 7, title: "Prove BIOS boot in QEMU" },
            BacklogTask { priority: "P1", id: 8, title: "Prove boot-to-shell with serial logs" },
            BacklogTask { priority: "P1", id: 9, title: "Add VGA framebuffer smoke test" },
            BacklogTask { priority: "P1", id: 10, title: "Add basic virtio-net networking test" },
            BacklogTask { priority: "P1", id: 11, title: "Add clean-install test on disposable virtual disk" },
            BacklogTask { priority: "P1", id: 12, title: "Add update rollback and recovery test" },
            BacklogTask { priority: "P2", id: 13, title: "Connect VFS, process creation, syscalls, and shell into runtime trace" },
            BacklogTask { priority: "P2", id: 14, title: "Add package metadata parsing and dependency resolution" },
            BacklogTask { priority: "P2", id: 15, title: "Add sandboxed package installation" },
            BacklogTask { priority: "P2", id: 16, title: "Add minimal desktop launch and input smoke tests" },
            BacklogTask { priority: "P2", id: 17, title: "Add diagnostics, crash logs, and known-issue report" },
            BacklogTask { priority: "P3", id: 18, title: "Add ARM64 QEMU after x86_64 target" },
            BacklogTask { priority: "P3", id: 19, title: "Add real-hardware validation program with named devices and logs" },
            BacklogTask { priority: "P3", id: 20, title: "Expand package compatibility after sandbox and rollback tests pass" },
        ]
    }

    pub fn generate_proposal_spec(&self) -> GapClosingPrProposalSpec {
        let severities = self.get_severities();
        let rules = self.get_rules();
        let workstreams = self.get_workstreams();

        let mut payload = String::from("PR: feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap\n");
        for s in &severities {
            payload.push_str(s.code());
            payload.push_str(": ");
            payload.push_str(s.title());
            payload.push('\n');
        }
        for r in &rules {
            payload.push_str(r.name());
            payload.push('\n');
        }
        for w in &workstreams {
            payload.push_str(w.title);
            payload.push('\n');
        }

        let patch_digest = fnv1a_gap_closing_digest(payload.as_bytes());

        GapClosingPrProposalSpec {
            pr_title: String::from("feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap"),
            pr_branch: String::from("feature/gap-closing-evidence-gated-roadmap"),
            severities_count: severities.len(),
            rules_count: rules.len(),
            workstreams_count: workstreams.len(),
            patch_digest,
            status: String::from("Approved / Specification Ready"),
        }
    }

    pub fn generate_patch_diff(&self) -> String {
        let spec = self.generate_proposal_spec();
        format!(
            "--- a/docs/roadmap/PR_SPEC_SIGMAOS_GAP_CLOSING_EVIDENCE_GATED_ROADMAP.md\n\
             +++ b/docs/roadmap/PR_SPEC_SIGMAOS_GAP_CLOSING_EVIDENCE_GATED_ROADMAP.md\n\
             @@ -0,0 +1,120 @@\n\
             +# PR Title: {}\n\
             +# PR Branch: {}\n\
             +# Severities Count: {}\n\
             +# Rules Count: {}\n\
             +# Workstreams Count: {}\n\
             +# Patch Digest: {:016x}\n\
             +# Status: {}\n",
            spec.pr_title,
            spec.pr_branch,
            spec.severities_count,
            spec.rules_count,
            spec.workstreams_count,
            spec.patch_digest,
            spec.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_closing_severities_and_statuses() {
        let engine = GapClosingPrEngine::new();
        let severities = engine.get_severities();
        assert_eq!(severities.len(), 4);
        assert_eq!(severities[0].code(), "S0");
        assert_eq!(severities[3].code(), "S3");

        assert_eq!(CanonicalStatus::Supported.as_str(), "Supported");
        assert!(CanonicalStatus::Proposed.allowed_evidence().contains("Design document"));
    }

    #[test]
    fn test_gap_closing_rules_and_workstreams() {
        let engine = GapClosingPrEngine::new();
        let rules = engine.get_rules();
        let workstreams = engine.get_workstreams();

        assert_eq!(rules.len(), 12);
        assert_eq!(rules[0].id(), 1);
        assert_eq!(rules[11].id(), 12);

        assert_eq!(workstreams.len(), 7);
        assert_eq!(workstreams[0].id, 1);
        assert_eq!(workstreams[6].id, 7);
    }

    #[test]
    fn test_evidence_proof_checklist() {
        let full_checklist = EvidenceProofChecklist::new_all_passed();
        assert_eq!(full_checklist.satisfied_count(), 10);
        assert!(full_checklist.is_gap_closed());

        let partial_checklist = EvidenceProofChecklist {
            implementation_exists: true,
            unit_tests_pass: true,
            ..Default::default()
        };
        assert_eq!(partial_checklist.satisfied_count(), 2);
        assert!(!partial_checklist.is_gap_closed());
    }

    #[test]
    fn test_gap_closing_proposal_and_patch_diff() {
        let engine = GapClosingPrEngine::new();
        let spec = engine.generate_proposal_spec();

        assert_eq!(spec.pr_branch, "feature/gap-closing-evidence-gated-roadmap");
        assert_ne!(spec.patch_digest, 0);

        let diff = engine.generate_patch_diff();
        assert!(diff.contains("feature/gap-closing-evidence-gated-roadmap"));
        assert!(diff.contains("Patch Digest:"));
    }
}
