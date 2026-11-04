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

## PR Sequence Roadmap
| 1 | Memory map parser |
| 2 | Early physical allocator |
| 3 | Page descriptor frames |
| 4 | Bitmap allocator |
| 5 | Buddy allocator |
| 6 | Free page lists |
| 7 | Allocation flags |
| 8 | Zeroing engine |
| 9 | Highmem support |
| 10 | Watermark tracking |
| 11 | Out-of-memory handler |
| 12 | Fragmentation analysis |
| 13 | NUMA stubbing |
| 14 | DMA buffer integration |
| 15 | Page state validation |
| 16 | Kernel stack allocator |
| 17 | Slab cache bridge |
| 18 | Stress testing |
| 19 | Concurrency locks |
| 20 | Observability interface |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic allocation functional.
- Integrated: Integrated into kernel boot flow.
- Supported: Production tested on x86_64, aarch64, riscv64.
