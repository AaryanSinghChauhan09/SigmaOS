# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 physical memory allocator & page tables
- Linux frame allocator & page compaction
- FreeBSD VM subsystem

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS memory components.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. All physical frame allocators must track exhaustion and leak metrics.

## Prohibited

- Unbounded raw pointer manipulation across boundaries
- Silent ignoring of memory allocation failures
- Bypassing virtual-to-physical address translation safety
- Status promotion without memory compaction or paging runtime evidence

## Required evidence

- Frame allocator & paging unit tests
- Memory exhaustion & leak test reports
- QEMU boot memory validation log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime paging path plus integration tests |
| Hardened | Heap fuzzing, exhaustion stress tests |
| Supported | Reproducible memory boot log |
