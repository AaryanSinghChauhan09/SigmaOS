// Integration and Concurrency Test Suite for Preemption Safety

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[test]
fn test_preemption_on_quantum_expiry() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(1, 0, 10, 1).unwrap();
    sched.create_process(2, 0, 10, 1).unwrap();

    sched.mark_ready(1).unwrap();
    sched.mark_ready(2).unwrap();

    // First task scheduled
    let p1 = sched.schedule_next(0).unwrap();
    assert_eq!(p1, Some(1));

    // Exhaust quantum (10 steps)
    for _ in 0..10 {
        let _ = sched.schedule_next(0);
    }

    // 11th step triggers quantum expiry preemption; task 2 is scheduled, task 1 moves back to READY
    let p2 = sched.schedule_next(0).unwrap();
    assert_eq!(p2, Some(2));
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Ready);
    assert_eq!(sched.process_table.get(&2).unwrap().state, ProcessState::Running);
}
