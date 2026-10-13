// Preemption Policy Test Suite for SigmaOS Scheduler

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[test]
fn test_quantum_expiration_preemption() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(1, 0, 10, 1).unwrap();
    sched.create_process(2, 0, 10, 1).unwrap();

    sched.mark_ready(1).unwrap();
    sched.mark_ready(2).unwrap();

    // Dispatch task 1 (READY -> RUNNING, sets remaining_quantum_ticks = 10)
    let active = sched.schedule_next(0).unwrap();
    assert_eq!(active, Some(1));
    assert_eq!(sched.process_table.get(&1).unwrap().remaining_quantum_ticks, 10);

    // Consume 10 ticks (quantum = 10)
    for _ in 0..10 {
        let current = sched.schedule_next(0).unwrap();
        assert_eq!(current, Some(1));
    }

    // On 11th call, quantum has hit 0, task 1 yields to READY, task 2 dispatched!
    let next = sched.schedule_next(0).unwrap();
    assert_eq!(next, Some(2));
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Ready);
    assert_eq!(sched.process_table.get(&2).unwrap().state, ProcessState::Running);
}

#[test]
fn test_nested_priority_preemption() {
    let mut sched = SchedulerCore::new(1);

    // Create low priority (prio 10) and high priority (prio 50) tasks
    sched.create_process(10, 0, 10, 1).unwrap();
    sched.create_process(50, 0, 50, 1).unwrap();

    sched.mark_ready(10).unwrap();
    sched.mark_ready(50).unwrap();

    // High priority task 50 must always be scheduled before low priority task 10
    let scheduled = sched.schedule_next(0).unwrap();
    assert_eq!(scheduled, Some(50));
}

#[test]
fn test_voluntary_yield_preemption() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(100, 0, 20, 1).unwrap();
    sched.mark_ready(100).unwrap();
    sched.schedule_next(0).unwrap(); // Running

    // Voluntary yield by putting process to sleep or stopping
    sched.stop_process(100).unwrap();
    assert_eq!(sched.process_table.get(&100).unwrap().state, ProcessState::Stopped);

    let next = sched.schedule_next(0).unwrap();
    assert_eq!(next, None);
}
