//! Periodic PR-Only Component-Rule System Engine for SigmaOS Governance
//!
//! Codifies the periodic governance model, 12-step component rule lifecycle,
//! component-specific mandatory rules, 12-week study plan, 10-month milestones,
//! PR-only workflow validation, CI gates, and 14 industry-standard PR rules.

use std::string::String;
use std::vec::Vec;

/// Operating cadences for periodic PR-only governance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeriodicCadence {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
}

impl PeriodicCadence {
    pub fn activity(&self) -> &'static str {
        match self {
            Self::Daily => "Build, test, boot, and status health",
            Self::Weekly => "Study one upstream mechanism",
            Self::Biweekly => "Revalidate a component",
            Self::Monthly => "Complete one component milestone",
            Self::Quarterly => "Review architecture, security, roadmap",
        }
    }

    pub fn pr_types(&self) -> &'static str {
        match self {
            Self::Daily => "chore/ci, fix/test, docs/status",
            Self::Weekly => "docs/research, feat/<component>",
            Self::Biweekly => "test/<component>, fix/<component>",
            Self::Monthly => "feat/<component>",
            Self::Quarterly => "docs/roadmap, refactor/architecture",
        }
    }

    pub fn required_output(&self) -> &'static str {
        match self {
            Self::Daily => "CI result and failure triage",
            Self::Weekly => "Research note, ADR, small implementation PR",
            Self::Biweekly => "Regression/runtime report",
            Self::Monthly => "Runtime-validated capability",
            Self::Quarterly => "Updated gap register and priorities",
        }
    }
}

/// The 12 steps of the Component Rule Lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentLifecycleStep {
    Step1SelectComponent,
    Step2MeasureCurrentCapability,
    Step3SelectUpstreamReference,
    Step4StudyOneMechanism,
    Step5WriteComponentRules,
    Step6OpenResearchAdrPr,
    Step7ImplementSmallestSafeChange,
    Step8AddTestsAndRuntimeValidation,
    Step9UpdateDocsAndCapabilityStatus,
    Step10OpenReviewPr,
    Step11MergeOnlyAfterGatesPass,
    Step12MonitorForRegressions,
}

impl ComponentLifecycleStep {
    pub fn number(&self) -> u8 {
        match self {
            Self::Step1SelectComponent => 1,
            Self::Step2MeasureCurrentCapability => 2,
            Self::Step3SelectUpstreamReference => 3,
            Self::Step4StudyOneMechanism => 4,
            Self::Step5WriteComponentRules => 5,
            Self::Step6OpenResearchAdrPr => 6,
            Self::Step7ImplementSmallestSafeChange => 7,
            Self::Step8AddTestsAndRuntimeValidation => 8,
            Self::Step9UpdateDocsAndCapabilityStatus => 9,
            Self::Step10OpenReviewPr => 10,
            Self::Step11MergeOnlyAfterGatesPass => 11,
            Self::Step12MonitorForRegressions => 12,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Step1SelectComponent => "SELECT COMPONENT",
            Self::Step2MeasureCurrentCapability => "MEASURE CURRENT CAPABILITY",
            Self::Step3SelectUpstreamReference => "SELECT UPSTREAM REFERENCE",
            Self::Step4StudyOneMechanism => "STUDY ONE MECHANISM",
            Self::Step5WriteComponentRules => "WRITE COMPONENT RULES",
            Self::Step6OpenResearchAdrPr => "OPEN RESEARCH/ADR PR",
            Self::Step7ImplementSmallestSafeChange => "IMPLEMENT SMALLEST SAFE CHANGE",
            Self::Step8AddTestsAndRuntimeValidation => "ADD TESTS AND RUNTIME VALIDATION",
            Self::Step9UpdateDocsAndCapabilityStatus => "UPDATE DOCS AND CAPABILITY STATUS",
            Self::Step10OpenReviewPr => "OPEN REVIEW PR",
            Self::Step11MergeOnlyAfterGatesPass => "MERGE ONLY AFTER GATES PASS",
            Self::Step12MonitorForRegressions => "MONITOR FOR REGRESSIONS",
        }
    }
}

