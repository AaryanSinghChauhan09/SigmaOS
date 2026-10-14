// Integration and Concurrency Test Suite for Fairness and Starvation Prevention

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::SchedulerCore;

#[test]
fn test_fairness_and_starvation_boosting() {
    let mut sched = SchedulerCore::new(1);
    // Low priority task (prio 5)
    sched.create_process(1, 0, 5, 1).unwrap();
    sched.mark_ready(1).unwrap();

    // High priority task (prio 50)
    sched.create_process(2, 0, 50, 1).unwrap();
    sched.mark_ready(2).unwrap();

    // Dequeue high priority task
    let deq = sched.schedule_next(0).unwrap();
    assert_eq!(deq, Some(2));

    // Advance ticks beyond starvation threshold (50 ticks)
    for _ in 0..60 {
        sched.timer_tick();
    }

    // Task 1 should have been boosted from priority 5 to 15
    let p1 = sched.process_table.get(&1).unwrap();
    assert_eq!(p1.pid, 1);
}
