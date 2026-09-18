# SigmaOS AI Agent Mutual Exclusion Operation Management Guidelines

## 1. Overview
SigmaOS implements robust mutual exclusion and synchronization primitives managed autonomously or interactively by AI system agents (such as `MutexGovernor`, `DeadlockPreventionEngine`, `PriorityInheritanceManager`, and `IrqSafeSpinlockGuard`). These guidelines define mutual exclusion types (`Mutex`, `Spinlock`, `RwLock`, `TicketLock`), lock hierarchy ordering rules, priority inheritance protocol (PIP), futex wait-queue sleep/wake dispatching, and IRQ-safe kernel lock guards for AI agents in SigmaOS.

## 2. Core Mutual Exclusion Management Principles

### 2.1 Mutual Exclusion Synchronization Primitives
- **Userspace / Adaptive Mutex (`Mutex<T>`)**: Fast-path atomic CAS spin followed by `sys_futex` sleep wait when contended, minimizing CPU busy-spinning.
- **Kernel Spinlock (`Spinlock<T>`)**: Lock-free atomic spinlock for short Ring-0 critical sections.
- **Reader-Writer Lock (`RwLock<T>`)**: Allows concurrent lockless readers (`read()`) while enforcing exclusive write access (`write()`), preventing reader starvation via fair queueing.
- **Ticket Lock (`TicketLock<T>`)**: FIFO ticket-based spinlock guaranteeing fair, bounded lock acquisition times for multi-core SMP scheduling.

### 2.2 Deadlock Prevention & Strict Lock Hierarchy
- **Global Lock Acquisition Hierarchy**: To prevent cyclic deadlock dependency traps, AI agents acquiring multiple locks simultaneously must observe strict global lock rank order (e.g. `VFS_INODE_LOCK` -> `BUFFER_CACHE_LOCK` -> `PMM_BITMAP_LOCK`).
- **Directed Lock Graph Checking**: `DeadlockPreventionEngine` verifies lock acquisition paths at runtime, raising a kernel panic or error if out-of-order acquisition is attempted.

### 2.3 Priority Inheritance Protocol (PIP)
- **Priority Inversion Mitigation**: When a low-priority process holds a mutual exclusion lock required by a high-priority AI task, `PriorityInheritanceManager` temporarily elevates the lock holder's priority to match the waiter's priority.
- **Automatic Priority Restoration**: Upon lock release (`unlock()`), the elevated process priority is restored to its baseline value.

### 2.4 IRQ-Safe Ring-0 Spinlock Guards
- **Interrupt Disabling (`irq_save` / `irq_restore`)**: Ring-0 spinlock acquisition inside hardware interrupt handlers disables local CPU interrupts (`cli` / `cpsid i`) to prevent deadlocks caused by re-entrant interrupt handlers.

---
*Maintained by the SigmaOS Kernel, Concurrency & Synchronization Steering Committee.*
