//! Repeatable Agent Operating Loop PR Proposal Engine
//! (`src/distro/repeatable_agent_operating_loop_pr.rs`)
//!
//! Zero-dependency, `#![no_std]` compliant Rust engine implementing the proposal,
//! verification matrix, permission sandbox rules, and patch generation for turning
//! SigmaOS into a repeatable agent operating loop:
//! - 7 Distro-Maintainer Agent Roles (Repository Steward, Build/Test Sentinel, Boot/Runtime Validator, Capability Auditor, Kernel/Subsystem Agents, Package/Compat Agent, Docs/Onboarding Agent).
//! - 9-Stage Repeatable Task Lifecycle (Select -> Read -> Isolate -> Implement -> Scoped Test -> Repo Checks -> Evidence Artifact -> Update Status -> PR Submit).
//! - 6-Level Evidence-Gated Status Vocabulary (`unimplemented`, `model-only`, `integration-pending`, `qemu-verified`, `hardware-verified`, `blocked`).
//! - Least-Privilege Permission Sandbox Matrix.
//! - 90-Day Execution Plan & Weekly Reporting Metrics.
//! - Machine-Readable Task TOML Schema Engine.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// 7 Distro-Maintainer Agent Roles
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DistroAgentRole {
    RepositorySteward,
    BuildAndTestSentinel,
    BootAndRuntimeValidator,
    CapabilityAuditor,
    KernelAndSubsystemAgent,
    PackageAndDistroCompatAgent,
    DocumentationAndOnboardingAgent,
}

impl DistroAgentRole {
    pub fn name(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "Repository Steward",
            Self::BuildAndTestSentinel => "Build and Test Sentinel",
            Self::BootAndRuntimeValidator => "Boot and Runtime Validator",
            Self::CapabilityAuditor => "Capability Auditor",
            Self::KernelAndSubsystemAgent => "Kernel and Subsystem Agent",
            Self::PackageAndDistroCompatAgent => "Package and Distro Compatibility Agent",
            Self::DocumentationAndOnboardingAgent => "Documentation and Onboarding Agent",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "steward",
            Self::BuildAndTestSentinel => "sentinel",
            Self::BootAndRuntimeValidator => "boot-validator",
            Self::CapabilityAuditor => "capability-auditor",
            Self::KernelAndSubsystemAgent => "kernel-subsystem",
            Self::PackageAndDistroCompatAgent => "package-compat",
            Self::DocumentationAndOnboardingAgent => "docs-onboarding",
        }
    }

    pub fn owner_path(&self) -> &'static str {
        match self {
            Self::RepositorySteward => ".github/",
            Self::BuildAndTestSentinel => "reports/test/",
            Self::BootAndRuntimeValidator => "reports/boot/",
            Self::CapabilityAuditor => "reports/capability/",
            Self::KernelAndSubsystemAgent => "src/kernel/",
            Self::PackageAndDistroCompatAgent => "src/package/",
            Self::DocumentationAndOnboardingAgent => "docs/",
        }
    }

    pub fn primary_responsibility(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "Repo hygiene, issue/PR triage, conventional commits, and staleness audits.",
            Self::BuildAndTestSentinel => "Automated clean builds, test execution, flaky test detection, and test manifests.",
            Self::BootAndRuntimeValidator => "QEMU boot-to-shell validation, serial console log capture, and boot matrix upkeep.",
            Self::CapabilityAuditor => "Evidence-backed truth table cross-referencing status claims against passing tests and boot traces.",
            Self::KernelAndSubsystemAgent => "Kernel entry, memory management, VFS, IPC, interrupts, and syscall ABI consistency.",
            Self::PackageAndDistroCompatAgent => "Package metadata schemas, dependency resolution, and sandboxed dry-run package execution.",
            Self::DocumentationAndOnboardingAgent => "Command synchronization between README/Makefile/scripts and Verified Today dashboard.",
        }
    }
}

/// 9-Stage Repeatable Task Lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatingLoopStage {
    Stage1SelectTask,
    Stage2ReadOwnershipAndMatrix,
    Stage3IsolateBranch,
    Stage4ImplementMinimalChange,
    Stage5RunScopedTests,
    Stage6RunRepoWideChecks,
    Stage7ProduceEvidenceArtifact,
    Stage8UpdateStatusWithEvidence,
    Stage9SubmitPrWithRollback,
}

