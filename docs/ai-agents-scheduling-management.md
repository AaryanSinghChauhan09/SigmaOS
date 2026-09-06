# 🇸🇴 AI Agents Scheduling Management Architecture in SigmaOS

## Executive Overview

SigmaOS introduces an **autonomous, sovereign AI Agent Scheduling Operation Management Architecture** that replaces rigid OS process schedulers with intelligent, real-time agentic governors. Operating directly within SigmaOS's zero-dependency `#![no_std]` Rust microkernel and userland layer, autonomous scheduling agents (`CpuSchedulerAgent`) continuously monitor CPU utilization, EEVDF virtual deadline lags, BORE burst scores, NUMA interconnect latencies, and thermal constraints to dynamically optimize thread execution paths across SMP and heterogeneous multi-core architectures.

By absorbing Linux kernel scheduling innovations (Linux 6.12+ `sched_ext` extensible eBPF schedulers, CachyOS BORE, EEVDF) and BSD interactivity frameworks (FreeBSD ULE dual queues, Apache NuttX POSIX RT preemption-threshold gating), SigmaOS AI Agents guarantee zero desktop jitter during heavy background AI inference or compilation workloads.

---

## 🌟 Architectural Principles & Linux/BSD Inspirations

SigmaOS unifies advanced CPU scheduling paradigms across Linux, FreeBSD, and real-time operating systems:

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                            SigmaOS AI Agent CPU Scheduler Governor                       │
│          (ACP / MCP Protocols, Dilithium-5 Attestation, OpenBSD Pledge Sandboxing)       │
└───────────────────────────┬──────────────────────────────────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┬──────────────────┐
         ▼                  ▼                  ▼                  ▼
┌─────────────────┐┌─────────────────┐┌─────────────────┐┌─────────────────┐
│ Linux EEVDF     ││ CachyOS BORE    ││ Linux 6.12+     ││ FreeBSD ULE     │
│ Fair Scheduling ││ Burst Response  ││ SchedExt eBPF   ││ Interactivity   │
│ (src/scheduler) ││ (src/kernel)    ││ (src/scheduler) ││ (src/scheduler) │
└─────────────────┘└─────────────────┘└─────────────────┘└─────────────────┘
```

### 1. Linux Kernel Scheduling Innovations
- **EEVDF (Earliest Eligible Virtual Deadline First - `src/scheduler/eevdf.rs`):** Calculates virtual runtime deadlines (`vruntime`) and lag bounds to achieve deterministic fair latency for user-facing applications.
- **CachyOS BORE (Burst-Oriented Response Enhancer - `src/kernel/bore.rs` & `CachyBoreScheduler` in `src/distro/linux_bsd_inspirations.rs`):** Tracks process burst patterns (`burst_time_ns`, `interactive_score`), boosting interactive task priority and assigning dynamic time slices.
- **Linux 6.12+ Extensible BPF Schedulers (`sched_ext` - `src/scheduler/ebpf_scheduler.rs` & `SovereignSchedExtEngine` in `src/distro/sovereign_nextgen_distro_leap.rs`):** Enables AI Agents to dynamically synthesize and load safe eBPF scheduling policies (`ScxSchedulerKind`) directly into active kernel space.

### 2. BSD & Real-Time Scheduling Paradigms
- **FreeBSD ULE Interactivity Engine (`src/scheduler/distro_schedulers.rs`):** Maintains dual interactive and batch queues (`batch_queue`) for SMP interactive task placement.
- **NUMA & Cache Locality (`src/scheduler/numa_scheduler.rs`):** Pins memory-intensive thread clusters to matching NUMA nodes, preserving L3 cache locality.
- **Energy-Aware Scheduling (EAS - `src/scheduler/energy_aware.rs`):** Directs latency-sensitive tasks to Performance cores (P-cores) while routing background AI/indexing tasks to Efficiency cores (E-cores).
- **Apache NuttX POSIX Real-Time Gating:** Enforces real-time preemption-threshold gating for deterministic deadline guarantees.

---

## 🤖 Core AI Scheduler Governors & Operations

### 1. Interactive Burst Score Optimizer (`CpuSchedulerAgent`)
- **Burst Profile Metrics:** Monitors task priority, interactive score (0..100), burst time, preferred core affinity, and IPC intensity.
- **Time Slice Calculation:** Computes quantum time slices dynamically based on target latency budgets:
  $$\text{Timeslice} = \frac{\text{SystemLatencyTarget}}{\max(1, N_{\text{tasks}})} + (100 - \text{InteractiveScore}) \times 10$$
- **Core Affinity Dispatch:** Selects optimal CPU cores based on `PerformancePCore`, `EfficiencyECore`, or `AnyCore` preferences.

### 2. Dynamic `sched_ext` eBPF Policy Synthesizer (`SovereignSchedExtEngine`)
- **Supported Scheduler Kinds:**
  - `ScxBlevdf`: EEVDF-based extensible BPF scheduler.
  - `ScxFlatcg`: Cgroup v2 flat-hierarchy scheduler.
  - `ScxRusty`: Rust-implemented extensible scheduler.
  - `ScxCentral`: Centralized single-core dispatcher for low-jitter gaming/audio.
- **Lifecycle Operations:** AI Agents hot-swap `sched_ext` policies at runtime via lock-free atomic pointer swaps without stopping process execution.

---

## 📡 Agent Protocol Integration (ACP / MCP)

### Agent Client Protocol (ACP)
- **JSON-RPC Scheduler Control:**
  - `sched_inspect`: Queries EEVDF lag, BORE interactivity scores, and core utilization heatmaps.
  - `sched_load_policy`: Loads a new `sched_ext` eBPF policy module.
  - `sched_pin_task`: Dynamically adjusts CPU core pinning and NUMA node affinity for target processes.

### Model Context Protocol (MCP)
- **Context Bridge:** Exposes thread latency statistics and cgroup v2 CPU quota allocations to local LLMs while enforcing OpenBSD `unveil` file boundaries.

---

## 🔒 Security, Attestation & Audit Governance

1. **Post-Quantum Cryptographic Attestation:**
   - Custom `sched_ext` eBPF scheduling modules are verified and signed using Dilithium-5 post-quantum digital signatures.
2. **Infinite Loop & Starvation Prevention:**
   - Maximum permitted step counters and watchdog timers prevent runaway or starved tasks during policy transitions.
3. **Immutable Audit Logging:**
   - Every CPU affinity change, policy hot-swap, and cgroup CPU quota adjustment is logged in the SigmaOS unified audit ledger (`UnifiedLogEntry`).

---

## 🛠️ Inspection & Manual Overrides

System administrators can inspect and override scheduling policies via `sigma-sh`:

```bash
# View active CPU scheduler status and BORE scores
sigma-sh> ai-agent status scheduler

# Inspect detailed task latency and core affinity
sigma-sh> ai-agent inspect task --pid=1001

# Hot-swap active sched_ext eBPF scheduler policy
sigma-sh> ai-agent set-policy --scheduler=scx_rusty

# Verify post-quantum signatures of active scheduler modules
sigma-sh> ai-agent verify-scheduler-signatures
```
