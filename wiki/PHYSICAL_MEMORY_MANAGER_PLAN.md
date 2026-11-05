# SigmaOS Physical Memory Manager (PMM) Master Architecture, Support Matrix, and PR Plan

## Overview
Comprehensive plan for SigmaOS Physical Memory Manager (PMM) architecture, page frame management, and allocation contracts.

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

## PR Sequence Matrix
| PR | Title |
|---|---|
| 1 | `docs(pmm): audit memory map parsers and boot handoff protocols` |
| 2 | `feat(pmm): implement bitmask and bitmap page allocator contract` |
| 3 | `feat(pmm): physical and virtual address boundary enforcement` |
| 4 | `feat(pmm): checked arithmetic and range validation framework` |
| 5 | `feat(pmm): explicit page ownership tracking and state machine` |
| 6 | `feat(pmm): separation of PMM frame tracking from VMM page tables` |
| 7 | `feat(pmm): security zeroing on frame allocation and recycling` |
| 8 | `feat(pmm): allocation context descriptors with GFP-style flags` |
| 9 | `feat(pmm): DMA buffer contiguous allocation and bounce buffer API` |
| 10 | `feat(pmm): memory zone abstraction for low-memory compatibility` |
| 11 | `feat(pmm): buddy allocator hierarchy for multi-order allocations` |
| 12 | `feat(pmm): memory hotplug infrastructure and sparse memory models` |
| 13 | `feat(pmm): read-only observability and memory statistics exporters` |
| 14 | `feat(pmm): memory compaction and fragmentation avoidance engine` |
| 15 | `feat(pmm): NUMA node distance topology and local allocation bias` |
| 16 | `feat(pmm): watermarks, background reclamation, and kswapd triggers` |
| 17 | `feat(pmm): per-CPU page frame caches for lock-free fast paths` |
| 18 | `feat(pmm): poison pattern memory sanitization on release` |
| 19 | `test(pmm): comprehensive physical memory stress, exhaustion, and fuzz suite` |
| 20 | `docs(pmm): publish physical memory architecture guide and support matrix` |

## Maintenance Cadence
- Every PR: run fast unit tests
- Weekly: review pending PRs and metrics
- Monthly: sync architecture roadmap
- Quarterly: full security audit
- Each release candidate: verify all tiers
- After each incident: conduct postmortem and update rules

## Completion Criteria
- Prototype: unit tests and initial flow
- Integrated: runtime path and QEMU evidence
- Supported: reproducible production verification
