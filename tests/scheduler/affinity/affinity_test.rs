// SigmaOS CPU Affinity & Core Load Balancing Test Suite

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{SchedulerCore, SchedulerError};

#[test]
fn test_affinity_mask_routing() {
    let mut sched = SchedulerCore::new(4); // 4 CPU cores

    // Affinity mask 0b0100 (CPU 2 only)
    sched.create_process(10, 0, 20, 0b0100).unwrap();
    sched.mark_ready(10).unwrap();

    assert!(sched.run_queues[2].contains(10));
    assert!(!sched.run_queues[0].contains(10));
}

#[test]
fn test_affinity_load_balancing() {
    let mut sched = SchedulerCore::new(2); // 2 CPU cores

    // Both CPUs permitted (0b0011)
    sched.create_process(1, 0, 20, 0b0011).unwrap();
    sched.mark_ready(1).unwrap(); // Goes to CPU 0

    sched.create_process(2, 0, 20, 0b0011).unwrap();
    sched.mark_ready(2).unwrap(); // CPU 0 has load 1, so goes to CPU 1

    assert_eq!(sched.run_queues[0].len(), 1);
    assert_eq!(sched.run_queues[1].len(), 1);
}

#[test]
fn test_invalid_affinity_rejection() {
    let mut sched = SchedulerCore::new(2);

    // Mask for CPU 4 (0b10000) on 2-CPU system -> InvalidCpuAffinity
    sched.create_process(99, 0, 20, 0b10000).unwrap();
    let res = sched.mark_ready(99);
    assert_eq!(res, Err(SchedulerError::InvalidCpuAffinity));
}
