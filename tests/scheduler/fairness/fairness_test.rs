// SigmaOS Fairness & Starvation Prevention Test Suite

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::SchedulerCore;

#[test]
fn test_starvation_prevention_boost() {
    let mut sched = SchedulerCore::new(1);

    // Create low priority process (prio 5)
    sched.create_process(50, 0, 5, 1).unwrap();
    sched.mark_ready(50).unwrap();

    // Advance ticks beyond starvation threshold (50 ticks)
    for _ in 0..60 {
        sched.timer_tick();
    }

    // Task priority boosted to 15
    let task = sched.run_queues[0].dequeue_next().unwrap();
    assert_eq!(task.pid, 50);
    assert_eq!(task.priority, 15);
}

#[test]
fn test_fairness_metric_vruntime() {
    let mut sched = SchedulerCore::new(1);

    sched.create_process(1, 0, 20, 1).unwrap();
    sched.mark_ready(1).unwrap();

    sched.schedule_next(0).unwrap();

    // Ticks accumulate vruntime
    for _ in 0..5 {
        sched.schedule_next(0).unwrap();
    }

    assert_eq!(sched.process_table.get(&1).unwrap().vruntime, 5);
}