impl OperatingLoopStage {
    pub fn step_number(&self) -> u8 {
        match self {
            Self::Stage1SelectTask => 1,
            Self::Stage2ReadOwnershipAndMatrix => 2,
            Self::Stage3IsolateBranch => 3,
            Self::Stage4ImplementMinimalChange => 4,
            Self::Stage5RunScopedTests => 5,
            Self::Stage6RunRepoWideChecks => 6,
            Self::Stage7ProduceEvidenceArtifact => 7,
            Self::Stage8UpdateStatusWithEvidence => 8,
            Self::Stage9SubmitPrWithRollback => 9,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Stage1SelectTask => "Select Task",
            Self::Stage2ReadOwnershipAndMatrix => "Read Ownership & Capability Matrix",
            Self::Stage3IsolateBranch => "Isolate Branch",
            Self::Stage4ImplementMinimalChange => "Implement Minimal Change",
            Self::Stage5RunScopedTests => "Run Scoped Tests",
            Self::Stage6RunRepoWideChecks => "Run Repository-Wide Checks",
            Self::Stage7ProduceEvidenceArtifact => "Produce Evidence Artifact",
            Self::Stage8UpdateStatusWithEvidence => "Update Status With Evidence",
            Self::Stage9SubmitPrWithRollback => "Submit PR With Rollback Note",
        }
    }
}

/// 6-Level Evidence-Gated Status Vocabulary
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatusLevel {
    Unimplemented,
    ModelOnly,
    IntegrationPending,
    QemuVerified,
    HardwareVerified,
    Blocked,
}

impl EvidenceStatusLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unimplemented => "unimplemented",
            Self::ModelOnly => "model-only",
            Self::IntegrationPending => "integration-pending",
            Self::QemuVerified => "qemu-verified",
            Self::HardwareVerified => "hardware-verified",
            Self::Blocked => "blocked",
        }
    }

    pub fn definition(&self) -> &'static str {
        match self {
            Self::Unimplemented => "No credible implementation found in codebase.",
            Self::ModelOnly => "Unit-tested model exists, but no connected runtime path.",
            Self::IntegrationPending => "Code exists but is not connected end-to-end to kernel boot/runtime.",
            Self::QemuVerified => "Passes automated QEMU boot and runtime execution tests.",
            Self::HardwareVerified => "Tested on named physical hardware with serial/kernel logs on record.",
            Self::Blocked => "Known dependency, design, or validation blocker prevents progress.",
        }
    }
}

/// Least-Privilege Agent Permission Matrix Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentPermissionMatrix {
    pub role: DistroAgentRole,
    pub fs_permission: &'static str,
    pub network_permission: &'static str,
    pub package_exec_permission: &'static str,
    pub git_write_permission: &'static str,
}

/// 90-Day Execution Plan Phase
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NinetyDayPlanPhase {
    pub timeframe: &'static str,
    pub title: &'static str,
    pub key_outcomes: Vec<&'static str>,
}

/// PR Proposal Specification Descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatableAgentOperatingLoopPrSpec {
    pub pr_title: String,
    pub pr_branch: String,
    pub roles: Vec<DistroAgentRole>,
    pub stages: Vec<OperatingLoopStage>,
    pub status_levels: Vec<EvidenceStatusLevel>,
    pub patch_digest: u64,
    pub status: String,
}

