//! Integration test for SigmaOS Kernel Subsystem Contracts & Dependencies

#[path = "../../../src/governance/kernel_lifecycle_program.rs"]
mod kernel_lifecycle_program;

use kernel_lifecycle_program::{BootState, KernelLifecycleProgramEngine, ReleaseGate};

#[test]
fn test_kernel_subsystem_contract_verification() {
    let engine = KernelLifecycleProgramEngine::new();
    let tasks = engine.tasks();
    assert!(!tasks.is_empty(), "Kernel tasks calendar must not be empty");

    let full_boot_seq = vec![
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

    assert!(engine.verify_boot_sequence(&full_boot_seq), "Boot state sequence verification failed");

    let all_gates = vec![
        ReleaseGate::G0CodeQuality,
        ReleaseGate::G1UnitIntegration,
        ReleaseGate::G2MemorySafety,
        ReleaseGate::G3BootSmoke,
        ReleaseGate::G4Recoverability,
        ReleaseGate::G5SecurityBaseline,
        ReleaseGate::G6HardwareEvidence,
        ReleaseGate::G7ReleaseNotesAudit,
    ];

    assert!(engine.evaluate_release_gates(&all_gates), "Release gates evaluation failed");
}
