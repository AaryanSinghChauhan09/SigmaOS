# SigmaOS Kernel Threads (kthreads)

## Overview

Kernel threads run entirely in kernel space with no user address space. Used for background kernel services like memory reclaim, I/O processing, and work queues.

**Location:** `src/kernel/sigma_kthread.rs`

---

## Standard Kernel Threads

SigmaOS initialises these kernel daemons on boot:

| Thread | Priority | Function |
|--------|---------|---------|
| `kswapd0` | Normal | Page reclaim and swap |
| `kworker/0:0` | Normal | Work queue processing |
| `kcompactd0` | Normal | Memory compaction |
| `ksoftirqd/0` | Normal | Soft IRQ processing |
| `migration/0` | RealTime | CPU load balancing |

---

## Thread Lifecycle

```
Created → Running → Sleeping → Running → Stopped → Zombie
                ↕
              Parked ←→ Running
```

---

## API Reference

```rust
let mut mgr = KthreadManager::new();

// Create and start a kernel thread
let id = mgr.create("my_kernel_func", user_data, "my-kthread");
mgr.start(id).unwrap();

// Thread function pattern (inside the thread)
while !thread.should_stop() {
    if thread.should_park() {
        thread.do_park();
        // ... wait for unpark signal ...
    }
    // ... do work ...
}

// Control from outside
mgr.wake(id).unwrap();
mgr.park(id).unwrap();
mgr.unpark(id).unwrap();
mgr.stop(id).unwrap();
mgr.join(id).unwrap(); // Free resources

// Thread pool
mgr.create_pool("io-workers", 2, 8);
mgr.add_to_pool("io-workers", id).unwrap();
```

---

## Priority Levels

| Level | Value | Use Case |
|-------|-------|---------|
| RealTime | 0 | migration, interrupt processing |
| High | 1 | I/O completion |
| Normal | 2 | General kernel work |
| Low | 3 | Background tasks |
| Idle | 4 | Runs only when CPU is idle |

---

## Comparison: Linux vs BSD vs SigmaOS

| Feature | Linux kthread | BSD kthread(9) | SigmaOS |
|---------|-------------|--------------|---------|
| should_stop() | Yes | Manual | Yes |
| park/unpark | Yes | No | Yes |
| CPU affinity | kthread_bind() | No | Yes |
| Priority | Yes | Yes | Yes |
| Pool support | CMWQ | No | Yes |
| no_std | No | No | **Yes** |
