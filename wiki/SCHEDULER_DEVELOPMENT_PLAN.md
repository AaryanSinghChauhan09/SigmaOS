# Scheduler Development Plan

## Proposed PR Sequence
| 1 | `docs(scheduler): audit scheduler status and define invariants` |
| 2 | Core task queue |
| 3 | Run queue bounds |
| 4 | Context switch safety |
| 5 | Voluntary yield |
| 6 | Preemption timer tick |
| 7 | Starvation boost |
| 8 | `feat(scheduler): implement EEVDF fair-class prototype` |
| 9 | BORE latency tweaks |
| 10 | CPU affinity routing |
| 11 | Multi-core load balance |
| 12 | Real-time FIFO/RR guardrails |
| 13 | cgroup CPU shares |
| 14 | Tracepoint instrumentation |
| 15 | Benchmark harness |
| 16 | QEMU validation |
| 17 | Scheduler regression suite |
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
- Prototype: Basic runqueue
- Integrated: Context switch validated
- Supported: Full production tier evidence

- **Prototype**
- **Integrated**
- **Supported**
