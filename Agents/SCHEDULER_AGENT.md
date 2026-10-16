# Process & Thread Scheduler Component Agent

## Role
`SigmaScheduler` is the core CPU scheduler for SigmaOS — responsible for preemptive, multi-core time allocation, real-time deadlines, energy-aware placement, and CPU affinity enforcement inspired by Linux CFS/EEVDF and FreeBSD ULE.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Fair Scheduling | CFS / EEVDF | ULE Scheduler | SigmaFairScheduler (vruntime & lag) |
| Real-time | SCHED_FIFO, SCHED_RR | rtprio | SigmaRT (deterministic static priority) |
| Deadline | SCHED_DEADLINE | — | SigmaDL (Earliest Deadline First) |
| CPU Affinity | `sched_setaffinity` | `cpuset` | SigmaAffinity |
| Energy-aware | EAS / schedutil | powerd | SigmaEnergySched |
| Runqueues | Per-CPU runqueues | Per-CPU runqueues | SigmaRunQueue (lockless intrusive trees) |

## Current SigmaOS Status
Priority-based round-robin and fair virtual runtime queuing implemented in `src/kernel/scheduler.rs` and `src/scheduler/`. Full EEVDF lag compensation and NUMA work-stealing are active enhancement targets.

## Responsibilities & Architecture
- Per-CPU runqueue management with periodic work-stealing load balancing.
- Virtual runtime (`vruntime`) tracking with lag compensation to prevent task starvation.
- Real-time FIFO and Round-Robin scheduling classes for audio and interactive display threads.
- Earliest Deadline First (EDF) scheduler for hard real-time latency guarantees.
- Priority inheritance protocols to prevent priority inversion on kernel mutexes.
- CPU frequency governor integration (schedutil) adjusting DVFS states based on runqueue load.
- SMT (hyper-threading) and NUMA topology awareness for cache-friendly task placement.

## Source Files
- `src/kernel/scheduler.rs`
- `src/sched/eevdf.rs`
- `src/sched/topology.rs`
- `src/sched/rt.rs`
- `src/scheduler/`

## Testing Strategy
```bash
rustc --test src/scheduler/sched_fair.rs --edition 2021 -o /tmp/test_sched && /tmp/test_sched --test-threads=1
```
- Sub-2-microsecond scheduling jitter tests.
- Synthetic thread starvation benchmarks with 10,000+ active threads.
- Cyclictest real-time latency verification under heavy synthetic I/O load.

## Future Development
1. **EEVDF scheduler** — Earliest Eligible Virtual Deadline First algorithm (Linux 6.6+ parity).
2. **cgroup v2 CPU controller** — Hierarchical weighted CPU shares and hard CFS bandwidth throttling.
3. **Pluggable BPF scheduler** — Pluggable userspace/eBPF scheduling policies (sched-ext).
