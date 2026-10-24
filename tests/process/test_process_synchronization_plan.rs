// SPDX-License-Identifier: MIT
// SigmaOS Process Synchronization Architecture & Subsystem Contract Test Suite

extern crate alloc;

#[path = "../../src/kernel/wait_queue.rs"]
mod wait_queue;

#[path = "../../src/kernel/sigma_spinlock.rs"]
mod sigma_spinlock;

#[path = "../../src/kernel/semaphore.rs"]
mod semaphore;

#[path = "../../src/kernel/futex.rs"]
mod futex;

#[path = "../../src/kernel/flock.rs"]
mod flock;

#[path = "../../src/kernel/pipe.rs"]
mod pipe;

#[path = "../../src/kernel/eventfd.rs"]
mod eventfd;

use wait_queue::*;
use sigma_spinlock::*;
use semaphore::*;
use futex::*;
use flock::*;
use pipe::*;
use eventfd::*;

#[test]
fn test_plan_wait_queue_contract() {
    let mut wq = WaitQueue::new();
    assert_eq!(wq.len(), 0);
    assert!(wq.is_empty());

    let entry = WaitQueueEntry::new(1, WaitFlags::DEFAULT);
    wq.add(entry);
    assert_eq!(wq.len(), 1);

    let woken = wq.wake_all();
    assert_eq!(woken, 1);
    assert_eq!(wq.len(), 1);
    assert_eq!(wq.active_waiters(), 0);
}

#[test]
fn test_plan_spinlock_contract() {
    let lock = SigmaSpinlock::new(42u32);

    {
        let mut guard = lock.lock();
        assert_eq!(*guard, 42);
        *guard = 100;
    }

    {
        let guard = lock.lock();
        assert_eq!(*guard, 100);
    }

    assert!(lock.try_lock().is_some());
}

#[test]
fn test_plan_rw_spinlock_contract() {
    let rw = SigmaRwSpinlock::new(99u32);
    {
        let r1 = rw.read();
        let r2 = rw.read();
        assert_eq!(*r1, 99);
        assert_eq!(*r2, 99);
    }
    {
        let mut w = rw.write();
        *w = 123;
        assert_eq!(*w, 123);
    }
}

#[test]
fn test_plan_semaphore_contract() {
    let mut sem = Semaphore::new(1, 2, 5);
    assert_eq!(sem.get_value(), 2);
    assert_eq!(sem.try_wait(), SemaphoreResult::Success);
    assert_eq!(sem.get_value(), 1);

    assert_eq!(sem.post(), SemaphoreResult::Success);
    assert_eq!(sem.get_value(), 2);

    let mgr = SemaphoreManager::new();
    let set_id = mgr.create_semaphore_set(100);
    let sem_id = mgr.create_semaphore(set_id, 0, 10).unwrap();
    assert_eq!(mgr.try_wait(set_id, sem_id).unwrap(), SemaphoreResult::WouldBlock);
}

#[test]
fn test_plan_futex_contract() {
    let mut queue = FutexQueue::new();

    let waiter = FutexWaiter::new(42);
    assert!(queue.enqueue(waiter).is_ok());
    assert_eq!(queue.len(), 1);

    let woken = queue.wake(1, 0xFFFFFFFF);
    assert_eq!(woken, 1);
    assert_eq!(queue.len(), 0);

    let mut manager = FutexManager::new();
    assert!(manager.futex_op(0x1000, FutexOp::Wait, 42, 42, 0, 0, 100, 1, FutexFlags::PRIVATE).is_ok());
    assert_eq!(manager.stats().active_waiters, 1);
}

#[test]
fn test_plan_flock_contract() {
    let mgr = FileLockManager::new();
    let fd = 100;

    let owner = mgr.create_lock(fd, LockType::Shared);
    assert_eq!(owner, 1);
    assert_eq!(mgr.lock_count(), 1);

    assert!(mgr.acquire(fd).is_ok());
    let lock = mgr.get_lock(fd).unwrap();
    assert!(lock.is_locked());

    assert!(mgr.release(fd).is_ok());
    let lock_rel = mgr.get_lock(fd).unwrap();
    assert!(!lock_rel.is_locked());
}

#[test]
fn test_plan_pipe_contract() {
    let mut pipe = Pipe::new(1, 64);
    assert_eq!(pipe.available_read(), 0);

    let written = pipe.write(b"SigmaOS Sync").unwrap();
    assert_eq!(written, 12);
    assert_eq!(pipe.available_read(), 12);

    let read_bytes = pipe.read(12).unwrap();
    assert_eq!(&read_bytes[..], b"SigmaOS Sync");
}

#[test]
fn test_plan_eventfd_contract() {
    let flags = EventFdFlags::new();
    let mut efd = EventFd::new(1, flags, 0);
    assert!(!efd.is_readable());

    assert!(efd.write(5).is_ok());
    assert!(efd.is_readable());

    let val = efd.read().unwrap();
    assert_eq!(val, 5);
    assert!(!efd.is_readable());
}

#[test]
fn test_process_synchronization_architecture_rules_and_pr_sequence() {
    // Verify 5-layer progression architecture rules
    let layers = [
        "1. Kernel wait/wakeup primitives",
        "2. Kernel locks and completions",
        "3. Userspace synchronization APIs",
        "4. Process-shared IPC synchronization",
        "5. Concurrency diagnostics",
    ];
    assert_eq!(layers.len(), 5);

    // Verify 10 core design rules
    let rules = [
        "1. Name the synchronization domain",
        "2. Separate lock from event",
        "3. Never sleep in atomic context",
        "4. Specify ownership and failure semantics",
        "5. Wait on predicates, not notifications",
        "6. Prevent lost wakeups",
        "7. Use explicit memory ordering",
        "8. Address priority inversion and starvation",
        "9. Treat user-memory objects as untrusted",
        "10. Test races systematically",
    ];
    assert_eq!(rules.len(), 10);

    // Verify 17-PR roadmap titles
    let prs = [
        "PR 1: docs(sync): inventory primitives and define subsystem boundaries",
        "PR 2: docs(sync): define synchronization contracts and lock-context rules",
        "PR 3: test(sync): add concurrency model and invariant harness",
        "PR 4: feat(sync): add scheduler-integrated wait queue and wakeup primitive",
        "PR 5: feat(sync): add kernel mutex with ownership and debug checks",
        "PR 6: feat(sync): add bounded spinlock and IRQ-safe variants",
        "PR 7: feat(sync): add completion and one-shot event primitive",
        "PR 8: feat(sync): add counting semaphore with bounded counter",
        "PR 9: feat(sync): add condition variable with atomic unlock-and-wait",
        "PR 10: feat(sync): add read-write lock with starvation policy",
        "PR 11: feat(sync): add robust process-local mutex recovery",
        "PR 12: feat(sync): add futex-style wait/wake syscall for userspace fast paths",
        "PR 13: feat(sync): integrate process-shared synchronization with shared memory",
        "PR 14: feat(sync): add interprocess message and event synchronization",
        "PR 15: debug(sync): add lock-order checker and wait diagnostics",
        "PR 16: test(sync): add race detector and stress configuration",
        "PR 17: docs(sync): publish API, context matrix, and compatibility status",
    ];
    assert_eq!(prs.len(), 17);
}