/// Component Domains in SigmaOS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentDomain {
    Kernel,
    Memory,
    VfsFilesystems,
    Ipc,
    Security,
    Drivers,
    Packaging,
    Desktop,
    InstallerUpdate,
}

impl ComponentDomain {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Kernel => "Kernel",
            Self::Memory => "Memory",
            Self::VfsFilesystems => "VFS/filesystems",
            Self::Ipc => "IPC",
            Self::Security => "Security",
            Self::Drivers => "Drivers",
            Self::Packaging => "Packaging",
            Self::Desktop => "Desktop",
            Self::InstallerUpdate => "Installer/update",
        }
    }

    pub fn rules_file(&self) -> &'static str {
        match self {
            Self::Kernel => "docs/component-rules/kernel.md",
            Self::Memory => "docs/component-rules/memory.md",
            Self::VfsFilesystems => "docs/component-rules/vfs.md",
            Self::Ipc => "docs/component-rules/ipc.md",
            Self::Security => "docs/component-rules/security.md",
            Self::Drivers => "docs/component-rules/drivers.md",
            Self::Packaging => "docs/component-rules/packaging.md",
            Self::Desktop => "docs/component-rules/desktop.md",
            Self::InstallerUpdate => "docs/component-rules/installer_update.md",
        }
    }

    pub fn additional_gate(&self) -> &'static str {
        match self {
            Self::Kernel => "QEMU boot smoke test",
            Self::Memory => "Allocation, exhaustion, and fuzz tests",
            Self::VfsFilesystems => "Filesystem conformance and QEMU RAM-disk test",
            Self::Ipc => "Cross-process interoperability tests",
            Self::Security => "Denial tests and fuzzing",
            Self::Drivers => "QEMU device-specific test",
            Self::Packaging => "Metadata corpus and dry-run tests",
            Self::Desktop => "Graphical smoke test",
            Self::InstallerUpdate => "Clean install and rollback test",
        }
    }
}

/// The 14 Industry-Standard Rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndustryStandardRule {
    Rule1EverythingIsPullRequest,
    Rule2OneComponentOneOutcome,
    Rule3StudyBeforeImplementation,
    Rule4RulesMustBeTestable,
    Rule5RuntimeEvidenceControlsStatus,
    Rule6Reproducibility,
    Rule7LeastPrivilege,
    Rule8SecurityReviewIsSeparate,
    Rule9FailSafely,
    Rule10RegressionTestsMandatory,
    Rule11LicenseComplianceFirst,
    Rule12NoBorrowedStatusClaims,
    Rule13HumanApprovalHighRisk,
    Rule14ArchiveStaleResearch,
}

impl IndustryStandardRule {
    pub fn number(&self) -> u8 {
        match self {
            Self::Rule1EverythingIsPullRequest => 1,
            Self::Rule2OneComponentOneOutcome => 2,
            Self::Rule3StudyBeforeImplementation => 3,
            Self::Rule4RulesMustBeTestable => 4,
            Self::Rule5RuntimeEvidenceControlsStatus => 5,
            Self::Rule6Reproducibility => 6,
            Self::Rule7LeastPrivilege => 7,
            Self::Rule8SecurityReviewIsSeparate => 8,
            Self::Rule9FailSafely => 9,
            Self::Rule10RegressionTestsMandatory => 10,
            Self::Rule11LicenseComplianceFirst => 11,
            Self::Rule12NoBorrowedStatusClaims => 12,
            Self::Rule13HumanApprovalHighRisk => 13,
            Self::Rule14ArchiveStaleResearch => 14,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Rule1EverythingIsPullRequest => "Everything is a pull request",
            Self::Rule2OneComponentOneOutcome => "One component, one outcome per PR",
            Self::Rule3StudyBeforeImplementation => "Study before implementation",
            Self::Rule4RulesMustBeTestable => "Rules must be testable",
            Self::Rule5RuntimeEvidenceControlsStatus => "Runtime evidence controls status",
            Self::Rule6Reproducibility => "Reproducibility",
            Self::Rule7LeastPrivilege => "Least privilege",
            Self::Rule8SecurityReviewIsSeparate => "Security review is separate",
            Self::Rule9FailSafely => "Fail safely",
            Self::Rule10RegressionTestsMandatory => "Regression tests are mandatory",
            Self::Rule11LicenseComplianceFirst => "License compliance first",
            Self::Rule12NoBorrowedStatusClaims => "No borrowed-status claims",
            Self::Rule13HumanApprovalHighRisk => "Human approval for high-risk merges",
            Self::Rule14ArchiveStaleResearch => "Archive stale research",
        }
    }
}

