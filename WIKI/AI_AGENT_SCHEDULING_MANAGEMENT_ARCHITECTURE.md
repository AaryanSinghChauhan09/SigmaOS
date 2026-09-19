# AI Agent Scheduling Operation Management Architecture in SigmaOS (`docs/AI_AGENT_SCHEDULING_MANAGEMENT_ARCHITECTURE.md`)

This document provides technical guidelines and reference specifications for AI agents developing, managing, and tuning **CPU process scheduling, real-time preemption, and thread affinity** in **SigmaOS**.

---

## 1. Overview & Scheduler Subsystem Reference

In SigmaOS, process scheduling operations are handled by native `#![no_std]` Rust modules:

- **EEVDF Scheduler (`src/scheduler/eevdf.rs`)**: Earliest Eligible Virtual Deadline First scheduling, managing virtual vruntime deadlines and latency lag compensation.
- **CachyOS BORE Scheduler (`src/kernel/bore.rs` & `CachyBoreScheduler` in `src/distro/linux_bsd_inspirations.rs`)**: Burst-Oriented Response Enhancer tracking process burst patterns (`BoreTaskProfile`), calculating quantum timeslices, and prioritizing interactive desktop tasks.
- **FreeBSD ULE Scheduler (`src/scheduler/distro_schedulers.rs`)**: Dual interactive/batch queue scheduling for SMP thread distribution.
- **Linux 6.12+ `sched_ext` Extensible eBPF Scheduler (`src/scheduler/ebpf_scheduler.rs` & `SovereignSchedExtEngine` in `src/distro/sovereign_nextgen_distro_leap.rs`)**: Programmable eBPF scheduling engine (`ScxSchedulerKind`).
- **NUMA & Energy-Aware Schedulers (`src/scheduler/numa_scheduler.rs` & `src/scheduler/energy_aware.rs`)**: Dynamic thread affinity, L3 cache alignment, and P/E core routing.

---

## 2. Core OOP Design Patterns for Scheduling Operations

AI agents extending or tuning the scheduling subsystem must adhere to the following design patterns:

### A. Strategy Pattern (`CachyBoreScheduler` & `SovereignSchedExtEngine`)
- Scheduler engines implement pluggable scheduling strategies selected dynamically based on system workload:

```rust
// Example: Registering and scheduling tasks via CachyBoreScheduler
let mut bore_sched = CachyBoreScheduler::new(10_000_000); // 10ms target latency

bore_sched.register_task(BoreTaskProfile {
    task_id: 1001,
    name: "zenith_compositor".to_string(),
    priority: 10,
    interactive_score: 95,
    burst_time_ns: 500_000,
    preferred_core: CoreTypePreference::PerformancePCore,
    ipc_intensity: 80,
});

// Calculate dynamic time slice
let timeslice_ns = bore_sched.calculate_timeslice_ns(1001);

// Pick next optimal task for Performance Core
let next_task = bore_sched.schedule_next_task(CoreTypePreference::PerformancePCore);
```

### B. Extensible BPF Policy Dispatch (`SovereignSchedExtEngine`)
- AI agents dynamically register, verify, and activate eBPF `sched_ext` scheduling tasks:

```rust
let mut sched_ext = SovereignSchedExtEngine::new();
sched_ext.select_scheduler_kind(ScxSchedulerKind::ScxRusty);

// Register task under active BPF policy
let task = SchedExtTask {
    pid: 2001,
    slice_ns: 5_000_000,
    state: ScxTaskState::Runnable,
    cpu_affinity_mask: 0x0F, // Cores 0-3
};
sched_ext.enqueue_task(task)?;
```

---

## 3. Best Practices for AI Agents Working with Scheduling

1. **Zero External Dependencies**:
   - Maintain `#![no_std]` Rust compatibility. Use `alloc::string::String`, `alloc::vec::Vec`, and `alloc::format!`.
2. **Interactive Jitter Prevention**:
   - Assign high interactive scores (80..100) and shorter quantum time slices to desktop compositor (`Zenith`) and audio threads.
3. **P/E Core Alignment**:
   - Route compute-heavy or background compilation tasks (`cargo`, `make`) to Efficiency cores (`EfficiencyECore`) to preserve battery life and prevent thermal throttling.
4. **Lock-Free Execution Paths**:
   - Ensure thread scheduling dispatch loops avoid heap allocations and mutex lock contention.

---

## 4. Verification & Testing

AI agents must verify scheduling changes using standalone unit tests and the inspection test runner:

```bash
# 1. Compile & run linux_bsd_inspirations standalone tests
rustc --edition=2021 --test src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# 2. Run global inspection test suite
./run_sigma_tests.sh
```

---
*End of docs/AI_AGENT_SCHEDULING_MANAGEMENT_ARCHITECTURE.md*
