// Tests for preemption points, quantum expiration, and voluntary yield

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[test]
fn test_time_slice_quantum_expiration_and_round_robin() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(1, 0, 10, 1).unwrap();
    sched.create_process(2, 0, 10, 1).unwrap();
    sched.mark_ready(1).unwrap();
    sched.mark_ready(2).unwrap();

    let scheduled = sched.schedule_next(0).unwrap();
    assert_eq!(scheduled, Some(1));
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Running);

    // Exhaust remaining quantum ticks (10 ticks) for process 1
    for _ in 0..10 {
        sched.schedule_next(0).unwrap();
    }

    // Call 11 causes process 1's quantum to expire, yielding to process 2
    let next_scheduled = sched.schedule_next(0).unwrap();
    assert_eq!(next_scheduled, Some(2));
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Ready);
    assert_eq!(sched.process_table.get(&2).unwrap().state, ProcessState::Running);
}

#[test]
fn test_voluntary_yield_preemption() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(1, 0, 10, 1).unwrap();
    sched.create_process(2, 0, 10, 1).unwrap();
    sched.mark_ready(1).unwrap();
    sched.mark_ready(2).unwrap();

    let p1 = sched.schedule_next(0).unwrap();
    assert_eq!(p1, Some(1));

    // Put process 1 to sleep or stop to yield CPU
    sched.stop_process(1).unwrap();

    let p2 = sched.schedule_next(0).unwrap();
    assert_eq!(p2, Some(2));
}