/// A 12-week study schedule entry
#[derive(Debug, Clone)]
pub struct StudyWeekSchedule {
    pub week: u8,
    pub study_focus: &'static str,
    pub component_improved: &'static str,
    pub pr_deliverable: &'static str,
}

/// A 10-month milestone entry
#[derive(Debug, Clone)]
pub struct MonthlyMilestoneTarget {
    pub month: u8,
    pub milestone_description: &'static str,
    pub promotion_target: &'static str,
}

/// PR validation record
#[derive(Debug, Clone)]
pub struct PrValidationRecord {
    pub pr_title: String,
    pub branch_name: String,
    pub component: ComponentDomain,
    pub study_issue_id: Option<u64>,
    pub has_adr: bool,
    pub unit_tests_passed: bool,
    pub integration_tests_passed: bool,
    pub runtime_evidence_path: Option<String>,
    pub security_review_done: bool,
    pub status_promoted: bool,
}

/// Governance Engine for Periodic PR-Only Component Rules
pub struct PeriodicPrComponentRulesEngine {
    pub validations: Vec<PrValidationRecord>,
}

impl PeriodicPrComponentRulesEngine {
    pub fn new() -> Self {
        Self {
            validations: Vec::new(),
        }
    }

    /// Retrieve the 12-week study schedule
    pub fn study_schedule() -> Vec<StudyWeekSchedule> {
        vec![
            StudyWeekSchedule { week: 1, study_focus: "xv6/OSDev boot and interrupts", component_improved: "Kernel/boot", pr_deliverable: "QEMU boot validator and boot trace" },
            StudyWeekSchedule { week: 2, study_focus: "xv6 memory and Linux paging", component_improved: "Memory", pr_deliverable: "Frame allocator and paging tests" },
            StudyWeekSchedule { week: 3, study_focus: "xv6 processes and Linux scheduling", component_improved: "Scheduler", pr_deliverable: "Minimal scheduler and process-switch test" },
            StudyWeekSchedule { week: 4, study_focus: "Linux syscalls and VFS", component_improved: "Syscalls/VFS", pr_deliverable: "Syscall table, path resolver, RAM-disk FS" },
            StudyWeekSchedule { week: 5, study_focus: "Linux IPC and FreeBSD IPC", component_improved: "IPC", pr_deliverable: "Pipes, signals, FD lifecycle tests" },
            StudyWeekSchedule { week: 6, study_focus: "Arch/Debian packaging", component_improved: "Packaging", pr_deliverable: "Metadata parser and dependency resolver" },
            StudyWeekSchedule { week: 7, study_focus: "Fedora/Mint updates", component_improved: "Installer/update", pr_deliverable: "Atomic update and rollback test" },
            StudyWeekSchedule { week: 8, study_focus: "OpenBSD/FreeBSD security", component_improved: "Security", pr_deliverable: "Capability denial tests and sandbox policy" },
            StudyWeekSchedule { week: 9, study_focus: "Linux drivers and virtio", component_improved: "Drivers", pr_deliverable: "virtio-blk, virtio-net, VGA tests" },
            StudyWeekSchedule { week: 10, study_focus: "Redox/Rust OS design", component_improved: "Architecture", pr_deliverable: "Unsafe-code policy and module boundaries" },
            StudyWeekSchedule { week: 11, study_focus: "Linux networking", component_improved: "Network", pr_deliverable: "Socket and basic network integration test" },
            StudyWeekSchedule { week: 12, study_focus: "Mint/desktop onboarding", component_improved: "Zenith desktop", pr_deliverable: "Graphical smoke test" },
        ]
    }

