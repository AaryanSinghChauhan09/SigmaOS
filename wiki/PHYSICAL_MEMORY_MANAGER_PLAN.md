# Physical Memory Manager Plan

## Architecture Invariants
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
| 1 | Boot memory map parser |
| 2 | Early bump allocator |
| 3 | Page frame database initialization |
| 4 | Buddy allocator core |
| 5 | Page order allocation API |
| 6 | Free page tracking and coalescing |
| 7 | Watermark and pressure notification |
| 8 | Zeroing thread / background page zeroing |
| 9 | Multi-zone boundaries (DMA, Normal) |
| 10 | Reserved region enforcement |
| 11 | Non-contiguous memory range joining |
| 12 | Locking and concurrency primitives |
| 13 | NUMA-aware node allocation policy |
| 14 | Page flags and refcounting |
| 15 | Diagnostic inspect counters |
| 16 | Stress test and fragmentation profiler |
| 17 | OOM detection trigger |
| 18 | Microbenchmarks for buddy allocate/free |
| 19 | Architecture support parity |
| 20 | Final documentation and hardening |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic buddy allocator working
- Integrated: Integrated into kernel page fault handler
- Supported: Production evidence on all target platforms
