# AI Agent Spinlock System Management Architecture in SigmaOS

This document specifies spinlock synchronization architectures, exponential backoff strategies, and contention tracking guidelines for AI agents working on kernel synchronization in SigmaOS.

---

## 🔒 1. Spinlock Subsystem Architecture

SigmaOS implements two primary spinlock primitives:

```
+-----------------------------------------------------------------------+
| Ticket Spinlock with Exponential Backoff (`src/kernel/classic_os.rs`) |
| Guarantees FIFO fairness across SMP cores & reduces L1/L2 cache line  |
| bouncing using atomic ticket/now_serving counters.                    |
+-----------------------------------------------------------------------+
| Fine-Grained Spinlock with Contention Stats                           |
| (`src/kernel/core/sovereign_scheduler.rs`)                            |
| Linux spinlock_t / FreeBSD mtx parity with acquire_count & contention |
| tracking for real-time latency diagnostics.                            |
+-----------------------------------------------------------------------+
```

---

## ⚙️ 2. Spinlock Rules for AI Agents

1. **FIFO Fairness (`TicketSpinlock`)**
   - Use `TicketSpinlock` for critical kernel sections where fair access among SMP cores is required.
   - Exponential backoff (`core::hint::spin_loop()`) MUST be applied while waiting for `now_serving` to match the assigned ticket.

2. **Contention Diagnostics (`FineGrainedSpinlock`)**
   - For scheduler runqueues and memory allocators, use `FineGrainedSpinlock` to track `acquire_count` and `contention_count`.
   - Log contention ratios if `contention_count / acquire_count > 0.15` (15% lock contention).

3. **Deadlock Prevention Guidelines**
   - **Lock Ordering:** Always acquire spinlocks in strict ascending hierarchical order.
   - **Non-Blocking Operations:** Never perform heap allocation, page allocation, or blocking IPC while holding a spinlock.
   - **Interrupt Safety:** Disable local interrupts (`spin_lock_irqsave`) prior to acquiring locks shared with IRQ handlers.

---

## ⚙️ 3. Verification Commands for Synchronization Agents

- **Classic OS Ticket Spinlock Tests:**
  `cargo test --lib -- kernel::classic_os::tests`
- **Fine-Grained Scheduler Spinlock Tests:**
  `cargo test --lib -- kernel::core::sovereign_scheduler::tests`
