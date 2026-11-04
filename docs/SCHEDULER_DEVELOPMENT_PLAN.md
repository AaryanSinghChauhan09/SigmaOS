# Scheduler Development Plan

Complete PRs 1–7 before deciding whether EEVDF or BORE becomes the default fair policy.

## Proposed PR Roadmap
| 1 | `docs(scheduler): audit scheduler status and define invariants` |
| 2 | `feat(scheduler): bounded run queue core` |
| 3 | `feat(scheduler): context switch state machine` |
| 4 | `feat(scheduler): preemption timer tick` |
| 5 | `feat(scheduler): priority and vruntime accounting` |
| 6 | `feat(scheduler): SMP core affinity` |
| 7 | `feat(scheduler): real-time FIFO and RR policies` |
| 8 | `feat(scheduler): implement EEVDF fair-class prototype` |
| 9 | `feat(scheduler): BORE burst-oriented response governor` |
| 10 | `feat(scheduler): energy-aware scheduling hooks` |
| 11 | `feat(scheduler): autogroup cgroup delegation` |
| 12 | `feat(scheduler): lockless work stealing` |
| 13 | `feat(scheduler): starvation boosting watchdog` |
| 14 | `feat(scheduler): latency sensitivity tiers` |
| 15 | `feat(scheduler): userland scheduler hints API` |
| 16 | `feat(scheduler): sched_ext eBPF dispatcher bridge` |
| 17 | `feat(scheduler): comprehensive benchmark suite` |
| 18 | `docs(scheduler): publish policy guide and support matrix` |

## Periodic Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Gates
- **Prototype**
- **Integrated**
- **Supported**
