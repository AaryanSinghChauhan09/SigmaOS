//! Distro-Maintainer Operating Model PR Gateway Engine
//! (`src/distro/distro_maintainer_operating_model_pr.rs`)
//!
//! `#![no_std]` compliant Rust engine implementing the proposal, governance rules, task schema,
//! agent roles, and verification suite for SigmaOS's distro-maintainer operating model.

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// Release channel classification for SigmaOS builds and packages
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

    pub fn requirements(&self) -> &'static str {
        match self {
            Self::Dev => "Build and unit tests pass on feature branch",
            Self::Nightly => "Build, unit tests, and QEMU x86_64 smoke test pass",
            Self::Validated => "Full boot matrix pass, installer test, known-issue list, rollback instructions",
            Self::Experimental => "New subsystem or compatibility feature, clearly labeled, not for daily use",
        }
    }
}

/// Agent roles in the distro-maintainer operating model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentRole {
    RepositorySteward,
    BuildAndTestSentinel,
    BootAndRuntimeValidator,
    CapabilityAuditor,
    KernelSubsystemAgent,
    PackageAndDistroCompatAgent,
    DocumentationAndOnboardingAgent,
}

impl AgentRole {
    pub fn title(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "Repository Steward",
            Self::BuildAndTestSentinel => "Build and Test Sentinel",
            Self::BootAndRuntimeValidator => "Boot and Runtime Validator",
            Self::CapabilityAuditor => "Capability Auditor",
            Self::KernelSubsystemAgent => "Kernel and Subsystem Agent",
            Self::PackageAndDistroCompatAgent => "Package and Distro Compatibility Agent",
            Self::DocumentationAndOnboardingAgent => "Documentation and Onboarding Agent",
        }
    }

    pub fn scope(&self) -> &'static str {
        match self {
            Self::RepositorySteward => "Repository cleanliness, issues/PRs, conventional commits, stale work audit",
            Self::BuildAndTestSentinel => "Build workflow, clean cargo/make test execution, flaky test classification",
            Self::BootAndRuntimeValidator => "Firmware-to-desktop QEMU boot matrix, serial logs, boot-to-shell contract",
            Self::CapabilityAuditor => "Preventing aspirational claims, capability truth table, evidence validation",
            Self::KernelSubsystemAgent => "Subsystem boundary ownership, invariant enforcement, unit & trace evidence",
            Self::PackageAndDistroCompatAgent => "Package metadata schemas, dry-run dependency graphs, isolated sandboxing",
            Self::DocumentationAndOnboardingAgent => "README command accuracy, 'What works today' updates, quickstart guide",
        }
    }
}

/// The 10 mandatory rules for agent operations in SigmaOS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MandatoryOperatingRule {
    Rule1EvidenceBeforeStatus,
    Rule2OneAgentOneBoundary,
    Rule3SmallChangesOnly,
    Rule4ReproducibilityMandatory,
    Rule5HumanApprovalHighImpact,
    Rule6SandboxUntrustedExecution,
    Rule7NoSilentFailure,
    Rule8ClaimsMustBeVersioned,
    Rule9SecurityChangesSeparateReview,
    Rule10PreferRuntimeProofOverModelProof,
}

