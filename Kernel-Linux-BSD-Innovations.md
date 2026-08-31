# Kernel Linux & BSD Innovations

This page documents the Linux and BSD inspired kernel primitives added to SigmaOS through the `feat/kernel-linux-bsd-innovations` branch series.

***

## 1. Linux-Inspired Kernel Primitives

### Completion Variables (`sigma_completion`)

Analogous to Linux `struct completion`, SigmaOS implements a wait/signal mechanism for kernel threads without requiring `std::sync`:

```rust
use crate::kernel::sync::Completion;

let comp = Completion::new();

// Thread A: waits for initialization
comp.wait();

// Thread B: signals completion  
comp.complete();
```

### Read-Copy-Update (RCU) Lite

A lightweight **RCU** implementation for protecting frequently-read, rarely-written kernel data structures:

*   `rcu_read_lock()` / `rcu_read_unlock()` — lock-free reader path
*   `rcu_assign_pointer()` — safely publishes new data
*   `call_rcu()` — deferred reclamation after grace period

### Workqueue Subsystem

Asynchronous work items that can be scheduled from interrupt context:

```rust
use crate::kernel::workqueue::{WorkQueue, WorkItem};

let wq = WorkQueue::new("sigma-wq", 4); // 4 worker threads
wq.queue(WorkItem::new(|| { /* deferred work */ }));
```

### kfifo (Lock-Free Ring Buffer)

Single-producer, single-consumer lock-free FIFO, inspired by Linux `kfifo`:

```rust
use crate::klib::kfifo::KFifo;

let mut fifo: KFifo<u8, 256> = KFifo::new();
fifo.push(42);
let val = fifo.pop(); // Some(42)
```

***

## 2. BSD-Inspired Kernel Primitives

### DragonFly LWKT (Lightweight Kernel Thread) Scheduler

Adapted from DragonFly BSD's LWKT subsystem:

*   Per-CPU run queues with no global lock
*   Token-based inter-CPU serialization (replaces coarse Big Kernel Lock)
*   Threads yield cooperatively within a CPU, preempted cross-CPU

### FreeBSD UMA (Universal Memory Allocator) Zone Allocator

A slab-like zone allocator for fixed-size kernel objects:

```rust
use crate::klib::uma::UmaZone;

// Allocate a zone for ProcessControlBlock objects
let pcb_zone: UmaZone<ProcessControlBlock> = UmaZone::new("pcb", 256);
let pcb = pcb_zone.alloc()?;
```

### NetBSD SDT (Statically Defined Tracing) Probes

Kernel-level DTrace-compatible tracing probes:

```rust
sdt_probe!(kernel, scheduler, context_switch, old_pid, new_pid);
```

Consumed by `sigma-trace` tool for live kernel observability.

### OpenBSD W^X (Write XOR Execute)

All memory pages are enforced to be either writable **or** executable, never both:

*   Implemented via page table entry flags at `paging.rs` level
*   JIT code paths use a two-stage: write to RW page → remap as RX
*   Prevents code injection attacks at the hardware level

***

## 3. Driver Trait Improvements

The `feat/kernel-linux-bsd-innovations` branch also refined the driver trait system to reduce boilerplate:

### Before

```rust
// Each driver had to manually implement 8+ trait methods
impl Driver for IntelE1000 {
    fn name(&self) -> &str { "Intel E1000" }
    fn version(&self) -> (u8, u8, u8) { (1, 0, 0) }
    fn probe(&self) -> Result<(), DriverError> { ... }
    fn init(&mut self) -> Result<(), DriverError> { ... }
    // ... 4 more required methods
}
```

### After

```rust
// Derive macro reduces boilerplate
#[derive(SigmaDriver)]
#[driver(name = "Intel E1000", version = "1.0.0")]
pub struct IntelE1000 { ... }

// Only implement the meaningful behavior
impl DriverBehavior for IntelE1000 {
    fn on_probe(&self) -> Result<(), DriverError> { ... }
    fn on_data_ready(&mut self, data: &[u8]) { ... }
}
```

***

## 4. Memory Management Improvements

### Huge Page (THP) Support

Transparent Huge Page allocation inspired by Linux's THP subsystem:

| Page Size | Architecture | Status |
|-----------|-------------|--------|
| 4 KB | x86\_64, AArch64 | Stable |
| 2 MB | x86\_64 (PDE) | Supported |
| 1 GB | x86\_64 (PDPTE) | Experimental |
| 16 KB | AArch64 | Supported |

### ASLR Improvements

*   Stack ASLR entropy increased to 28 bits (from 24)
*   Heap ASLR now randomizes within 1TB virtual address space
*   mmap ASLR randomizes both base address and mapping order

***

## See Also

*   [Scheduler Architecture](Scheduler-Architecture)
*   [Memory Management](Memory-Management)
*   [Driver Development Guide](Driver-Development-Guide)
*   [Gaming Performance Mode](Gaming-Performance-Mode)
*   [BSD Securelevels](EndeavourOS-PAM-BSD-Securelevels)
