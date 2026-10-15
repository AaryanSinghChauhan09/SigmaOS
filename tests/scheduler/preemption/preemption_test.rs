// SigmaOS Preemption Safety & Quantum Expiry Test Suite

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[test]
fn test_preemption_time_slice_expiry() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(10, 0, 15, 1).unwrap();
    sched.create_process(11, 0, 15, 1).unwrap();

    sched.mark_ready(10).unwrap();
    sched.mark_ready(11).unwrap();

    // Schedule task 10 or 11
    let pid1 = sched.schedule_next(0).unwrap().unwrap();

    // Consume quantum
    for _ in 0..10 {
        sched.schedule_next(0).unwrap();
    }

    // Next schedule tick preempts pid1 and schedules the other process
    let pid2 = sched.schedule_next(0).unwrap().unwrap();
    assert_ne!(pid1, pid2);
    assert_eq!(sched.process_table.get(&pid1).unwrap().state, ProcessState::Ready);
    assert_eq!(sched.process_table.get(&pid2).unwrap().state, ProcessState::Running);
}

#[test]
fn test_voluntary_yield_preemption() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(1, 0, 20, 1).unwrap();
    sched.mark_ready(1).unwrap();
    sched.schedule_next(0).unwrap(); // Running

    // Voluntary yield via stop or sleep
    sched.stop_process(1).unwrap();
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Stopped);

    // Core is now clear
    let next = sched.schedule_next(0).unwrap();
    assert_eq!(next, None);
}