impl MandatoryOperatingRule {
    pub fn number(&self) -> u8 {
        match self {
            Self::Rule1EvidenceBeforeStatus => 1,
            Self::Rule2OneAgentOneBoundary => 2,
            Self::Rule3SmallChangesOnly => 3,
            Self::Rule4ReproducibilityMandatory => 4,
            Self::Rule5HumanApprovalHighImpact => 5,
            Self::Rule6SandboxUntrustedExecution => 6,
            Self::Rule7NoSilentFailure => 7,
            Self::Rule8ClaimsMustBeVersioned => 8,
            Self::Rule9SecurityChangesSeparateReview => 9,
            Self::Rule10PreferRuntimeProofOverModelProof => 10,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Rule1EvidenceBeforeStatus => "Evidence before status",
            Self::Rule2OneAgentOneBoundary => "One agent, one boundary",
            Self::Rule3SmallChangesOnly => "Small changes only",
            Self::Rule4ReproducibilityMandatory => "Reproducibility is mandatory",
            Self::Rule5HumanApprovalHighImpact => "Human approval for high-impact actions",
            Self::Rule6SandboxUntrustedExecution => "Sandbox untrusted execution",
            Self::Rule7NoSilentFailure => "No silent failure",
            Self::Rule8ClaimsMustBeVersioned => "Claims must be versioned",
            Self::Rule9SecurityChangesSeparateReview => "Security changes get separate review",
            Self::Rule10PreferRuntimeProofOverModelProof => "Prefer runtime proof over model proof",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Rule1EvidenceBeforeStatus => "No status update to working/implemented without passing test, QEMU boot, or hardware log evidence.",
            Self::Rule2OneAgentOneBoundary => "Agents modify only declared owned paths. Cross-boundary changes require integration review.",
            Self::Rule3SmallChangesOnly => "PRs address one concern. Large reformatting or multi-subsystem rewrites are forbidden.",
            Self::Rule4ReproducibilityMandatory => "Every report records exact commands, git commit hash, toolchain version, OS image, and execution duration.",
            Self::Rule5HumanApprovalHighImpact => "Security boundaries, bootloader, installer, package execution, and release channel changes require human approval.",
            Self::Rule6SandboxUntrustedExecution => "Untrusted packages, models, or scripts run in isolated sandboxes with restricted VFS/network access.",
            Self::Rule7NoSilentFailure => "Uncompleted tasks generate a blocked task file containing step, log excerpt, and reproduction steps.",
            Self::Rule8ClaimsMustBeVersioned => "Capability claims include last-verified date and link; older claims downgrade to needs-revalidation.",
            Self::Rule9SecurityChangesSeparateReview => "Capability, privilege, crypto, and sandbox changes require dedicated human security review.",
            Self::Rule10PreferRuntimeProofOverModelProof => "When both exist, runtime boot/QEMU proof controls public status over unit test model proof.",
        }
    }
}

/// Declarative Task Specification structure matching `.agents/tasks/*.toml`
#[derive(Debug, Clone)]
pub struct TaskSpecification {
    pub id: String,
    pub title: String,
    pub agent: String,
    pub priority: String,
    pub repeat: String,
    pub scope: Vec<String>,
    pub steps: Vec<String>,
    pub evidence: Vec<String>,
    pub success: Vec<String>,
    pub on_failure: String,
}

impl TaskSpecification {
    pub fn to_toml(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("id = \"{}\"\n", self.id));
        out.push_str(&format!("title = \"{}\"\n", self.title));
        out.push_str(&format!("agent = \"{}\"\n", self.agent));
        out.push_str(&format!("priority = \"{}\"\n", self.priority));
        out.push_str(&format!("repeat = \"{}\"\n", self.repeat));

        out.push_str("scope = [\n");
        for s in &self.scope {
            out.push_str(&format!("  \"{}\",\n", s));
        }
        out.push_str("]\n\n");

        out.push_str("steps = [\n");
        for st in &self.steps {
            out.push_str(&format!("  \"{}\",\n", st));
        }
        out.push_str("]\n\n");

        out.push_str("evidence = [\n");
        for ev in &self.evidence {
            out.push_str(&format!("  \"{}\",\n", ev));
        }
        out.push_str("]\n\n");

        out.push_str("success = [\n");
        for sc in &self.success {
            out.push_str(&format!("  \"{}\",\n", sc));
        }
        out.push_str("]\n\n");

        out.push_str(&format!("on_failure = \"{}\"\n", self.on_failure));
        out
    }
}

/// Main Distro-Maintainer Operating Model PR Gateway Engine
pub struct DistroMaintainerOperatingModelPrEngine;

impl DistroMaintainerOperatingModelPrEngine {
    pub fn new() -> Self {
        Self
    }

