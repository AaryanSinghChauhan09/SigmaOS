#[macro_use]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

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
use scheduler::{CfsScheduler, Priority, SchedulerPolicy};

#[test]
fn test_kernel_scheduler_algorithm_inspection() {
    let mut sched = CfsScheduler::new(1000000, 20000000);
    let _p1 = sched.create_process(Priority::high(), SchedulerPolicy::Normal);
    let _p2 = sched.create_process(Priority::normal(), SchedulerPolicy::Normal);

    assert_eq!(sched.runnable_count(), 2);
    let scheduled = sched.pick_next_task();
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
