# Comparative Analysis: Process Scheduling Mechanisms

| Operating System | Mechanism | Data Structure | Security Isolation | SigmaOS Decision |
|---|---|---|---|---|
| xv6 | Round-Robin | Process Array Scan | Basic Preemption | Replaced with EEVDF |
| Linux Kernel | EEVDF / CFS | Red-Black Tree (`cfs_rq`) | cgroups v2 Quotas | Adopted Virtual Deadline Calculation |
| FreeBSD | ULE Scheduler | Multi-Queue Runqueues | Jail Resource Limits | Evaluated |
| SigmaOS | Hybrid EEVDF | `Spinlock<BTreeMap>` | cgroup Quotas + Pledge Masks | Native Implementation |

## Tradeoff Analysis
Linux EEVDF provides optimal latency bounds for interactive tasks. SigmaOS adapts virtual deadline logic into safe Rust concurrency primitives.
