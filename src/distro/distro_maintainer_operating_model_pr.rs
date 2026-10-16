//! Distro-Maintainer Operating Model PR Proposal Engine
//! (`src/distro/distro_maintainer_operating_model_pr.rs`)
//!
//! Zero-dependency, `#![no_std]` compliant Rust engine implementing the proposal, verification matrix,
//! and patch generation for SigmaOS's distro-maintainer operating model:
//! - 7 Dedicated Agent Roles.
//! - 9-Step Task Lifecycle.
//! - 10 Mandatory Operating Rules.
//! - 4 Release Channels (`dev`, `nightly`, `validated`, `experimental`).
//! - Contract Directories (`.agents/`, `reports/`, `tools/`).
//! - First 10 Initial Backlog Tasks.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// 7 Agent Roles in the Distro-Maintainer Operating Model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentRole {
    RepositorySteward,
    BuildAndTestSentinel,
    BootAndRuntimeValidator,
    CapabilityAuditor,
    KernelAndSubsystemAgents,
    PackageAndDistroCompatibilityAgent,
    DocumentationAndOnboardingAgent,
}

impl AgentRole {
    pub fn name(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "Repository Steward",
            Self::BuildAndTestSentinel => "Build and Test Sentinel",
            Self::BootAndRuntimeValidator => "Boot and Runtime Validator",
            Self::CapabilityAuditor => "Capability Auditor",
            Self::KernelAndSubsystemAgents => "Kernel and Subsystem Agents",
            Self::PackageAndDistroCompatibilityAgent => "Package and Distro Compatibility Agent",
            Self::DocumentationAndOnboardingAgent => "Documentation and Onboarding Agent",
        }
    }

    pub fn purpose(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "Maintain repository hygiene, coherence, and review readiness.",
            Self::BuildAndTestSentinel => "Make existing build and test workflows trustworthy.",
            Self::BootAndRuntimeValidator => "Prove the firmware-to-desktop path in QEMU and target environments.",
            Self::CapabilityAuditor => "Prevent aspirational completion and enforce evidence-gated claims.",
            Self::KernelAndSubsystemAgents => "Maintain ownership boundaries and evidence for core kernel/subsystems.",
            Self::PackageAndDistroCompatibilityAgent => "Maintain package metadata compatibility and sandboxed execution.",
            Self::DocumentationAndOnboardingAgent => "Ensure contributor and agent onboarding usability.",
        }
    }

    pub fn primary_deliverable(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "reports/steward/daily-status.md",
            Self::BuildAndTestSentinel => "reports/build/latest.json",
            Self::BootAndRuntimeValidator => "reports/boot/qemu_x86_64.json",
            Self::CapabilityAuditor => "reports/capability/truth_table.json",
            Self::KernelAndSubsystemAgents => "reports/test/subsystem_traces.log",
            Self::PackageAndDistroCompatibilityAgent => "reports/package/compatibility_matrix.json",
            Self::DocumentationAndOnboardingAgent => "reports/capability/verified_today.md",
        }
    }
}

/// 9-Step Standard Task Lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskLifecycleStage {
    Select = 1,
    Plan = 2,
    Isolate = 3,
    Execute = 4,
    Verify = 5,
    Record = 6,
    UpdateStatus = 7,
    Review = 8,
    FollowUp = 9,
}

impl TaskLifecycleStage {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Select => "SELECT",
            Self::Plan => "PLAN",
            Self::Isolate => "ISOLATE",
            Self::Execute => "EXECUTE",
            Self::Verify => "VERIFY",
            Self::Record => "RECORD",
            Self::UpdateStatus => "UPDATE STATUS",
            Self::Review => "REVIEW",
            Self::FollowUp => "FOLLOW UP",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Select => "Read agent ownership file, capability matrix, and task queue.",
            Self::Plan => "Identify affected paths, tests, risks, and rollback method.",
            Self::Isolate => "Create a branch, workspace, or container.",
            Self::Execute => "Make the smallest possible change.",
            Self::Verify => "Run scoped tests, repository checks, and runtime tests.",
            Self::Record => "Write logs, metrics, hashes, and a structured report.",
            Self::UpdateStatus => "Change capability status only if evidence supports it.",
            Self::Review => "Submit PR; human review is mandatory for high-impact changes.",
            Self::FollowUp => "If verification fails, create a blocked task with exact failing stage and log excerpt.",
        }
    }
}

