// Integration and Concurrency Test Suite for Context Switch Integrity

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[test]
fn test_context_switch_integrity() {
    let mut sched = SchedulerCore::new(1);
    assert!(sched.create_process(1, 0, 10, 1).is_ok());
    assert!(sched.mark_ready(1).is_ok());

    // Switch context to task 1
    let next_pid = sched.schedule_next(0).unwrap();
    assert_eq!(next_pid, Some(1));
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Running);

    // Yield/Sleep context
    assert!(sched.sleep_process(1).is_ok());
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Sleeping);
}
