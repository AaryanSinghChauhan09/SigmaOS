# Kernel Scheduler Agent

## Role
The SigmaOS Kernel Scheduler (`SigmaScheduler`) is responsible for CPU time allocation across all runnable processes and threads. It implements preemptive, priority-based, real-time-capable scheduling derived from Linux CFS and FreeBSD ULE.

## Status
🟡 In Progress — Core round-robin in place; CFS-class fairness and deadline scheduling needed.

## Source File
`src/scheduler/` (multiple modules: `schedutil.rs`, `sched_deadline.rs`, `sched_fair.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Fair scheduling | CFS (Completely Fair Scheduler) | ULE Scheduler | SigmaFairScheduler |
| Real-time | SCHED_FIFO, SCHED_RR | rtprio | SigmaRT |
| Deadline | SCHED_DEADLINE | — | SigmaDL (planned) |
| CPU affinity | `sched_setaffinity` | `cpuset` | SigmaAffinity |
| Governor | schedutil / ondemand | powerd | SigmaGov |

## Responsibilities
- Process/thread selection per CPU runqueue
- Priority inheritance and inversion prevention
- Real-time FIFO/RR scheduling classes
- NUMA-aware load balancing
- Idle thread management and power-aware scheduling
- CPU frequency governor integration (schedutil)
- Preemption point management (CONFIG_PREEMPT equivalent)

## Implementation Notes
- Use red-black tree for O(log n) task insertion/deletion (like Linux CFS)
- vruntime-based virtual time tracking for fairness
- Per-CPU runqueues with periodic load balancing
- `#![no_std]` — use `alloc::collections::BTreeMap` for runqueue
- Integrate with `SigmaResource` for cgroup-based CPU limits

## Testing
```bash
rustc --test src/scheduler/sched_fair.rs --edition 2021 -o /tmp/test_sched && /tmp/test_sched --test-threads=1
```

## Future Development
1. **EEVDF Scheduler** — Earliest Eligible Virtual Deadline First (Linux 6.6+)
2. **SMT-aware scheduling** — hyper-threading topology awareness
3. **Energy-aware scheduling (EAS)** — ARM big.LITTLE support
4. **cgroup v2 CPU controller** — weighted CPU shares
5. **Latency-nice** — fine-grained latency class for audio/GUI
6. **BPF scheduler hooks** — pluggable scheduler policies via SigmaBPF
