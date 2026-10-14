// Integration and Concurrency Test Suite for CPU Affinity Governance

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore, SchedulerError};

#[test]
fn test_cpu_affinity_restrictions() {
    let mut sched = SchedulerCore::new(2);

    // Task restricted to CPU core 1 (bitmask 0b10 = 2)
    sched.create_process(1, 0, 10, 0b10).unwrap();
    sched.mark_ready(1).unwrap();

    // Scheduling on CPU 0 should not pick task 1
    let task_on_cpu0 = sched.schedule_next(0).unwrap();
    assert_eq!(task_on_cpu0, None);

    // Scheduling on CPU 1 picks task 1
    let task_on_cpu1 = sched.schedule_next(1).unwrap();
    assert_eq!(task_on_cpu1, Some(1));
}

#[test]
fn test_affinity_update() {
    let mut sched = SchedulerCore::new(2);
    sched.create_process(1, 0, 10, 0b01).unwrap();

    let pcb = sched.process_table.get_mut(&1).unwrap();
    pcb.cpu_affinity = 0b11;
    assert_eq!(sched.process_table.get(&1).unwrap().cpu_affinity, 0b11);
}
