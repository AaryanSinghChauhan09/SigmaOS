# SigmaOS Physical Memory Manager Plan

## Core Invariants
- Only explicitly usable RAM is allocatable.
- Keep physical and virtual addresses distinct.
- Use checked arithmetic everywhere.
- Define ownership and page states.
- Keep PMM and VMM responsibilities separate.
- Zero memory before exposing it to a less-trusted consumer.
- Make allocation context explicit.
- Keep DMA policy in a DMA API.
- Do not overbuild NUMA/zones prematurely.
- Support observability without exposing writable allocator internals.
- Evidence gates capability claims.

## PR Roadmap
| PR | Title |
|---|---|
| 1 | `docs(pmm): audit memory maps and define allocator invariants` |
| 2 | `feat(pmm): implement early boot bump allocator` |
| 3 | `feat(pmm): implement bitmap-based frame allocator` |
| 4 | `feat(pmm): implement buddy allocator for multi-order frames` |
| 5 | `feat(pmm): support reserved hardware memory exclusions` |
| 6 | `feat(pmm): add checked arithmetic and boundary bounds` |
| 7 | `feat(pmm): track frame ownership and page states` |
| 8 | `feat(pmm): implement zero-on-free security semantics` |
| 9 | `feat(pmm): integrate kernel direct map physical access` |
| 10 | `feat(pmm): add lockless per-cpu allocation magazines` |
| 11 | `feat(pmm): support low-memory watermark and reclaim wakeup` |
| 12 | `feat(pmm): provide NUMA topology node awareness` |
| 13 | `feat(pmm): implement DMA32 zone isolation` |
| 14 | `feat(pmm): add memory pressure notification events` |
| 15 | `feat(pmm): implement hotplug memory addition` |
| 16 | `feat(pmm): add frame compaction and defragmentation` |
| 17 | `test(pmm): add memory exhaustion and fuzzing harness` |
| 18 | `feat(pmm): expose procfs and sysfs memory telemetry` |
| 19 | `docs(pmm): publish allocator guide and performance benchmarks` |
| 20 | `release(pmm): certify physical memory manager readiness` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic functionality verified.
- Integrated: Cross-module integration passed.
- Supported: Reproducible evidence with high test coverage.