    /// Returns the list of initial ten core tasks for the 90-day plan
    pub fn get_first_ten_tasks(&self) -> Vec<TaskSpecification> {
        vec![
            TaskSpecification {
                id: "AGENT-001".to_string(),
                title: "Define agent registry and ownership boundaries".to_string(),
                agent: "repository-steward".to_string(),
                priority: "P0".to_string(),
                repeat: "once".to_string(),
                scope: vec![".agents/registry.toml".to_string()],
                steps: vec![
                    "Define agent ownership paths".to_string(),
                    "Set tool permission boundaries".to_string(),
                    "Verify registry TOML syntax".to_string(),
                ],
                evidence: vec![".agents/registry.toml".to_string()],
                success: vec!["registry TOML parses cleanly".to_string()],
                on_failure: "Create blocked task with registry syntax error".to_string(),
            },
            TaskSpecification {
                id: "POL-002".to_string(),
                title: "Establish mandatory test evidence policy".to_string(),
                agent: "capability-auditor".to_string(),
                priority: "P0".to_string(),
                repeat: "once".to_string(),
                scope: vec![".agents/policies/test-evidence.md".to_string()],
                steps: vec![
                    "Document required evidence types".to_string(),
                    "Integrate policy into PR checklist".to_string(),
                ],
                evidence: vec![".agents/policies/test-evidence.md".to_string()],
                success: vec!["policy document created".to_string()],
                on_failure: "Create blocked task with missing policy items".to_string(),
            },
            TaskSpecification {
                id: "CLI-003".to_string(),
                title: "Build agent CLI runner for automated task execution".to_string(),
                agent: "build-test-sentinel".to_string(),
                priority: "P0".to_string(),
                repeat: "weekly".to_string(),
                scope: vec!["tools/agent_cli.py".to_string()],
                steps: vec![
                    "Parse task TOML specifications".to_string(),
                    "Execute runbook checks".to_string(),
                    "Write structured evidence reports".to_string(),
                ],
                evidence: vec!["tools/agent_cli.py".to_string()],
                success: vec!["python tools/agent_cli.py --list-tasks succeeds".to_string()],
                on_failure: "Create blocked task with CLI execution log".to_string(),
            },
            TaskSpecification {
                id: "CI-004".to_string(),
                title: "Enforce PR fast checks CI workflow".to_string(),
                agent: "build-test-sentinel".to_string(),
                priority: "P0".to_string(),
                repeat: "per-pr".to_string(),
                scope: vec![".github/workflows/verify.yml".to_string()],
                steps: vec![
                    "Run cargo check --lib".to_string(),
                    "Run cargo test --lib".to_string(),
                    "Run ./run_sigma_tests.sh".to_string(),
                ],
                evidence: vec!["reports/test/test_manifest.json".to_string()],
                success: vec!["CI workflow exits 0 on clean code".to_string()],
                on_failure: "Create blocked task with CI log".to_string(),
            },
            TaskSpecification {
                id: "BOOT-X86-001".to_string(),
                title: "Validate QEMU x86_64 boot-to-shell contract".to_string(),
                agent: "boot-runtime-validator".to_string(),
                priority: "P0".to_string(),
                repeat: "daily".to_string(),
                scope: vec!["installer/".to_string(), "src/kernel/".to_string()],
                steps: vec![
                    "Build ISO/boot artifact".to_string(),
                    "Launch QEMU x86_64 with serial logging".to_string(),
                    "Capture serial output and verify shell prompt".to_string(),
                ],
                evidence: vec![
                    "reports/boot/qemu_x86_64.log".to_string(),
                    "reports/boot/qemu_x86_64.json".to_string(),
                ],
                success: vec![
                    "QEMU process completes or reaches shell within timeout".to_string(),
                    "no kernel panic in serial log".to_string(),
                ],
                on_failure: "Create blocked task with serial log excerpt".to_string(),
            },
            TaskSpecification {
                id: "AUDIT-001".to_string(),
                title: "Audit capability claims against test and source evidence".to_string(),
                agent: "capability-auditor".to_string(),
                priority: "P0".to_string(),
                repeat: "daily".to_string(),
                scope: vec!["CAPABILITY_MATRIX.toml".to_string(), "FEATURE_STATUS.toml".to_string()],
                steps: vec![
                    "Parse capability entries".to_string(),
                    "Verify source and test references".to_string(),
                    "Downgrade unevidenced claims to model-only".to_string(),
                ],
                evidence: vec!["reports/capability/truth_table.json".to_string()],
                success: vec!["truth table generated with zero false claims".to_string()],
                on_failure: "Create blocked task with audit mismatch details".to_string(),
            },
            TaskSpecification {
                id: "DASH-007".to_string(),
                title: "Generate 'Verified Today' status dashboard".to_string(),
                agent: "documentation-agent".to_string(),
                priority: "P1".to_string(),
                repeat: "daily".to_string(),
                scope: vec!["reports/capability/verified_today.md".to_string()],
                steps: vec![
                    "Read truth_table.json".to_string(),
                    "Format Markdown summary table".to_string(),
                ],
                evidence: vec!["reports/capability/verified_today.md".to_string()],
                success: vec!["verified_today.md matches evidence json".to_string()],
                on_failure: "Create blocked task with dashboard generation log".to_string(),
            },
            TaskSpecification {
                id: "ROADMAP-008".to_string(),
                title: "Convert roadmap specifications to executable tasks".to_string(),
                agent: "repository-steward".to_string(),
                priority: "P1".to_string(),
                repeat: "weekly".to_string(),
                scope: vec![".agents/tasks/open/".to_string()],
                steps: vec![
                    "Extract milestones from roadmap proposals".to_string(),
                    "Generate task TOML specifications".to_string(),
                ],
                evidence: vec![".agents/tasks/open/".to_string()],
                success: vec!["tasks created with valid schema".to_string()],
                on_failure: "Create blocked task with unparsed spec".to_string(),
            },
            TaskSpecification {
                id: "PKG-001".to_string(),
                title: "Validate package metadata parsing corpus and dry-run policy".to_string(),
                agent: "package-compat-agent".to_string(),
                priority: "P1".to_string(),
                repeat: "weekly".to_string(),
                scope: vec!["src/package/".to_string()],
                steps: vec![
                    "Parse .deb, .rpm, Arch desc, and BSD pkg-plist manifests".to_string(),
                    "Construct dependency graph without binary execution".to_string(),
                ],
                evidence: vec!["reports/package/compatibility_matrix.json".to_string()],
                success: vec!["100% manifest parse success on fixed corpus".to_string()],
                on_failure: "Create blocked task with malformed manifest log".to_string(),
            },
            TaskSpecification {
                id: "NIGHTLY-001".to_string(),
                title: "Assemble and verify nightly release channel artifact".to_string(),
                agent: "repository-steward".to_string(),
                priority: "P1".to_string(),
                repeat: "daily".to_string(),
                scope: vec!["build/".to_string()],
                steps: vec![
                    "Verify clean cargo check & test pass".to_string(),
                    "Verify QEMU x86_64 boot pass".to_string(),
                    "Generate release manifest and rollback notes".to_string(),
                ],
                evidence: vec!["reports/build/nightly_release.json".to_string()],
                success: vec!["nightly artifact hash recorded and verified".to_string()],
                on_failure: "Create blocked task with release blocker stage".to_string(),
            },
        ]
    }

