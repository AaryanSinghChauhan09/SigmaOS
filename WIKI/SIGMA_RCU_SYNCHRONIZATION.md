# SigmaOS RCU Synchronization

## Overview

Read-Copy-Update (RCU) is a SigmaOS synchronization mechanism for data structures that are read far more frequently than they are written.  The sovereign implementation lives in `src/kernel/sigma_rcu.rs`.

**Core guarantee:** readers are *never blocked*, never spin, and never acquire a lock on the fast path.

---

## How RCU Works

### The three primitives

```
rcu_read_lock()          ← enter read-side critical section
  use shared data        ← zero-cost; just an atomic increment
rcu_read_unlock()        ← exit critical section (drop guard)

── (writer) ──────────────────────────────────────────────────
rcu_assign_pointer(new)  ← atomically publish new value
synchronize_rcu()        ← wait for all pre-existing readers
free(old)                ← safe to reclaim now
```

### Grace period

A **grace period** is the interval between `rcu_assign_pointer` and the moment when all pre-existing readers have exited their critical sections.  After a grace period completes, the old value is guaranteed to be unreachable.

SigmaOS tracks grace periods with a 64-bit generation counter (`GENERATION`) and a global reader count (`READER_COUNT`):

```
synchronize_rcu():
  1. GENERATION += 1          ← new readers enter generation N+1
  2. spin while READER_COUNT > 0
     ← old readers drain naturally; no forced context switch
```

---

## API Reference

### Free functions (Linux-style)

| Function | Description |
|----------|-------------|
| `rcu_read_lock()` | Enter read-side critical section |
| `rcu_read_unlock()` | Exit read-side critical section |
| `synchronize_rcu()` | Block until all prior readers finish |

### `RcuCell<T>` — the protected data cell

```rust
let cell = RcuCell::new(config);

// Reader (no locking overhead)
let guard = cell.read_lock();
println!("{:?}", guard.get());
// guard dropped → rcu_read_unlock() called automatically

// Writer
cell.rcu_assign_pointer(new_config);
cell.synchronize_rcu(); // wait for old readers
// old config is now safe to free (would be done here in a real OS)
```

### `RcuReadGuard<T>`

RAII guard returned by `RcuCell::read_lock()`.  Provides `.get() -> &T`.

Dropping the guard automatically calls `rcu_read_unlock()`.

---

## Comparison with Linux Kernel RCU

| Aspect | Linux `rcu` | SigmaOS `sigma_rcu` |
|--------|------------|---------------------|
| Reader cost | Per-CPU counter, no global bus traffic | Single global `AtomicUsize` |
| Writer cost | `synchronize_rcu` may sleep | `synchronize_rcu` spins |
| Memory reclaim | `call_rcu` callback | Manual after `synchronize_rcu` |
| Preemptible RCU | ✅ (`PREEMPT_RCU`) | ❌ (future work) |
| Sleepable RCU | ✅ (`SRCU`) | ❌ (future work) |
| SMP correctness | Full (per-CPU QS tracking) | Single-node model |
| Grace period | Quiescent-state based | Reader-count based |
| `call_rcu` (async) | ✅ | ❌ (future work) |

### Performance notes

The sovereign implementation uses a single global reader count for simplicity. A production SMP implementation would use **per-CPU reader counts** (accessed without cross-CPU cache invalidation) and a **quiescent-state** mechanism (context switch or `cond_resched`) to detect grace periods without spinning.

---

## Use Cases in SigmaOS

| Subsystem | RCU usage |
|-----------|-----------|
| Routing table (`net/stack.rs`) | Lock-free route lookups |
| Process credential (UID/GID) | Cheap reads on every syscall |
| Module list | Iterate loaded modules without locks |
| sysfs/procfs (`fs/sigma_vfs.rs`) | Read filesystem metadata concurrently |
| Signal handler table | Readers never block on signal delivery |

---

## Design Invariants

1. **Read-side critical sections must be short** — holding `RcuReadGuard` prevents writers from completing `synchronize_rcu`.
2. **No sleeping inside read-side critical section** — in the current non-preemptible model, the reader count would never drop to zero.
3. **One writer at a time** — `rcu_assign_pointer` is not itself atomic; callers must hold an outer mutex when multiple writers exist.

---

## Example: Lock-free Config Hot-reload

```rust
static KERNEL_CONFIG: RcuCell<KernelConfig> = RcuCell::new(KernelConfig::default());

// Fast path — any CPU, any time
fn current_hz() -> u32 {
    KERNEL_CONFIG.read_lock().get().timer_hz
}

// Slow path — config update (rare)
fn update_config(new: KernelConfig) {
    KERNEL_CONFIG.rcu_assign_pointer(new);
    KERNEL_CONFIG.synchronize_rcu();
    // old KernelConfig now safely dropped
}
```

---

## See Also

- [`SIGMA_CONCURRENCY_PRIMITIVES.md`](SIGMA_CONCURRENCY_PRIMITIVES.md) — spinlocks, semaphores
- [`SIGMA_VFS_LAYER.md`](SIGMA_VFS_LAYER.md) — VFS uses RCU for mount table
- Linux kernel documentation: `Documentation/RCU/`
