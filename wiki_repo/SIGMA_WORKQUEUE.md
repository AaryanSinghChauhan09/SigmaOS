# SigmaOS Kernel Work Queue

## Overview

The SigmaOS work queue subsystem provides deferred execution of kernel work. Tasks too heavy for interrupt context are submitted to work queues and processed asynchronously.

**Location:** `src/kernel/sigma_workqueue.rs`

---

## Design

Inspired by:
- Linux `kernel/workqueue.c` (CMWQ — Concurrency Managed Work Queue)
- FreeBSD `kern/subr_taskqueue.c`

---

## Work Queue Types

| Type | Description |
|------|-------------|
| `SigmaWorkQueue(ordered=true)` | FIFO execution order |
| `SigmaWorkQueue(ordered=false)` | Priority-ordered execution |
| `BoundWorkQueue` | CPU-affine, for cache locality |

---

## Priority Levels

```rust
WorkPriority::Background  // 0 — run when idle
WorkPriority::Normal      // 1 — default
WorkPriority::High        // 2 — before normal
WorkPriority::Critical    // 3 — first in queue
```

---

## API Reference

```rust
// Create a priority-ordered work queue
let mut wq = SigmaWorkQueue::new("events", false);

// Submit work
wq.submit(WorkItem::new(my_func, data, "my-work", WorkPriority::Normal)).unwrap();

// Submit delayed work (execute after 100ms)
wq.submit(WorkItem::new_delayed(cleanup_fn, 0, "cleanup", 100_000_000)).unwrap();

// Advance time for delayed work
wq.advance_time(100_000_000);

// Process one item
wq.process_one();

// Flush all ready items
let count = wq.flush();

// Shutdown — drain all work
wq.drain();
```

---

## CPU-Bound Work Queue

```rust
let mut bound_wq = BoundWorkQueue::new("cpu0-events", 0); // CPU 0
bound_wq.submit(WorkItem::new(cpu_work, 0, "cpu-work", WorkPriority::High)).unwrap();
bound_wq.flush_if_online();
```

---

## Comparison

| Feature | Linux workqueue | BSD taskqueue | SigmaOS |
|---------|----------------|---------------|---------|
| Priority ordering | No (FIFO) | No | **Yes** |
| Delayed work | Yes | No | Yes |
| CPU binding | Yes | Yes | Yes |
| Bounded depth | No | No | **Yes** |
| no_std | No | No | **Yes** |
