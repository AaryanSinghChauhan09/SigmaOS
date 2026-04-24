
# Kernel Services Architecture


SigmaOS entirely decouples and secures the core triplet of Operating System design: **Interrupts, Memory Management, and Scheduling**.

Located in `modules/core/kernel/`.


## Competitive Advantages (USPs) over Linux



### 1. Capability-Routed Interrupt Controller (`interrupts.c`)

- **Standard Linux**: A kernel module can hook any IRQ, potentially crashing the system or intercepting sensitive keyboard/network data.
- **SigmaOS USP**: seL4-inspired capability routing. A driver must possess a cryptographically verified `CAP_IRQ_BIND` token to register an interrupt handler. Furthermore, the kernel features **Self-Healing IRQ Storm Protection** — if faulty hardware fires an interrupt >1000 times a millisecond, the kernel automatically isolates the vector to prevent a system lockup.


### 2. Unified Memory Manager (`memory_manager.c`)

- **Standard Linux**: Uses standard Virtual Memory Area (VMA) linked lists and monolithic `mmap` calls.
- **SigmaOS USP**: Integrates directly with the `Memory-as-Contracts` system. A Page Fault is only resolved if the process holds a valid cryptographic lease for that physical page. It also natively supports Copy-on-Write (CoW) memory versioning snapshots.


### 3. Policy-Agnostic Core Scheduler (`scheduler_core.c`)

- **Standard Linux**: Uses the Completely Fair Scheduler (CFS). Replacing it requires deep kernel patching and recompilation.
- **SigmaOS USP**: The core scheduler contains zero algorithmic logic. It simply maintains the state machine and strictly enforces Sovereign Token expiries on every clock tick. To pick the next process, it invokes `policy_schedule_next()`, delegating the logic to a hot-swappable Service Capsule (like `ai_scheduler.c` or a Real-Time policy).
