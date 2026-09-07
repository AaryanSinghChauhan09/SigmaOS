# AI Agent Guidelines: Concurrent Thread Management in SigmaOS

## 📌 1. Overview & Architectural Principles

In **SigmaOS**, thread management provides the execution abstractions for concurrent kernel tasks, driver shards, bottom-half workers, and userland application threads.

As an AI agent, you must manage concurrent thread lifecycles using **zero-allocation thread pools, guard-page-protected kernel stacks, lock-free work-stealing queues, and hardware-enforced context switching**.

---

## 🏗️ 2. Thread Model Architecture & State Machine

SigmaOS employs a **Hybrid Thread Model**:
* **1:1 Kernel Thread Mapping:** Used for microkernel shards, interrupt bottom halves, hardware drivers, and real-time EDF tasks.
* **M:N Userland Fiber Model:** Used for asynchronous networking pipelines (`IoUringEngine`, `AsyncRuntime`) to map $M$ userland coroutines onto $N$ kernel worker threads.

```
                  +-----------------------------------+
                  |          INITIALIZED              |
                  +-----------------------------------+
                                    |
                                    v
                            +---------------+
                            |     READY     | <---------+
                            +---------------+           |
                               |          ^             |
                    Scheduled  |          | Preempted   | Unblocked
                               v          |             |
                            +---------------+           |
                            |    RUNNING    |           |
                            +---------------+           |
                               |          |             |
                   I/O Wait or |          +-------------+
                      Sleep    v          Blocked
                            +---------------+
                            |    WAITING    |
                            +---------------+
                                    |
                                    v Terminated
                            +---------------+
                            |   TERMINATED  |
                            +---------------+
```

---

## ⚙️ 3. Core Thread Control Block (`SystemThread`) & Contexts

* **Module Location:** `src/kernel/structures.rs`, `src/kernel/scheduler.rs`, `src/process/advanced_process_control.rs`
* **Key Fields:**
  * `tid`: Unique 64-bit Thread ID (`u64`).
  * `pid`: Parent Process ID (`u64`).
  * `state`: Thread State (`ThreadState::Ready`, `Running`, `Blocked`, `Suspended`, `Terminated`).
  * `context`: CPU Context register snapshot (`CpuContext`: `rax`, `rbx`, `rcx`, `rdx`, `rsi`, `rdi`, `rsp`, `rbp`, `rip`, `rflags`, `fs_base`).
  * `kernel_stack_base` / `kernel_stack_top`: $16\text{ KB}$ aligned stack with bottom guard page.
  * `priority`: Scheduling priority class (`IrqlLevel`, `EdfDeadline`, `BoreLatencyScore`).

### 3.1 Context Switch Invariants
* **Hardware Register Saving:** Callee-saved registers (`rbx`, `rsp`, `rbp`, `r12`..`r15` on x86_64; `x19`..`x29`, `sp` on AArch64) must be saved onto the thread's kernel stack during context switches.
* **TLS Base Update:** The thread-local storage register (`fs_base` / `TPIDR_EL0`) MUST be reloaded atomically via `wrmsr` / `msr` on every switch.
* **Canary Verification:** Context switch assembly checks the stack canary (`0xDEADBEEF_CAFE_BABE`) before restoring registers to detect stack overflows.

---

## 🤹 4. Work-Stealing Thread Pools & Task Queues

For high-concurrency workloads, SigmaOS uses lock-free work-stealing thread pools:
* **Grand Central Dispatch (GCD) Model (`GcdDispatchQueue`):**
  * Implements `GcdPriority::High`, `Default`, `Low`, `Background` queues (`src/kernel/linux_bsd_innovations.rs`).
  * Idle worker threads steal tasks from the tail of adjacent busy thread queues using atomic CAS ring buffers.
* **Workqueue Kernel Threads:**
  * Linux-compatible workqueues (`src/kernel/irq/workqueue.rs`) execute deferred bottom-half work items outside of IRQ context.

---

## 🛡️ 5. AI Agent Rules & Code Patterns

1. **Stack Guard Page Enforcement:**
   * Every allocated thread stack MUST map the lowest virtual page ($4\text{ KB}$) with `PageTableFlags::NO_PERMISSIONS` (unmapped / no-access) to trap stack overflows as Page Faults (`#PF`).
2. **Reclaim Terminated Threads Immediately:**
   * Never leave threads in `ThreadState::Terminated` without unmapping their stacks and returning their `SystemThread` TCB to the slab allocator.
3. **Interrupt Safety:**
   * Thread queue manipulations inside kernel schedulers must execute with interrupts disabled or within an IRQ-safe spinlock scope.

---

## 🧪 6. Standalone Testing Procedures

AI agents can verify thread TCB structures, context switching, and thread state transitions via standalone unit compilation:

```bash
# Test kernel structures, TCBs, APC queues, and thread state transitions
rustc --test --edition=2021 src/kernel/structures.rs -o build/struct_tests && ./build/struct_tests && rm build/struct_tests

# Test CachyOS BORE scheduler & thread runqueue management
rustc --test --edition=2021 src/kernel/scheduler.rs -o build/sched_tests && ./build/sched_tests && rm build/sched_tests
```
