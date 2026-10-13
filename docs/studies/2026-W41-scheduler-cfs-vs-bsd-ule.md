# Study: Linux CFS vs BSD ULE Scheduler Mechanisms

## Reference Systems
- **Linux (CFS - Completely Fair Scheduler):** Uses Red-Black trees ordered by virtual runtime (`vruntime`). Provides latency targets (`sysctl_sched_latency`), minimum granularity, and dynamic priority scaling via nice values.
- **FreeBSD / OpenBSD (ULE Scheduler):** Uses multi-queue interactivity heuristics, separate run queues for real-time, interactive, and batch threads, and explicit CPU affinity topology tracking.
- **Redox / xv6 / Other:** xv6 uses simple round-robin with process lock governance; Redox uses priority-tiered round-robin with scheme IPC yielding.

## Mechanism Under Study
- **Problem:** Balancing CPU fairness, interactive latency responsiveness, and SMP affinity without priority inversion or thread starvation.
- **Interface:** `sched_yield()`, `nice()`, `sched_setaffinity()`, `sched_setscheduler()`.
- **Data Structures:** 64-priority level run queues with 64-bit selection bitmaps and Red-Black vruntime tracking.
- **Concurrency Model:** Lock-free / spinlock-protected per-CPU run queues with load-balancing IPIs.
- **Failure Behavior:** Starvation protection via periodic boost intervals; bounded run queue capacity.

## SigmaOS Current State
- **Code Location:** `src/scheduler/core.rs`, `src/scheduler/run_queue.rs`, `src/kernel/tier2_usability_stack.rs`.
- **Existing Tests:** `tests/scheduler/`, `tests/scheduler/scheduler_test.rs`.
- **Verified Runtime Behavior:** Bounded 64-priority run queues with 64-bit bitmap selection, starvation boosting, and 6-state POSIX lifecycle transitions.
- **Known Gaps:** Multi-core SMP load balancing across heterogenous CPU topology requires active QEMU SMP boot stress testing.

## Adaptation Decision
- **Adapt:**
- **Reason:** Adapt Linux CFS vruntime tracking for fair thread time-slice calculation combined with BSD ULE per-CPU run queue topology and starvation boosting to ensure deterministic low-latency context switching in safe Rust.

## Proposed PRs
- [x] PR 1: Bounded 64-priority level run queue bitmap scheduler core.
- [ ] PR 2: Multi-core SMP load-balancing dynamic scheduling migration.

## Acceptance Evidence
- [x] `cargo check --lib`
- [x] `cargo test --lib`
- [x] `./run_sigma_tests.sh`
- [x] QEMU / boot integration test suite (`tests/boot/boot_to_shell_test.py`)