/// 10 Mandatory Operating Rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MandatoryOperatingRule {
    Rule1EvidenceBeforeStatus,
    Rule2OneAgentOneBoundary,
    Rule3SmallChangesOnly,
    Rule4ReproducibilityMandatory,
    Rule5HumanApprovalForHighImpact,
    Rule6SandboxUntrustedExecution,
    Rule7NoSilentFailure,
    Rule8ClaimsMustBeVersioned,
    Rule9SecurityChangesGetSeparateReview,
    Rule10PreferRuntimeProofOverModelProof,
}

impl MandatoryOperatingRule {
    pub fn id(&self) -> u8 {
        match self {
            Self::Rule1EvidenceBeforeStatus => 1,
            Self::Rule2OneAgentOneBoundary => 2,
            Self::Rule3SmallChangesOnly => 3,
            Self::Rule4ReproducibilityMandatory => 4,
            Self::Rule5HumanApprovalForHighImpact => 5,
            Self::Rule6SandboxUntrustedExecution => 6,
            Self::Rule7NoSilentFailure => 7,
            Self::Rule8ClaimsMustBeVersioned => 8,
            Self::Rule9SecurityChangesGetSeparateReview => 9,
            Self::Rule10PreferRuntimeProofOverModelProof => 10,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Rule1EvidenceBeforeStatus => "Evidence before status",
            Self::Rule2OneAgentOneBoundary => "One agent, one boundary",
            Self::Rule3SmallChangesOnly => "Small changes only",
            Self::Rule4ReproducibilityMandatory => "Reproducibility is mandatory",
            Self::Rule5HumanApprovalForHighImpact => "Human approval for high-impact actions",
            Self::Rule6SandboxUntrustedExecution => "Sandbox untrusted execution",
            Self::Rule7NoSilentFailure => "No silent failure",
            Self::Rule8ClaimsMustBeVersioned => "Claims must be versioned",
            Self::Rule9SecurityChangesGetSeparateReview => "Security changes get separate review",
            Self::Rule10PreferRuntimeProofOverModelProof => "Prefer runtime proof over model proof",
        }
    }
}

/// 4 Distro Release Channels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseChannel {
    Dev,
    Nightly,
    Validated,
    Experimental,
}

impl ReleaseChannel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Nightly => "nightly",
            Self::Validated => "validated",
            Self::Experimental => "experimental",
        }
    }

    pub fn requirement(&self) -> &'static str {
        match self {
            Self::Dev => "Build and unit tests pass.",
            Self::Nightly => "Build, unit tests, and QEMU smoke test pass.",
            Self::Validated => "Full boot matrix, installer test, known-issue list, and rollback instructions.",
            Self::Experimental => "Clearly labeled experimental subsystem, not suitable for daily use.",
        }
    }
}

/// Declarative Task Specification Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFileSpec {
    pub id: &'static str,
    pub title: &'static str,
    pub agent: &'static str,
    pub priority: &'static str,
    pub repeat: &'static str,
}

/// PR Proposal Spec Descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroMaintainerPrProposalSpec {
    pub pr_title: String,
    pub pr_branch: String,
    pub roles: Vec<AgentRole>,
    pub stages: Vec<TaskLifecycleStage>,
    pub rules: Vec<MandatoryOperatingRule>,
    pub channels: Vec<ReleaseChannel>,
    pub initial_tasks: Vec<TaskFileSpec>,
    pub patch_digest: u64,
    pub status: String,
}

