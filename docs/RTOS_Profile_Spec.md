# SigmaOS RTOS Profile Technical Specification

## Overview

The SigmaOS RTOS profile targets hard real-time workloads: industrial controllers, robotics, automotive ECUs, and safety-critical embedded systems. It uses an **Earliest Deadline First (EDF)** scheduler, static memory allocation, and a minimal kernel footprint with IRQ latency < 10 µs.

---

## EDF Scheduler

### Task Model

Each real-time task is defined by:
- **Period** `T` — task repeats every `T` microseconds
- **Worst-Case Execution Time** `C` — maximum CPU time per period
- **Deadline** `D` — must complete within `D` µs of period start (usually `D = T`)
- **Utilisation** `U = C / T`

### Schedulability (Liu-Layland Bound)

For `n` tasks, the system is schedulable under EDF if:

```
Σ (C_i / T_i) ≤ 1.0
```

EDF achieves 100% CPU utilisation for independent tasks (vs ~69% for Rate-Monotonic).

### Kernel Data Structures

```rust
// kernel/src/sched/edf.rs

use core::collections::BinaryHeap;
use core::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct EdfTask {
    pub id:            TaskId,
    pub period_us:     u64,
    pub wcet_us:       u64,
    pub deadline_us:   u64,    // absolute, in monotonic ns
    pub next_release:  u64,    // next activation time
}

// Min-heap by absolute deadline
impl Ord for EdfTask {
    fn cmp(&self, other: &Self) -> Ordering {
        other.deadline_us.cmp(&self.deadline_us) // reversed for min-heap
    }
}
impl PartialOrd for EdfTask { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }

pub struct EdfScheduler {
    ready_queue: BinaryHeap<EdfTask>,
    timer_us:    u64,
}

impl EdfScheduler {
    pub fn next_task(&mut self) -> Option<TaskId> {
        self.ready_queue.peek().map(|t| t.id)
    }

    pub fn tick(&mut self, now_us: u64) {
        self.timer_us = now_us;
        // Release tasks whose next_release <= now
    }
}
```

---

## IRQ Latency Target: < 10 µs

Achieving < 10 µs IRQ-to-handler latency requires:

1. **Interrupt nesting**: critical IRQs use a dedicated high-priority interrupt vector (x86: IOAPIC priority steering; ARM: GIC priority grouping).
2. **No spin-lock contention in IRQ path**: all IRQ handlers are lock-free (use atomic operations + per-CPU data structures).
3. **No dynamic allocation in IRQ context**: all buffers pre-allocated at init.
4. **Minimal IRQ handler**: immediately post a message to a real-time task's mailbox, return.
5. **Cache warming**: real-time task stacks pinned to L1 cache via `CLFLUSHOPT` prefetch.

### IRQ Latency Measurement

```rust
// kernel/src/bench/irq_latency.rs
pub fn measure_irq_latency() -> u64 {
    let t0 = read_tsc();
    trigger_test_irq(); // software interrupt via `int 0x20`
    // IRQ handler records TSC at entry:
    let t1 = IRQ_ENTRY_TSC.load(Ordering::Acquire);
    t1 - t0 // in TSC cycles; convert to µs
}
```

---

## Memory: Static Allocation Only

The RTOS profile **disables the heap allocator** at compile time:

```rust
// kernel/src/alloc/rtos_alloc.rs

#[cfg(feature = "rtos")]
#[global_allocator]
static ALLOCATOR: StaticBumpAllocator = StaticBumpAllocator::new();

/// A bump allocator over a statically sized arena.
/// Panics if arena is exhausted — enforces bounded memory use.
pub struct StaticBumpAllocator {
    arena: [u8; 512 * 1024], // 512 KB fixed RT arena
    next:  AtomicUsize,
}
```

All real-time task stacks, message queues, and buffers are allocated at init time from this arena.

---

## Boot Sequence (RTOS Profile)

```
sigma-boot-rtos.efi
  │  Skip PCI enumeration (not needed for most RTOS targets)
  │  Load kernel ELF to fixed physical address (no KASLR in RTOS profile)
  ▼
kernel_main_rtos()
  │  Setup GDT + IDT (minimal: only used exceptions + RT IRQ vectors)
  │  Calibrate TSC (use CPUID + HPET fallback)
  │  Initialise EDF scheduler with pre-defined task table
  │  Mount devfs only (no SigmaFS in minimal RTOS profile)
  │  Start real-time tasks from /etc/sigma/rtos-tasks.toml
  ▼
EDF scheduler runs
  │  Tasks: sigma-can-driver, sigma-can-stack, sigma-plc-logic, etc.
  ▼
(No sigma-sh on minimal RTOS profile; optional debug shell on UART)
```

---

## POSIX.1b Real-Time Extensions Compliance

Target compliance:

| POSIX.1b Feature | Status |
|---|---|
| `SCHED_FIFO` / `SCHED_RR` | ✅ Mapped to EDF task with WCET == period |
| `clock_gettime(CLOCK_REALTIME)` | ✅ |
| `clock_nanosleep` | ✅ |
| `mlock` / `mlockall` | ✅ (all RTOS memory is pre-locked) |
| `mq_open` / `mq_send` / `mq_receive` | ✅ (sigma-bus RT message queues) |
| `sem_open` | ✅ |
| `pthread_attr_setschedparam` | 🔄 (partial — EDF priority mapping) |
