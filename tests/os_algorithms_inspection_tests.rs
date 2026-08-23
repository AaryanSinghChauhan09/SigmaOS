// SigmaOS Core OS & Component Algorithms Inspection Test Suite
// Verifies working mechanisms across key operating system algorithms:
// - Earliest Deadline First (EDF) Real-Time Scheduling Algorithm
// - Probabilistic Lottery Scheduling Algorithm
// - LCG Pseudo-Random Draw Engine
// - Consensus-Based Process Audit Ledger (Cryptographic Chained Blocks)

#[path = "../src/kernel/structures.rs"]
mod structures;

use structures::*;

#[test]
fn test_edf_realtime_scheduling_algorithm_inspection() {
    let manager = AdvancedAlgorithmsManager::new(12345);

    let task1 = EdfTask {
        tid: 101,
        absolute_deadline: 1000,
        remaining_execution: 10,
        is_active: true,
    };
    let task2 = EdfTask {
        tid: 102,
        absolute_deadline: 500, // Urgenter deadline
        remaining_execution: 5,
        is_active: true,
    };

    assert!(manager.add_edf_task(task1).is_ok());
    assert!(manager.add_edf_task(task2).is_ok());

    let scheduled = manager.schedule_edf().unwrap();
    assert_eq!(scheduled.tid, 102); // Task 2 scheduled due to earlier deadline
}

#[test]
fn test_lottery_scheduling_algorithm_inspection() {
    let manager = AdvancedAlgorithmsManager::new(99999);

    let task1 = LotteryTask {
        tid: 201,
        tickets: 70,
        is_active: true,
    };
    let task2 = LotteryTask {
        tid: 202,
        tickets: 30,
        is_active: true,
    };

    assert!(manager.add_lottery_task(task1).is_ok());
    assert!(manager.add_lottery_task(task2).is_ok());

    let scheduled = manager.schedule_lottery();
    assert!(scheduled.is_some());
}

#[test]
fn test_process_audit_ledger_chain_integrity_inspection() {
    let manager = AdvancedAlgorithmsManager::new(55555);

    assert!(manager.audit_process_event(1000, 1, 501, 0xABC).is_ok());
    assert!(manager.audit_process_event(1001, 2, 502, 0xDEF).is_ok());

    assert!(manager.verify_ledger_integrity());
}
