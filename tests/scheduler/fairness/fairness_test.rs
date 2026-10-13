// Fairness & Starvation Prevention Test Suite for SigmaOS Scheduler

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[test]
fn test_starvation_prevention_boosting() {
    let mut sched = SchedulerCore::new(1);

    // Create a low priority background process (prio 5)
    sched.create_process(99, 0, 5, 1).unwrap();
    sched.mark_ready(99).unwrap();

    let initial_prio = sched.process_table.get(&99).unwrap().priority;
    assert_eq!(initial_prio, 5);

    // Simulate timer ticks beyond default starvation threshold (50 ticks)
    for _ in 0..60 {
        sched.timer_tick();
    }

    // Starved process in run queue should be boosted by +10 priority levels
    let deq_task = sched.run_queues[0].dequeue_next().unwrap();
    assert_eq!(deq_task.pid, 99);
    assert_eq!(deq_task.priority, 15);
}

#[test]
fn test_vruntime_accounting_fairness() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(1, 0, 20, 1).unwrap();
    sched.mark_ready(1).unwrap();

    assert_eq!(sched.process_table.get(&1).unwrap().vruntime, 0);

    // Dispatch task 1 (READY -> RUNNING)
    sched.schedule_next(0).unwrap();

    // Run task 1 for 5 ticks while RUNNING
    for _ in 0..5 {
        sched.schedule_next(0).unwrap();
    }

    let pcb = sched.process_table.get(&1).unwrap();
    assert_eq!(pcb.vruntime, 5);
    assert_eq!(pcb.total_scheduled_ticks, 5);
    assert_eq!(pcb.state, ProcessState::Running);
}
