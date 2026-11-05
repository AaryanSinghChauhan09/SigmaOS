# SigmaOS Scheduler Development Plan

## Overview
Comprehensive plan for scheduling subsystem implementation, policies, and roadmap.

Complete PRs 1–7 before deciding whether EEVDF or BORE becomes the default fair policy.

## PR Sequence Matrix
| PR | Title |
|---|---|
| 1 | `docs(scheduler): audit scheduler status and define invariants` |
| 2 | `feat(scheduler): implement core runqueue abstraction` |
| 3 | `feat(scheduler): implement context switch primitive` |
| 4 | `feat(scheduler): bounded preemption quantum timer` |
| 5 | `feat(scheduler): task priority state tracking` |
| 6 | `feat(scheduler): basic round-robin time slicing` |
| 7 | `feat(scheduler): cfs vruntime tracking contract` |
| 8 | `feat(scheduler): implement EEVDF fair-class prototype` |
| 9 | `feat(scheduler): BORE burst-oriented responsiveness engine` |
| 10 | `feat(scheduler): real-time FIFO and RR scheduling classes` |
| 11 | `feat(scheduler): CPU affinity mask enforcement` |
| 12 | `feat(scheduler): multi-core load balancing across CPUs` |
| 13 | `feat(scheduler): sleep and wait-channel synchronization` |
| 14 | `feat(scheduler): starvation prevention and priority boosting` |
| 15 | `feat(scheduler): cgroups v2 cpu controller bandwidth limits` |
| 16 | `feat(scheduler): scheduler diagnostics and telemetry exporters` |
| 17 | `test(scheduler): multi-core preemption and latency stress suite` |
| 18 | `docs(scheduler): publish policy guide and support matrix` |

## Maintenance Cadence
- Every PR: run fast unit tests
- Weekly: review pending PRs and metrics
- Monthly: sync architecture roadmap
- Quarterly: full security audit
- Each release candidate: verify all tiers
- After each incident: conduct postmortem and update rules

## Completion Gates
- **Prototype**
- **Integrated**
- **Supported**
