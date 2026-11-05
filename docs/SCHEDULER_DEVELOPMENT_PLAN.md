# Scheduler Development Plan

## Proposed PR Roadmap
| PR | Title |
|---|---|
| 1 | `docs(scheduler): audit scheduler status and define invariants` |
| 2 | `feat(scheduler): implement bounded run queues` |
| 3 | `feat(scheduler): context switch state machine` |
| 4 | `feat(scheduler): preemption and timer slice handler` |
| 5 | `feat(scheduler): vruntime fair metrics` |
| 6 | `feat(scheduler): starvation prevention boost` |
| 7 | `feat(scheduler): smp core affinity routing` |
| 8 | `feat(scheduler): implement EEVDF fair-class prototype` |
| 9 | `feat(scheduler): add BORE interactivity governor` |
| 10 | `feat(scheduler): integrate FIFO and RR realtime policies` |
| 11 | `feat(scheduler): cpu bandwidth cgroups integration` |
| 12 | `feat(scheduler): lockless work stealing across cores` |
| 13 | `feat(scheduler): sched_ext ebpf user scheduler support` |
| 14 | `feat(scheduler): dynamic frequency pstate governor integration` |
| 15 | `test(scheduler): multi-core latency benchmark suite` |
| 16 | `feat(scheduler): crash dump task backtrace recorder` |
| 17 | `feat(scheduler): procfs schedstat telemetry integration` |
| 18 | `docs(scheduler): publish policy guide and support matrix` |

Complete PRs 1–7 before deciding whether EEVDF or BORE becomes the default fair policy.

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Gates
- **Prototype**: basic mechanics verified.
- **Integrated**: scheduler integrated into kernel lifecycle.
- **Supported**: benchmark evidence and regression suites passing.
