# ⏱️ SigmaOS Scheduler Development Plan

This document details the architectural design and implementation plan for the **SigmaOS Real-Time Scheduler**, taking inspiration from **Preempt-RT** (preemptive hard real-time scheduling) and **Real-Time Linux co-kernels** (like Xenomai).

---

## 🗺️ Architectural Inspiration
*   **Preempt-RT:** Introduces sleeping spinlocks, priority inheritance, and fully preemptive interrupt threads.
*   **Xenomai / RT-Linux:** Utilizes a dual-kernel co-scheduler where high-priority real-time threads are managed by a micro-scheduler that intercepts interrupts before the standard scheduler.

---

## 🏗️ OOP Design & Priority Queues

SigmaOS implements a multi-class predictive scheduler using a strict state hierarchy and polymorphic priority classes:

```text
               +-----------------------------------------+
               |             Scheduler Core              |
               +-----------------------------------------+
                                    |
                    +---------------+---------------+
                    v                               v
       +-------------------------+     +-------------------------+
       |   EDF Priority Class    |     |   CFS Priority Class    |
       |  (Hard deadlines first) |     |  (Fair CPU time slices) |
       +-------------------------+     +-------------------------+
```

### Thread/Process State Machine:
```text
  State::Created ➡️ State::Ready ➡️ State::Running ➡️ State::Blocked ➡️ State::Terminated
```

### Polymorphic Scheduling Interface:
```rust
pub trait SchedulingClass {
    fn enqueue_thread(&mut self, thread: ThreadControlBlock);
    fn dequeue_thread(&mut self) -> Option<ThreadControlBlock>;
    fn on_tick(&mut self, elapsed_ms: u64);
    fn get_class_priority(&self) -> u32;
}
```

---

## 🛠️ Multi-Language Architecture (Rust, Zig, Nim)

### ⚡ Rust: Earliest Deadline First (EDF) Class
```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdfThread {
    pub id: usize,
    pub absolute_deadline_ms: u64,
}

impl Ord for EdfThread {
    fn cmp(&self, other: &Self) -> Ordering {
        // Earliest deadline has highest priority (min-heap)
        other.absolute_deadline_ms.cmp(&self.absolute_deadline_ms)
    }
}

impl PartialOrd for EdfThread {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct EdfSchedulerClass {
    heap: BinaryHeap<EdfThread>,
}

impl EdfSchedulerClass {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }

    pub fn schedule_next(&mut self) -> Option<EdfThread> {
        self.heap.pop()
    }
}
```

### ⚡ Zig: Priority Inheritance Lock (Mutex)
```zig
const std = @import("std");

pub const Thread = struct {
    id: usize,
    base_priority: u32,
    current_priority: u32,
};

pub const Mutex = struct {
    owner: ?*Thread,
    waiters_priority_max: u32,

    pub fn lock(self: *Mutex, thread: *Thread) void {
        if (self.owner == null) {
            self.owner = thread;
            return;
        }

        // Priority Inheritance: elevate owner priority if wait priority is higher
        const owner_thread = self.owner.?;
        if (thread.current_priority > owner_thread.current_priority) {
            owner_thread.current_priority = thread.current_priority;
        }
    }
};
```

### ⚡ Nim: Tickless Timer Scheduler
```nim
type
  TimerEvent* = object
    fireTime*: uint64
    callback*: proc() {.cdecl.}

proc scheduleTimer*(fireTime: uint64, cb: proc() {.cdecl.}) {.exportc, cdecl.} =
  # Register Timer callback for tickless kernel execution
  discard
```

---

## 📈 Quality Assurance & Interrupt Audits

1.  **Preemption Latency Test:** Measure context-switch delays when a hard real-time interrupt preempts a low-priority thread (aiming for sub-microsecond response).
2.  **Priority Inheritance Audit:** Verify that priority inversion situations are automatically resolved.
