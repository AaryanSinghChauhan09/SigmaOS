# SigmaOS CPU Scheduler Status and Invariants

## Architecture Inventory and Subsystem Context

SigmaOS's published `FEATURE_STATUS.toml` identifies three scheduling implementations:
- CFS (`src/scheduler/cfs.rs` and `src/kernel/cfs_scheduler.rs`)
- BORE (`src/scheduler/bore.rs`)
- Real-time FIFO/RR (`src/scheduler/distro_schedulers.rs` and `src/kernel/roundrobin.rs`)

Their evidence is currently unit-level—fairness, burst scoring, FIFO ordering, and RR rotation—while the stated next steps include integrating EEVDF (`src/scheduler/eevdf.rs`), improving group scheduling, and adding deadline scheduling. The architecture inventory also highlights that x86_64 context switching still needs integration with the scheduler and APIC timer calibration in QEMU, while AArch64 and RISC-V boot paths need QEMU validation. These are the key gaps: connect scheduler policy to real runnable tasks, timer preemption, and context switching before extending the policy set.

Linux's EEVDF documentation describes fair scheduling using virtual runtime/lag and virtual deadlines, with latency-sensitive tasks receiving shorter requested slices; FreeBSD's current scheduler work highlights fairness, predictable behavior, CPU affinity, and hybrid CPU support as active concerns. These are useful design and testing references—not a requirement to copy either kernel's implementation.

## Scheduler Policy and Invariants

Adopt the following 9 rules before changing scheduling behavior:

1. **Separate policy from mechanism.** The scheduler core owns run-queue locking, task-state transitions, timer interactions, context-switch hooks, and CPU selection interfaces. Scheduling classes own policy decisions. Avoid a hybrid CFS/BORE/EEVDF policy until the class boundaries and precedence rules are explicit.

2. **Define task states and transitions.** Document runnable, running, blocked, stopped, exiting, and idle states, with valid enqueue/dequeue/wakeup/preemption transitions. A task must be on at most one run queue and must never run on two CPUs simultaneously.

3. **Fair scheduling needs a measurable contract.** Specify relative CPU share by weight, wakeup latency targets, starvation bounds, and how sleeping tasks accrue or decay fairness debt. Linux EEVDF is a reference for combining fairness with virtual deadlines and handling sleeper behavior.

4. **Real-time policies need guardrails.** FIFO/RR tasks must have explicit priority semantics, quantum behavior, privilege policy, and starvation controls. Do not label a policy real-time until interrupt, timer, preemption, and worst-case latency paths are measured.

5. **Scheduler changes require data, not anecdotes.** Record throughput, fairness, wake-to-run latency, context-switch rate, CPU migrations, run-queue imbalance, and idle time. Preserve baseline workloads and hardware/QEMU configuration for comparisons.

6. **Concurrency is part of correctness.** Specify run-queue lock ordering, preemption and interrupt assumptions, per-CPU state protection, memory ordering, and context-switch ownership. Linux's preemptible-kernel guidance emphasizes that preemption creates concurrency/reentrancy hazards, including for per-CPU state.

7. **Resource controls must affect real scheduling.** SigmaOS's inventory says CPU controllers exist as a model but still need scheduler integration. Do not advertise CPU quotas/weights as enforced until scheduler paths actually account for them.

8. **Keep architecture status separate.** A scheduler is not Supported on AArch64 or RISC-V because it is tested on x86_64; validate each architecture's timer, context switch, idle path, interrupt return, and CPU topology.

9. **Keep diagnostics bounded and stable.** Expose versioned scheduler counters through an appropriate read-only interface. Linux's schedstats documentation warns that counter formats evolve and tools must account for versioning.
