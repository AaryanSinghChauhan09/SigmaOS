# SigmaOS Physical Memory Manager Plan

## Architecture Overview
The SigmaOS Physical Memory Manager (PMM) operates in two distinct stages: early boot memblock allocation transitioning to the runtime allocator.

## Rules and Invariants
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

## Proposed PR Sequence
| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
| 1 | `docs(pmm): audit memory maps and boot handoff` | Audit early boot memory mapping | Usable regions validated |
| 2 | `feat(pmm): implement early boot memblock allocator` | Memblock reservation allocator | Early allocations succeed |
| 3 | `feat(pmm): add frame tracking metadata` | Bitmap / struct page frames | Frame state tracked |
| 4 | `feat(pmm): implement buddy allocator` | Order-based physical allocator | Power-of-two blocks |
| 5 | `feat(pmm): add watermarks and page reclaim notifications` | Low watermark triggers | Reclaim daemon notified |
| 6 | `feat(pmm): zero allocated pages before exposure` | Security scrubbing | Zero-filled frames |
| 7 | `feat(pmm): physical address checked arithmetic` | Overflow-safe physical math | Checked math validated |
| 8 | `feat(pmm): define frame ownership contracts` | Kernel vs user ownership | State transitions safe |
| 9 | `feat(pmm): support contiguous DMA buffer allocation` | Contiguous physical ranges | Alignment preserved |
| 10 | `feat(pmm): add memory hotplug stubs` | Hotplug interface | Offline handling safe |
| 11 | `feat(pmm): implement NUMA topology queries` | Node-aware topology queries | Bounded NUMA support |
| 12 | `feat(pmm): per-CPU frame caches` | Local CPU free lists | Lock contention reduced |
| 13 | `feat(pmm): add early boot to runtime allocator transition` | Handover protocol | Memblock retired |
| 14 | `feat(pmm): add page poisoning in debug builds` | Diagnostics sanitization | Use-after-free caught |
| 15 | `feat(pmm): read-only memory statistics interface` | Global and per-zone metrics | Introspection safe |
| 16 | `test(pmm): stress testing and fault injection` | Allocation under OOM pressure | Exhaustion handled |
| 17 | `security(pmm): enforce privilege boundaries` | Kernel address isolation | Protection verified |
| 18 | `feat(pmm): multi-arch physical memory support` | AArch64 and RISC-V mappings | Architecture parity |
| 19 | `docs(pmm): publish physical memory driver guide` | Architecture documentation | Manuals complete |
| 20 | `release(pmm): certify physical memory subsystem` | Full evidence certification | Release criteria met |

## Recommended First Milestone
The recommended first milestone prioritizes PRs 1-9 & 13 to establish a working physical allocator on x86_64 QEMU.

## Periodic Maintenance PR Cadences
- Every PR: Automated frame layout and buddy allocation tests.
- Weekly: Memory leak audit and fragmentation tracking.
- Monthly: NUMA and hotplug regression suite.
- Quarterly: Architecture handoff and boot map audits.
- Each release candidate: Stress test certification under heavy workloads.
- After each incident: Triage and regression test additions.

## Component-Specific Rules
- Frame tracking: Maintain bounds without leaking allocator internal pointers.
- Reclamation: Notify consumers cleanly without hard-blocking.

## PR Review Template & Merge Gates
PRs must include memory state definitions and leak verification.

## Completion Criteria
- Prototype: Modeled buddy allocator and memblock staging.
- Integrated: Booted on QEMU x86_64 passing all frame allocations.
- Supported: Proven on physical hardware across all architectures.
