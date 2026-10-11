// Standalone Unit Test Suite for SigmaOS Scheduler Core & Run Queue Subsystem

#[path = "../../src/scheduler/core.rs"]
mod core;
#[path = "../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};
use run_queue::{BoundedRunQueue, RunQueueError, TaskRunRef};

#[test]
fn test_bounded_run_queue_basic() {
    let mut rq = BoundedRunQueue::new(0, 10);
    assert!(rq.is_empty());
    assert_eq!(rq.len(), 0);

    let task1 = TaskRunRef::new(100, 10, 1, 0);
    let task2 = TaskRunRef::new(101, 20, 1, 0);

    assert!(rq.enqueue(task1, 0).is_ok());
    assert!(rq.enqueue(task2, 0).is_ok());
    assert_eq!(rq.len(), 2);
    assert!(rq.contains(100));
    assert!(rq.contains(101));

    // High priority task2 (prio 20) should dequeue before task1 (prio 10)
    let deq = rq.dequeue_next().unwrap();
    assert_eq!(deq.pid, 101);

    let deq2 = rq.dequeue_next().unwrap();
    assert_eq!(deq2.pid, 100);

    assert!(rq.is_empty());
}

#[test]
fn test_bounded_run_queue_capacity_limit() {
    let mut rq = BoundedRunQueue::new(0, 3);
    assert!(rq.enqueue(TaskRunRef::new(1, 10, 1, 0), 0).is_ok());
    assert!(rq.enqueue(TaskRunRef::new(2, 10, 1, 0), 0).is_ok());
    assert!(rq.enqueue(TaskRunRef::new(3, 10, 1, 0), 0).is_ok());

    // Exceed capacity
    let res = rq.enqueue(TaskRunRef::new(4, 10, 1, 0), 0);
    assert_eq!(res, Err(RunQueueError::CapacityExceeded));
}

#[test]
fn test_scheduler_process_lifecycle() {
    let mut sched = SchedulerCore::new(2);

    // Create process
    assert!(sched.create_process(1, 0, 10, 1).is_ok());
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::New);

    // Transition NEW -> READY
    assert!(sched.mark_ready(1).is_ok());
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Ready);

    // Schedule: READY -> RUNNING
    let next = sched.schedule_next(0).unwrap();
    assert_eq!(next, Some(1));
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Running);

    // Sleep: RUNNING -> SLEEPING
    assert!(sched.sleep_process(1).is_ok());
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Sleeping);

    // Wake: SLEEPING -> READY
    assert!(sched.wake_process(1).is_ok());
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Ready);

    // Re-schedule: READY -> RUNNING
    let next2 = sched.schedule_next(0).unwrap();
    assert_eq!(next2, Some(1));

    // Stop process test
    assert!(sched.stop_process(1).is_ok());
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Stopped);

    // Resume from STOPPED -> READY -> RUNNING
    assert!(sched.mark_ready(1).is_ok());
    let _ = sched.schedule_next(0).unwrap();

    // Exit: RUNNING -> ZOMBIE
    assert!(sched.exit_process(1, 42).is_ok());
    assert_eq!(sched.process_table.get(&1).unwrap().state, ProcessState::Zombie);

    // Cannot reap non-zombie checks, then reap: ZOMBIE -> REAPED
    let exit_code = sched.reap_process(1).unwrap();
    assert_eq!(exit_code, 42);
    assert!(!sched.process_table.contains_key(&1));
}

#[test]
fn test_lost_wakeup_prevention() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(10, 0, 10, 1).unwrap();
    sched.mark_ready(10).unwrap();
    sched.schedule_next(0).unwrap(); // Now RUNNING

    // Send wake event before process goes to sleep
    sched.wake_process(10).unwrap();
    assert!(sched.process_table.get(&10).unwrap().wake_event_pending);

    // Attempt sleep: should detect pending event and stay READY instead of sleeping!
    sched.sleep_process(10).unwrap();
    assert_eq!(sched.process_table.get(&10).unwrap().state, ProcessState::Ready);
    assert!(!sched.process_table.get(&10).unwrap().wake_event_pending);
}

#[test]
fn test_starvation_boosting() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(5, 0, 5, 1).unwrap();
    sched.mark_ready(5).unwrap();

    // Timer ticks beyond starvation threshold
    for _ in 0..60 {
        sched.timer_tick();
    }

    // Task priority should have been boosted
    let deq = sched.run_queues[0].dequeue_next().unwrap();
    assert_eq!(deq.pid, 5);
    assert_eq!(deq.priority, 15);
}

#[test]
fn test_orphan_reparenting() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(1, 0, 10, 1).unwrap(); // Parent 1 (init)
    sched.create_process(10, 1, 10, 1).unwrap(); // Parent 10
    sched.create_process(20, 10, 10, 1).unwrap(); // Child 20 (ppid 10)

    // Process 10 exits
    sched.exit_process(10, 0).unwrap();

    // Child 20 should now be reparented to init (PID 1)
    assert_eq!(sched.process_table.get(&20).unwrap().ppid, 1);
}
