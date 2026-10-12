// Tests for fairness metrics and starvation prevention

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::SchedulerCore;

#[test]
fn test_fairness_and_starvation_prevention() {
    let mut sched = SchedulerCore::new(1);

    // Process 1: low priority (prio 5)
    sched.create_process(1, 0, 5, 1).unwrap();
    sched.mark_ready(1).unwrap();

    // Process 2: high priority (prio 50)
    sched.create_process(2, 0, 50, 1).unwrap();
    sched.mark_ready(2).unwrap();

    // Advance timer ticks beyond starvation threshold (50 ticks)
    for _ in 0..55 {
        sched.timer_tick();
    }

    // Process 1 should have been boosted in priority (from 5 to 15) in the run queue to prevent starvation
    let task1 = sched.run_queues[0].remove_task(1).unwrap();
    assert_eq!(task1.priority, 15);
}