/// FNV-1a checksum calculation for PR patch content verification
pub fn fnv1a_maintainer_digest(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Master PR Proposal Gateway Engine for Distro-Maintainer Operating Model
#[derive(Debug, Clone, Default)]
pub struct DistroMaintainerPrEngine;

impl DistroMaintainerPrEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn get_agent_roles(&self) -> Vec<AgentRole> {
        alloc::vec![
            AgentRole::RepositorySteward,
            AgentRole::BuildAndTestSentinel,
            AgentRole::BootAndRuntimeValidator,
            AgentRole::CapabilityAuditor,
            AgentRole::KernelAndSubsystemAgents,
            AgentRole::PackageAndDistroCompatibilityAgent,
            AgentRole::DocumentationAndOnboardingAgent,
        ]
    }

    pub fn get_lifecycle_stages(&self) -> Vec<TaskLifecycleStage> {
        alloc::vec![
            TaskLifecycleStage::Select,
            TaskLifecycleStage::Plan,
            TaskLifecycleStage::Isolate,
            TaskLifecycleStage::Execute,
            TaskLifecycleStage::Verify,
            TaskLifecycleStage::Record,
            TaskLifecycleStage::UpdateStatus,
            TaskLifecycleStage::Review,
            TaskLifecycleStage::FollowUp,
        ]
    }

    pub fn get_operating_rules(&self) -> Vec<MandatoryOperatingRule> {
        alloc::vec![
            MandatoryOperatingRule::Rule1EvidenceBeforeStatus,
            MandatoryOperatingRule::Rule2OneAgentOneBoundary,
            MandatoryOperatingRule::Rule3SmallChangesOnly,
            MandatoryOperatingRule::Rule4ReproducibilityMandatory,
            MandatoryOperatingRule::Rule5HumanApprovalForHighImpact,
            MandatoryOperatingRule::Rule6SandboxUntrustedExecution,
            MandatoryOperatingRule::Rule7NoSilentFailure,
            MandatoryOperatingRule::Rule8ClaimsMustBeVersioned,
            MandatoryOperatingRule::Rule9SecurityChangesGetSeparateReview,
            MandatoryOperatingRule::Rule10PreferRuntimeProofOverModelProof,
        ]
    }

    pub fn get_release_channels(&self) -> Vec<ReleaseChannel> {
        alloc::vec![
            ReleaseChannel::Dev,
            ReleaseChannel::Nightly,
            ReleaseChannel::Validated,
            ReleaseChannel::Experimental,
        ]
    }

    pub fn get_first_ten_tasks(&self) -> Vec<TaskFileSpec> {
        alloc::vec![
            TaskFileSpec { id: "REG-001", title: "Create .agents/registry.toml defining agents and scopes", agent: "repository-steward", priority: "P0", repeat: "once" },
            TaskFileSpec { id: "POL-001", title: "Create .agents/policies/test-evidence.md policy", agent: "capability-auditor", priority: "P0", repeat: "once" },
            TaskFileSpec { id: "CLI-001", title: "Build tools/agent_cli.py task runner", agent: "build-test-sentinel", priority: "P0", repeat: "once" },
            TaskFileSpec { id: "CI-001", title: "Add CI workflow for documented check/test commands", agent: "build-test-sentinel", priority: "P0", repeat: "once" },
            TaskFileSpec { id: "BOOT-X86-001", title: "Implement QEMU x86_64 boot-to-shell validator", agent: "boot-runtime-validator", priority: "P0", repeat: "daily" },
            TaskFileSpec { id: "AUDIT-001", title: "Audit every working claim in capability files", agent: "capability-auditor", priority: "P0", repeat: "weekly" },
            TaskFileSpec { id: "DASH-001", title: "Generate Verified Today dashboard from test/boot evidence", agent: "documentation-onboarding-agent", priority: "P1", repeat: "daily" },
            TaskFileSpec { id: "TASK-001", title: "Convert five largest roadmap docs into task files", agent: "repository-steward", priority: "P1", repeat: "once" },
            TaskFileSpec { id: "PKG-001", title: "Add package metadata compatibility test corpus and dry-run policy", agent: "package-distro-compatibility-agent", priority: "P1", repeat: "weekly" },
            TaskFileSpec { id: "NIGHTLY-001", title: "Define first nightly artifact requirements", agent: "boot-runtime-validator", priority: "P0", repeat: "daily" },
        ]
    }

    pub fn generate_proposal_spec(&self) -> DistroMaintainerPrProposalSpec {
        let roles = self.get_agent_roles();
        let stages = self.get_lifecycle_stages();
        let rules = self.get_operating_rules();
        let channels = self.get_release_channels();
        let initial_tasks = self.get_first_ten_tasks();

        let mut payload = String::from("PR: feat(distro): Adopt Distro-Maintainer Operating Model\n");
        for r in &roles {
            payload.push_str(r.name());
            payload.push('\n');
        }
        for s in &stages {
            payload.push_str(s.name());
            payload.push('\n');
        }
        for ru in &rules {
            payload.push_str(ru.title());
            payload.push('\n');
        }

        let patch_digest = fnv1a_maintainer_digest(payload.as_bytes());

        DistroMaintainerPrProposalSpec {
            pr_title: String::from("feat(distro): Adopt Distro-Maintainer Operating Model and Evidence-Gated Agent Framework"),
            pr_branch: String::from("feature/distro-maintainer-operating-model"),
            roles,
            stages,
            rules,
            channels,
            initial_tasks,
            patch_digest,
            status: String::from("Proposed / Specification Ready"),
        }
    }

    pub fn generate_patch_diff(&self) -> String {
        let spec = self.generate_proposal_spec();
        format!(
            "--- a/docs/roadmap/PR_SPEC_DISTRO_MAINTAINER_OPERATING_MODEL.md\n\
             +++ b/docs/roadmap/PR_SPEC_DISTRO_MAINTAINER_OPERATING_MODEL.md\n\
             @@ -0,0 +1,100 @@\n\
             +# PR Title: {}\n\
             +# PR Branch: {}\n\
             +# Agent Roles Count: {}\n\
             +# Lifecycle Stages Count: {}\n\
             +# Rules Count: {}\n\
             +# Release Channels Count: {}\n\
             +# Initial Backlog Tasks Count: {}\n\
             +# Patch Digest: {:016x}\n\
             +# Status: {}\n",
            spec.pr_title,
            spec.pr_branch,
            spec.roles.len(),
            spec.stages.len(),
            spec.rules.len(),
            spec.channels.len(),
            spec.initial_tasks.len(),
            spec.patch_digest,
            spec.status
        )
    }

    pub fn verify_operating_rules_satisfied(&self, rules_satisfied: usize) -> bool {
        rules_satisfied >= 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_roles_and_deliverables() {
        let engine = DistroMaintainerPrEngine::new();
        let roles = engine.get_agent_roles();
        assert_eq!(roles.len(), 7);
        assert_eq!(roles[0].name(), "Repository Steward");
        assert_eq!(roles[1].primary_deliverable(), "reports/build/latest.json");
    }

    #[test]
    fn test_task_lifecycle_stages() {
        let engine = DistroMaintainerPrEngine::new();
        let stages = engine.get_lifecycle_stages();
        assert_eq!(stages.len(), 9);
        assert_eq!(stages[0].name(), "SELECT");
        assert_eq!(stages[8].name(), "FOLLOW UP");
    }

    #[test]
    fn test_mandatory_operating_rules() {
        let engine = DistroMaintainerPrEngine::new();
        let rules = engine.get_operating_rules();
        assert_eq!(rules.len(), 10);
        assert_eq!(rules[0].id(), 1);
        assert_eq!(rules[9].id(), 10);
    }

    #[test]
    fn test_release_channels() {
        let engine = DistroMaintainerPrEngine::new();
        let channels = engine.get_release_channels();
        assert_eq!(channels.len(), 4);
        assert_eq!(channels[0].name(), "dev");
        assert_eq!(channels[1].name(), "nightly");
    }

    #[test]
    fn test_first_ten_tasks() {
        let engine = DistroMaintainerPrEngine::new();
        let tasks = engine.get_first_ten_tasks();
        assert_eq!(tasks.len(), 10);
        assert_eq!(tasks[0].id, "REG-001");
        assert_eq!(tasks[4].id, "BOOT-X86-001");
    }

    #[test]
    fn test_proposal_spec_and_patch_diff() {
        let engine = DistroMaintainerPrEngine::new();
        let spec = engine.generate_proposal_spec();
        assert_eq!(spec.pr_branch, "feature/distro-maintainer-operating-model");
        assert_ne!(spec.patch_digest, 0);

        let patch = engine.generate_patch_diff();
        assert!(patch.contains("feature/distro-maintainer-operating-model"));
        assert!(patch.contains("Patch Digest:"));
    }

    #[test]
    fn test_operating_rules_satisfaction() {
        let engine = DistroMaintainerPrEngine::new();
        assert!(engine.verify_operating_rules_satisfied(10));
        assert!(!engine.verify_operating_rules_satisfied(9));
    }
}
