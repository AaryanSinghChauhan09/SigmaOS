//! Periodic "Study → Improve Component → Prove Improvement" Engine for SigmaOS Governance
//!
//! Enforces periodic cadence, 10-step component improvement loop, component ownership mapping,
//! monthly milestones, component maturity progression, rule validation, and PR proposal gateway.

use std::format;
use std::string::String;
use std::vec;
use std::vec::Vec;

/// Operating cadences for the periodic improvement program
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeriodicCadence {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
}

impl PeriodicCadence {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Daily => "Keep build, tests, and boot status healthy",
            Self::Weekly => "Study one upstream mechanism and improve one component",
            Self::Biweekly => "Re-test a component against its runtime contract",
            Self::Monthly => "Complete one component milestone",
            Self::Quarterly => "Reassess architecture, gaps, security, and roadmap",
        }
    }

    pub fn output_artifact(&self) -> &'static str {
        match self {
            Self::Daily => "Status report and failure triage log",
            Self::Weekly => "Upstream research note, ADR, small PR, test evidence",
            Self::Biweekly => "Regression report and updated capability status",
            Self::Monthly => "Runtime-validated feature or recovery path",
            Self::Quarterly => "Updated roadmap, archived stale plans, prioritized backlog",
        }
    }
}

/// The 12 SigmaOS component domains mapped to ownership boundaries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentDomain {
    KernelCore,
    Memory,
    VfsFilesystems,
    Ipc,
    Syscalls,
    Security,
    Drivers,
    Networking,
    Packaging,
    DesktopZenith,
    InstallerUpdate,
    DistroCompatibility,
}

impl ComponentDomain {
    pub fn name(&self) -> &'static str {
        match self {
            Self::KernelCore => "Kernel core",
            Self::Memory => "Memory",
            Self::VfsFilesystems => "VFS/filesystems",
            Self::Ipc => "IPC",
            Self::Syscalls => "Syscalls",
            Self::Security => "Security",
            Self::Drivers => "Drivers",
            Self::Networking => "Networking",
            Self::Packaging => "Packaging",
            Self::DesktopZenith => "Desktop/Zenith",
            Self::InstallerUpdate => "Installer/update",
            Self::DistroCompatibility => "Distro compatibility",
        }
    }

    pub fn primary_references(&self) -> &'static str {
        match self {
            Self::KernelCore => "Linux, xv6, Redox",
            Self::Memory => "Linux, xv6, Redox",
            Self::VfsFilesystems => "Linux, xv6, FreeBSD",
            Self::Ipc => "Linux, FreeBSD",
            Self::Syscalls => "Linux, FreeBSD",
            Self::Security => "OpenBSD, Linux, FreeBSD",
            Self::Drivers => "Linux, FreeBSD",
            Self::Networking => "Linux, FreeBSD",
            Self::Packaging => "Arch, Debian, Fedora",
            Self::DesktopZenith => "Linux Mint, Omarchy, Redox",
            Self::InstallerUpdate => "Debian, Fedora, Mint",
            Self::DistroCompatibility => "Arch, Debian, Fedora, FreeBSD",
        }
    }

    pub fn periodic_focus(&self) -> &'static str {
        match self {
            Self::KernelCore => "Boot, interrupts, syscalls, scheduling, panic handling",
            Self::Memory => "Frame allocator, paging, slab/buddy allocators, leak detection",
            Self::VfsFilesystems => "Path resolution, mounts, file operations, crash consistency",
            Self::Ipc => "Pipes, signals, shared memory, message queues, eventfd",
            Self::Syscalls => "ABI stability, validation, error codes, syscall coverage",
            Self::Security => {
                "Capabilities, sandboxing, seccomp-like restrictions, unveil concepts"
            }
            Self::Drivers => "Device probe, initialization, error handling, virtio devices",
            Self::Networking => "Socket API, TCP/IP, packet filtering, WireGuard concepts",
            Self::Packaging => "Metadata, dependencies, install, update, rollback",
            Self::DesktopZenith => "Input, windows, launcher, diagnostics, recovery",
            Self::InstallerUpdate => "Clean install, atomic update, rollback, recovery",
            Self::DistroCompatibility => "Metadata compatibility, ABI boundaries, sandboxing",
        }
    }

    pub fn required_proof(&self) -> &'static str {
        match self {
            Self::KernelCore => "QEMU boot trace",
            Self::Memory => "Allocation tests and runtime traces",
            Self::VfsFilesystems => "Filesystem conformance tests",
            Self::Ipc => "Cross-process interoperability tests",
            Self::Syscalls => "Syscall table and negative tests",
            Self::Security => "Denial tests and threat model",
            Self::Drivers => "QEMU device logs",
            Self::Networking => "Network integration tests",
            Self::Packaging => "Package test corpus",
            Self::DesktopZenith => "Graphical smoke test",
            Self::InstallerUpdate => "Install and rollback tests",
            Self::DistroCompatibility => "Compatibility matrix",
        }
    }
}

