//! SigmaOS Maintenance Operating Model Engine
//!
//! Enforces scheduled, evidence-based maintenance cadences (Daily, Weekly, Biweekly,
//! Monthly, Quarterly, Semiannual) across security, releases, dependencies, boot validation,
//! documentation, packaging, installer recovery, and community review.
//!
//! Every recurring task must produce a reviewable artifact (PR, issue, report, test result, or decision).
//! Enforces status honesty, evidence validation, and release readiness gates.

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Frequency of recurring tasks in the maintenance calendar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaintenanceCadence {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Semiannual,
}

impl MaintenanceCadence {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
            Self::Biweekly => "Biweekly",
            Self::Monthly => "Monthly",
            Self::Quarterly => "Quarterly",
            Self::Semiannual => "Semiannual",
        }
    }
}

/// Verification level distinguishing code existence from verified system behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskResultStatus {
    CodeExists = 0,
    TestsPass = 1,
    SystemBehaviorVerified = 2,
}

impl TaskResultStatus {
    pub fn is_verified(&self) -> bool {
        matches!(self, Self::SystemBehaviorVerified)
    }
}

/// Reviewable artifact required for every completed maintenance task
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskArtifact {
    PullRequest { pr_number: usize, title: String },
    Issue { issue_number: usize, title: String },
    Report { title: String, path: String },
    TestResult { suite_name: String, passed: bool },
    DocumentedDecision { adr_number: usize, title: String },
}

impl TaskArtifact {
    pub fn summary(&self) -> String {
        match self {
            Self::PullRequest { pr_number, title } => format!("PR #{}: {}", pr_number, title),
            Self::Issue {
                issue_number,
                title,
            } => format!("Issue #{}: {}", issue_number, title),
            Self::Report { title, path } => format!("Report '{}' at {}", title, path),
            Self::TestResult { suite_name, passed } => {
                format!(
                    "Test Suite '{}': {}",
                    suite_name,
                    if *passed { "PASS" } else { "FAIL" }
                )
            }
            Self::DocumentedDecision { adr_number, title } => {
                format!("ADR-{:04}: {}", adr_number, title)
            }
        }
    }
}

/// Role responsible for executing specific recurring tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaintenanceRole {
    BuildCiMaintainer,
    ReleaseEngineer,
    SecurityMaintainer,
    ComponentMaintainer,
    DocsMaintainer,
    PlatformMaintainer,
    TestingMaintainer,
    InstallerMaintainer,
    PackagingMaintainer,
    ReleaseManager,
    TechLead,
    DriverMaintainer,
    ComplianceOwner,
}

impl MaintenanceRole {
    pub fn title(&self) -> &'static str {
        match self {
            Self::BuildCiMaintainer => "Build/CI Maintainer",
            Self::ReleaseEngineer => "Release Engineer",
            Self::SecurityMaintainer => "Security Maintainer",
            Self::ComponentMaintainer => "Component Maintainer",
            Self::DocsMaintainer => "Docs Maintainer",
            Self::PlatformMaintainer => "Platform Maintainer",
            Self::TestingMaintainer => "Testing Maintainer",
            Self::InstallerMaintainer => "Installer Maintainer",
            Self::PackagingMaintainer => "Packaging Maintainer",
            Self::ReleaseManager => "Release Manager",
            Self::TechLead => "Tech Lead",
            Self::DriverMaintainer => "Driver Maintainer",
            Self::ComplianceOwner => "Compliance Owner",
        }
    }
}

/// Task definition in the recurring maintenance calendar
#[derive(Debug, Clone)]
pub struct RecurringTaskSpec {
    pub id: String,
    pub name: String,
    pub cadence: MaintenanceCadence,
    pub owner_role: MaintenanceRole,
    pub required_output: String,
}

/// Release stability tiers and required evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseTier {
    Experimental = 0,
    DeveloperPreview = 1,
    TestDistribution = 2,
    ReleaseCandidate = 3,
    Stable = 4,
}

impl ReleaseTier {
    pub fn required_evidence(&self) -> &'static str {
        match self {
            Self::Experimental => "Build and unit tests pass",
            Self::DeveloperPreview => "QEMU boot reaches defined shell/diagnostic state",
            Self::TestDistribution => "Clean VM installation and recovery path verified",
            Self::ReleaseCandidate => "Full regression, security review, known-issue list",
            Self::Stable => {
                "Hardware matrix, update/rollback, support policy, and security process verified"
            }
        }
    }
}

