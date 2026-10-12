//! Standalone I/O Scheduler Integration & Unit Tests

#[path = "../../src/io/mod.rs"]
pub mod io;

use io::{
    IoOperationKind, IoPriority, IoScheduler, IoStage, SchedulerError,
};

#[test]
fn test_scheduler_full_10_stage_lifecycle() {
    let mut scheduler = IoScheduler::new(8);

    let req_id = scheduler
        .submit_request(1, IoOperationKind::Read, 1000, 4096, IoPriority::RealTime)
        .expect("Submit should succeed");

    assert_eq!(scheduler.pending_count(), 1);

    let req = scheduler.dispatch_next().expect("Should dispatch pending RT request");
    assert_eq!(req.id, req_id);
    assert_eq!(req.stage, IoStage::Execute);
    assert_eq!(scheduler.active_count(), 1);

    let final_stage = scheduler.complete_request(req_id, Ok(())).expect("Completion should succeed");
    assert_eq!(final_stage, IoStage::Free);
    assert_eq!(scheduler.active_count(), 0);
}

#[test]
fn test_scheduler_queue_overflow() {
    let mut scheduler = IoScheduler::new(1);
    let _ = scheduler.submit_request(1, IoOperationKind::Write, 500, 512, IoPriority::BestEffort).unwrap();

    let res = scheduler.submit_request(1, IoOperationKind::Write, 501, 512, IoPriority::BestEffort);
    assert_eq!(res, Err(SchedulerError::QueueFull));
}