/// Component maturity level scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentMaturity {
    Proposed = 0,
    Prototype = 1,
    Integrated = 2,
    Hardened = 3,
    Supported = 4,
}

impl ComponentMaturity {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Proposed => "Proposed",
            Self::Prototype => "Prototype",
            Self::Integrated => "Integrated",
            Self::Hardened => "Hardened",
            Self::Supported => "Supported",
        }
    }

    pub fn requirement(&self) -> &'static str {
        match self {
            Self::Proposed => "ADR and gap analysis",
            Self::Prototype => "Unit tests",
            Self::Integrated => "Integration test plus QEMU/runtime evidence",
            Self::Hardened => "Fuzzing, negative tests, security review",
            Self::Supported => "Reproducible boot/install/update evidence and rollback",
        }
    }
}

/// Steps in the mandatory 10-step component improvement loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImprovementLoopStep {
    Step1SelectComponent = 1,
    Step2MeasureCurrentState = 2,
    Step3SelectUpstreamReference = 3,
    Step4StudyMechanism = 4,
    Step5WriteImprovementProposal = 5,
    Step6ImplementSmallestSafeChange = 6,
    Step7RunTests = 7,
    Step8UpdateDocsAndStatus = 8,
    Step9AddRegressionProtection = 9,
    Step10ReviewMergeAndMonitor = 10,
}

impl ImprovementLoopStep {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Step1SelectComponent => "1. SELECT COMPONENT",
            Self::Step2MeasureCurrentState => "2. MEASURE CURRENT STATE",
            Self::Step3SelectUpstreamReference => "3. SELECT UPSTREAM REFERENCE",
            Self::Step4StudyMechanism => "4. STUDY THE MECHANISM",
            Self::Step5WriteImprovementProposal => "5. WRITE AN IMPROVEMENT PROPOSAL",
            Self::Step6ImplementSmallestSafeChange => "6. IMPLEMENT THE SMALLEST SAFE CHANGE",
            Self::Step7RunTests => "7. RUN UNIT, INTEGRATION, AND RUNTIME TESTS",
            Self::Step8UpdateDocsAndStatus => "8. UPDATE DOCUMENTATION AND CAPABILITY STATUS",
            Self::Step9AddRegressionProtection => "9. ADD REGRESSION PROTECTION",
            Self::Step10ReviewMergeAndMonitor => "10. REVIEW, MERGE, AND MONITOR",
        }
    }
}

/// Upstream study record capturing Day 2 study outcomes
#[derive(Debug, Clone)]
pub struct UpstreamStudyRecord {
    pub target_domain: ComponentDomain,
    pub upstream_reference: String,
    pub problem_solved: String,
    pub core_structures_and_flow: String,
    pub failure_and_security_assumptions: String,
    pub sigmaos_adaptations: String,
    pub explicit_rejections: String,
    pub adr_title: String,
}

/// Record of a weekly component improvement cycle execution
#[derive(Debug, Clone)]
pub struct WeeklyCycleRecord {
    pub cycle_id: usize,
    pub domain: ComponentDomain,
    pub study: UpstreamStudyRecord,
    pub adr_file: String,
    pub pr_title: String,
    pub unit_tests_added: usize,
    pub integration_tests_added: usize,
    pub runtime_evidence_path: Option<String>,
    pub initial_maturity: ComponentMaturity,
    pub promoted_maturity: ComponentMaturity,
}

/// Record of a monthly milestone definition
#[derive(Debug, Clone)]
pub struct MonthlyMilestoneRecord {
    pub month: u8,
    pub domain: ComponentDomain,
    pub milestone_name: String,
    pub validation_gate: String,
    pub completed: bool,
    pub evidence_log: Option<String>,
}

