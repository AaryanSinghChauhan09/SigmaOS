# SigmaOS Concurrency Primitives

## Overview

SigmaOS provides a complete set of sovereign, `no_std`-compatible synchronization primitives for both single-core and SMP scenarios.  All primitives live in `src/kernel/`.

---

## Primitive Inventory

| Primitive | File | Primary use case |
|-----------|------|-----------------|
| `SigmaSpinlock<T>` | `sigma_spinlock.rs` | Short critical sections, interrupt handlers |
| `SigmaRwSpinlock<T>` | `sigma_spinlock.rs` | Read-heavy shared data |
| `RcuCell<T>` | `sigma_rcu.rs` | Mostly-read, rarely-written data |
| `SigmaMutex<T>` | `mutex.rs` | Long critical sections with blocking |
| `SigmaSemaphore` | `semaphore.rs` | Resource counting / producer-consumer |
| Sequence lock | *(future)* | Time-sensitive reads with rare writes |

---

## Spinlocks (`SigmaSpinlock<T>`)

### Algorithm: ticket spinlock

A **ticket spinlock** issues sequential tickets to waiters, guaranteeing FIFO ordering and freedom from starvation.

```
Acquire:                        Release:
  my_ticket = next++              serving++
  spin while serving != my_ticket
```

### Backoff strategy

Instead of a tight spin (which degrades L1/L2 cache performance on multicore systems), SigmaOS uses **exponential backoff**:

```rust
let mut backoff = 1;
loop {
    if serving == my_ticket { break; }
    for _ in 0..backoff { core::hint::spin_loop(); }
    backoff = (backoff * 2).min(256);
}
```

`core::hint::spin_loop()` emits `PAUSE` on x86 and `YIELD` on ARM, reducing speculative execution stalls and memory bus traffic.

### API

```rust
let lock = SigmaSpinlock::new(0u32);

// Blocking acquire
let mut guard = lock.lock();
*guard += 1;
// drop(guard) → unlock

// Non-blocking attempt
if let Some(mut g) = lock.try_lock() {
    *g = 99;
}
```

### When to use

- Interrupt handlers (cannot sleep)
- Very short critical sections (< ~100 cycles)
- Protecting scheduler run queues

### When NOT to use

- Long operations (I/O, memory allocation) — use `SigmaMutex` instead
- Read-heavy data — use `SigmaRwSpinlock` or RCU

---

## Reader-Writer Spinlock (`SigmaRwSpinlock<T>`)

Allows **concurrent readers** but **exclusive writers**.

### State encoding

A single `AtomicUsize` encodes both the reader count and write-lock state:

```
bit 31      : write-lock bit (1 = locked by writer)
bits 30..0  : reader count
```

### API

```rust
let rw = SigmaRwSpinlock::new(vec![1, 2, 3]);

// Multiple concurrent readers
let r1 = rw.read();
let r2 = rw.read();
println!("{:?}", *r1);
drop(r1); drop(r2);

// Exclusive writer
let mut w = rw.write();
w.push(4);
// drop(w) → write_unlock
```

### Fairness note

The current implementation does not prevent writer starvation in the presence of a continuous stream of readers.  A future version will add a "write-pending" bit that blocks new readers once a writer is waiting.

---

## RCU — Read-Copy-Update (`RcuCell<T>`)

See [`SIGMA_RCU_SYNCHRONIZATION.md`](SIGMA_RCU_SYNCHRONIZATION.md) for full documentation.

**Summary:**

- Readers: zero-cost (one atomic increment)
- Writers: publish via `rcu_assign_pointer`, then `synchronize_rcu` to wait for old readers
- Best for: routing tables, credential structures, module lists

```rust
let cell = RcuCell::new(config);
let guard = cell.read_lock();  // never blocks
println!("{:?}", guard.get());
```

---

## Mutex (`SigmaMutex<T>`)

A sleeping mutex backed by the SigmaOS scheduler.  When a thread fails to acquire the lock, it is **parked** (removed from the run queue) and woken when the lock is released.

```rust
let m = SigmaMutex::new(state);
let guard = m.lock(); // parks if contended; does not spin
```

### Differences from spinlock

| Property | `SigmaSpinlock` | `SigmaMutex` |
|----------|----------------|-------------|
| Contention handling | Spin | Park / wake |
| Usable in interrupts | ✅ | ❌ |
| Suitable for long holds | ❌ | ✅ |
| Overhead (no contention) | ~3 ns | ~15 ns |

---

## Semaphore (`SigmaSemaphore`)

A counting semaphore for resource management and producer-consumer synchronization.

```rust
let sem = SigmaSemaphore::new(4); // 4 available slots

// Consumer
sem.wait();   // decrements; parks if 0
use_resource();
sem.post();   // increments; wakes one waiter
```

### Use cases

- Limiting concurrent database connections
- IPC buffer flow control
- `SigmaMessageQueue` internal wait list

### Binary semaphore (mutex alternative)

`SigmaSemaphore::new(1)` behaves as a non-owning mutex (useful for signalling between different threads where the acquirer and releaser differ).

---

## Sequence Lock *(planned)*

A sequence lock allows readers to detect concurrent writes without holding any lock, at the cost of potentially having to retry the read.

```
read:
  loop:
    seq = seqcount.load (must be even)
    read data
    if seqcount.load == seq: break  // no writer interrupted us

write:
  seqcount += 1 (make odd)
  update data
  seqcount += 1 (make even)
```

Ideal for: system time, jiffies, hardware performance counters.

---

## Choosing the Right Primitive

```
Is data mostly read, rarely written?
├─ Yes, reads very frequent → RcuCell
├─ Yes, writes need to be seen immediately → SigmaRwSpinlock
└─ No (balanced reads/writes):
    Can the critical section block/sleep?
    ├─ No (interrupt handler, short path) → SigmaSpinlock
    └─ Yes → SigmaMutex

Need resource counting? → SigmaSemaphore
Need to detect concurrent writes cheaply? → Sequence lock (planned)
```

---

## Memory Ordering Summary

| Operation | Ordering used | Rationale |
|-----------|---------------|-----------|
| Spinlock acquire | `Acquire` | See all writes before the lock |
| Spinlock release | `Release` | Publish all writes to next holder |
| RCU `rcu_assign_pointer` | `SeqCst` fence | Total ordering across all CPUs |
| RCU reader count increment | `Acquire` | |
| RCU reader count decrement | `Release` | |
| Ticket counter fetch_add | `Relaxed` | Only the ordering of `now_serving` matters |

---

## See Also

- [`SIGMA_RCU_SYNCHRONIZATION.md`](SIGMA_RCU_SYNCHRONIZATION.md)
- [`SIGMA_IPC_PIPES.md`](SIGMA_IPC_PIPES.md)
- [`SIGMA_MESSAGE_QUEUE.md`](SIGMA_MESSAGE_QUEUE.md)
- `src/kernel/sigma_spinlock.rs`
- `src/kernel/sigma_rcu.rs`
