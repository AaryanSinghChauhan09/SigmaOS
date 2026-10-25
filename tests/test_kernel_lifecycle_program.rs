// SPDX-License-Identifier: MIT
// Standalone integration test suite for SigmaOS Kernel Lifecycle Program Engine

#[path = "../src/governance/kernel_lifecycle_program.rs"]
mod kernel_lifecycle_program;

use kernel_lifecycle_program::*;

#[test]
fn test_kernel_lifecycle_task_cadences() {
    let engine = KernelLifecycleProgramEngine::new();
    assert_eq!(engine.tasks().len(), 14);

    let pr_tasks: Vec<_> = engine
        .tasks()
        .iter()
        .filter(|t| t.cadence == KernelTaskCadence::EveryPr)
        .collect();
    assert_eq!(pr_tasks.len(), 1);
    assert_eq!(pr_tasks[0].required_pr_prefix, "fix/feat/test/chore");

    let weekly_tasks: Vec<_> = engine
        .tasks()
        .iter()
        .filter(|t| t.cadence == KernelTaskCadence::Weekly)
        .collect();
    assert_eq!(weekly_tasks.len(), 2);

    let monthly_tasks: Vec<_> = engine
        .tasks()
        .iter()
        .filter(|t| t.cadence == KernelTaskCadence::Monthly)
        .collect();
    assert_eq!(monthly_tasks.len(), 4);
}

#[test]
fn test_boot_sequence_verification_11_states() {
    let engine = KernelLifecycleProgramEngine::new();
    let valid_states = vec![
        BootState::Firmware,
        BootState::Bootloader,
        BootState::KernelEntry,
        BootState::ArchInit,
        BootState::MemoryInit,
        BootState::InterruptInit,
        BootState::SchedulerInit,
        BootState::VfsInit,
        BootState::DriverInit,
        BootState::UserspaceStart,
        BootState::ShellReady,
    ];

    assert!(engine.verify_boot_sequence(&valid_states));

    // Test out of order sequence
    let invalid_states = vec![
        BootState::Firmware,
        BootState::Bootloader,
        BootState::KernelEntry,
        BootState::MemoryInit, // Paging skipping ArchInit
        BootState::ArchInit,
        BootState::InterruptInit,
        BootState::SchedulerInit,
        BootState::VfsInit,
        BootState::DriverInit,
        BootState::UserspaceStart,
        BootState::ShellReady,
    ];
    assert!(!engine.verify_boot_sequence(&invalid_states));
}

#[test]
fn test_release_gates_g0_g7_evaluation() {
    let engine = KernelLifecycleProgramEngine::new();
    let gates = vec![
        ReleaseGate::G0CodeQuality,
        ReleaseGate::G1UnitIntegration,
        ReleaseGate::G2MemorySafety,
        ReleaseGate::G3BootSmoke,
        ReleaseGate::G4Recoverability,
        ReleaseGate::G5SecurityBaseline,
        ReleaseGate::G6HardwareEvidence,
        ReleaseGate::G7ReleaseNotesAudit,
    ];

    assert!(engine.evaluate_release_gates(&gates));
    assert_eq!(ReleaseGate::G0CodeQuality.code(), "G0");
    assert_eq!(ReleaseGate::G3BootSmoke.code(), "G3");
    assert_eq!(ReleaseGate::G7ReleaseNotesAudit.code(), "G7");
}

#[test]
fn test_health_metrics_evaluation() {
    let engine = KernelLifecycleProgramEngine::new();
    let metrics = engine.current_health_metrics();

    assert!(metrics.is_healthy());
    assert_eq!(metrics.main_ci_pass_rate_pct, 100);
    assert_eq!(metrics.boot_to_shell_pass_rate_pct, 100);
    assert_eq!(metrics.pr_with_tests_pct, 100);
    assert_eq!(metrics.unsafe_documented_pct, 100);
    assert_eq!(metrics.syscall_contracts_pct, 100);
    assert_eq!(metrics.parser_fuzz_targets_pct, 100);
    assert_eq!(metrics.driver_evidence_backed_pct, 100);
    assert_eq!(metrics.diagnostic_output_subsystems_pct, 100);
}