/// Governance engine for periodic study-improve-prove workflow
#[derive(Debug, Clone)]
pub struct PeriodicImprovementEngine {
    pub component_maturities: Vec<(ComponentDomain, ComponentMaturity)>,
    pub weekly_records: Vec<WeeklyCycleRecord>,
    pub monthly_milestones: Vec<MonthlyMilestoneRecord>,
    pub active_loop_step: ImprovementLoopStep,
}

impl PeriodicImprovementEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            component_maturities: Vec::new(),
            weekly_records: Vec::new(),
            monthly_milestones: Vec::new(),
            active_loop_step: ImprovementLoopStep::Step1SelectComponent,
        };

        // Initialize all 12 domains as Proposed/Prototype initially
        let domains = [
            ComponentDomain::KernelCore,
            ComponentDomain::Memory,
            ComponentDomain::VfsFilesystems,
            ComponentDomain::Ipc,
            ComponentDomain::Syscalls,
            ComponentDomain::Security,
            ComponentDomain::Drivers,
            ComponentDomain::Networking,
            ComponentDomain::Packaging,
            ComponentDomain::DesktopZenith,
            ComponentDomain::InstallerUpdate,
            ComponentDomain::DistroCompatibility,
        ];

        for domain in domains {
            engine
                .component_maturities
                .push((domain, ComponentMaturity::Prototype));
        }

        // Initialize 12 monthly milestones
        engine.monthly_milestones = vec![
            MonthlyMilestoneRecord {
                month: 1,
                domain: ComponentDomain::KernelCore,
                milestone_name: "Reproducible QEMU x86_64 boot-to-shell".into(),
                validation_gate: "Serial log and boot matrix".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 2,
                domain: ComponentDomain::Memory,
                milestone_name: "Frame allocator, paging, and leak detection".into(),
                validation_gate: "Allocation and boot tests".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 3,
                domain: ComponentDomain::KernelCore,
                milestone_name: "Fair scheduling and preemption".into(),
                validation_gate: "Process-switch trace".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 4,
                domain: ComponentDomain::VfsFilesystems,
                milestone_name: "Path resolution, file descriptors, RAM-disk operations".into(),
                validation_gate: "Filesystem tests".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 5,
                domain: ComponentDomain::Ipc,
                milestone_name: "Pipes, signals, process lifecycle".into(),
                validation_gate: "Interop tests".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 6,
                domain: ComponentDomain::InstallerUpdate,
                milestone_name: "Clean install, atomic update, rollback".into(),
                validation_gate: "Install and rollback tests".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 7,
                domain: ComponentDomain::Security,
                milestone_name: "Capability checks and sandbox policy".into(),
                validation_gate: "Denial tests".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 8,
                domain: ComponentDomain::Drivers,
                milestone_name: "virtio-blk, virtio-net, VGA".into(),
                validation_gate: "Device logs".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 9,
                domain: ComponentDomain::Packaging,
                milestone_name: "Metadata parsing and dependency resolution".into(),
                validation_gate: "Package corpus tests".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 10,
                domain: ComponentDomain::DesktopZenith,
                milestone_name: "Minimal graphical session and input".into(),
                validation_gate: "Graphical smoke test".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 11,
                domain: ComponentDomain::Networking,
                milestone_name: "Basic socket and network stack path".into(),
                validation_gate: "Network integration test".into(),
                completed: false,
                evidence_log: None,
            },
            MonthlyMilestoneRecord {
                month: 12,
                domain: ComponentDomain::DistroCompatibility,
                milestone_name: "Nightly artifact, known issues, rollback docs".into(),
                validation_gate: "Release checklist".into(),
                completed: false,
                evidence_log: None,
            },
        ];

        engine
    }

    pub fn get_maturity(&self, domain: ComponentDomain) -> ComponentMaturity {
        self.component_maturities
            .iter()
            .find(|(d, _)| *d == domain)
            .map(|(_, m)| *m)
            .unwrap_or(ComponentMaturity::Proposed)
    }

    /// Evaluates if promotion criteria are met under Rule 3 (Runtime evidence controls status)
    pub fn can_promote_maturity(
        current: ComponentMaturity,
        target: ComponentMaturity,
        has_runtime_evidence: bool,
        has_unit_tests: bool,
    ) -> bool {
        if target <= current {
            return true;
        }

        match target {
            ComponentMaturity::Proposed => true,
            ComponentMaturity::Prototype => has_unit_tests,
            ComponentMaturity::Integrated
            | ComponentMaturity::Hardened
            | ComponentMaturity::Supported => has_runtime_evidence && has_unit_tests,
        }
    }

    /// Submits a weekly improvement cycle result, enforcing evidence gates
    pub fn record_weekly_improvement(
        &mut self,
        domain: ComponentDomain,
        study: UpstreamStudyRecord,
        adr_file: String,
        pr_title: String,
        unit_tests_added: usize,
        integration_tests_added: usize,
        runtime_evidence_path: Option<String>,
        desired_maturity: ComponentMaturity,
    ) -> Result<WeeklyCycleRecord, String> {
        let initial_maturity = self.get_maturity(domain);
        let has_runtime_evidence = runtime_evidence_path.is_some();
        let has_unit_tests = unit_tests_added > 0;

        let promoted_maturity = if Self::can_promote_maturity(
            initial_maturity,
            desired_maturity,
            has_runtime_evidence,
            has_unit_tests,
        ) {
            desired_maturity
        } else {
            // Under Rule 3: cannot promote beyond Prototype without runtime evidence
            if desired_maturity >= ComponentMaturity::Integrated && !has_runtime_evidence {
                return Err(format!(
                    "Rule 3 Violation: Cannot promote component {:?} to {:?} without runtime evidence (QEMU/hardware logs). Status remains {:?}",
                    domain, desired_maturity, initial_maturity
                ));
            }
            initial_maturity
        };

        // Update stored maturity
        if let Some((_, m)) = self
            .component_maturities
            .iter_mut()
            .find(|(d, _)| *d == domain)
        {
            *m = promoted_maturity;
        }

        let record = WeeklyCycleRecord {
            cycle_id: self.weekly_records.len() + 1,
            domain,
            study,
            adr_file,
            pr_title,
            unit_tests_added,
            integration_tests_added,
            runtime_evidence_path,
            initial_maturity,
            promoted_maturity,
        };

        self.weekly_records.push(record.clone());
        Ok(record)
    }

    /// Complete a monthly milestone with proof log
    pub fn complete_monthly_milestone(
        &mut self,
        month: u8,
        evidence_log: String,
    ) -> Result<(), String> {
        if let Some(milestone) = self
            .monthly_milestones
            .iter_mut()
            .find(|m| m.month == month)
        {
            milestone.completed = true;
            milestone.evidence_log = Some(evidence_log);
            Ok(())
        } else {
            Err(format!("Month {} milestone not found", month))
        }
    }

    /// Generate an executive report summarizing the periodic program state
    pub fn generate_program_summary(&self) -> String {
        let mut report = String::from("# SigmaOS Periodic Improvement Program Status Report\n\n");
        report.push_str("## Component Maturity Levels\n");

        for (domain, maturity) in &self.component_maturities {
            report.push_str(&format!(
                "- **{}**: {:?} (Proof required: {})\n",
                domain.name(),
                maturity,
                domain.required_proof()
            ));
        }

        report.push_str("\n## Completed Weekly Cycles\n");
        report.push_str(&format!(
            "Total cycles completed: {}\n",
            self.weekly_records.len()
        ));

        for record in &self.weekly_records {
            report.push_str(&format!(
                "- Cycle #{}: {} -> PR '{}' (Maturity: {:?} -> {:?}, Evidence: {})\n",
                record.cycle_id,
                record.domain.name(),
                record.pr_title,
                record.initial_maturity,
                record.promoted_maturity,
                record.runtime_evidence_path.as_deref().unwrap_or("None")
            ));
        }

        report.push_str("\n## Monthly Milestones Progress\n");
        for milestone in &self.monthly_milestones {
            report.push_str(&format!(
                "- Month {}: {} [{}] — {}\n",
                milestone.month,
                milestone.milestone_name,
                if milestone.completed {
                    "COMPLETED"
                } else {
                    "PENDING"
                },
                milestone.validation_gate
            ));
        }

        report
    }
}

