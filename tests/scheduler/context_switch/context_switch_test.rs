// Context Switch Integrity & Safety Test Suite for SigmaOS Scheduler

#[path = "../../../src/scheduler/core.rs"]
mod core;
#[path = "../../../src/scheduler/run_queue.rs"]
mod run_queue;

use core::{ProcessState, SchedulerCore};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuRegisterContext {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub cr3: u64,
}

impl CpuRegisterContext {
    pub fn new_simulated(pid: u64, stack_ptr: u64, entry_point: u64, page_table: u64) -> Self {
        Self {
            rax: 0x1111_0000 | pid,
            rbx: 0x2222_0000 | pid,
            rcx: 0x3333_0000 | pid,
            rdx: 0x4444_0000 | pid,
            rsi: 0x5555_0000 | pid,
            rdi: 0x6666_0000 | pid,
            rbp: stack_ptr + 0x1000,
            rsp: stack_ptr,
            rip: entry_point,
            cr3: page_table,
        }
    }
}

#[test]
fn test_context_switch_state_and_register_integrity() {
    let mut sched = SchedulerCore::new(2);

    sched.create_process(101, 1, 20, 1).unwrap();
    sched.create_process(102, 1, 20, 1).unwrap();

    let ctx1 = CpuRegisterContext::new_simulated(101, 0x7FFF_0000, 0x4000_1000, 0x1A00_0000);
    let ctx2 = CpuRegisterContext::new_simulated(102, 0x7FFF_8000, 0x4000_2000, 0x1B00_0000);

    sched.mark_ready(101).unwrap();
    sched.mark_ready(102).unwrap();

    // Context switch 1: Dispatch PID 101
    let scheduled = sched.schedule_next(0).unwrap();
    assert_eq!(scheduled, Some(101));
    assert_eq!(sched.process_table.get(&101).unwrap().state, ProcessState::Running);

    // Verify register state invariants
    assert_eq!(ctx1.rip, 0x4000_1000);
    assert_eq!(ctx1.cr3, 0x1A00_0000);

    // Quantum expiration triggers context switch from 101 to 102
    for _ in 0..11 {
        sched.schedule_next(0).unwrap();
    }

    let scheduled2 = sched.schedule_next(0).unwrap();
    assert_eq!(scheduled2, Some(102));
    assert_eq!(sched.process_table.get(&102).unwrap().state, ProcessState::Running);
    assert_eq!(ctx2.cr3, 0x1B00_0000);
}

#[test]
fn test_lost_wakeup_protection_during_switch() {
    let mut sched = SchedulerCore::new(1);
    sched.create_process(201, 1, 15, 1).unwrap();
    sched.mark_ready(201).unwrap();
    sched.schedule_next(0).unwrap();

    // Wake event arrives while process is attempting to sleep
    sched.wake_process(201).unwrap();
    assert!(sched.process_table.get(&201).unwrap().wake_event_pending);

    // Attempt sleep: must remain READY due to lost-wakeup check
    sched.sleep_process(201).unwrap();
    assert_eq!(sched.process_table.get(&201).unwrap().state, ProcessState::Ready);
    assert!(!sched.process_table.get(&201).unwrap().wake_event_pending);
}
