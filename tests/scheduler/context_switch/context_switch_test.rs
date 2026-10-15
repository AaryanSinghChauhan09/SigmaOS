// SigmaOS Context Switch Subsystem Integrity Test Suite

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[test]
fn test_context_switch_active_pid_tracking() {
    let mut sched = SchedulerCore::new(2);

    sched.create_process(100, 0, 20, 1).unwrap();
    sched.create_process(101, 0, 20, 1).unwrap();

    sched.mark_ready(100).unwrap();
    sched.mark_ready(101).unwrap();

    // CPU 0 schedules task 100 or 101
    let scheduled_pid = sched.schedule_next(0).unwrap();
    assert!(scheduled_pid.is_some());
    let scheduled_pid = scheduled_pid.unwrap();

    assert_eq!(sched.active_pids[0], Some(scheduled_pid));
    assert_eq!(sched.process_table.get(&scheduled_pid).unwrap().state, ProcessState::Running);
}

#[test]
fn test_context_switch_quantum_expiry() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(200, 0, 20, 1).unwrap();
    sched.create_process(201, 0, 20, 1).unwrap();
    sched.mark_ready(200).unwrap();
    sched.mark_ready(201).unwrap();

    let p = sched.schedule_next(0).unwrap().unwrap();
    assert_eq!(p, 200);

    // Run down quantum (10 ticks)
    for _ in 0..10 {
        let res = sched.schedule_next(0).unwrap();
        assert_eq!(res, Some(200));
    }

    // 11th call triggers quantum expiration -> 200 yields to READY and 201 is scheduled
    let p2 = sched.schedule_next(0).unwrap().unwrap();
    assert_eq!(p2, 201);
    assert_eq!(sched.process_table.get(&200).unwrap().state, ProcessState::Ready);
    assert_eq!(sched.process_table.get(&201).unwrap().state, ProcessState::Running);
}

#[test]
fn test_lost_wakeup_context_switch_safety() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(300, 0, 10, 1).unwrap();
    sched.mark_ready(300).unwrap();
    sched.schedule_next(0).unwrap(); // Running

    // Event wakes task before sleep
    sched.wake_process(300).unwrap();

    // Sleep call should remain READY due to pending wake event
    sched.sleep_process(300).unwrap();
    assert_eq!(sched.process_table.get(&300).unwrap().state, ProcessState::Ready);
}