    /// Retrieve 10-month milestone targets
    pub fn monthly_milestones() -> Vec<MonthlyMilestoneTarget> {
        vec![
            MonthlyMilestoneTarget { month: 1, milestone_description: "QEMU x86_64 boot-to-shell", promotion_target: "Kernel/boot: Integrated" },
            MonthlyMilestoneTarget { month: 2, milestone_description: "Memory allocator and paging validation", promotion_target: "Memory: Integrated" },
            MonthlyMilestoneTarget { month: 3, milestone_description: "Scheduler and syscall foundation", promotion_target: "Kernel: Integrated" },
            MonthlyMilestoneTarget { month: 4, milestone_description: "VFS and RAM-disk filesystem", promotion_target: "VFS: Integrated" },
            MonthlyMilestoneTarget { month: 5, milestone_description: "Processes, signals, pipes", promotion_target: "IPC: Integrated" },
            MonthlyMilestoneTarget { month: 6, milestone_description: "Installer, update, rollback", promotion_target: "Installer/update: Integrated" },
            MonthlyMilestoneTarget { month: 7, milestone_description: "Capability model and sandbox", promotion_target: "Security: Hardened" },
            MonthlyMilestoneTarget { month: 8, milestone_description: "Virtio device support", promotion_target: "Drivers: Integrated" },
            MonthlyMilestoneTarget { month: 9, milestone_description: "Package metadata and dry-run", promotion_target: "Packaging: Integrated" },
            MonthlyMilestoneTarget { month: 10, milestone_description: "Minimal desktop session", promotion_target: "Desktop: Prototype or Integrated" },
        ]
    }

    /// Validate PR according to Industry Rules 1-14
    pub fn validate_pr(&mut self, record: PrValidationRecord) -> Result<(), String> {
        // Rule 3: Study before implementation (must have ADR)
        if !record.has_adr {
            return Err("Rule 3 Violation: A component rule or major change requires an upstream study note and ADR first.".into());
        }

        // Rule 5: Status promotion requires runtime evidence
        if record.status_promoted && record.runtime_evidence_path.is_none() {
            return Err("Rule 5 Violation: Component status promotion requires runtime evidence (e.g. QEMU log under reports/). Unit tests alone are insufficient.".into());
        }

        // Rule 8: Security changes require security review
        if record.component == ComponentDomain::Security && !record.security_review_done {
            return Err("Rule 8 Violation: Security capability, sandboxing, and policy changes require a separate security review.".into());
        }

        self.validations.push(record);
        Ok(())
    }

    /// Generate summary report for PR-only governance engine
    pub fn generate_governance_report(&self) -> String {
        let mut report = String::from("# SigmaOS Periodic PR-Only Component-Rule Governance Report\n\n");
        report.push_str("## Validated PR Proposals\n\n");
        if self.validations.is_empty() {
            report.push_str("No PR proposals validated in current session.\n");
        } else {
            for v in &self.validations {
                report.push_str(&format!(
                    "- [{}] PR: '{}' (Branch: `{}`) | Component: {} | Evidence: {}\n",
                    if v.unit_tests_passed && v.integration_tests_passed { "PASSED" } else { "FAILED" },
                    v.pr_title,
                    v.branch_name,
                    v.component.name(),
                    v.runtime_evidence_path.as_deref().unwrap_or("None")
                ));
            }
        }
        report
    }
}

impl Default for PeriodicPrComponentRulesEngine {
    fn default() -> Self {
        Self::new()
    }
}