/// Driver hardware support categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardwareDriverClass {
    Serial,
    Rtc,
    Display,
    Input,
    Storage,
    Usb,
    Network,
    Audio,
}

/// Hardware audit status entry
#[derive(Debug, Clone)]
pub struct HardwareAuditEntry {
    pub driver_class: HardwareDriverClass,
    pub status: TaskResultStatus,
    pub evidence_path: Option<String>,
    pub known_limits: String,
}

/// Engine managing the SigmaOS Maintenance Operating Model
#[derive(Debug, Clone)]
pub struct MaintenanceOperatingModelEngine {
    pub tasks: Vec<RecurringTaskSpec>,
    pub completed_artifacts: Vec<(String, TaskArtifact, TaskResultStatus)>,
    pub hardware_matrix: Vec<HardwareAuditEntry>,
}

impl MaintenanceOperatingModelEngine {
    pub fn new() -> Self {
        let tasks = vec![
            RecurringTaskSpec {
                id: "daily_ci".into(),
                name: "CI health triage".into(),
                cadence: MaintenanceCadence::Daily,
                owner_role: MaintenanceRole::BuildCiMaintainer,
                required_output: "Fix PR or issue with owner".into(),
            },
            RecurringTaskSpec {
                id: "weekly_deps".into(),
                name: "Dependency and toolchain review".into(),
                cadence: MaintenanceCadence::Weekly,
                owner_role: MaintenanceRole::ReleaseEngineer,
                required_output: "chore(deps) PR".into(),
            },
            RecurringTaskSpec {
                id: "weekly_security".into(),
                name: "Security advisory review".into(),
                cadence: MaintenanceCadence::Weekly,
                owner_role: MaintenanceRole::SecurityMaintainer,
                required_output: "security(...) PR or triage issue".into(),
            },
            RecurringTaskSpec {
                id: "weekly_study".into(),
                name: "Open-source study".into(),
                cadence: MaintenanceCadence::Weekly,
                owner_role: MaintenanceRole::ComponentMaintainer,
                required_output: "docs/studies/... PR".into(),
            },
            RecurringTaskSpec {
                id: "biweekly_gap".into(),
                name: "Component gap review".into(),
                cadence: MaintenanceCadence::Biweekly,
                owner_role: MaintenanceRole::ComponentMaintainer,
                required_output: "Gap issue + scoped feature PR".into(),
            },
            RecurringTaskSpec {
                id: "biweekly_docs".into(),
                name: "Documentation accuracy audit".into(),
                cadence: MaintenanceCadence::Biweekly,
                owner_role: MaintenanceRole::DocsMaintainer,
                required_output: "docs(...) PR".into(),
            },
            RecurringTaskSpec {
                id: "monthly_boot".into(),
                name: "Boot and smoke-test validation".into(),
                cadence: MaintenanceCadence::Monthly,
                owner_role: MaintenanceRole::PlatformMaintainer,
                required_output: "Boot report + fixes".into(),
            },
            RecurringTaskSpec {
                id: "monthly_fuzz".into(),
                name: "Fuzzing and regression review".into(),
                cadence: MaintenanceCadence::Monthly,
                owner_role: MaintenanceRole::TestingMaintainer,
                required_output: "Fuzz findings + fixes".into(),
            },
            RecurringTaskSpec {
                id: "monthly_installer".into(),
                name: "Installer/recovery rehearsal".into(),
                cadence: MaintenanceCadence::Monthly,
                owner_role: MaintenanceRole::InstallerMaintainer,
                required_output: "VM install report + fixes".into(),
            },
            RecurringTaskSpec {
                id: "monthly_package".into(),
                name: "Package/update rehearsal".into(),
                cadence: MaintenanceCadence::Monthly,
                owner_role: MaintenanceRole::PackagingMaintainer,
                required_output: "Package transaction report".into(),
            },
            RecurringTaskSpec {
                id: "quarterly_release".into(),
                name: "Release readiness review".into(),
                cadence: MaintenanceCadence::Quarterly,
                owner_role: MaintenanceRole::ReleaseManager,
                required_output: "Release checklist PR".into(),
            },
            RecurringTaskSpec {
                id: "quarterly_architecture".into(),
                name: "Architecture and roadmap review".into(),
                cadence: MaintenanceCadence::Quarterly,
                owner_role: MaintenanceRole::TechLead,
                required_output: "ADR/roadmap PR".into(),
            },
            RecurringTaskSpec {
                id: "quarterly_hardware".into(),
                name: "Hardware support audit".into(),
                cadence: MaintenanceCadence::Quarterly,
                owner_role: MaintenanceRole::DriverMaintainer,
                required_output: "Hardware matrix PR".into(),
            },
            RecurringTaskSpec {
                id: "semiannual_compliance".into(),
                name: "License, provenance, and supply-chain audit".into(),
                cadence: MaintenanceCadence::Semiannual,
                owner_role: MaintenanceRole::ComplianceOwner,
                required_output: "SBOM/provenance PR".into(),
            },
        ];

        let hardware_classes = [
            HardwareDriverClass::Serial,
            HardwareDriverClass::Rtc,
            HardwareDriverClass::Display,
            HardwareDriverClass::Input,
            HardwareDriverClass::Storage,
            HardwareDriverClass::Usb,
            HardwareDriverClass::Network,
            HardwareDriverClass::Audio,
        ];

        let hardware_matrix = hardware_classes
            .iter()
            .map(|&driver_class| HardwareAuditEntry {
                driver_class,
                status: TaskResultStatus::CodeExists,
                evidence_path: None,
                known_limits: "Pending physical/QEMU driver verification evidence".into(),
            })
            .collect();

        Self {
            tasks,
            completed_artifacts: Vec::new(),
            hardware_matrix,
        }
    }