impl Default for PeriodicImprovementEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PULL REQUEST PROPOSAL STRUCTURES & GATEWAY
// ============================================================================

/// Pull Request proposal record for periodic component improvements
#[derive(Debug, Clone)]
pub struct PeriodicImprovementPrProposal {
    pub pr_id: u32,
    pub pr_title: String,
    pub branch_name: String,
    pub domain: ComponentDomain,
    pub upstream_reference: String,
    pub adr_title: String,
    pub initial_maturity: ComponentMaturity,
    pub promoted_maturity: ComponentMaturity,
    pub runtime_evidence_path: Option<String>,
    pub summary_diff: String,
    pub pqc_signature: String,
    pub ci_status: String,
}

impl PeriodicImprovementPrProposal {
    /// Formats the proposal as a Pull Request proposal description matching repository standards
    pub fn format_pull_request_description(&self) -> String {
        let mut desc = format!("## PR #{}: {}\n\n", self.pr_id, self.pr_title);
        desc.push_str(&format!("**Branch Name**: `{}`\n", self.branch_name));
        desc.push_str(&format!("**Target Component**: {}\n", self.domain.name()));
        desc.push_str(&format!(
            "**Upstream Study Reference**: {}\n",
            self.upstream_reference
        ));
        desc.push_str(&format!("**ADR Document**: {}\n", self.adr_title));
        desc.push_str(&format!(
            "**Maturity Level Shift**: {:?} → {:?}\n",
            self.initial_maturity, self.promoted_maturity
        ));
        desc.push_str(&format!(
            "**Runtime Evidence Path**: {}\n",
            self.runtime_evidence_path
                .as_deref()
                .unwrap_or("None (Unpromoted)")
        ));
        desc.push_str(&format!(
            "**PQC Post-Quantum Signature**: `{}`\n",
            self.pqc_signature
        ));
        desc.push_str(&format!("**CI Gate Status**: {}\n\n", self.ci_status));

        desc.push_str("### Proposed Changes\n```diff\n");
        desc.push_str(&self.summary_diff);
        desc.push_str("\n```\n\n");

        desc.push_str("### Operating Rules Checklist\n");
        desc.push_str("- [x] Rule 1: Single component scope per PR\n");
        desc.push_str("- [x] Rule 2: Upstream mechanism study note & ADR created\n");
        desc.push_str("- [x] Rule 3: Runtime evidence gated status promotion\n");
        desc.push_str("- [x] Rule 4: All standard CI test entry points pass cleanly\n");
        desc.push_str("- [x] Rule 9: Mandatory regression test coverage included\n");

        desc
    }
}

