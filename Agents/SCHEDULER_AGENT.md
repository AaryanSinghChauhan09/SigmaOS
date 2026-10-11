# Scheduler Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
The SigmaOS scheduler is responsible for fair, efficient, and responsive allocation of CPU time across all runnable tasks, supporting both interactive workloads (low-latency) and batch/throughput workloads. It must match or exceed Linux CFS latency on single-core and scale linearly to 256+ cores.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` support)
- **Systems-level:** Zig (comptime, explicit allocators, C interop when needed)
- **Scripting/tooling:** Nim (fast compile, GC-free mode available)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for OS components
- **unsafe Rust:** Only with `// SAFETY:` justification comment; prefer safe alternatives

## 3. Architecture Inspiration

### Linux Kernel
- **CFS (Completely Fair Scheduler):** Virtual runtime (`vruntime`) tracking; red-black tree runqueue; `min_vruntime` for fairness; `sched_latency_ns` and `sched_min_granularity_ns` tuning
- **EEVDF (Earliest Eligible Virtual Deadline First):** Linux 6.6+ replacement for CFS; eligible time + virtual deadline for better latency
- **RT scheduler:** `SCHED_FIFO` and `SCHED_RR` for real-time tasks; priority ceiling 0-99
- **Deadline scheduler (`SCHED_DEADLINE`):** EDF + CBS for hard real-time; runtime/deadline/period parameters
- **`sched_domains`:** Hierarchical CPU topology (SMT → core → socket → NUMA node) for intelligent load balancing
- **NOHZ:** Tickless kernel; per-CPU timer wheels; eliminates idle-CPU timer interrupts

### FreeBSD
- **ULE Scheduler:** O(1) interactive/batch detection; interactivity score decay; separate runqueues per priority
- **4BSD Scheduler (legacy):** Dynamic priority with aging; useful for understanding decay algorithms
- **Per-CPU runqueues:** Lock-free fast path for same-CPU wakeup

### OpenBSD
- **Simple priority-based scheduler:** 128 priority levels; straightforward; auditable code
- **Security-conscious design:** No speculative execution hints in scheduler paths (Spectre mitigation)

### NetBSD
- **M:N threading (historical):** Lessons learned from complex scheduler-activations; why 1:1 is better
- **NUMA-aware scheduling:** `src/sys/kern/kern_cpu.c` NUMA topology detection

### Arch Linux
- **linux-zen patches:** `BORE (Burst-Oriented Response Enhancer)` scheduler patch for desktop responsiveness
- **Preemption model:** `PREEMPT_DYNAMIC` allowing runtime preemption model switching

### NixOS
- Declarative CPU governor configuration
- Reproducible scheduler tuning via `boot.kernelParams`

## 4. SigmaOS Source Files
- `src/kernel/sched/mod.rs` — Scheduler subsystem root, `schedule()` entry point
- `src/kernel/sched/eevdf.rs` — EEVDF algorithm implementation
- `src/kernel/sched/rt.rs` — Real-time scheduler (FIFO/RR)
- `src/kernel/sched/deadline.rs` — Deadline scheduler (EDF+CBS)
- `src/kernel/sched/runqueue.rs` — Per-CPU runqueue (red-black tree)
- `src/kernel/sched/load_balance.rs` — SMP load balancing and task migration
- `src/kernel/sched/topology.rs` — CPU topology discovery (SMT/NUMA)
- `src/kernel/sched/idle.rs` — Idle task and NOHZ tickless logic
- `src/kernel/sched/stats.rs` — Scheduler statistics and tracing hooks
- `src/kernel/sched/cgroup.rs` — CPU cgroup bandwidth control

## 5. Current Implementation Status
| Feature | Status | Notes |
|---------|--------|-------|
| Basic round-robin scheduler | ✅ Implemented | Single-core only |
| Priority levels | 🚧 Partial | Static priorities; no dynamic adjustment |
| EEVDF / CFS-style fairness | 🔲 Missing | No vruntime tracking |
| Real-time (FIFO/RR) | 🔲 Missing | No RT scheduling class |
| Deadline scheduling | 🔲 Missing | No EDF/CBS |
| SMP load balancing | 🔲 Missing | Single-core only |
| NOHZ tickless | 🔲 Missing | Fixed HZ tick |
| CPU topology awareness | 🔲 Missing | No NUMA/SMT detection |
| Scheduler statistics | 🔲 Missing | No /proc/schedstat equivalent |
| CPU cgroup control | 🔲 Missing | No bandwidth limiting |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** EEVDF runqueue: Implement red-black tree keyed on virtual deadline; `vruntime` tracking per task; `eligible_time` computation. This is the foundation of fair scheduling.
2. **[PRIORITY: HIGH]** Preemption: Enable kernel preemption at safe points; `need_resched` flag per CPU; preemption counter for critical sections.
3. **[PRIORITY: HIGH]** SMP load balancing: Per-CPU runqueues; periodic `load_balance()` triggered by scheduler tick; pull migration from overloaded CPUs.
4. **[PRIORITY: HIGH]** Real-time scheduling class: `SCHED_FIFO` and `SCHED_RR` with 0-99 priority levels; preempt normal tasks always.
5. **[PRIORITY: MEDIUM]** CPU topology: Parse ACPI SLIT/SRAT for NUMA topology; build `sched_domain` hierarchy.
6. **[PRIORITY: MEDIUM]** NOHZ: Skip scheduler tick on idle CPUs; use one-shot timer for next wakeup.
7. **[PRIORITY: MEDIUM]** Deadline scheduler: `SCHED_DEADLINE` with runtime/deadline/period; admission control.
8. **[PRIORITY: LOW]** CPU cgroup bandwidth: `cpu.cfs_quota_us` / `cpu.cfs_period_us` throttling.

