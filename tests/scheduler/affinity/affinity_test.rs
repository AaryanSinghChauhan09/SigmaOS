// CPU Affinity & NUMA Topology Test Suite for SigmaOS Scheduler

pub mod klib {
    pub use std::collections::HashMap;
}

#[path = "../../../src/scheduler/affinity.rs"]
mod affinity;
#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use affinity::{CpuAffinityMask, ProcessCpuAssigner};
use core::{ProcessState, SchedulerCore};

#[test]
fn test_cpu_affinity_mask_operations() {
    let mut mask = CpuAffinityMask::from_cores(&[0, 2, 4]);
    assert_eq!(mask.count_assigned_cores(), 3);
    assert!(mask.is_core_set(0));
    assert!(!mask.is_core_set(1));
    assert!(mask.is_core_set(2));

    mask.set_core(1);
    assert!(mask.is_core_set(1));
    assert_eq!(mask.count_assigned_cores(), 4);

    mask.clear_core(0);
    assert!(!mask.is_core_set(0));
}

#[test]
fn test_process_cpu_assigner_and_optimal_core_selection() {
    let mut assigner = ProcessCpuAssigner::new(16);

    // Assign PID 100 affinity to CPU cores 0 and 1
    let mask = CpuAffinityMask::from_cores(&[0, 1]);
    assigner.assign_process_affinity(100, mask).unwrap();

    let queried_mask = assigner.get_process_affinity(100);
    assert_eq!(queried_mask, mask);

    // Core loads: Core 0 has load 80, Core 1 has load 20
    let core_loads = vec![80, 20, 5, 0];
    let optimal = assigner.select_optimal_core_for_task(100, &core_loads);
    assert_eq!(optimal, 1); // Selects core 1 (lowest load among assigned cores 0 & 1)

    // Thread pinning
    assert!(assigner.pin_thread_to_core(500, 3).is_ok());
    assert_eq!(assigner.thread_pinned_core.get(&500), Some(&3));
}

#[test]
fn test_scheduler_core_affinity_dispatch() {
    let mut sched = SchedulerCore::new(4);

    // Create task restricted to CPU core 2 (affinity mask = 0b0100 = 4)
    sched.create_process(30, 0, 20, 4).unwrap();
    sched.mark_ready(30).unwrap();

    let pcb = sched.process_table.get(&30).unwrap();
    assert_eq!(pcb.state, ProcessState::Ready);
    assert_eq!(pcb.current_cpu, 2);
}