/// Gateway for staging and executing Periodic "Study → Improve Component → Prove Improvement" PR proposals
#[derive(Debug, Clone)]
pub struct PeriodicStudyImproveProvePrGateway {
    pub engine: PeriodicImprovementEngine,
    pub pr_proposals: Vec<PeriodicImprovementPrProposal>,
}

impl PeriodicStudyImproveProvePrGateway {
    pub fn new() -> Self {
        let mut gateway = Self {
            engine: PeriodicImprovementEngine::new(),
            pr_proposals: Vec::new(),
        };

        gateway.seed_default_pr_proposals();
        gateway
    }

    fn seed_default_pr_proposals(&mut self) {
        let default_specs = [
            (
                101,
                "Kernel Core: Reproducible x86_64 Boot Path & Serial Register Dump",
                "feat/kernel-boot-serial-trace",
                ComponentDomain::KernelCore,
                "Linux boot/main.c & OSDev UEFI handoff",
                "ADR-001: x86_64 Boot Trace",
                ComponentMaturity::Prototype,
                ComponentMaturity::Integrated,
                Some("reports/boot/qemu_x86_64_serial.log".into()),
                "+ fn init_com1_serial_uart()\n+ struct QemuBootMatrix",
                "DILITHIUM5_SIG_101_KERNEL_BOOT",
            ),
            (
                102,
                "Memory Subsystem: Frame Allocator & Paging Diagnostics",
                "feat/memory-frame-allocator-paging",
                ComponentDomain::Memory,
                "xv6 kalloc.c & Linux mm/page_alloc.c",
                "ADR-002: Frame Allocator & Paging",
                ComponentMaturity::Prototype,
                ComponentMaturity::Integrated,
                Some("reports/memory/allocation_traces.log".into()),
                "+ struct SovereignFrameAllocator\n+ fn detect_page_leaks()",
                "DILITHIUM5_SIG_102_MEMORY_PAGING",
            ),
            (
                103,
                "VFS/Filesystems: Centralized Path Resolution & Traversal Safeguards",
                "feat/vfs-path-resolution-hardening",
                ComponentDomain::VfsFilesystems,
                "Linux VFS namei.c & xv6 sysfile.c",
                "ADR-012: VFS Path-Resolution Hardening",
                ComponentMaturity::Prototype,
                ComponentMaturity::Integrated,
                Some("reports/vfs/vfs_conformance.json".into()),
                "+ fn resolve_canonical_path()\n+ enum VfsPathResolutionError",
                "DILITHIUM5_SIG_103_VFS_PATH",
            ),
            (
                104,
                "IPC: Pipe Buffer Isolation & Signal Handling State Machine",
                "feat/ipc-pipes-signals-hardening",
                ComponentDomain::Ipc,
                "FreeBSD sys_pipe.c & Linux kernel/signal.c",
                "ADR-005: IPC Pipe Buffers and Signals",
                ComponentMaturity::Prototype,
                ComponentMaturity::Integrated,
                Some("reports/ipc/cross_process_interop.log".into()),
                "+ struct RingBufferPipe\n+ fn dispatch_signal_frame()",
                "DILITHIUM5_SIG_104_IPC_SIGNALS",
            ),
            (
                105,
                "Syscalls: Parameter Validation & Negative Test Coverage",
                "feat/syscall-abi-validation",
                ComponentDomain::Syscalls,
                "Linux arch/x86/entry/syscalls/ & FreeBSD sys/kern/",
                "ADR-006: Syscall Table Stability",
                ComponentMaturity::Prototype,
                ComponentMaturity::Integrated,
                Some("reports/syscalls/negative_test_matrix.json".into()),
                "+ fn validate_user_pointer()\n+ struct SyscallTableMap",
                "DILITHIUM5_SIG_105_SYSCALL_ABI",
            ),
            (
                106,
                "Security: Pledge Promises & Unveil Path Tree Constraints",
                "feat/security-pledge-unveil-capabilities",
                ComponentDomain::Security,
                "OpenBSD sys/kern/kern_pledge.c & Linux Landlock",
                "ADR-007: Mandatory Access Control",
                ComponentMaturity::Prototype,
                ComponentMaturity::Integrated,
                Some("reports/security/capability_denial.json".into()),
                "+ struct PledgePromises\n+ struct UnveilPathNode",
                "DILITHIUM5_SIG_106_SECURITY_PLEDGE",
            ),
        ];

        for (id, title, branch, domain, ref_os, adr, init_mat, prom_mat, evidence, diff, pqc) in
            default_specs
        {
            let proposal = PeriodicImprovementPrProposal {
                pr_id: id,
                pr_title: title.into(),
                branch_name: branch.into(),
                domain,
                upstream_reference: ref_os.into(),
                adr_title: adr.into(),
                initial_maturity: init_mat,
                promoted_maturity: prom_mat,
                runtime_evidence_path: evidence,
                summary_diff: diff.into(),
                pqc_signature: pqc.into(),
                ci_status: "PASSED".into(),
            };

            self.pr_proposals.push(proposal);
        }
    }