    /// Register a completed task with reviewable artifact and evidence status
    pub fn record_task_completion(
        &mut self,
        task_id: &str,
        artifact: TaskArtifact,
        status: TaskResultStatus,
    ) -> Result<(), String> {
        if self.tasks.iter().any(|t| t.id == task_id) {
            self.completed_artifacts
                .push((task_id.to_string(), artifact, status));
            Ok(())
        } else {
            Err(format!(
                "Task ID '{}' not found in maintenance calendar",
                task_id
            ))
        }
    }

    /// Evaluates release tier readiness based on verified artifacts
    pub fn evaluate_release_readiness(&self, target_tier: ReleaseTier) -> bool {
        let boot_verified = self
            .completed_artifacts
            .iter()
            .any(|(id, _, status)| id == "monthly_boot" && status.is_verified());

        let installer_verified = self
            .completed_artifacts
            .iter()
            .any(|(id, _, status)| id == "monthly_installer" && status.is_verified());

        match target_tier {
            ReleaseTier::Experimental => true,
            ReleaseTier::DeveloperPreview => boot_verified,
            ReleaseTier::TestDistribution => boot_verified && installer_verified,
            ReleaseTier::ReleaseCandidate | ReleaseTier::Stable => {
                boot_verified
                    && installer_verified
                    && self.hardware_matrix.iter().all(|e| e.status.is_verified())
            }
        }
    }

    /// Update hardware audit matrix entry with verified evidence
    pub fn update_hardware_evidence(
        &mut self,
        driver_class: HardwareDriverClass,
        status: TaskResultStatus,
        evidence_path: Option<String>,
        limits: String,
    ) {
        if let Some(entry) = self
            .hardware_matrix
            .iter_mut()
            .find(|e| e.driver_class == driver_class)
        {
            entry.status = status;
            entry.evidence_path = evidence_path;
            entry.known_limits = limits;
        }
    }
}

impl Default for MaintenanceOperatingModelEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maintenance_calendar_initialization() {
        let engine = MaintenanceOperatingModelEngine::new();
        assert_eq!(engine.tasks.len(), 14);
        assert_eq!(engine.hardware_matrix.len(), 8);
    }

    #[test]
    fn test_release_readiness_evaluation() {
        let mut engine = MaintenanceOperatingModelEngine::new();

        // Initially Experimental is ready, DeveloperPreview is not
        assert!(engine.evaluate_release_readiness(ReleaseTier::Experimental));
        assert!(!engine.evaluate_release_readiness(ReleaseTier::DeveloperPreview));

        // Add boot verification evidence
        assert!(engine
            .record_task_completion(
                "monthly_boot",
                TaskArtifact::Report {
                    title: "October Boot Report".into(),
                    path: "reports/boot/october.log".into()
                },
                TaskResultStatus::SystemBehaviorVerified,
            )
            .is_ok());

        assert!(engine.evaluate_release_readiness(ReleaseTier::DeveloperPreview));
        assert!(!engine.evaluate_release_readiness(ReleaseTier::TestDistribution));
    }
}
