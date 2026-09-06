# AI Agent Development Instructions for Kernel Task & Scheduler Subsystems (`src/kernel/sched/` & `src/scheduler/`)

This directory tree contains the kernel task scheduling algorithms, CPU affinity managers, EEVDF (Earliest Eligible Virtual Deadline First), MLFQ (Multi-Level Feedback Queue), thermal-throttling schedulers, eBPF-driven scheduling policies, gaming performance boosters, and NUMA-aware task dispatchers for SigmaOS.

## Subsystem Architecture & Directives

1. **Kernel Task Structures & Preemption (`src/kernel/sched/task.rs` & `scheduler.rs`)**
   - Manage thread control blocks (TCBs), execution contexts, priority levels (0-139), and quanta slice allocations.
   - Context switches must save/restore hardware register state atomically and maintain strict interrupt safety (`IrqSafeSpinlock`).

2. **Linux & BSD Scheduler Parity (`src/scheduler/eevdf.rs` & `distro_schedulers.rs`)**
   - EEVDF scheduler calculates virtual runtime (`vruntime`) and lag parameters to guarantee proportional fairness.
   - FreeBSD ULE and Linux CFS/EEVDF algorithms must enforce latency bounds for interactive userland workloads.

3. **eBPF-extensible Scheduling (`src/scheduler/ebpf_scheduler.rs`)**
   - Dynamically load verified eBPF scheduling bytecode (`sched_ext` parity).
   - Fall back to standard kernel MLFQ if custom eBPF schedulers panic or exceed runtime execution bounds.

4. **Thermal & Energy-Aware Scheduling (`sigma_thermal_sched.rs` & `energy_aware.rs`)**
   - Monitor CPU core temperatures and migrate compute-heavy tasks off overheated cores before triggering hardware clock throttling.

5. **`no_std` Pure Rust & Verification**
   - Core scheduling algorithms in `src/kernel/sched/` must be pure `no_std`.
   - Verify changes with `cargo check --lib` prior to submitting changes.
