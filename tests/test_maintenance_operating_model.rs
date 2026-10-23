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

    assert!(engine.evaluate_release_readiness(ReleaseTier::Stable));
}