/// FNV-1a checksum calculation for PR patch content verification
pub fn fnv1a_agent_loop_digest(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Master PR Proposal Gateway Engine for Repeatable Agent Operating Loop
#[derive(Debug, Clone, Default)]
pub struct RepeatableAgentOperatingLoopPrEngine;

impl RepeatableAgentOperatingLoopPrEngine {
    pub fn new() -> Self {
        Self
    }

    /// Returns all 7 Distro-Maintainer Agent Roles.
    pub fn get_roles(&self) -> Vec<DistroAgentRole> {
        alloc::vec![
            DistroAgentRole::RepositorySteward,
            DistroAgentRole::BuildAndTestSentinel,
            DistroAgentRole::BootAndRuntimeValidator,
            DistroAgentRole::CapabilityAuditor,
            DistroAgentRole::KernelAndSubsystemAgent,
            DistroAgentRole::PackageAndDistroCompatAgent,
            DistroAgentRole::DocumentationAndOnboardingAgent,
        ]
    }

    /// Returns all 9 stages of the Repeatable Operating Loop.
    pub fn get_stages(&self) -> Vec<OperatingLoopStage> {
        alloc::vec![
            OperatingLoopStage::Stage1SelectTask,
            OperatingLoopStage::Stage2ReadOwnershipAndMatrix,
            OperatingLoopStage::Stage3IsolateBranch,
            OperatingLoopStage::Stage4ImplementMinimalChange,
            OperatingLoopStage::Stage5RunScopedTests,
            OperatingLoopStage::Stage6RunRepoWideChecks,
            OperatingLoopStage::Stage7ProduceEvidenceArtifact,
            OperatingLoopStage::Stage8UpdateStatusWithEvidence,
            OperatingLoopStage::Stage9SubmitPrWithRollback,
        ]
    }

    /// Returns all 6 evidence status levels.
    pub fn get_status_levels(&self) -> Vec<EvidenceStatusLevel> {
        alloc::vec![
            EvidenceStatusLevel::Unimplemented,
            EvidenceStatusLevel::ModelOnly,
            EvidenceStatusLevel::IntegrationPending,
            EvidenceStatusLevel::QemuVerified,
            EvidenceStatusLevel::HardwareVerified,
            EvidenceStatusLevel::Blocked,
        ]
    }

    /// Returns the complete least-privilege permission matrix across all agent classes.
    pub fn get_permission_matrix(&self) -> Vec<AgentPermissionMatrix> {
        alloc::vec![
            AgentPermissionMatrix {
                role: DistroAgentRole::DocumentationAndOnboardingAgent,
                fs_permission: "Read repo, write docs/ and reports/",
                network_permission: "Optional allowlisted",
                package_exec_permission: "No",
                git_write_permission: "PR only",
            },
            AgentPermissionMatrix {
                role: DistroAgentRole::BuildAndTestSentinel,
                fs_permission: "Read repo, write reports/ and temp dirs",
                network_permission: "No",
                package_exec_permission: "No",
                git_write_permission: "PR only",
            },
            AgentPermissionMatrix {
                role: DistroAgentRole::BootAndRuntimeValidator,
                fs_permission: "Read repo, write reports/boot/",
                network_permission: "No",
                package_exec_permission: "No",
                git_write_permission: "PR only",
            },
            AgentPermissionMatrix {
                role: DistroAgentRole::CapabilityAuditor,
                fs_permission: "Read-only plus reports/",
                network_permission: "No",
                package_exec_permission: "No",
                git_write_permission: "Issue / PR only",
            },
            AgentPermissionMatrix {
                role: DistroAgentRole::PackageAndDistroCompatAgent,
                fs_permission: "Sandbox only",
                network_permission: "No default",
                package_exec_permission: "Sandboxed dry-run first",
                git_write_permission: "PR only",
            },
            AgentPermissionMatrix {
                role: DistroAgentRole::KernelAndSubsystemAgent,
                fs_permission: "Scoped source paths",
                network_permission: "No",
                package_exec_permission: "No",
                git_write_permission: "PR only",
            },
            AgentPermissionMatrix {
                role: DistroAgentRole::RepositorySteward,
                fs_permission: "Read repo, write reports/steward/",
                network_permission: "No",
                package_exec_permission: "No",
                git_write_permission: "PR only",
            },
        ]
    }

    /// Returns the 90-Day Execution Plan phases.
    pub fn get_90_day_plan(&self) -> Vec<NinetyDayPlanPhase> {
        alloc::vec![
            NinetyDayPlanPhase {
                timeframe: "Days 1–14",
                title: "Establish Truth",
                key_outcomes: alloc::vec![
                    "Create agent registry, task schema, report directories, and CI verification jobs.",
                    "Run full capability audit across README, FEATURE_STATUS.toml, CAPABILITY_MATRIX.toml, docs, and source.",
                    "Produce 'Verified Today' dashboard and freeze unverified claims.",
                ],
            },
            NinetyDayPlanPhase {
                timeframe: "Days 15–35",
                title: "Make Build Trustworthy",
                key_outcomes: alloc::vec![
                    "Automate clean builds and test runs with failure classification.",
                    "Detect flaky tests and require machine-readable verification reports on every PR.",
                    "Repair mechanical failures autonomously; route design failures to human review.",
                ],
            },
            NinetyDayPlanPhase {
                timeframe: "Days 36–60",
                title: "Prove One Boot Path",
                key_outcomes: alloc::vec![
                    "Focus exclusively on QEMU x86_64 boot-to-shell.",
                    "Maintain boot matrix and retain every serial console log artifact.",
                    "Identify hard runtime blockers and assign dedicated subsystem agents.",
                ],
            },
            NinetyDayPlanPhase {
                timeframe: "Days 61–75",
                title: "Connect Subsystems to Runtime",
                key_outcomes: alloc::vec![
                    "Connect minimum set of subsystems (memory, VFS, init, shell) to real boot path.",
                    "Replace 'model passes test' claims with runtime trace evidence.",
                    "Add integration tests failing if subsystem disconnects from boot path.",
                ],
            },
            NinetyDayPlanPhase {
                timeframe: "Days 76–90",
                title: "Add Distro-Style Release Discipline",
                key_outcomes: alloc::vec![
                    "Define dev, nightly, and validated release channels.",
                    "Publish only QEMU-verified nightly build artifacts.",
                    "Add rollback instructions and begin sandboxed package metadata compatibility audit.",
                ],
            },
        ]
    }

    /// Generate PR Proposal Spec Descriptor.
    pub fn generate_proposal_spec(&self) -> RepeatableAgentOperatingLoopPrSpec {
        let roles = self.get_roles();
        let stages = self.get_stages();
        let status_levels = self.get_status_levels();

        let mut payload = String::from("PR: feat(distro): Turn Repository into Repeatable Agent Operating Loop\n");
        for r in &roles {
            payload.push_str(r.code());
            payload.push_str(": ");
            payload.push_str(r.name());
            payload.push('\n');
        }
        for s in &stages {
            payload.push_str(s.title());
            payload.push('\n');
        }

        let patch_digest = fnv1a_agent_loop_digest(payload.as_bytes());

        RepeatableAgentOperatingLoopPrSpec {
            pr_title: String::from("feat(distro): Turn Repository into Repeatable Agent Operating Loop"),
            pr_branch: String::from("feat/agent-operating-loop"),
            roles,
            stages,
            status_levels,
            patch_digest,
            status: String::from("Approved / Implementation Ready"),
        }
    }

    /// Generate sample patch diff for the PR specification.
    pub fn generate_patch_diff(&self) -> String {
        let spec = self.generate_proposal_spec();
        format!(
            "--- a/docs/roadmap/PR_SPEC_REPEATABLE_AGENT_OPERATING_LOOP.md\n\
             +++ b/docs/roadmap/PR_SPEC_REPEATABLE_AGENT_OPERATING_LOOP.md\n\
             @@ -0,0 +1,120 @@\n\
             +# PR Title: {}\n\
             +# PR Branch: {}\n\
             +# Roles Count: {}\n\
             +# Stages Count: {}\n\
             +# Status Levels Count: {}\n\
             +# Patch Digest: {:016x}\n\
             +# Status: {}\n",
            spec.pr_title,
            spec.pr_branch,
            spec.roles.len(),
            spec.stages.len(),
            spec.status_levels.len(),
            spec.patch_digest,
            spec.status
        )
    }

    /// Verify 10-point definition of "Task Done" checklist.
    pub fn verify_definition_of_done(&self, evidence_points: usize) -> bool {
        evidence_points >= 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distro_agent_roles() {
        let engine = RepeatableAgentOperatingLoopPrEngine::new();
        let roles = engine.get_roles();
        assert_eq!(roles.len(), 7);
        assert_eq!(roles[0].code(), "steward");
        assert_eq!(roles[1].code(), "sentinel");
        assert_eq!(roles[2].code(), "boot-validator");
    }

    #[test]
    fn test_operating_loop_stages() {
        let engine = RepeatableAgentOperatingLoopPrEngine::new();
        let stages = engine.get_stages();
        assert_eq!(stages.len(), 9);
        assert_eq!(stages[0].step_number(), 1);
        assert_eq!(stages[8].step_number(), 9);
    }

    #[test]
    fn test_status_levels_and_permission_matrix() {
        let engine = RepeatableAgentOperatingLoopPrEngine::new();
        let status_levels = engine.get_status_levels();
        let matrix = engine.get_permission_matrix();

        assert_eq!(status_levels.len(), 6);
        assert_eq!(status_levels[3].as_str(), "qemu-verified");
        assert_eq!(matrix.len(), 7);
    }

    #[test]
    fn test_proposal_generation_and_patch() {
        let engine = RepeatableAgentOperatingLoopPrEngine::new();
        let spec = engine.generate_proposal_spec();
        assert_eq!(spec.pr_branch, "feat/agent-operating-loop");
        assert_ne!(spec.patch_digest, 0);

        let patch = engine.generate_patch_diff();
        assert!(patch.contains("feat/agent-operating-loop"));
        assert!(patch.contains("Patch Digest:"));
    }

    #[test]
    fn test_definition_of_done_verification() {
        let engine = RepeatableAgentOperatingLoopPrEngine::new();
        assert!(engine.verify_definition_of_done(10));
        assert!(!engine.verify_definition_of_done(9));
    }
}
