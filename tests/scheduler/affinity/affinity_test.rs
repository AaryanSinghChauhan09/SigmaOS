// Tests for CPU affinity mask enforcement and migration

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{SchedulerCore, SchedulerError};

#[test]
fn test_cpu_affinity_mask_enforcement() {
    let mut sched = SchedulerCore::new(4);

    // Process pinned strictly to CPU 2 (affinity mask: 1 << 2 = 4)
    sched.create_process(1, 0, 10, 1 << 2).unwrap();
    sched.mark_ready(1).unwrap();

    let pcb = sched.process_table.get(&1).unwrap();
    assert_eq!(pcb.current_cpu, 2);

    // Scheduling on CPU 0 should yield no task
    let next_cpu0 = sched.schedule_next(0).unwrap();
    assert_eq!(next_cpu0, None);

    // Scheduling on CPU 2 should yield Process 1
    let next_cpu2 = sched.schedule_next(2).unwrap();
    assert_eq!(next_cpu2, Some(1));
}

#[test]
fn test_cpu_affinity_migration() {
    let mut sched = SchedulerCore::new(4);

    // Process initially on CPU 0 (affinity mask: 1)
    sched.create_process(1, 0, 10, 1).unwrap();
    sched.mark_ready(1).unwrap();

    // Migrate CPU affinity to CPU 3 (affinity mask: 1 << 3 = 8)
    sched.set_cpu_affinity(1, 1 << 3).unwrap();

    let pcb = sched.process_table.get(&1).unwrap();
    assert_eq!(pcb.current_cpu, 3);
}

#[test]
fn test_invalid_cpu_affinity_rejected() {
    let mut sched = SchedulerCore::new(2);
    assert_eq!(sched.create_process(1, 0, 10, 0).is_ok(), true); // 0 normalized to 1
    assert_eq!(sched.set_cpu_affinity(1, 0), Err(SchedulerError::InvalidCpuAffinity));
}
