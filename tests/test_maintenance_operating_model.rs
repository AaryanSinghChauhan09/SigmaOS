// SPDX-License-Identifier: MIT
// Standalone integration test suite for SigmaOS Maintenance Operating Model Engine

#[path = "../src/governance/maintenance_operating_model.rs"]
mod maintenance_operating_model;

use maintenance_operating_model::*;

#[test]
fn test_maintenance_calendar_tasks_count() {
    let engine = MaintenanceOperatingModelEngine::new();
    assert_eq!(engine.tasks.len(), 14);

    let daily_tasks: Vec<_> = engine
        .tasks
        .iter()
        .filter(|t| t.cadence == MaintenanceCadence::Daily)
        .collect();
    assert_eq!(daily_tasks.len(), 1);
    assert_eq!(daily_tasks[0].id, "daily_ci");

    let weekly_tasks: Vec<_> = engine
        .tasks
        .iter()
        .filter(|t| t.cadence == MaintenanceCadence::Weekly)
        .collect();
    assert_eq!(weekly_tasks.len(), 3);

    let biweekly_tasks: Vec<_> = engine
        .tasks
        .iter()
        .filter(|t| t.cadence == MaintenanceCadence::Biweekly)
        .collect();
    assert_eq!(biweekly_tasks.len(), 2);

    let monthly_tasks: Vec<_> = engine
        .tasks
        .iter()
        .filter(|t| t.cadence == MaintenanceCadence::Monthly)
        .collect();
    assert_eq!(monthly_tasks.len(), 4);

    let quarterly_tasks: Vec<_> = engine
        .tasks
        .iter()
        .filter(|t| t.cadence == MaintenanceCadence::Quarterly)
        .collect();
    assert_eq!(quarterly_tasks.len(), 3);

    let semiannual_tasks: Vec<_> = engine
        .tasks
        .iter()
        .filter(|t| t.cadence == MaintenanceCadence::Semiannual)
        .collect();
    assert_eq!(semiannual_tasks.len(), 1);
}

#[test]
fn test_maintenance_cadences_and_roles() {
    let cadences = [
        (MaintenanceCadence::Daily, "Daily"),
        (MaintenanceCadence::Weekly, "Weekly"),
        (MaintenanceCadence::Biweekly, "Biweekly"),
        (MaintenanceCadence::Monthly, "Monthly"),
        (MaintenanceCadence::Quarterly, "Quarterly"),
        (MaintenanceCadence::Semiannual, "Semiannual"),
    ];
    for (cadence, expected_name) in cadences {
        assert_eq!(cadence.name(), expected_name);
    }

    let roles = [
        (MaintenanceRole::BuildCiMaintainer, "Build/CI Maintainer"),
        (MaintenanceRole::ReleaseEngineer, "Release Engineer"),
        (MaintenanceRole::SecurityMaintainer, "Security Maintainer"),
        (MaintenanceRole::ComponentMaintainer, "Component Maintainer"),
        (MaintenanceRole::DocsMaintainer, "Docs Maintainer"),
        (MaintenanceRole::PlatformMaintainer, "Platform Maintainer"),
        (MaintenanceRole::TestingMaintainer, "Testing Maintainer"),
        (MaintenanceRole::InstallerMaintainer, "Installer Maintainer"),
        (MaintenanceRole::PackagingMaintainer, "Packaging Maintainer"),
        (MaintenanceRole::ReleaseManager, "Release Manager"),
        (MaintenanceRole::TechLead, "Tech Lead"),
        (MaintenanceRole::DriverMaintainer, "Driver Maintainer"),
        (MaintenanceRole::ComplianceOwner, "Compliance Owner"),
    ];
    for (role, expected_title) in roles {
        assert_eq!(role.title(), expected_title);
    }
}

#[test]
fn test_task_artifacts_and_status_variants() {
    let pr = TaskArtifact::PullRequest {
        pr_number: 42,
        title: "feat(boot): add boot smoke".into(),
    };
    assert!(pr.summary().contains("PR #42"));

    let issue = TaskArtifact::Issue {
        issue_number: 10,
        title: "gap(kernel): missing futex".into(),
    };
    assert!(issue.summary().contains("Issue #10"));

    let report = TaskArtifact::Report {
        title: "Boot Smoke".into(),
        path: "build/boot.log".into(),
    };
    assert!(report.summary().contains("Report 'Boot Smoke' at build/boot.log"));

    let test_res = TaskArtifact::TestResult {
        suite_name: "scheduler_tests".into(),
        passed: true,
    };
    assert!(test_res.summary().contains("Test Suite 'scheduler_tests': PASS"));

    let adr = TaskArtifact::DocumentedDecision {
        adr_number: 7,
        title: "Boot validation before drivers".into(),
    };
    assert!(adr.summary().contains("ADR-0007: Boot validation before drivers"));

    let statuses = [
        (TaskResultStatus::CodeExists, false),
        (TaskResultStatus::TestsPass, false),
        (TaskResultStatus::SystemBehaviorVerified, true),
    ];
    for (st, verified) in statuses {
        assert_eq!(st.is_verified(), verified);
    }
}

