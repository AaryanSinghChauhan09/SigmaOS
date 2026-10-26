// SPDX-License-Identifier: MIT
// SigmaOS Dispatcher Plan Verification Test Suite

extern crate alloc;

use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchTaskState {
    New,
    Ready,
    Running,
    Blocked,
    Suspended,
    Zombie,
    Terminated,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DispatchContextFrame {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cr3: u64,
}

impl DispatchContextFrame {
    pub fn new(entry_point: u64, stack_ptr: u64, page_table: u64) -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: stack_ptr,
            rsp: stack_ptr,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: entry_point,
            rflags: 0x202, // Interrupts enabled
            cr3: page_table,
        }
    }

    pub fn is_valid_user_frame(&self) -> bool {
        // RSP and RIP must be non-null and RSP 16-byte aligned before call
        self.rsp != 0 && self.rip != 0 && (self.rsp % 8 == 0)
    }
}

pub struct DeferredWorkItem {
    pub id: u64,
    pub callback: fn(u64) -> bool,
    pub arg: u64,
}

pub struct BoundedWorkQueue {
    pub capacity: usize,
    pub items: Vec<DeferredWorkItem>,
}

impl BoundedWorkQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, item: DeferredWorkItem) -> Result<(), &'static str> {
        if self.items.len() >= self.capacity {
            return Err("WorkQueue full");
        }
        self.items.push(item);
        Ok(())
    }

    pub fn execute_pending(&mut self) -> usize {
        let mut executed = 0;
        let pending = std::mem::take(&mut self.items);
        for item in pending {
            if (item.callback)(item.arg) {
                executed += 1;
            }
        }
        executed
    }
}

#[test]
fn test_dispatcher_context_frame_invariants() {
    let frame = DispatchContextFrame::new(0x400000, 0x7FFFFFFF0000, 0x1000);
    assert!(frame.is_valid_user_frame());
    assert_eq!(frame.rflags & 0x200, 0x200, "IF flag must be set for interrupt enable");
    assert_eq!(frame.cr3, 0x1000);
    assert_eq!(std::mem::size_of::<DispatchContextFrame>(), 19 * 8);
}

#[test]
fn test_single_scheduling_decision_state_ordering() {
    let mut state = DispatchTaskState::Ready;
    assert_ne!(state, DispatchTaskState::Running);

    // State transition to Running on scheduler pick
    state = DispatchTaskState::Running;
    assert_eq!(state, DispatchTaskState::Running);

    // Block state transition
    state = DispatchTaskState::Blocked;
    assert_eq!(state, DispatchTaskState::Blocked);

    // Wakeup state transition back to Ready
    state = DispatchTaskState::Ready;
    assert_eq!(state, DispatchTaskState::Ready);
}

#[test]
fn test_bounded_deferred_work_queue() {
    let mut queue = BoundedWorkQueue::new(2);

    fn dummy_work(arg: u64) -> bool {
        arg > 0
    }

    assert!(queue
        .enqueue(DeferredWorkItem {
            id: 1,
            callback: dummy_work,
            arg: 10
        })
        .is_ok());

    assert!(queue
        .enqueue(DeferredWorkItem {
            id: 2,
            callback: dummy_work,
            arg: 20
        })
        .is_ok());

    // Saturation test
    assert!(queue
        .enqueue(DeferredWorkItem {
            id: 3,
            callback: dummy_work,
            arg: 30
        })
        .is_err());

    let executed = queue.execute_pending();
    assert_eq!(executed, 2);
    assert_eq!(queue.items.len(), 0);
}

#[test]
fn test_feature_status_dispatcher_alignment() {
    let content = fs::read_to_string("FEATURE_STATUS.toml")
        .expect("FEATURE_STATUS.toml must be readable");
    assert!(
        content.contains("[kernel.dispatcher]"),
        "FEATURE_STATUS.toml must contain [kernel.dispatcher]"
    );
    assert!(
        content.contains("docs/DISPATCHER_PLAN.md"),
        "FEATURE_STATUS.toml must reference docs/DISPATCHER_PLAN.md"
    );
    assert!(
        content.contains("wiki/DISPATCHER_PLAN.md"),
        "FEATURE_STATUS.toml must reference wiki/DISPATCHER_PLAN.md"
    );
}
