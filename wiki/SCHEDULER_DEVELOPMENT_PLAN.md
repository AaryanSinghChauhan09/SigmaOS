# Scheduler Development Plan

## Proposed PR Sequence
| 1 | `docs(scheduler): audit scheduler status and define invariants` |
| 2 | Sched core |
| 3 | Run queue |
| 4 | Context switch |
| 5 | Preemption |
| 6 | Fairness |
| 7 | Affinity |
| 8 | `feat(scheduler): implement EEVDF fair-class prototype` |
| 9 | BORE scheduler |
| 10 | Real-time class |
| 11 | Deadline class |
| 12 | Idle class |
| 13 | Load balancer |
| 14 | Energy-aware scheduling |
| 15 | Starvation prevention |
| 16 | Tracing & telemetry |
| 17 | Benchmarks & verification |
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
- **Prototype**
- **Integrated**
- **Supported**
