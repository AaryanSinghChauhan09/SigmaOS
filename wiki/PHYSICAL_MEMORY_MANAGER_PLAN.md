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

## PR Roadmap
| 1 | Baseline memory map audit |
| 2 | Page frame allocator core |
| 3 | Page state tracking |
| 4 | Bitmap allocator implementation |
| 5 | Buddy allocator implementation |
| 6 | Free page scrubbing and zeroing |
| 7 | Allocation flags and context |
| 8 | Physical page refcounting |
| 9 | Reclaim watermarks and notification |
| 10 | Reserved memory regions protection |
| 11 | Early boot bootmem handoff |
| 12 | Contiguous physical memory allocator |
| 13 | Multi-pool allocation policies |
| 14 | Page poison and corruption detection |
| 15 | Lockless page freelists |
| 16 | Cache-coloring page placement |
| 17 | NUMA node physical awareness |
| 18 | Microbenchmarking suite |
| 19 | Memory telemetry and observability |
| 20 | Multi-arch verification and stress testing |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic allocation
- Integrated: Kernel runtime path
- Supported: Production multi-arch evidence