## 7. Implementation Guidelines for AI Agents

### Adding New Features
1. New scheduling classes implement the `SchedClass` trait in `src/kernel/sched/mod.rs`.
2. Per-CPU data accessed via `per_cpu!(var, cpu_id)` macro — never global mutable state.
3. Runqueue lock must be held for all runqueue modifications; document lock order.
4. Avoid allocating memory in the scheduler hot path — use pre-allocated structures.
5. Always run `cargo check --target x86_64-unknown-none` before committing.
6. Follow `Stability > Security > Performance` priority.
7. Commit format: `feat(scheduler): <description>`

### Fixing Bugs
1. Reproduce with a minimal test case in QEMU; use `schedtool` or `chrt` equivalent.
2. Fix in safe Rust first; resort to unsafe only for per-CPU access patterns.
3. Add regression test.
4. Commit format: `fix(scheduler): <description>`

### Code Quality Checklist
- [ ] `cargo check` passes with 0 errors
- [ ] No `clippy::unused_variables` warnings
- [ ] No hard-coded credentials or secrets
- [ ] No `unsafe` without `// SAFETY:` justification comment
- [ ] Error handling uses `Result<T, E>` not `unwrap()` in production code
- [ ] All public functions have doc comments

## 8. Key Algorithms & Data Structures
- **Red-Black Tree:** O(log n) insert/delete/min for EEVDF runqueue; keyed on `virtual_deadline`
- **EEVDF:** `vruntime += weight * actual_runtime`; eligible when `eligible_time <= current_vruntime`; pick leftmost eligible node
- **Load balance:** Weighted load = sum of task weights; pull from busiest CPU when imbalance > threshold
- **Interactivity heuristic:** Exponential decay of sleep/run ratio (ULE-inspired); boost priority of interactive tasks
- **Timer wheel:** Hierarchical 512-slot wheel for O(1) timer insertion/expiry

## 9. Security Considerations
- **Scheduler-based side channels:** Avoid leaking information via scheduling decisions (e.g., Spectre v4 variants); use `sched_yield` barriers where needed.
- **Priority inversion:** Implement priority inheritance for mutex-holding tasks.
- **Resource starvation:** Guarantee minimum CPU time to all tasks via bandwidth controller.
- **CPU isolation:** `isolcpus` equivalent to dedicate cores to specific tasks.
- **Seccomp integration:** Scheduler must cooperate with syscall filtering; no scheduler bypass via `sched_setscheduler`.

## 10. Performance Targets
| Metric | Linux CFS | SigmaOS Target |
|--------|-----------|----------------|
| Context switch latency | ~1.5µs | < 2µs |
| Scheduler tick overhead | ~2µs | < 2.5µs |
| Wakeup latency (idle CPU) | ~10µs | < 15µs |
| Load balance interval | 1-4ms | 1-4ms |
| Max runnable tasks (stable) | 100k+ | 10k+ initially |

## 11. Testing Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_eevdf_fairness_two_tasks() {
        // Two equal-weight tasks should each get ~50% CPU over 100ms
    }

    #[test_case]
    fn test_rt_preempts_normal() {
        // A SCHED_FIFO task should immediately preempt a CFS task
    }

    #[test_case]
    fn test_load_balance_migrates_task() {
        // With 2 CPUs and 3 tasks, load_balance should migrate 1 task
    }

    #[test_case]
    fn test_no_starvation() {
        // Low-priority task must get at least min_granularity time within sched_latency
    }
}
```

## 12. Dependencies Between Components
- **Depends on:** Kernel (process/thread structs, locking), Memory Management (task stack allocation), Timekeeping (tick, clock_gettime), IPC (wakeup on semaphore/mutex)
- **Used by:** All userspace processes and kernel threads; IPC (sleep/wakeup); Power Management (CPU idle)

## 13. Maintenance Instructions for Future AI Agents
1. When Linux releases a new kernel, check `kernel/sched/` changelog — EEVDF and BORE patches change frequently.
2. Compare SigmaOS vruntime accounting with Linux `kernel/sched/fair.c`.
3. Prioritize stability: never change the `Task` struct layout without updating all callers.
4. Always update this file when making changes to `src/kernel/sched/`.
5. Wiki page for this component: `Scheduler-Component-Guide` on GitHub Wiki.

## 14. References & Standards
- Linux source: `kernel/sched/`, `Documentation/scheduler/`
- FreeBSD source: `sys/kern/sched_ule.c`
- EEVDF paper: "Earliest Eligible Virtual Deadline First" (Stoica & Zhang, 1996)
- POSIX.1-2017 scheduling: `sched_setscheduler(2)`, `sched_getparam(2)`
- SigmaOS Wiki page: `Scheduler-Component-Guide`
