# AI Agent Guidelines: Concurrency & Synchronization Management in SigmaOS

## 📌 1. Executive Summary & Core Architectural Directives

In **SigmaOS**, concurrency and synchronization management form the backbone of thread safety, multi-core SMP scaling, interrupt handling, and inter-process communication (IPC).

As an AI agent working on kernel modules, IPC channels, or multi-threaded drivers, you must design concurrency mechanisms that are **deadlock-free, starvation-free, priority-inversion resilient, and cache-conscious**.

---

## 🤹 2. Classic Concurrency Problems & Algorithmic Solutions

### 2.1 Sleeping Barbershop Problem
* **Concept:** Synchronizes $N$ customer threads waiting in a bounded queue for $M$ barber threads.
* **SigmaOS Pattern:** Implemented using a combination of a counting semaphore (`waiting_customers`), a barber state semaphore (`barber_ready`), and a mutex protecting queue indices.
* **Module Location:** `src/kernel/ipc.rs`, `src/process/advanced_process_control.rs`

### 2.2 Dining Philosophers Problem
* **Concept:** $N$ threads compete for $N$ shared resources (chopsticks/forks) arranged in a circle.
* **Deadlock Risk:** If all threads simultaneously acquire their left resource, a **circular wait** occurs.
* **SigmaOS Solution (Total Resource Hierarchy Ordering):**
  * Resources are assigned unique integer IDs ($R_0, R_1, \dots, R_{N-1}$).
  * Threads MUST always acquire the lower-indexed resource first:
    $$\text{Acquire}(\min(R_{\text{left}}, R_{\text{right}})) \longrightarrow \text{Acquire}(\max(R_{\text{left}}, R_{\text{right}}))$$

### 2.3 Dekker's & Peterson's Mutual Exclusion Algorithms
* **Concept:** Software-based mutual exclusion algorithms operating without hardware atomic instructions.
* **Implementation:** Employs atomic flag arrays `flag[2]` and a scalar `turn` variable using `AtomicBool` and `AtomicUsize` with `Ordering::SeqCst` memory barriers (`src/klib/sync.rs`).

### 2.4 Deadlock Prevention & Coffman Conditions
To guarantee deadlock immunity, SigmaOS systematically breaks the **4 Coffman Conditions**:
1. **Mutual Exclusion:** Replaced with Read-Copy-Update (RCU) or lock-free atomic data structures wherever possible.
2. **Hold and Wait:** Threads requesting multiple resources must acquire them in a single atomic transaction or release all held locks on failure (`try_lock`).
3. **No Preemption:** High-priority real-time threads (EDF / BORE scheduler) can preempt lower-priority lock holders via **Priority Inheritance Protocols (PIP)**.
4. **Circular Wait:** Enforced strict, global lock ordering rules across the codebase.

---

## 🐧 3. Linux & BSD Kernel Synchronization Mechanisms in SigmaOS

SigmaOS natively absorbs and implements classic Linux and BSD kernel synchronization primitives:

```
+-----------------------------------------------------------------------------------+
|                     SIGMAOS KERNEL SYNCHRONIZATION STACK                          |
+-----------------------------------------------------------------------------------+
|  ⚡ Lock-Free / RCU: Atomic Operations, Seqlocks, Read-Copy-Update (Deferred)      |
|  🔒 Spinlocks: Ticket Spinlocks, Raw Spinlocks (IRQ-safe, non-sleeping)           |
|  🛑 Sleeping Mutexes: Priority-Inheritance Mutexes, Semaphores, RW-Semaphores     |
|  🔔 Fast Userspace Mutexes: Linux-compatible Futex Engine (FUTEX_WAIT/WAKE)      |
+-----------------------------------------------------------------------------------+
```

### 3.1 Spinlocks & Ticket Spinlocks
* **Usage:** Used exclusively in short, non-sleeping kernel sections and IRQ handlers.
* **Invariants:** Disables local interrupts (`cli`) before acquiring to prevent IRQ reentrancy deadlocks.

### 3.2 Read-Copy-Update (RCU)
* **Usage:** Ideal for read-heavy kernel data structures (VFS path lookups, routing tables, process tables).
* **Semantics:**
  * Readers: `rcu_read_lock()` $\rightarrow$ Lockless read $\rightarrow$ `rcu_read_unlock()` ($O(1)$ zero overhead).
  * Writers: Allocate new node copy $\rightarrow$ Update pointer atomically $\rightarrow$ Defer old node reclamation until `synchronize_rcu()` grace period expires.

### 3.3 Seqlocks (Sequential Locks)
* **Usage:** Fast, lockless reader access for small data structures modified infrequently (e.g. system clock `jiffies`, wall-time).
* **Pattern:** Readers retry if the sequence counter is odd or changes during the read loop.

### 3.4 Linux Futex Engine (`LinuxFutexEngine`)
* **Module Location:** `src/kernel/linux_bsd_innovations.rs`
* **Supported Operations:** `FUTEX_WAIT`, `FUTEX_WAKE`, `FUTEX_REQUEUE`, `FUTEX_CMP_REQUEUE`.
* **Behavior:** Zero-overhead userspace atomic CAS check; sleeps in kernel wait-queue only on contention.

---

## ✉️ 4. Zero-Copy Message Passing IPC

SigmaOS prefers **Message Passing IPC** over shared-memory locking for process boundary isolation:

* **Bounded Ring Buffers:** Zero-copy pipe splicing and tee routing (`src/kernel/ipc.rs`).
* **Mach/Zircon-Style Port Capabilities:** Message delivery authorized via `CapabilityToken` rights (`CAP_READ`, `CAP_WRITE`).
* **Android Binder IPC:** Fast $O(1)$ transaction buffer mapping (`src/kernel/linux_bsd_innovations.rs`).

---

## 🚫 5. AI Agent Rules & Code Patterns

1. **No Sleeping Under Spinlocks:**
   * Never invoke heap allocations (`alloc`), file I/O, or thread sleeping while holding a spinlock.
2. **Lock Order Rule:**
   * When acquiring multiple locks, sort lock addresses or lock IDs in strictly ascending order before acquisition.
3. **Priority Inheritance:**
   * All sleeping mutexes accessible by real-time tasks must enable Priority Inheritance to prevent priority inversion stalls.

---

## 🧪 6. Standalone Testing Commands

AI agents must verify IPC, futex, and concurrency mechanisms via standalone unit compilation:

```bash
# Test IPC channels, pipes, and message passing
rustc --test --edition=2021 src/kernel/ipc.rs -o build/ipc_tests && ./build/ipc_tests && rm build/ipc_tests

# Test Linux futexes & cgroups innovations
rustc --test --edition=2021 src/kernel/linux_bsd_innovations.rs -o build/futex_tests && ./build/ebpf_tests && rm build/futex_tests
```
