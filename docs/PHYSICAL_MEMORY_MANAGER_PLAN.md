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
| 1 | PR 1 description |
| 2 | PR 2 description |
| 3 | PR 3 description |
| 4 | PR 4 description |
| 5 | PR 5 description |
| 6 | PR 6 description |
| 7 | PR 7 description |
| 8 | PR 8 description |
| 9 | PR 9 description |
| 10 | PR 10 description |
| 11 | PR 11 description |
| 12 | PR 12 description |
| 13 | PR 13 description |
| 14 | PR 14 description |
| 15 | PR 15 description |
| 16 | PR 16 description |
| 17 | PR 17 description |
| 18 | PR 18 description |
| 19 | PR 19 description |
| 20 | PR 20 description |

## Periodic Cadence
- Every PR: Local verification and linting.
- Weekly: Memory leak check and sanity run.
- Monthly: Stress testing and boundary analysis.
- Quarterly: Architecture review and optimization.
- Each release candidate: Full end-to-end regression.
- After each incident: Root cause analysis and mitigation.

## Completion Criteria
- Prototype: Basic boot allocator functional.
- Integrated: Linked into kernel initialization sequence.
- Supported: Production-grade stress tests passing.
