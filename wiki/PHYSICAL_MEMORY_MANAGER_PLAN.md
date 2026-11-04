# Physical Memory Manager (PMM) Architecture & PR Plan

## Invariants and Rules
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

## PR Roadmap Sequence
| PR | Title |
|---|---|
| 1 | PMM Audit |
| 2 | Memory Map Parser |
| 3 | Page Frame Allocator |
| 4 | Buddy Allocator |
| 5 | Bitmap Tracker |
| 6 | Page State Transitions |
| 7 | Zero Page Pool |
| 8 | DMA Zone Allocator |
| 9 | NUMA Topology |
| 10 | Watermark Metrics |
| 11 | Cache Eviction Interface |
| 12 | OOM Killer Protocol |
| 13 | Lockless Free List |
| 14 | Memory Compaction |
| 15 | Huge Page 2MB/1GB Support |
| 16 | Hotplug Memory Add/Remove |
| 17 | Hardware Error Poisoning |
| 18 | Microbenchmarks |
| 19 | Virtual Machine Memory Ballooning |
| 20 | Production Certification |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype:
- Integrated:
- Supported:
