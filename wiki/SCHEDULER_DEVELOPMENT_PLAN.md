# Scheduler Development Plan

## PR Roadmap Sequence
| PR | Title | Status |
|---|---|---|
| 1 | `docs(scheduler): audit scheduler status and define invariants` | Done |
| 2 | `feat(scheduler): run queue data structure` | Done |
| 3 | `feat(scheduler): priority and quantum calculation` | Done |
| 4 | `feat(scheduler): context switch mechanics` | Done |
| 5 | `feat(scheduler): preemption trigger` | Done |
| 6 | `feat(scheduler): starvation boost` | Done |
| 7 | `feat(scheduler): cpu affinity mask` | Done |
| 8 | `feat(scheduler): implement EEVDF fair-class prototype` | Done |
| 9 | `feat(scheduler): BORE interactivity heuristics` | Done |
| 10 | `feat(scheduler): real-time FIFO/RR scheduling` | Done |
| 11 | `feat(scheduler): idle task accounting` | Done |
| 12 | `feat(scheduler): multi-core load balancing` | Done |
| 13 | `feat(scheduler): energy aware scheduling` | Done |
| 14 | `feat(scheduler): cgroups v2 cpu controller` | Done |
| 15 | `feat(scheduler): latency sensitivity flags` | Done |
| 16 | `feat(scheduler): tracepoint hooks` | Done |
| 17 | `test(scheduler): microbenchmarks suite` | Done |
| 18 | `docs(scheduler): publish policy guide and support matrix` | Done |

Complete PRs 1–7 before deciding whether EEVDF or BORE becomes the default fair policy.

## Maintenance Cadence
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
