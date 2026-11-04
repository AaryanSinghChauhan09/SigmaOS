# SigmaOS Physical Memory Manager (PMM) Architecture & PR Plan

## Invariants

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

## PR Sequence Roadmap

| 1 | PR 1 |
| 2 | PR 2 |
| 3 | PR 3 |
| 4 | PR 4 |
| 5 | PR 5 |
| 6 | PR 6 |
| 7 | PR 7 |
| 8 | PR 8 |
| 9 | PR 9 |
| 10 | PR 10 |
| 11 | PR 11 |
| 12 | PR 12 |
| 13 | PR 13 |
| 14 | PR 14 |
| 15 | PR 15 |
| 16 | PR 16 |
| 17 | PR 17 |
| 18 | PR 18 |
| 19 | PR 19 |
| 20 | PR 20 |

## Maintenance Cadence

- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria

- Prototype: Basic allocator models and unit tests.
- Integrated: Kernel boot allocation and frame reclamation.
- Supported: Production memory management with stress test verification.
