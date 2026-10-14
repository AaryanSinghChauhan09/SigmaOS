//! Periodic "Study → Improve Component → Prove Improvement" Engine for SigmaOS Governance
//!
//! Enforces periodic cadence, 10-step component improvement loop, component ownership mapping,
//! monthly milestones, component maturity progression, and rule validation.

use std::format;
use std::string::{String, ToString};
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
            Self::Security => "Capabilities, sandboxing, seccomp-like restrictions, unveil concepts",
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
            ComponentMaturity::Integrated | ComponentMaturity::Hardened | ComponentMaturity::Supported => {
                has_runtime_evidence && has_unit_tests
            }
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
        if let Some(milestone) = self.monthly_milestones.iter_mut().find(|m| m.month == month) {
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
        report.push_str(&format!("Total cycles completed: {}\n", self.weekly_records.len()));

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
                if milestone.completed { "COMPLETED" } else { "PENDING" },
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
}