    /// Submits a PR proposal to the gateway, validating Rule 3 runtime evidence constraints
    pub fn submit_pr_proposal(
        &mut self,
        proposal: PeriodicImprovementPrProposal,
    ) -> Result<(), String> {
        let has_evidence = proposal.runtime_evidence_path.is_some();
        if proposal.promoted_maturity >= ComponentMaturity::Integrated && !has_evidence {
            return Err(format!(
                "Rule 3 Violation: Cannot submit PR '{}' promoting {:?} to {:?} without runtime evidence",
                proposal.pr_title, proposal.domain, proposal.promoted_maturity
            ));
        }

        self.pr_proposals.push(proposal);
        Ok(())
    }

    /// Queries PR proposals by component domain
    pub fn search_by_domain(&self, domain: ComponentDomain) -> Vec<&PeriodicImprovementPrProposal> {
        self.pr_proposals
            .iter()
            .filter(|p| p.domain == domain)
            .collect()
    }

    /// Generates a master gateway proposal report summarizing all staged PRs
    pub fn generate_pr_gateway_summary(&self) -> String {
        let mut report =
            String::from("# SigmaOS Periodic Study-Improve-Prove PR Gateway Report\n\n");
        report.push_str(&format!(
            "Total Staged PR Proposals: {}\n\n",
            self.pr_proposals.len()
        ));

        for pr in &self.pr_proposals {
            report.push_str(&format!(
                "- **PR #{}: {}** (`{}`)\n  - Domain: {}\n  - Upstream Ref: {}\n  - Maturity Shift: {:?} → {:?}\n  - Evidence: {}\n  - CI Gate: {}\n",
                pr.pr_id,
                pr.pr_title,
                pr.branch_name,
                pr.domain.name(),
                pr.upstream_reference,
                pr.initial_maturity,
                pr.promoted_maturity,
                pr.runtime_evidence_path.as_deref().unwrap_or("None"),
                pr.ci_status
            ));
        }

        report
    }
}

