# AI Agent Guidelines for SigmaOS Scheduling Algorithms Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Process, CPU, I/O, and Distro Scheduling Algorithms**.

---

## 1. System Architecture & Scheduler Subsystem Layout

SigmaOS implements a comprehensive suite of production-oriented scheduling algorithms inspired by Linux, FreeBSD, DragonFly BSD, and modern performance-tuned kernels across `src/scheduler/`:

| Scheduler Engine | Primary Source File | Primary Feature & Algorithm |
| :--- | :--- | :--- |
| **EEVDF Scheduler (`EevdfScheduler`)** | `src/scheduler/distro_schedulers.rs`, `src/scheduler/eevdf.rs` | Earliest Eligible Virtual Deadline First (Linux 6.6+ default CFS replacement) |
| **BORE Scheduler (`BoreScheduler`)** | `src/scheduler/distro_schedulers.rs` | Burst-Oriented Response Enhancer (CachyOS / Gaming latency optimizer) |
| **PDS Scheduler (`PdsScheduler`)** | `src/scheduler/distro_schedulers.rs` | Priority-Deadline-Skiplist CPU scheduler |
| **MuQSS Scheduler (`MuqssScheduler`)** | `src/scheduler/distro_schedulers.rs` | Multiple Queue Skiplist Scheduler |
| **CFS Scheduler (`CfsScheduler`)** | `src/scheduler/distro_schedulers.rs` | Completely Fair Scheduler (Red-Black tree vruntime tracking) |
| **SCHED_DEADLINE (`SchedDeadline`)** | `src/scheduler/distro_schedulers.rs` | Earliest Deadline First (EDF) hard real-time scheduling |
| **POSIX Real-Time (`PosixRtFifoRrScheduler`)** | `src/scheduler/distro_schedulers.rs` | SCHED_FIFO and SCHED_RR real-time priority queues |
| **FreeBSD ULE (`FreeBsdUleScheduler`)** | `src/scheduler/distro_schedulers.rs` | FreeBSD SMP interactive/non-interactive queue scheduling |
| **DragonFly BSD Work-Stealing (`DragonFlyBsdWorkStealingScheduler`)** | `src/scheduler/distro_schedulers.rs` | Per-CPU runqueue work stealing for lockless load balancing |
| **CacuLLE Scheduler (`CaculeScheduler`)** | `src/scheduler/distro_schedulers.rs` | CACULE (Capacity-Aware CPU Load Evaluator based on RB-tree) |
| **Energy-Aware Scheduler (`EnergyAwareScheduler`)** | `src/scheduler/distro_schedulers.rs` | ARM energy model & CPU power efficiency scaling |
| **SCHED_EXT eBPF (`SchedExtBpfScheduler`)** | `src/scheduler/distro_schedulers.rs` | Extensible eBPF-based userland programmable scheduling |
| **Kyber I/O Scheduler (`KyberIoScheduler`)** | `src/scheduler/distro_schedulers.rs` | Low-latency NVMe / SSD block I/O request scheduler |
| **BFQ I/O Scheduler (`BfqIoScheduler`)** | `src/scheduler/distro_schedulers.rs` | Budget Fair Queueing storage request scheduler |
| **AI Predictive Scheduler (`AiPredictiveScheduler`)** | `src/scheduler/distro_schedulers.rs` | Workload classification & heuristic CPU frequency boosting |

---

## 2. Core Scheduling Mechanics & Code Patterns

AI agents modifying task scheduling or I/O dispatching algorithms must adhere to these patterns:

### 1. EEVDF Scheduling (`EevdfScheduler`)
Calculates virtual runtime (`vruntime`), lag, and virtual deadline (`vdeadline`) for fair CPU time distribution:
- Tasks with $lag > 0$ are eligible.
- Tasks are picked in order of earliest virtual deadline among eligible candidates.

```rust
use sigma::scheduler::distro_schedulers::{EevdfScheduler, SchedTask};

let mut scheduler = EevdfScheduler::new();
scheduler.add_task(SchedTask::new(1, "browser", 100, 10));
let next = scheduler.pick_next_task();
```

### 2. BORE Latency Booster (`BoreScheduler`)
Optimizes interactive desktop and gaming responsiveness by tracking task burstiness:
- Tasks with short execution bursts relative to sleep duration are granted temporary priority boosts.

```rust
use sigma::scheduler::distro_schedulers::BoreScheduler;

let mut bore = BoreScheduler::new();
bore.on_task_wake(pid, burst_time_ns);
```

### 3. Kyber & BFQ Block I/O Schedulers (`KyberIoScheduler`, `BfqIoScheduler`)
Regulates storage request latency and throughput:
- **Kyber:** Enforces strict read and write latency targets (e.g. 2ms read / 10ms write targets for NVMe storage).
- **BFQ:** Allocates disk sector time budgets per process entity (`BfqProcessEntity`).

```rust
use sigma::scheduler::distro_schedulers::KyberIoScheduler;

let mut kyber = KyberIoScheduler::new(2000, 10000); // 2ms read target, 10ms write target
kyber.dispatch_io_request(req_id, is_write);
```

---

## 3. Testing & Verification Protocol for AI Agents

When modifying scheduling algorithms, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suites for distro schedulers and eevdf:

```bash
rustc --test --edition=2021 src/scheduler/distro_schedulers.rs -o build/test_sched && ./build/test_sched
rustc --test --edition=2021 src/scheduler/eevdf.rs -o build/test_eevdf && ./build/test_eevdf
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core scheduling subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Performance Directives

- **Zero-Allocation Hot Ticks:** In-kernel `pick_next_task` decisions must be $O(1)$ or $O(\log N)$ bounded without dynamic heap allocations.
- **Fairness & Anti-Starvation:** Ensure Real-Time (SCHED_FIFO/RR) tasks do not indefinitely starve normal interactive or batch tasks.
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
