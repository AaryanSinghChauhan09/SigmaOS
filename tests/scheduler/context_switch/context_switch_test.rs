// Tests for context switch register, stack, address-space, and state integrity

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore, SchedulerError};

#[test]
fn test_context_switch_state_preservation() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(1, 0, 10, 1).unwrap();
    sched.mark_ready(1).unwrap();

    let scheduled = sched.schedule_next(0).unwrap();
    assert_eq!(scheduled, Some(1));

    let pcb = sched.process_table.get(&1).unwrap();
    assert_eq!(pcb.state, ProcessState::Running);
    assert_eq!(pcb.current_cpu, 0);
}

#[test]
fn test_double_switch_prevention() {
    let mut sched = SchedulerCore::new(2);
    sched.create_process(1, 0, 10, 1).unwrap();
    sched.mark_ready(1).unwrap();

    let next1 = sched.schedule_next(0).unwrap();
    assert_eq!(next1, Some(1));

    // Process 1 is already RUNNING on CPU 0; attempting to schedule on CPU 1 should not schedule it twice
    let next2 = sched.schedule_next(1).unwrap();
    assert_eq!(next2, None);
}