#[test]
fn test_task_completion_recording() {
    let mut engine = MaintenanceOperatingModelEngine::new();

    let pr_artifact = TaskArtifact::PullRequest {
        pr_number: 101,
        title: "fix(ci): repair nightly boot-smoke workflow".to_string(),
    };

    assert!(engine
        .record_task_completion(
            "daily_ci",
            pr_artifact.clone(),
            TaskResultStatus::SystemBehaviorVerified
        )
        .is_ok());
    assert_eq!(engine.completed_artifacts.len(), 1);
    assert_eq!(engine.completed_artifacts[0].1, pr_artifact);
    assert!(engine.completed_artifacts[0].2.is_verified());

    // Error on invalid task ID
    assert!(engine
        .record_task_completion(
            "non_existent_task",
            pr_artifact,
            TaskResultStatus::CodeExists
        )
        .is_err());
}

#[test]
fn test_hardware_matrix_updates() {
    let mut engine = MaintenanceOperatingModelEngine::new();
    assert_eq!(engine.hardware_matrix.len(), 8);

    // Default status is CodeExists
    assert!(!engine.hardware_matrix[0].status.is_verified());

    engine.update_hardware_evidence(
        HardwareDriverClass::Serial,
        TaskResultStatus::SystemBehaviorVerified,
        Some("reports/hardware/serial_qemu.log".to_string()),
        "Verified 16550 UART serial boot logging under QEMU".to_string(),
    );

    let serial_entry = engine
        .hardware_matrix
        .iter()
        .find(|e| e.driver_class == HardwareDriverClass::Serial)
        .unwrap();
    assert!(serial_entry.status.is_verified());
    assert_eq!(
        serial_entry.evidence_path.as_deref(),
        Some("reports/hardware/serial_qemu.log")
    );
}

#[test]
fn test_release_tier_readiness_gates() {
    let mut engine = MaintenanceOperatingModelEngine::new();

    // Verify required_evidence descriptions
    assert!(ReleaseTier::Experimental.required_evidence().contains("Build and unit tests pass"));
    assert!(ReleaseTier::DeveloperPreview.required_evidence().contains("shell/diagnostic"));
    assert!(ReleaseTier::TestDistribution.required_evidence().contains("VM installation"));
    assert!(ReleaseTier::ReleaseCandidate.required_evidence().contains("Full regression"));
    assert!(ReleaseTier::Stable.required_evidence().contains("Hardware matrix"));

    // Experimental is ready by default
    assert!(engine.evaluate_release_readiness(ReleaseTier::Experimental));
    assert!(!engine.evaluate_release_readiness(ReleaseTier::DeveloperPreview));

    // Record boot validation
    engine
        .record_task_completion(
            "monthly_boot",
            TaskArtifact::Report {
                title: "QEMU Boot Validation".to_string(),
                path: "reports/boot/qemu.log".to_string(),
            },
            TaskResultStatus::SystemBehaviorVerified,
        )
        .unwrap();

    assert!(engine.evaluate_release_readiness(ReleaseTier::DeveloperPreview));
    assert!(!engine.evaluate_release_readiness(ReleaseTier::TestDistribution));

    // Record installer rehearsal
    engine
        .record_task_completion(
            "monthly_installer",
            TaskArtifact::Report {
                title: "Installer VM Rehearsal".to_string(),
                path: "reports/installer/vm_rehearsal.log".to_string(),
            },
            TaskResultStatus::SystemBehaviorVerified,
        )
        .unwrap();

    assert!(engine.evaluate_release_readiness(ReleaseTier::TestDistribution));
    assert!(!engine.evaluate_release_readiness(ReleaseTier::ReleaseCandidate));
    assert!(!engine.evaluate_release_readiness(ReleaseTier::Stable));

    // Mark all hardware classes as verified
    let classes = [
        HardwareDriverClass::Serial,
        HardwareDriverClass::Rtc,
        HardwareDriverClass::Display,
        HardwareDriverClass::Input,
        HardwareDriverClass::Storage,
        HardwareDriverClass::Usb,
        HardwareDriverClass::Network,
        HardwareDriverClass::Audio,
    ];

    for c in classes {
        engine.update_hardware_evidence(
            c,
            TaskResultStatus::SystemBehaviorVerified,
            Some("reports/hardware/audit.log".to_string()),
            "Verified on QEMU / Bare-metal target".to_string(),
        );
    }

    assert!(engine.evaluate_release_readiness(ReleaseTier::ReleaseCandidate));
    assert!(engine.evaluate_release_readiness(ReleaseTier::Stable));
}