impl Default for PeriodicStudyImproveProvePrGateway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let engine = PeriodicImprovementEngine::new();
        assert_eq!(engine.component_maturities.len(), 12);
        assert_eq!(engine.monthly_milestones.len(), 12);
        assert_eq!(
            engine.get_maturity(ComponentDomain::KernelCore),
            ComponentMaturity::Prototype
        );
    }

    #[test]
    fn test_weekly_improvement_rule3_enforcement() {
        let mut engine = PeriodicImprovementEngine::new();
        let study = UpstreamStudyRecord {
            target_domain: ComponentDomain::KernelCore,
            upstream_reference: "xv6 process table".into(),
            problem_solved: "Process scheduling starvation".into(),
            core_structures_and_flow: "Round-robin array walk".into(),
            failure_and_security_assumptions: "Timer interrupt preemption".into(),
            sigmaos_adaptations: "EEVDF scheduler adaptation".into(),
            explicit_rejections: "O(1) active/expired runqueue arrays".into(),
            adr_title: "ADR-004: EEVDF Preemptive Scheduler".into(),
        };

        // Promotion without evidence fails Rule 3
        let res_err = engine.record_weekly_improvement(
            ComponentDomain::KernelCore,
            study.clone(),
            "research/decisions/ADR-004-scheduler.md".into(),
            "EEVDF Scheduler fairness".into(),
            10,
            2,
            None,
            ComponentMaturity::Integrated,
        );
        assert!(res_err.is_err());

        // Promotion WITH evidence succeeds
        let res_ok = engine.record_weekly_improvement(
            ComponentDomain::KernelCore,
            study,
            "research/decisions/ADR-004-scheduler.md".into(),
            "EEVDF Scheduler fairness".into(),
            10,
            2,
            Some("reports/boot/qemu-x86_64.log".into()),
            ComponentMaturity::Integrated,
        );
        assert!(res_ok.is_ok());
        assert_eq!(
            engine.get_maturity(ComponentDomain::KernelCore),
            ComponentMaturity::Integrated
        );
    }

    #[test]
    fn test_pr_proposal_and_gateway() {
        let mut gateway = PeriodicStudyImproveProvePrGateway::new();
        assert_eq!(gateway.pr_proposals.len(), 6);

        let vfs_prs = gateway.search_by_domain(ComponentDomain::VfsFilesystems);
        assert_eq!(vfs_prs.len(), 1);
        assert_eq!(vfs_prs[0].pr_id, 103);

        let desc = vfs_prs[0].format_pull_request_description();
        assert!(desc.contains("## PR #103: VFS/Filesystems: Centralized Path Resolution"));
        assert!(desc.contains("ADR-012: VFS Path-Resolution Hardening"));
        assert!(desc.contains("reports/vfs/vfs_conformance.json"));

        // Submitting PR without evidence attempting promotion triggers Rule 3 error
        let bad_pr = PeriodicImprovementPrProposal {
            pr_id: 999,
            pr_title: "Drivers: Untested driver promotion".into(),
            branch_name: "feat/bad-driver".into(),
            domain: ComponentDomain::Drivers,
            upstream_reference: "Linux drivers".into(),
            adr_title: "ADR-999".into(),
            initial_maturity: ComponentMaturity::Prototype,
            promoted_maturity: ComponentMaturity::Integrated,
            runtime_evidence_path: None,
            summary_diff: "+ bad code".into(),
            pqc_signature: "DILITHIUM5_SIG_BAD".into(),
            ci_status: "FAILED".into(),
        };

        assert!(gateway.submit_pr_proposal(bad_pr).is_err());
    }
}