    /// Validates high-impact action approval requirement
    pub fn requires_human_approval(&self, action: &str) -> bool {
        let high_impact_keywords = [
            "kernel_security",
            "bootloader",
            "installer",
            "update_mechanism",
            "execute_package_script",
            "download_external_binary",
            "change_release_channel",
            "delete_subsystem",
            "force_push",
        ];

        for kw in &high_impact_keywords {
            if action.contains(kw) {
                return true;
            }
        }
        false
    }

    /// Generates full PR proposal documentation string for the operating model
    pub fn generate_pr_proposal(&self) -> String {
        let mut pr = String::new();
        pr.push_str("# PR Proposal: Distro-Maintainer Operating Model for SigmaOS\n\n");
        pr.push_str("## Summary\n\n");
        pr.push_str("Establishes a structured distro-maintainer operating model for SigmaOS, converting\n");
        pr.push_str("autonomous agents into scoped package/subsystem maintainers that produce verifiable evidence,\n");
        pr.push_str("enforce human approval for high-impact changes, and adhere to a 90-day bootable reality roadmap.\n\n");

        pr.push_str("## 1. Open-Source OS Practice to Agent Mapping\n\n");
        pr.push_str("| Open-Source OS Practice | SigmaOS Agent Practice |\n");
        pr.push_str("|---|---|\n");
        pr.push_str("| Package maintainer owns a package | One agent owns one subsystem or module boundary |\n");
        pr.push_str("| Debian/Arch release process | Changes enter through PRs with test evidence |\n");
        pr.push_str("| ArchWiki reproducibility | Every recurring task has a machine-readable runbook |\n");
        pr.push_str("| FreeBSD/OpenBSD security discipline | Least privilege, sandboxing, audit trails |\n");
        pr.push_str("| ISO build and smoke testing | Every integration change must pass a boot/runtime gate |\n");
        pr.push_str("| Stable, testing, unstable channels | verified, experimental, and blocked states |\n");
        pr.push_str("| Regression tracking | Every failed test or boot becomes a tracked task |\n");
        pr.push_str("| Reproducible builds | Fixed toolchain, lockfiles, recorded commands and hashes |\n\n");

        pr.push_str("## 2. Mandatory Rules\n\n");
        for rule in [
            MandatoryOperatingRule::Rule1EvidenceBeforeStatus,
            MandatoryOperatingRule::Rule2OneAgentOneBoundary,
            MandatoryOperatingRule::Rule3SmallChangesOnly,
            MandatoryOperatingRule::Rule4ReproducibilityMandatory,
            MandatoryOperatingRule::Rule5HumanApprovalHighImpact,
            MandatoryOperatingRule::Rule6SandboxUntrustedExecution,
            MandatoryOperatingRule::Rule7NoSilentFailure,
            MandatoryOperatingRule::Rule8ClaimsMustBeVersioned,
            MandatoryOperatingRule::Rule9SecurityChangesSeparateReview,
            MandatoryOperatingRule::Rule10PreferRuntimeProofOverModelProof,
        ] {
            pr.push_str(&format!("- **Rule {}: {}**: {}\n", rule.number(), rule.title(), rule.description()));
        }

        pr.push_str("\n## 3. Release Channel Governance\n\n");
        for channel in [
            ReleaseChannel::Dev,
            ReleaseChannel::Nightly,
            ReleaseChannel::Validated,
            ReleaseChannel::Experimental,
        ] {
            pr.push_str(&format!("- **{}**: {}\n", channel.name(), channel.requirements()));
        }

        pr.push_str("\n## 4. First Ten Tasks\n\n");
        for task in self.get_first_ten_tasks() {
            pr.push_str(&format!("### [{}] {}\n", task.id, task.title));
            pr.push_str(&format!("- **Agent**: `{}` | **Priority**: `{}`\n", task.agent, task.priority));
            pr.push_str(&format!("- **On Failure**: {}\n\n", task.on_failure));
        }

        pr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distro_maintainer_engine_proposal() {
        let engine = DistroMaintainerOperatingModelPrEngine::new();
        let proposal = engine.generate_pr_proposal();
        assert!(proposal.contains("PR Proposal: Distro-Maintainer Operating Model for SigmaOS"));
        assert!(proposal.contains("Mandatory Rules"));
    }

    #[test]
    fn test_first_ten_tasks() {
        let engine = DistroMaintainerOperatingModelPrEngine::new();
        let tasks = engine.get_first_ten_tasks();
        assert_eq!(tasks.len(), 10);
        assert_eq!(tasks[0].id, "AGENT-001");
    }

    #[test]
    fn test_human_approval() {
        let engine = DistroMaintainerOperatingModelPrEngine::new();
        assert!(engine.requires_human_approval("kernel_security_policy"));
        assert!(!engine.requires_human_approval("run_unit_tests"));
    }
}
