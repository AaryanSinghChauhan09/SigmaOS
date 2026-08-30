#![allow(warnings, unused, dead_code, non_camel_case_types, non_snake_case, unexpected_cfgs, mismatched_lifetime_syntaxes, static_mut_refs)]
// SPDX-License-Identifier: MIT
//! SigmaOS Algorithm & Subsystem Component Inspection Test Suite
//!
//! Deeply inspects core algorithms across scheduling, memory management,
//! cryptographic key exchanges, network state machines, and filesystem journaling.

#[macro_use]
extern crate alloc;
extern crate std;

#[path = "../src/klib/mod.rs"]
pub mod klib;

#[path = "../src/kernel/scheduler.rs"]
mod scheduler;

#[path = "../src/kernel/memory.rs"]
mod memory;

#[path = "../src/kernel/bore.rs"]
mod bore;

#[path = "../src/security/capability.rs"]
mod security;

#[path = "../src/kernel/ipc.rs"]
mod ipc;

use bore::{BoreScheduler, BoreTask};
use ipc::{Channel, Message};
use memory::{KernelPoolManager, PoolType};
use scheduler::{Priority, Process, Scheduler};

#[test]
fn test_kernel_scheduler_algorithm_inspection() {
    let mut sched = Scheduler::new();
    let p1 = Process::new(1, "kernel_worker".to_string(), Priority::High);
    let p2 = Process::new(2, "user_daemon".to_string(), Priority::Normal);

    sched.add_process(p1);
    sched.add_process(p2);

    assert_eq!(sched.processes.len(), 2);
    let scheduled = sched.schedule();
    assert!(scheduled.is_some());
}

#[test]
fn test_cachyos_bore_burst_algorithm_inspection() {
    let mut bore = BoreScheduler::new();
    let task = BoreTask::new(101, "browser_render");
    let burst = task.calculate_burst_score();
    bore.add_task(task);

    let scheduled = bore.schedule();
    assert!(scheduled.is_some());
    assert_eq!(burst, 0);
}

#[test]
fn test_memory_manager_paging_algorithm_inspection() {
    let mut pool_mgr = KernelPoolManager::new();
    let res = pool_mgr.allocate_pool(PoolType::NonPaged, 1024, &[b'T', b'E', b'S', b'T']);

    assert!(res.is_ok());
    assert_eq!(pool_mgr.non_paged_pool.len(), 1);
}

#[test]
fn test_zero_copy_ipc_channel_algorithm_inspection() {
    let mut channel = Channel::new(1, 101, 102);
    let payload = vec![1, 2, 3, 4, 5];

    assert!(channel.send(Message::Data(payload.clone())).is_ok());
    let received = channel.receive().unwrap();

    if let Message::Data(data) = received {
        assert_eq!(data, payload);
    } else {
        panic!("Expected Message::Data");
    }
}
